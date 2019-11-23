use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{unit_vector, Vec3};
use rand::Rng;

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

fn color<T: Hittable>(r: &Ray, world: &T) -> Vec3 {
    let mut rec: HitRecord = Default::default();

    if world.hit(&r, 0.0, std::f32::MAX, &mut rec) {
        0.5 * Vec3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        )
    } else {
        let unit_direction = unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn rnd() -> f32 {
    rand::thread_rng().gen_range(0.0, 1.0) as f32
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    print!("P3\n{} {}\n255\n", nx, ny);

    let list = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];
    let world = HittableList::new(list);
    let cam = Camera::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u = (i as f32 + rnd()) / nx as f32;
                let v = (j as f32 + rnd()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }
            col /= ns as f32;

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
