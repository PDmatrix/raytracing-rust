use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hittable in self {
            if let Some(candidate_hit) = hittable.hit(r, t_min, t_max) {
                match hit {
                    None => hit = Some(candidate_hit),
                    Some(prev) => {
                        if candidate_hit.t < prev.t {
                            hit = Some(candidate_hit)
                        }
                    }
                }
            }
        }

        hit
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub(crate) fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { radius, center }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                return get_hit_record(self, r, t)
            }
            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                return get_hit_record(self, r, t)
            }
        }

        None
    }
}

fn get_hit_record(sphere: &Sphere, r: &Ray, t: f32) -> Option<HitRecord> {
    let p = r.point_at_parameter(t);

    return Some(HitRecord {
        t,
        p,
        normal: (p - sphere.center) / sphere.radius,
    });
}
