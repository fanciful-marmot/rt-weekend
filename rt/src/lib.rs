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
pub fn cast_ray(ray: Ray, world: &Box<dyn Hittable>, depth: u32) -> Vec3 {
    // 0.0001 is to  avoid reintersecting the same object on bounces
    let hit = world.intersects_ray(&ray, (0.001, f32::MAX));

    let color = match hit {
        None => {
            let unit_direction = ray.direction.make_unit();
            let t = 0.5 * (unit_direction.y + 1.0);

            // Lerp blue and white
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
        Some(hit) => {
            if depth < MAX_DEPTH {
                match hit.material.scatter(&ray, &hit) {
                    Some(scatter) => scatter.attenuation * cast_ray(scatter.ray, world, depth + 1),
                    None => Vec3::new(0.0, 0.0, 0.0),
                }
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
    };

    color
}

pub fn output_buffer(
    width: u32,
    height: u32,
    samples: u32,
    camera: &Camera,
    scene: &Box<dyn Hittable>,
) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let size = (width * height * 4) as usize;
    let mut data: Vec<u8> = vec![0; size];
    for y in 0..height {
        for x in 0..width {
            // Get pixel index in array
            let i = ((y * width + x) * 4) as usize;

            // Cast rays
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                // Get uv coordinate. Flipping y because of encoding order in PNG
                // Jitter the ray by a random amount
                let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                let v = ((height - y) as f32 + rng.gen::<f32>()) / height as f32;
                let ray = camera.get_ray(u, v);

                color += cast_ray(ray, &scene, 0);
            }
            color /= samples as f32;

            // Gamma correction.
            // TODO: Is it needed?
            color.set(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());

            // Write colour value as u8 into buffer
            let (r, g, b) = color.as_rgb();
            data[i] = r;
            data[i + 1] = g;
            data[i + 2] = b;
            data[i + 3] = 255;
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
            .with_fn("dielectric", Material::new_dielectric);
    }
}

impl rhai::CustomType for Sphere {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder.with_name("Sphere").with_fn("sphere", Sphere::new);
    }
}
