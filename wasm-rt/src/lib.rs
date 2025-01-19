use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{console, CanvasRenderingContext2d, ImageData};

use rhai::packages::Package;
use rhai::Engine;
use rhai_rand::RandomPackage;

use rt::camera::Camera;
use rt::geometry::{BVHNode, Hittable, HittableList, Sphere};
use rt::material::Material;
use rt::math::Vec3;
use rt::output_buffer;

#[wasm_bindgen]
pub fn render(script: &str, ctx: CanvasRenderingContext2d) {
    console::log_1(&"Building engine...".into());
    let mut engine = Engine::new();
    engine
        .build_type::<Vec3>()
        .build_type::<Camera>()
        .build_type::<Material>()
        .build_type::<Sphere>()
        .register_fn(
            "render",
            move |w: i64, h: i64, s: i64, c: Camera, scene: rhai::Array, _p: &str| {
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

                console::log_1(&"Rendering...".into());
                let width = w as u32;
                let height = h as u32;
                // Data MUST be in RGBA format
                let data = output_buffer(width, height, s as u32, &c, &world);
                console::log_1(&"Gathering image data...".into());

                if let Ok(image_data) =
                    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), 400, 200)
                {
                    console::log_1(&"Writing to canvas...".into());
                    if let Err(_e) = ctx.put_image_data(&image_data, 0.0, 0.0) {
                        console::log_1(&"Failed to write to canvas".into());
                    }
                } else {
                    console::log_1(&"Failed to gather image data".into());
                }
            },
        );

    // Add RNG support
    let random = RandomPackage::new();
    random.register_into_engine(&mut engine);

    console::log_1(&"Running script...".into());
    if let Err(e) = engine.eval::<()>(&script) {
        println!("{}", e);
    }
    console::log_1(&"Script done".into());
}
