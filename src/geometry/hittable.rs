use std::ops::Index;

use crate::geometry::AABB;
use crate::material::Material;
use crate::math::{Ray, Vec3};

pub struct Hit<'a> {
    pub t: f32, // t stands for time?
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub trait Hittable {
    fn intersects_ray(&self, ray: &Ray, t_range: (f32, f32)) -> Option<Hit>;
    fn bounding_box(&self) -> Option<&AABB>;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
    aabb: Option<AABB>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            list: Vec::new(),
            // TODO: Using MIN/MAX seems hacky. At this point it really has no bounding box
            aabb: None,
        }
    }

    pub fn from_vec(list: Vec<Box<dyn Hittable>>) -> HittableList {
        let mut aabb: Option<AABB> = None;
        for hittable in &list {
            match hittable.bounding_box() {
                Some(bbox) => match &mut aabb {
                    Some(self_aabb) => self_aabb.expand(bbox),
                    None => aabb = Some(bbox.clone()),
                },
                None => {}
            }
        }

        HittableList { list, aabb }
    }

    pub fn with_capacity(n: usize) -> HittableList {
        HittableList {
            list: Vec::with_capacity(n),
            aabb: None,
        }
    }

    pub fn push<'a>(&'a mut self, hittable: Box<dyn Hittable>) {
        match hittable.bounding_box() {
            Some(aabb) => match &mut self.aabb {
                Some(self_aabb) => self_aabb.expand(aabb),
                None => self.aabb = Some(aabb.clone()),
            },
            None => {}
        }
        self.list.push(hittable);
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl Index<usize> for HittableList {
    type Output = Box<dyn Hittable>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.list[i]
    }
}

impl Hittable for HittableList {
    fn intersects_ray(&self, ray: &Ray, t_range: (f32, f32)) -> Option<Hit> {
        let mut hit = None;
        let mut range = t_range;
        for hittable in self.list.iter() {
            match hittable.intersects_ray(ray, range) {
                None => {}
                Some(new_hit) => {
                    range.1 = new_hit.t;
                    hit = Some(new_hit)
                }
            }
        }

        hit
    }

    fn bounding_box(&self) -> Option<&AABB> {
        match &self.aabb {
            Some(aabb) => Some(&aabb),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestHittable {
        aabb: AABB,
    }

    impl Hittable for TestHittable {
        fn intersects_ray(&self, _: &Ray, _: (f32, f32)) -> Option<Hit> {
            None
        }

        fn bounding_box(&self) -> Option<&AABB> {
            Some(&self.aabb)
        }
    }

    #[test]
    fn bounding_box() {
        // Setup
        let bounds1 = AABB::new(Vec3::new_zeroes(), Vec3::new(1.0, 1.0, 1.0));
        let hittable1 = TestHittable {
            aabb: bounds1.clone(),
        };
        let hittable2 = TestHittable {
            aabb: AABB::new(Vec3::new(-1.0, 0.0, -1.0), Vec3::new(1.0, 2.0, 1.0)),
        };
        let mut combined_aabb = hittable1.bounding_box().unwrap().clone();
        combined_aabb.expand(hittable2.bounding_box().unwrap());
        let mut hit_list = HittableList::new();

        // List should have no bounds yet
        assert!(hit_list.bounding_box().is_none());

        // Bounds should match first object
        hit_list.push(Box::new(hittable1));
        assert_eq!(hit_list.bounding_box().unwrap(), &bounds1);

        // Bounds should be the combination of both
        hit_list.push(Box::new(hittable2));
        assert_eq!(hit_list.bounding_box().unwrap(), &combined_aabb);
    }
}
