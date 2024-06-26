use math_obj::Hittable;

use crate::color::Color;
use crate::math_obj::{Point, Ray, Sphere, Vec3};
use std::io::{self, Write};

pub mod color;
pub mod math_obj;

const IMAGE_HEIGHT: i32 = 1024;
const IMAGE_WIDTH: i32 = 1024;
const ASPECT_RATIO: f32 = (IMAGE_HEIGHT as f32) / (IMAGE_WIDTH as f32);
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT / ASPECT_RATIO;

fn main() {
    // vp is short for viewport
    let vp_v = Vec3::new(0., -VIEWPORT_HEIGHT as f32, 0.) / IMAGE_HEIGHT as f32;
    let vp_u = Vec3::new(VIEWPORT_WIDTH as f32, 0., 0.) / IMAGE_WIDTH as f32;
    let focal_point = Vec3::new(0., 0., 0.5);
    let pixel00_loc = Point::new(-VIEWPORT_WIDTH * 0.5, VIEWPORT_HEIGHT * 0.5, 0.) + focal_point;
    let camera_origin = Point::new(0., 0., 0.);
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("256");
    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + i as f32 * vp_v + j as f32 * vp_u;
            // println!("{}", pixel_center);
            let r = Ray::new(camera_origin, pixel_center);
            let pixel_color = ray_color(r);

            println!("{}", pixel_color);
        }
        eprint!("\rthere are {} lines remaining", IMAGE_HEIGHT - i);
        io::stderr().flush().unwrap();
    }
    eprintln!("\rDone!");
}

fn ray_color(r: Ray) -> Color {
    let center = Vec3::new(0., 0., 2.);
    let sphere = Sphere { center, radius: 1. };
    match sphere.hit(&r, 0., 0.) {
        None => {
            let unit_vec = r.direction().normalized();
            let a = 0.5 * (unit_vec.y() + 1.0);
            let col = (1. - a) * Vec3::new(1., 1., 1.) + a * Vec3::new(0.5, 0.7, 1.);
            col.to_color()
        }
        Some(hit_record) => {
            let normal = hit_record.normal;
            (0.5 * (normal + Vec3::new(1., 1., 1.))).to_color()
        }
    }
}
