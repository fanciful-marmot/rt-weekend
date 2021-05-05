use crate::math::{Ray, Vec3};
use std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    bottom_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(look_from: &Vec3, look_at: &Vec3, v_up: &Vec3, v_fov: f32, aspect: f32) -> Camera {
        let half_theta = v_fov * PI / 360.0;
        let half_height = (half_theta).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).make_unit();
        let u = v_up.cross(&w).make_unit();
        let v = w.cross(&u);

        let origin = look_from.clone();

        Camera {
            bottom_left: origin - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin.clone(),
            direction: ((self.bottom_left + u * self.horizontal + v * self.vertical) - self.origin)
                .make_unit(),
        }
    }
}
