use crate::camera::Camera;
use crate::hittable::{Hittable, Sphere};
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::vec3::{Vec3};
use std::io::BufWriter;
use std::fs::File;

mod camera;
mod hittable;
mod material;
mod ray;
mod renderer;
mod vec3;

fn main() {
    let nx = 300;
    let ny = 300;
    let ns = 10;
    let world: Vec<Box<dyn Hittable>> = random_scene()
        .into_iter()
        .map(|s| Box::new(s) as Box<dyn Hittable>)
        .collect();

    let look_from = Vec3::new(10.0, 1.8, 2.4);
    let look_at = Vec3::new(0.0, 0.0, 0.5);
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

    let pixels = renderer::render(&world, &cam, nx, ny, ns);

    let w = BufWriter::new(File::create("a.png").unwrap());

    let mut encoder = png::Encoder::new(w, nx as u32, ny as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();
}

fn random_scene() -> Vec<Sphere> {
    let mut spheres = vec![
        Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Box::new(Lambertian {
                albedo: Vec3::new(0.5, 0.5, 0.5),
            }),
        ),
        Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Box::new(Dielectric::new(1.5)),
        ),
        Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Box::new(Lambertian {
                albedo: Vec3::new(0.4, 0.2, 0.1),
            }),
        ),
        Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Box::new(Metal {
                albedo: Vec3::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            }),
        ),
    ];
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

            let material: Box<dyn Material>;
            if choose_mat < 0.8 {
                material = Box::new(Lambertian {
                    albedo: Vec3::new(
                        rand::random::<f32>() * rand::random::<f32>(),
                        rand::random::<f32>() * rand::random::<f32>(),
                        rand::random::<f32>() * rand::random::<f32>(),
                    ),
                });
            } else if choose_mat < 0.9 {
                material = Box::new(Metal {
                    albedo: Vec3::new(
                        0.5 * (1.0 + rand::random::<f32>()),
                        0.5 * (1.0 + rand::random::<f32>()),
                        0.5 * (1.0 + rand::random::<f32>()),
                    ),
                    fuzz: 0.5 * rand::random::<f32>(),
                });
            } else {
                material = Box::new(Dielectric::new(1.5));
            }

            spheres.push(Sphere::new(center, 0.2, material))
        }
    }

    spheres
}
