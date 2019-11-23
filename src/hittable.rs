use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: 0.0,
            normal: Vec3::new(0.0, 0.0, 0.0),
            p: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
