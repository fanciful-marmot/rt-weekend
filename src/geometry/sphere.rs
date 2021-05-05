use crate::geometry::{Hit, Hittable, AABB};
use crate::material::Material;
use crate::math::{Ray, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub material: Material,
    pub radius: f32,
    aabb: AABB,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        let r3 = Vec3::new(radius, radius, radius);
        Sphere {
            center,
            radius,
            material,
            aabb: AABB {
                min: center - r3,
                max: center + r3,
            },
        }
    }
}

impl Hittable for Sphere {
    fn intersects_ray(&self, ray: &Ray, t_range: (f32, f32)) -> Option<Hit> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut t = (-b - discriminant.sqrt()) / a;
            if t < t_range.0 || t > t_range.1 {
                // t was out of range, try the other t
                t = (-b + discriminant.sqrt()) / a;
            }
            if t > t_range.0 && t < t_range.1 {
                // t was in range
                let point = ray.point_at_parameter(t);
                let normal = (point - self.center).make_unit();

                return Some(Hit {
                    t,
                    point,
                    normal,
                    material: &self.material,
                });
            }
        }

        None
    }

    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.aabb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersects_ray() {
        // unit sphere
        let mat = Material::new_lambertian(Vec3::new(0.8, 0.8, 0.8));
        let sphere = Sphere::new(Vec3::new_zeroes(), 1.0, mat);

        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -2.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        // Ray should hit front of sphere in t range [0, 100]
        let hit = sphere.intersects_ray(&ray, (0.0, 100.0)).unwrap();
        assert_eq!(hit.point, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(hit.t, 1.0);

        // Ray should hit back of sphere in t range [1.5, 100]
        let hit = sphere.intersects_ray(&ray, (1.5, 100.0)).unwrap();
        assert_eq!(hit.point, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(hit.t, 3.0);

        // Ray should miss in t range [-100, 0] (facing backwards)
        assert!(sphere.intersects_ray(&ray, (-100.0, 0.0)).is_none());

        // Ray should miss in t range [3.1, 100.0] (starting after the sphere)
        assert!(sphere.intersects_ray(&ray, (-100.0, 0.0)).is_none());

        // Ray should miss entirely
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -2.0),
            direction: Vec3::new(0.0, 1.0, 0.0),
        };
        assert!(sphere.intersects_ray(&ray, (-100.0, 100.0)).is_none());
    }

    #[test]
    fn bounding_box() {
        // shifted unit sphere
        let mat = Material::new_lambertian(Vec3::new(0.8, 0.8, 0.8));
        let sphere = Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0, mat);

        let aabb = sphere.bounding_box().unwrap();
        assert_eq!(aabb.min, Vec3::new(0.0, 1.0, 2.0));
        assert_eq!(aabb.max, Vec3::new(2.0, 3.0, 4.0));
    }
}
