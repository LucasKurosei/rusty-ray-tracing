use crate::math_obj::{Vec3, Ray, Point};
use crate::color::Color;
use std::io::{self, Write};

pub mod math_obj;
pub mod color;

const IMAGE_HEIGHT: i32 = 256;
const IMAGE_WIDTH: i32 = 256;
const ASPECT_RATIO: f32 = (IMAGE_HEIGHT as f32) / (IMAGE_WIDTH as f32);
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT / ASPECT_RATIO;
fn main() {
    // vp is short for viewport
    let vp_v = Vec3::new(0., -VIEWPORT_HEIGHT as f32, 0.) / IMAGE_HEIGHT as f32;
    let vp_u = Vec3::new(VIEWPORT_WIDTH as f32, 0., 0.)/IMAGE_WIDTH as f32;
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
        eprint!("\rthere are {} lines remaining", IMAGE_HEIGHT-i);
        io::stderr()
            .flush()
            .unwrap();
    }
    eprintln!("\rDone!");
}

fn hits_sphere(center: &Point, radius: f32, r: &Ray) -> f32 {
    let oc = *center-r.origin();
    let a = r.direction().norm_squared();
    let h = r.direction().dot(&oc);
    let c = oc.norm_squared()-radius*radius;
    let delta = h*h-a*c;
    if delta < 0. {
        -1.
    } else {
        (h-delta.sqrt()) / a
    }
}
fn ray_color(r: Ray) -> Color {
    let unit_vec = r.direction().normalized();
    let sphere_origin = Vec3::new(0., 0., 2.5);
    let a = 0.5*(unit_vec.y()+1.0);
    let col = (1.-a)*Vec3::new(1., 1., 1.) + a*Vec3::new(0.5, 0.7, 1.);
    let t = hits_sphere(&sphere_origin, 1., &r);
    if t >= 0. {
        let N = r.at(t)-sphere_origin;
        let N = N.normalized();
        (0.5*(N+Vec3::new(1., 1., 1.))).to_color()
    } else {
        col.to_color()
    }
}
