use crate::math::{Ray, Vec3};

#[derive(Clone, PartialEq, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    pub fn merge(box1: &AABB, box2: &AABB) -> AABB {
        let mut aabb = box1.clone();
        aabb.expand(box2);

        aabb
    }

    pub fn hit(&self, r: &Ray, t_range: (f32, f32)) -> bool {
        let mut t_range = t_range;
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let t0 = (self.min[a] - r.origin[a]) * inv_d;
            let t1 = (self.max[a] - r.origin[a]) * inv_d;
            // Swap them if inv_d < 0
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            if t0 > t_range.0 {
                t_range.0 = t0;
            }
            if t1 < t_range.1 {
                t_range.1 = t1;
            }

            if t_range.1 <= t_range.0 {
                return false;
            }
        }

        true
    }

    // Expands this AABB to contain the other
    pub fn expand(&mut self, other: &AABB) {
        self.min.set(
            f32::min(self.min.x, other.min.x),
            f32::min(self.min.y, other.min.y),
            f32::min(self.min.z, other.min.z),
        );
        self.max.set(
            f32::max(self.max.x, other.max.x),
            f32::max(self.max.y, other.max.y),
            f32::max(self.max.z, other.max.z),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit() {
        let aabb = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));

        // Hit through center
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -2.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        assert_eq!(aabb.hit(&ray, (0.0, 100.0)), true);

        // Miss, pointing backwards
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -2.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
        };
        assert_eq!(aabb.hit(&ray, (0.0, 100.0)), false);

        // Hit, starting inside box
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        assert_eq!(aabb.hit(&ray, (0.0, 100.0)), true);
    }

    #[test]
    fn merge() {
        let aabb1 = AABB::new(Vec3::new_zeroes(), Vec3::new(1.0, 2.0, 3.0));
        let aabb2 = AABB::new(Vec3::new(-1.0, 0.0, 1.0), Vec3::new(-0.5, 2.0, 4.0));
        let expected_min = Vec3::new(-1.0, 0.0, 0.0);
        let expected_max = Vec3::new(1.0, 2.0, 4.0);

        let aabb = AABB::merge(&aabb1, &aabb2);
        assert_eq!(aabb.min, expected_min);
        assert_eq!(aabb.max, expected_max);
    }

    #[test]
    fn expand() {
        let aabb1 = AABB::new(Vec3::new_zeroes(), Vec3::new(1.0, 2.0, 3.0));
        let aabb2 = AABB::new(Vec3::new(-1.0, 0.0, 1.0), Vec3::new(-0.5, 2.0, 4.0));
        let expected_min = Vec3::new(-1.0, 0.0, 0.0);
        let expected_max = Vec3::new(1.0, 2.0, 4.0);

        let mut aabb1_expand = aabb1.clone();
        aabb1_expand.expand(&aabb2);

        let mut aabb2_expand = aabb2.clone();
        aabb2_expand.expand(&aabb1);

        // Order shouldn't matter
        assert_eq!(aabb1_expand, aabb2_expand);
        // Check just one box
        assert_eq!(aabb1_expand.min, expected_min);
        assert_eq!(aabb1_expand.max, expected_max);
    }
}
