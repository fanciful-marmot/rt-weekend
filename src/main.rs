use clap::{App, Arg};
use std::fs::File;
use std::io::Read;

use rlua::Lua;

use rt_weekend::camera::Camera;
use rt_weekend::geometry::{BVHNode, Hittable, HittableList};
use rt_weekend::math::Vec3;
use rt_weekend::{output_png, output_window};

fn run_lua(script: &String) -> rlua::Result<()> {
    let lua = Lua::new();

    // Load contex
    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        // Add vec3
        let vec3_constr =
            lua_ctx.create_function(|_, (x, y, z): (f32, f32, f32)| Ok(Vec3::new(x, y, z)))?;
        globals.set("vec3", vec3_constr)?;

        // Add camera constructor
        let camera_constr = lua_ctx.create_function(
            |_, (look_from, look_at, v_up, v_fov, aspect): (Vec3, Vec3, Vec3, f32, f32)| {
                Ok(Camera::new(&look_from, &look_at, &v_up, v_fov, aspect))
            },
        )?;
        globals.set("camera", camera_constr)?;

        // Add render function
        let render = lua_ctx.create_function(
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

        Ok(())
    })?;

    // Run script
    lua.context(|lua_ctx| lua_ctx.load(script).exec())
}

fn main() -> rlua::Result<()> {
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
