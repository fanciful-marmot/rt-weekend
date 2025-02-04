use clap::Parser;

use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use minifb::{Key, Window, WindowOptions};

use rand::Rng;

use rhai::packages::Package;
use rhai::Engine;
use rhai_rand::RandomPackage;

use rt::camera::Camera;
use rt::geometry::{BVHNode, Hittable, HittableList, Sphere};
use rt::material::Material;
use rt::math::Vec3;
use rt::{cast_ray, f32_buf_to_u8, output_buffer};

// Writes a u8 data buffer in RGBA format to a png file
fn write_png(file_name: &str, width: u32, height: u32, data: &[u8]) {
    let path = Path::new(file_name);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap(); // Save
}

pub fn output_png(
    width: u32,
    height: u32,
    samples: u32,
    camera: &Camera,
    scene: Vec<Sphere>,
    skybox_scale: f32,
    threads: u32,
    output_path: &str,
) {
    let mutex = Arc::new(Mutex::new(vec![0.0f32; (width * height * 3) as usize]));

    let mut handles = vec![];

    let samples_per_thread = (samples / threads).max(1);

    for _ in 0..threads {
        let thrd_mutex = Arc::clone(&mutex);
        let thrd_camera = camera.clone();
        let thrd_scene = scene.clone();
        let handle = thread::spawn(move || {
            let box_list: Vec<Box<dyn Hittable>> = thrd_scene
                .iter()
                .map(|s| Box::new(s.clone()) as Box<dyn Hittable>)
                .collect();
            let world: Box<dyn Hittable> = if box_list.len() > 10 {
                Box::new(BVHNode::new(box_list))
            } else {
                Box::new(HittableList::from_vec(box_list))
            };

            let data = output_buffer(
                width,
                height,
                samples_per_thread,
                &thrd_camera,
                &world,
                skybox_scale,
                &|_v: &Vec<f32>, _s: f32| {},
            );

            {
                let mut shared = thrd_mutex.lock().unwrap();
                for (i, v) in data.iter().enumerate() {
                    (*shared)[i] += v;
                }
            }
        });
        handles.push(handle);
    }

    // Join the threads
    for handle in handles {
        handle.join().unwrap();
    }

    let shared = mutex.lock().unwrap();
    let data = f32_buf_to_u8(&(*shared), (samples_per_thread * threads) as f32);
    write_png(output_path, width, height, &data);
}

