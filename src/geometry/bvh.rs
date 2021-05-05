use rand::Rng;
use std::cmp::Ordering;

use crate::geometry::{Hit, Hittable, AABB};
use crate::math::Ray;

pub struct BVHNode {
    aabb: Option<AABB>,
    children: (Box<dyn Hittable>, Option<Box<dyn Hittable>>),
}

impl BVHNode {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> BVHNode {
        let mut objects = objects;
        let axis = rand::thread_rng().gen_range(0, 3);

        let comparator = |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| {
            let (min_a, min_b) = (
                a.bounding_box().unwrap().min[axis],
                b.bounding_box().unwrap().min[axis],
            );

            if min_a < min_b {
                Ordering::Less
            } else if min_a > min_b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        };

        let (left, right): (Box<dyn Hittable>, Option<Box<dyn Hittable>>) = match objects.len() {
            1 => (objects.remove(0), None),
            2 => {
                if comparator(&objects[0], &objects[1]) == Ordering::Less {
                    (objects.remove(0), Some(objects.remove(0)))
                } else {
                    (objects.remove(1), Some(objects.swap_remove(0)))
                }
            }
            _ => {
                objects.sort_unstable_by(comparator);

                let mid = objects.len() / 2;

                let mut left = objects;
                let right = left.split_off(mid);

                (
                    Box::new(BVHNode::new(left)),
                    Some(Box::new(BVHNode::new(right))),
                )
            }
        };

        let aabb1 = left.bounding_box();
        let aabb2 = match &right {
            Some(hittable) => hittable.bounding_box(),
            None => None,
        };

        let aabb = match (aabb1, aabb2) {
            (Some(box1), Some(box2)) => Some(AABB::merge(box1, box2)),
            (Some(box1), None) => Some(box1.clone()),
            (None, Some(box2)) => Some(box2.clone()),
            (None, None) => None,
        };

        BVHNode {
            aabb,
            children: (left, right),
        }
    }
}

impl Hittable for BVHNode {
    fn intersects_ray(&self, ray: &Ray, t_range: (f32, f32)) -> Option<Hit> {
        let hit = match &self.aabb {
            Some(aabb) => aabb.hit(&ray, t_range),
            None => true,
        };

        if hit {
            let left_hit = self.children.0.intersects_ray(&ray, t_range);
            let t_max = match &left_hit {
                Some(hit) => hit.t,
                None => t_range.1,
            };
            let right_hit = match &self.children.1 {
                Some(hittable) => hittable.intersects_ray(&ray, (t_range.0, t_max)),
                None => None,
            };

            if right_hit.is_some() {
                right_hit
            } else {
                left_hit
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<&AABB> {
        match &self.aabb {
            Some(aabb) => Some(&aabb),
            None => None,
        }
    }
}
