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
    fn scatter<'a>(&self, _r_in: &Ray, hit_record: &HitRecord<'a>) -> Option<Scatter> {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        Some(Scatter {
            scattered: Some(Ray::new(hit_record.p, target - hit_record.p)),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter<'a>(&self, r_in: &Ray, hit_record: &HitRecord<'a>) -> Option<Scatter> {
        let reflected = reflect(&unit_vector(&r_in.direction()), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
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

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter<'a>(&self, r_in: &Ray, hit_record: &HitRecord<'a>) -> Option<Scatter> {
        let reflected = reflect(&r_in.direction(), &hit_record.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let (outward_normal, ni_over_nt, cosine) = if dot(r_in.direction(), hit_record.normal) > 0.0
        {
            (
                -hit_record.normal,
                self.ref_idx,
                self.ref_idx * dot(r_in.direction(), hit_record.normal) / r_in.direction().length(),
            )
        } else {
            (
                hit_record.normal,
                1.0 / self.ref_idx,
                -dot(r_in.direction(), hit_record.normal) / r_in.direction().length(),
            )
        };

        if let Some(refracted) = refract(&r_in.direction(), &outward_normal, ni_over_nt) {
            if rand::random::<f32>() >= schlick(cosine, self.ref_idx) {
                return Some(Scatter {
                    attenuation,
                    scattered: Some(Ray::new(hit_record.p, refracted)),
                });
            }
        }

        let scattered = Some(Ray::new(hit_record.p, reflected));
        Some(Scatter {
            attenuation,
            scattered,
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

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = unit_vector(v);
    let dt = dot(uv, *n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0. {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
