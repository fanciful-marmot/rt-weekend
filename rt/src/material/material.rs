use crate::geometry::Hit;
use crate::math::{random_in_unit_sphere, schlick, Ray, Vec3};
use rand::Rng;

pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Option<Ray>,
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<Scatter> {
        // Diffuse bounce (uniformly random direction)
        Some(Scatter {
            ray: Some(Ray {
                origin: hit.point,
                direction: hit.normal + random_in_unit_sphere(),
                // direction: hit.normal + random_unit_vector(),
                // direction: hit.normal + random_in_hemisphere(&hit.normal),
            }),
            attenuation: self.albedo,
        })
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
    roughness: f32,
}

impl Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = Vec3::reflect(&ray.direction.make_unit(), &hit.normal)
            + self.roughness * random_in_unit_sphere();

        if reflected.dot(&hit.normal) > 0.0 {
            Some(Scatter {
                attenuation: self.albedo.clone(),
                ray: Some(Ray {
                    origin: hit.point.clone(),
                    direction: reflected,
                }),
            })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = Vec3::reflect(&ray.direction, &hit.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let ni_over_nt;
        let outward_normal;
        let cosine;
        if ray.direction.dot(&hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.refraction_index;
            cosine =
                self.refraction_index * ray.direction.dot(&hit.normal) / ray.direction.length();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.refraction_index;
            cosine = -ray.direction.dot(&hit.normal) / ray.direction.length();
        }

        let direction = match Vec3::refract(&ray.direction, &outward_normal, ni_over_nt) {
            Some(refracted) => {
                if rand::thread_rng().gen::<f32>() < schlick(cosine, self.refraction_index) {
                    reflected
                } else {
                    refracted
                }
            }
            None => reflected,
        };

        Some(Scatter {
            attenuation,
            ray: Some(Ray {
                origin: hit.point.clone(),
                direction,
            }),
        })
    }
}

#[derive(Copy, Clone)]
pub struct Emissive {
    emittance: Vec3,
}

impl Emissive {
    fn scatter(&self, _: &Ray, _: &Hit) -> Option<Scatter> {
        Some(Scatter {
            ray: None,
            attenuation: self.emittance,
        })
    }
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    Emissive(Emissive),
}

impl Material {
    pub fn new_lambertian(albedo: Vec3) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }

    pub fn new_metal(albedo: Vec3, roughness: f32) -> Material {
        Material::Metal(Metal { albedo, roughness })
    }

    pub fn new_dielectric(refraction_index: f32) -> Material {
        Material::Dielectric(Dielectric { refraction_index })
    }

    pub fn new_emissive(emittance: Vec3) -> Material {
        Material::Emissive(Emissive { emittance })
    }

    pub fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        match hit.material {
            Material::Lambertian(l) => l.scatter(ray, hit),
            Material::Metal(m) => m.scatter(ray, hit),
            Material::Dielectric(m) => m.scatter(ray, hit),
            Material::Emissive(m) => m.scatter(ray, hit),
        }
    }
}
