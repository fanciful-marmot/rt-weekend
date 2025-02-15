pub mod camera;
pub mod geometry;
pub mod material;
pub mod math;

use camera::Camera;
use geometry::{Hittable, Sphere};
use material::Material;
use math::{Ray, Vec3};
use rand::Rng;

const MAX_DEPTH: u32 = 16;

// Take ownership of the ray so it can be dropped sooner
pub fn cast_ray(ray: Ray, world: &Box<dyn Hittable>, skybox_scale: f32, depth: u32) -> Vec3 {
    // 0.0001 is to  avoid reintersecting the same object on bounces
    let hit = world.intersects_ray(&ray, (0.001, f32::MAX));

    let color = match hit {
        None => {
            let unit_direction = ray.direction.make_unit();
            let t = 0.5 * (unit_direction.y + 1.0);

            // Lerp blue and white, scale down/up for general brightness
            // TODO: Could be fancier
            skybox_scale * ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
        }
        Some(hit) => {
            if depth < MAX_DEPTH {
                match hit.material.scatter(&ray, &hit) {
                    Some(scatter) => {
                        scatter.attenuation
                            * match scatter.ray {
                                Some(ray) => cast_ray(ray, world, skybox_scale, depth + 1),
                                None => Vec3::new_uniform(1.0),
                            }
                    }
                    None => Vec3::new(0.0, 0.0, 0.0),
                }
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
    };

    color
}

// Converts an f32 buffer to u8.
// f32 values are clamped to [0, 1], gamma corrected, and then mapped to [0, 255]
pub fn f32_buf_to_u8(fb: &[f32]) -> Vec<u8> {
    let pixels = fb.len() / 3;
    let mut vu8: Vec<u8> = vec![0; pixels * 4];
    for i in 0..pixels {
        let vu8i = i * 4;
        let fbi = i * 3;
        // Gamma correction.
        vu8[vu8i] = (fb[fbi].clamp(0.0, 1.0).sqrt() * 255.99) as u8;
        vu8[vu8i + 1] = (fb[fbi + 1].clamp(0.0, 1.0).sqrt() * 255.99) as u8;
        vu8[vu8i + 2] = (fb[fbi + 2].clamp(0.0, 1.0).sqrt() * 255.99) as u8;
        vu8[vu8i + 3] = 255;
    }

    vu8
}

pub fn output_buffer(
    width: u32,
    height: u32,
    samples: u32,
    camera: &Camera,
    scene: &Box<dyn Hittable>,
    skybox_scale: f32, // TODO: Make this a proper skybox/gradient control
    on_progress: &impl Fn(&Vec<f32>, f32),
) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    let size = (width * height * 3) as usize;
    let mut data: Vec<f32> = vec![0.0; size];
    for s in 0..samples {
        for y in 0..height {
            for x in 0..width {
                // Get pixel index in array
                let i = ((y * width + x) * 3) as usize;

                // Cast ray
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                // Get uv coordinate. Flipping y because of encoding order in PNG
                // Jitter the ray by a random amount
                let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                let v = ((height - y) as f32 + rng.gen::<f32>()) / height as f32;
                let ray = camera.get_ray(u, v);

                color += cast_ray(ray, &scene, skybox_scale, 0);

                // Write colour value into buffer
                data[i] += color.x;
                data[i + 1] += color.y;
                data[i + 2] += color.z;
            }
        }

        // Send results every 10 samples
        if s % 10 == 0 {
            on_progress(&data, (s + 1) as f32);
        }
    }

    data
}

// Rhai bindings

impl rhai::CustomType for Vec3 {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name("Vec3")
            .with_fn("vec3", Self::new)
            // Indexer get/set functions that do not panic on invalid indices
            .with_indexer_get_set(
                |vec: &mut Self, idx: i64| -> Result<f32, Box<rhai::EvalAltResult>> {
                    match idx {
                        0 => Ok(vec.x),
                        1 => Ok(vec.y),
                        2 => Ok(vec.z),
                        _ => Err(rhai::EvalAltResult::ErrorIndexNotFound(
                            idx.into(),
                            rhai::Position::NONE,
                        )
                        .into()),
                    }
                },
                |vec: &mut Self, idx: i64, value: f32| -> Result<(), Box<rhai::EvalAltResult>> {
                    match idx {
                        0 => vec.x = value,
                        1 => vec.y = value,
                        2 => vec.z = value,
                        _ => {
                            return Err(rhai::EvalAltResult::ErrorIndexNotFound(
                                idx.into(),
                                rhai::Position::NONE,
                            )
                            .into())
                        }
                    }
                    Ok(())
                },
            );
    }
}

impl rhai::CustomType for Camera {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder.with_name("Camera").with_fn("camera", Self::new);
    }
}

impl rhai::CustomType for Material {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name("Material")
            .with_fn("lambertian", Material::new_lambertian)
            .with_fn("metal", Material::new_metal)
            .with_fn("dielectric", Material::new_dielectric)
            .with_fn("emissive", Material::new_emissive);
    }
}

impl rhai::CustomType for Sphere {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder.with_name("Sphere").with_fn("sphere", Sphere::new);
    }
}
