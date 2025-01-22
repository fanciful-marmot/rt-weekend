use clap::Parser;

use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::Path;

use minifb::{Key, Window, WindowOptions};

use rand::Rng;

use rhai::packages::Package;
use rhai::Engine;
use rhai_rand::RandomPackage;

use rt::camera::Camera;
use rt::geometry::{BVHNode, Hittable, HittableList, Sphere};
use rt::material::Material;
use rt::math::Vec3;
use rt::{cast_ray, output_buffer};

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
    scene: &Box<dyn Hittable>,
    output_path: &str,
) {
    let data = output_buffer(width, height, samples, camera, scene, &|_v: Vec<u8>| {});

    write_png(output_path, width, height, &data);
}

pub fn output_window(width: usize, height: usize, camera: &Camera, scene: &Box<dyn Hittable>) {
    const MAX_SAMPLES: u32 = 100;
    let mut rng = rand::thread_rng();

    let mut screen_buffer: Vec<u32> = vec![0; width * height];
    let mut data: Vec<f32> = vec![0.0; width * height * 3]; // Store the sum of each pass. Each channel is a f32

    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let mut count = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if count < MAX_SAMPLES {
            // Accumulate samples in the data buffer
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
                    let ray = camera.get_ray(u, v);
                    color += cast_ray(ray, scene, 0);

                    // Acculumate colors
                    data[i] += color.x;
                    data[i + 1] += color.y;
                    data[i + 2] += color.z;
                }
            }

            // Update sample count
            count += 1;

            // Write data buffer into screen buffer
            for i in 0..screen_buffer.len() {
                let r = ((data[i * 3] / count as f32).sqrt() * 255.99) as u32;
                let g = ((data[i * 3 + 1] / count as f32).sqrt() * 255.99) as u32;
                let b = ((data[i * 3 + 2] / count as f32).sqrt() * 255.99) as u32;

                screen_buffer[i] = 255 << 24 | r << 16 | g << 8 | b;
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&screen_buffer, width, height)
            .unwrap();
    }
}

fn run_script(script: &str, window: bool) -> Result<(), Box<rhai::EvalAltResult>> {
    let mut engine = Engine::new();
    engine
        .build_type::<Vec3>()
        .build_type::<Camera>()
        .build_type::<Material>()
        .build_type::<Sphere>()
        .register_fn(
            "render",
            move |w: i64, h: i64, s: i64, c: Camera, scene: rhai::Array, p: &str| {
                let mut list: Vec<Box<dyn Hittable>> = Vec::new();
                for sphere in scene.iter() {
                    match sphere.clone().try_cast::<Sphere>() {
                        Some(s) => list.push(Box::new(s)),
                        None => (),
                    }
                }

                let world: Box<dyn Hittable> = if list.len() > 10 {
                    Box::new(BVHNode::new(list))
                } else {
                    Box::new(HittableList::from_vec(list))
                };

                if window {
                    output_window(w as usize, h as usize, &c, &world);
                } else {
                    output_png(w as u32, h as u32, s as u32, &c, &world, p);
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
    let result = run_script(&script, args.window);

    match result {
        Ok(()) => println!("Done!"),
        Err(e) => println!("Failed: {}", e),
    }
}
