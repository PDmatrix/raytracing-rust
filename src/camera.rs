use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) struct Camera {
    pub(crate) origin: Vec3,
    pub(crate) lower_left_corner: Vec3,
    pub(crate) horizontal: Vec3,
    pub(crate) vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
