use clap::Parser;
use std::fs::File;
use std::io::Read;

use rhai::packages::Package;
use rhai::Engine;
use rhai_rand::RandomPackage;

use rt_weekend::camera::Camera;
use rt_weekend::geometry::{BVHNode, Hittable, HittableList, Sphere};
use rt_weekend::material::Material;
use rt_weekend::math::Vec3;
use rt_weekend::{output_png, output_window};

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
