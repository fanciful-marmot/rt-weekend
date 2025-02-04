use wasm_bindgen::prelude::*;
use web_sys::{console, js_sys};

use rhai::packages::Package;
use rhai::Engine;
use rhai_rand::RandomPackage;

use rt::camera::Camera;
use rt::geometry::{BVHNode, Hittable, HittableList, Sphere};
use rt::material::Material;
use rt::math::Vec3;
use rt::{f32_buf_to_u8, output_buffer};

#[wasm_bindgen]
pub fn render(script: &str, on_progress: js_sys::Function) {
    console::log_1(&"Building engine...".into());
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
                  _p: &str| {
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

                let p = |data: &Vec<f32>, s: f32| {
                    let this = JsValue::null();
                    let du8 = f32_buf_to_u8(&data, s);
                    let _ = on_progress.call1(&this, &JsValue::from(du8.as_ptr()));
                };

                // Data MUST be in RGBA format
                let data = output_buffer(width, height, s as u32, &c, &world, skybox_scale, &p);
                p(&data, s as f32);
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
