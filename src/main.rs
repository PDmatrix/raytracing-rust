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
    let nx = 1280;
    let ny = 800;
    let ns = 150;
    print!("P3\n{} {}\n255\n", nx, ny);
    let r = (std::f32::consts::PI / 4.0).cos();
    let world: Vec<Box<dyn Hittable>> = random_scene()
        .into_iter()
        .map(|s| Box::new(s) as Box<dyn Hittable>)
        .collect();

    let look_from = Vec3::new(10.0, 1.8, 2.4);
    let look_at = Vec3::new(0.0, 0.0, 0.5);
    //let dist_to_focus = (look_from - look_at).length();
    let dist_to_focus = (look_from - Vec3::new(4.0, 1.0, 0.0)).length();
    let aperture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        (nx as f32) / (ny as f32),
        aperture,
        dist_to_focus,
    );
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

fn random_scene() -> Vec<Sphere> {
    let n = 500;
    let mut spheres = Vec::with_capacity(n);
    spheres.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
    ));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Vec3::new(
                (a as f32) + 0.9 * rand::random::<f32>(),
                0.2,
                (b as f32) + 0.9 * rand::random::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            // diffuse
            if choose_mat < 0.8 {
                spheres.push(Sphere::new(
                    center,
                    0.2,
                    Box::new(Lambertian {
                        albedo: Vec3::new(
                            rand::random::<f32>() * rand::random::<f32>(),
                            rand::random::<f32>() * rand::random::<f32>(),
                            rand::random::<f32>() * rand::random::<f32>(),
                        ),
                    }),
                ))
            } else if choose_mat < 0.9 {
                spheres.push(Sphere::new(
                    center,
                    0.2,
                    Box::new(Metal {
                        albedo: Vec3::new(
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>()),
                        ),
                        fuzz: 0.5 * rand::random::<f32>(),
                    }),
                ))
            } else {
                spheres.push(Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5))))
            }
        }
    }

    spheres.push(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    ));
    spheres.push(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }),
    ));
    spheres.push(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    ));

    spheres
}
