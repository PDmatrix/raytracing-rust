use crate::camera::Camera;
use crate::hittable::{Hittable, Sphere};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

fn color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    let hit = world.hit(&r, 0.001, std::f32::MAX);

    match hit {
        Some(hit_record) => {
            if depth < 50 {
                if let Some(scatter) = hit_record.material.scatter(r, &hit_record) {
                    if let Some(bounce) = scatter.scattered {
                        return scatter.attenuation * color(&bounce, world, depth + 1);
                    }
                }
            }

            Vec3::new(0., 0., 0.)
        }
        None => {
            let unit_direction = unit_vector(&r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    print!("P3\n{} {}\n255\n", nx, ny);

    let spheres = vec![
        Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian {
                albedo: Vec3::new(0.1, 0.2, 0.5),
            }),
        ),
        Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian {
                albedo: Vec3::new(0.8, 0.8, 0.0),
            }),
        ),
        Sphere::new(
            Vec3::new(1., 0., -1.),
            0.5,
            Box::new(Metal {
                albedo: Vec3::new(0.8, 0.6, 0.2),
                fuzz: 0.3,
            }),
        ),
        Sphere::new(Vec3::new(-1., 0., -1.), 0.5, Box::new(Dielectric::new(1.5))),
        Sphere::new(
            Vec3::new(-1., 0., -1.),
            -0.45,
            Box::new(Dielectric::new(1.5)),
        ),
    ];
    let world: Vec<Box<dyn Hittable>> = spheres
        .into_iter()
        .map(|s| Box::new(s) as Box<dyn Hittable>)
        .collect();

    let cam = Camera {
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0),
    };
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = (j as f32 + rand::random::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= ns as f32;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