pub fn output_window(
    width: usize,
    height: usize,
    camera: &Camera,
    scene: Vec<Sphere>,
    skybox_scale: f32,
    threads: u32,
) {
    const MAX_SAMPLES: u32 = 400;
    let post_every: u32 = 10;

    let mut screen_buffer: Vec<u32> = vec![0; width * height];

    // Accumulation buffer
    let mutex = Arc::new(Mutex::new((
        vec![0.0f32; (width * height * 3) as usize],
        0.0f32,
    ))); // Store the sum of each pass. Each channel is a f32

    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~1 fps update rate
    window.set_target_fps(1);

    // Launch threads
    let mut handles = vec![];
    let samples_per_thread = (MAX_SAMPLES / threads).max(1);
    for id in 0..threads {
        let thrd_mutex = Arc::clone(&mutex);
        let thrd_camera = camera.clone();
        let thrd_scene = scene.clone();
        let handle = thread::spawn(move || {
            let box_list: Vec<Box<dyn Hittable>> = thrd_scene
                .iter()
                .map(|s| Box::new(s.clone()) as Box<dyn Hittable>)
                .collect();
            let world: Box<dyn Hittable> = if box_list.len() > 10 {
                Box::new(BVHNode::new(box_list))
            } else {
                Box::new(HittableList::from_vec(box_list))
            };

            let mut rng = rand::thread_rng();

            let mut data = vec![0.0f32; (width * height * 3) as usize];
            let mut count: u32 = 0;

            for s in 0..samples_per_thread {
                // Accumulate samples in the thread's data buffer
                for y in 0..height {
                    for x in 0..width {
                        // Get pixel index in array
                        let i = ((y * width + x) * 3) as usize;
                        // Cast rays
                        let mut color = Vec3::new(0.0, 0.0, 0.0);
                        // Get uv coordinate. Flipping y because of encoding order in PNG
                        // Jitter the ray by a random amount
                        let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                        let v = ((height - y) as f32 + rng.gen::<f32>()) / height as f32;
                        let ray = thrd_camera.get_ray(u, v);
                        color += cast_ray(ray, &world, skybox_scale, 0);

                        // Acculumate colors
                        data[i] += color.x;
                        data[i + 1] += color.y;
                        data[i + 2] += color.z;
                    }
                }
                count += 1;

                if s % post_every == id {
                    println!("thrd {}: posting {}", id, s);
                    // Update shared buffer
                    {
                        let mut shared = thrd_mutex.lock().unwrap();
                        for (i, v) in data.iter().enumerate() {
                            (*shared.0)[i] += v;
                        }
                        (*shared).1 += count as f32;
                    }
                    // Clear buffers
                    count = 0;
                    data.fill(0.0);
                }
            }

            // Update shared buffer one last time
            if count > 0 {
                let mut shared = thrd_mutex.lock().unwrap();
                for (i, v) in data.iter().enumerate() {
                    (*shared.0)[i] += v;
                }
                (*shared).1 += count as f32;
            }
        });
        handles.push(handle);
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let samples: f32;
        let data: Vec<f32>;
        {
            // TODO: Read/write lock instead of mutex?
            let shared = mutex.lock().unwrap();
            data = shared.0.clone();
            samples = shared.1;
        }
        println!("main: samples {}", samples);

        // Write data buffer into screen buffer
        for i in 0..screen_buffer.len() {
            let r = (((data[i * 3] / samples).sqrt() * 255.99) as u32).min(255);
            let g = (((data[i * 3 + 1] / samples).sqrt() * 255.99) as u32).min(255);
            let b = (((data[i * 3 + 2] / samples).sqrt() * 255.99) as u32).min(255);

            screen_buffer[i] = 255 << 24 | r << 16 | g << 8 | b;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&screen_buffer, width, height)
            .unwrap();
    }
}

fn run_script(script: &str, window: bool, threads: u32) -> Result<(), Box<rhai::EvalAltResult>> {
    let mut engine = Engine::new();
    engine
        .build_type::<Vec3>()
        .build_type::<Camera>()
        .build_type::<Material>()
        .build_type::<Sphere>()
        .register_fn(
            "render",
            move |w: i64,
                  h: i64,
                  s: i64,
                  c: Camera,
                  scene: rhai::Array,
                  skybox_scale: f32,
                  p: &str| {
                let mut list: Vec<Sphere> = Vec::new();
                for sphere in scene.iter() {
                    match sphere.clone().try_cast::<Sphere>() {
                        Some(s) => list.push(s),
                        None => (),
                    }
                }

                if window {
                    output_window(w as usize, h as usize, &c, list, skybox_scale, threads);
                } else {
                    output_png(
                        w as u32,
                        h as u32,
                        s as u32,
                        &c,
                        list,
                        skybox_scale,
                        threads,
                        p,
                    );
                }
            },
        );

    // Add RNG support
    let random = RandomPackage::new();
    random.register_into_engine(&mut engine);

    engine.eval::<()>(&script)
}

#[derive(Parser, Debug)]
#[command(name = "rt_weekend")]
#[command(version = "1.0")]
#[command(author = "fanciful-marmot")]
#[command(about = "A ray tracer written in Rust", long_about = None)]
struct Args {
    /// .rhai file describing the scene to render
    #[arg(short, long)]
    scene: String,

    /// Output incrementally to window instead
    #[arg(short, long)]
    window: bool,

    /// How many threads to use
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..=8))]
    threads: u32,
}

fn main() {
    let args = Args::parse();

    let file_path = args.scene;

    // Read the script file
    let mut script = String::new();
    let mut script_file = File::open(&file_path).expect("could not open script");
    script_file
        .read_to_string(&mut script)
        .expect("could not read script");

    // Run script
    let result = run_script(&script, args.window, args.threads);

    match result {
        Ok(()) => println!("Done!"),
        Err(e) => println!("Failed: {}", e),
    }
}
