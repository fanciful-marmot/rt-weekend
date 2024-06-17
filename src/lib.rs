use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use minifb::{Key, Window, WindowOptions};
use mlua::{Error, FromLua, Lua, Result, UserData, Value};
use rand::Rng;

pub mod camera;
pub mod geometry;
pub mod material;
pub mod math;

use camera::Camera;
use geometry::{Hittable, Sphere};
use material::Material;
use math::{Ray, Vec3};

const MAX_DEPTH: u32 = 16;

// Writes a u8 data buffer in RGBA format to a png file
fn write_png(file_name: &str, width: u32, height: u32, data: &[u8]) {
    let path = Path::new(file_name);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap(); // Save
}

// Take ownership of the ray so it can be dropped sooner
fn cast_ray(ray: Ray, world: &Box<dyn Hittable>, depth: u32) -> Vec3 {
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

pub fn output_png(
    width: u32,
    height: u32,
    samples: u32,
    camera: &Camera,
    scene: &Box<dyn Hittable>,
    output_path: &str,
) {
    let mut rng = rand::thread_rng();
    let size = (width * height * 3) as usize;
    let mut data: Vec<u8> = vec![0; size];
    for y in 0..height {
        for x in 0..width {
            // Get pixel index in array
            let i = ((y * width + x) * 3) as usize;

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
        }
    }

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
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

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

// Lua bindings
fn conversion_error<T>(from: &'static str, to: &'static str, reason: &str) -> Result<T> {
    Err(Error::FromLuaConversionError {
        from,
        to,
        message: Some(String::from(reason)),
    })
}

impl UserData for Vec3 {}
impl UserData for Camera {}

impl<'lua> FromLua<'lua> for Vec3 {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> Result<Self> {
        match value {
            Value::Table(table) => {
                let x: f32 = table.get("x")?;
                let y: f32 = table.get("y")?;
                let z: f32 = table.get("z")?;

                Ok(Vec3::new(x, y, z))
            }
            _ => conversion_error("Value", "Vec3", "expected table"),
        }
    }
}

impl<'lua> FromLua<'lua> for Camera {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> Result<Self> {
        match value {
            Value::Table(table) => {
                let look_from: Vec3 = table.get("look_from")?;
                let look_at: Vec3 = table.get("look_at")?;
                let v_up: Vec3 = table.get("v_up")?;
                let v_fov: f32 = table.get("v_fov")?;
                let aspect: f32 = table.get("aspect")?;

                Ok(Camera::new(&look_from, &look_at, &v_up, v_fov, aspect))
            }
            _ => conversion_error("Value", "Vec3", "expected table"),
        }
    }
}

impl<'lua> FromLua<'lua> for Material {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        match value {
            Value::Table(table) => {
                let material_type: String = table.get("type")?;
                match &material_type[..] {
                    "lambertian" => Ok(Material::new_lambertian(table.get("albedo")?)),
                    "metal" => Ok(Material::new_metal(
                        Vec3::from_lua(table.get("albedo")?, lua)?,
                        f32::from_lua(table.get("roughness")?, lua)?,
                    )),
                    "dielectric" => Ok(Material::new_dielectric(f32::from_lua(
                        table.get("refractive_index")?,
                        lua,
                    )?)),
                    _ => conversion_error(
                        "Value",
                        "Material",
                        &format!("unknown material type {}", material_type)[..],
                    ),
                }
            }
            _ => conversion_error("Value", "Material", "expected table"),
        }
    }
}

impl<'lua> FromLua<'lua> for Sphere {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        match value {
            Value::Table(table) => {
                let position = Vec3::from_lua(table.get("position")?, lua)?;
                let radius = table.get("radius")?;
                let material = Material::from_lua(table.get("material")?, lua)?;

                Ok(Sphere::new(position, radius, material))
            }
            _ => conversion_error("Value", "Material", "expected table"),
        }
    }
}

// Generate hittable list from Lua table
impl<'lua> FromLua<'lua> for Box<dyn Hittable> {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        match &value {
            Value::Table(table) => {
                let object_type = String::from_lua(table.get("type")?, lua)?;
                let result = match &object_type[..] {
                    "sphere" => Sphere::from_lua(value, lua),
                    _ => conversion_error(
                        "Value",
                        "dyn Hittable",
                        &format!("Unknown type {}", object_type)[..],
                    ),
                };

                Ok(Box::new(result.unwrap()))
            }
            _ => Err(Error::FromLuaConversionError {
                from: "Value",
                to: "HittableList",
                message: Some(String::from(
                    "table must be a sequence with at least one object",
                )),
            }),
        }
    }
}
