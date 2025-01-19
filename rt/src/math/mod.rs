pub mod ray;
pub mod vec3;

pub use self::ray::Ray;
pub use self::vec3::Vec3;

use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let v111 = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - v111;
        if p.length_sq() < 1.0 {
            break p;
        }
    }
}

// pub fn random_unit_vector() -> Vec3 {
//     let mut rng = rand::thread_rng();
//     let a = rng.gen_range::<f32>(0.0, std::f32::consts::PI * 2.0);
//     let z = rng.gen_range::<f32>(-1.0, 1.0);
//     let r = (1.0 - z * z).sqrt();

//     Vec3::new(r * a.cos(), r * a.sin(), z)
// }

// pub fn random_in_hemisphere(n: &Vec3) -> Vec3 {
//     let in_unit_sphere = random_in_unit_sphere();
//     if in_unit_sphere.dot(&n) > 0.0 {
//         in_unit_sphere
//     } else {
//         -in_unit_sphere
//     }
// }

pub fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 *= r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
