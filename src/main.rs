use clap::{App, Arg};
use std::fs::File;
use std::io::Read;

use mlua::Lua;

use rt_weekend::camera::Camera;
use rt_weekend::geometry::{BVHNode, Hittable, HittableList};
use rt_weekend::{output_png, output_window};

fn run_lua(script: &String) -> mlua::Result<()> {
    let lua = Lua::new();

    // Load contex
    let globals = lua.globals();

    // Add render function
    let render = lua.create_function(
        |_,
         (width, height, samples, camera, hitlist, output_path): (
            u32,
            u32,
            u32,
            Camera,
            Vec<Box<dyn Hittable>>,
            Option<String>,
        )| {
            let world: Box<dyn Hittable> = if hitlist.len() > 10 {
                Box::new(BVHNode::new(hitlist))
            } else {
                Box::new(HittableList::from_vec(hitlist))
            };

            match output_path {
                Some(path) => output_png(width, height, samples, &camera, &world, &path),
                None => output_window(width as usize, height as usize, &camera, &world),
            }

            Ok(())
        },
    )?;

    globals.set("render", render)?;

    // Run script
    lua.load(script).exec()
}

fn main() -> mlua::Result<()> {
    let matches = App::new("rt_weekend")
        .version("1.0")
        .author("fanciful-marmot")
        .about("A ray tracer written in Rust")
        .arg(
            Arg::with_name("scene")
                .index(1)
                .help(".lua file describing the scene to render")
                .required(true),
        )
        .get_matches();

    let file_path = matches.value_of("scene").unwrap();

    // Read the script file
    let mut script = String::new();
    let mut script_file = File::open(&file_path).expect("could not open script");
    script_file
        .read_to_string(&mut script)
        .expect("could not read script");

    // Run script
    run_lua(&script)
}
