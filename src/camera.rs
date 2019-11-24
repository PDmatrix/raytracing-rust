use crate::ray::Ray;
use crate::vec3::{cross, dot, unit_vector, Vec3};

pub(crate) struct Camera {
    pub(crate) origin: Vec3,
    pub(crate) lower_left_corner: Vec3,
    pub(crate) horizontal: Vec3,
    pub(crate) vertical: Vec3,

    pub(crate) u: Vec3,
    pub(crate) v: Vec3,
    pub(crate) w: Vec3,
    pub(crate) lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        v_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = v_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = unit_vector(&(look_from - look_at));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);
        Self {
            lens_radius: aperture / 2.0,
            w,
            u,
            v,
            lower_left_corner: look_from
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: look_from,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0)
            - Vec3::new(1.0, 1.0, 0.0);
        if dot(p, p) < 1.0 {
            return p;
        }
    }
}

/*
vec3 random_in_unit_disk() {
    vec3 p;
    do {
        p = 2.0*vec3(random_double(),random_double(),0) - vec3(1,1,0);
    } while (dot(p,p) >= 1.0);
    return p;
}
*/
