use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{dot, unit_vector, Vec3};

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter<'a>(&self, r_in: &Ray, hit_record: &HitRecord<'a>) -> Option<Scatter> {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        Some(Scatter {
            scattered: Some(Ray::new(hit_record.p, target - hit_record.p)),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Metal {
    fn scatter<'a>(&self, r_in: &Ray, hit_record: &HitRecord<'a>) -> Option<Scatter> {
        let reflected = reflect(&unit_vector(&r_in.direction()), &hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);
        Some(Scatter {
            attenuation: self.albedo,
            scattered: if dot(scattered.direction(), hit_record.normal) > 0. {
                Some(scattered)
            } else {
                None
            },
        })
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p =
            2.0 * Vec3::new(
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>(),
            ) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2. * dot(*v, *n) * *n
}
