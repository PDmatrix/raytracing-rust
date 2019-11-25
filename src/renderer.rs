use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

pub(crate) fn render(
    world: &dyn Hittable,
    camera: &Camera,
    nx: usize,
    ny: usize,
    ns: usize,
) -> Vec<u8> {
    let mut pixels: Vec<u8> = Vec::with_capacity(nx * ny * 3);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = (j as f32 + rand::random::<f32>()) / ny as f32;
                let r = camera.get_ray(u, v);
                col += color(&r, world, 0);
            }
            col /= ns as f32;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

            pixels.push((255.99 * col[0]) as u8);
            pixels.push((255.99 * col[1]) as u8);
            pixels.push((255.99 * col[2]) as u8);
        }
    }

    pixels
}

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
