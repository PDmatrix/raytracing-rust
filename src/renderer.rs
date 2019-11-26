use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};
use rayon::prelude::*;

pub(crate) fn render(
    world: &dyn Hittable,
    camera: &Camera,
    nx: usize,
    ny: usize,
    ns: usize,
) -> Vec<u8> {
    let pixels = (0..ny)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            (0..nx).into_par_iter().flat_map(move |i| {
                let mut col = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..ns {
                    let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                    let v = (j as f32 + rand::random::<f32>()) / ny as f32;
                    let r = camera.get_ray(u, v);
                    col += color(&r, world, 0);
                }
                col /= ns as f32;
                col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

                (0..3).into_par_iter().map(move |k| (255.99 * col[k as usize]).min(255.0) as u8)
            })
        })
        .collect();

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
