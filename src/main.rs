mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec;
use hittable::Hittable;

use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec::unit_vector;
use crate::vec::{dot, Color, Point, Vec3};
use std::fmt::Write;
use std::io::stderr;
use std::io::Write as IoWrite;
use std::time::Instant;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction: Vec3 = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let start = Instant::now();
    let mut s = String::new();
    write!(s, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r, &world);
            write!(s, "{}\n", pixel_color.write_color()).unwrap()
        }
    }
    println!("{}", s);
    let duration = start.elapsed();
    eprintln!("\nDone in {:?}", duration)
}
