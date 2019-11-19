use crate::vec3::Vec3;

mod vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let col = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2 as f32;
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
