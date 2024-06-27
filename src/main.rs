use crate::color::Color;
use crate::math_obj::{Point, Ray, Sphere, Vec3};
use math_obj::{HitRecord, Hittable};

use std::io::{self, Write};

pub mod color;
pub mod math_obj;

const IMAGE_HEIGHT: i32 = 256;
const IMAGE_WIDTH: i32 = 256;
const ASPECT_RATIO: f32 = (IMAGE_HEIGHT as f32) / (IMAGE_WIDTH as f32);
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT / ASPECT_RATIO;

fn main() {
    // vp is short for viewport
    let vp_v = Vec3::new(0., -VIEWPORT_HEIGHT as f32, 0.) / IMAGE_HEIGHT as f32;
    let vp_u = Vec3::new(VIEWPORT_WIDTH as f32, 0., 0.) / IMAGE_WIDTH as f32;
    let focal_point = Vec3::new(0., 0., 1.0);
    let camera_origin = Point::new(0., 0., 0.);
    let pixel00_loc =
        Point::new(-VIEWPORT_WIDTH * 0.5, VIEWPORT_HEIGHT * 0.5, 0.) + focal_point + camera_origin;
    let centers = vec![
        Vec3::new(-1., 0., 2.),
        Vec3::new(0., 2., 3.),
        Vec3::new(1., 0., 2.),
    ];
    let mut hittables = Vec::new();
    for i in 0..3 {
        hittables.push(Sphere {
            center: centers[i],
            radius: 0.5,
            color: centers[i].abs().normalized().to_color(),
        });
    }
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("256");
    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + i as f32 * vp_v + j as f32 * vp_u;
            // println!("{}", pixel_center);
            let r = Ray::new(camera_origin, pixel_center);
            let pixel_color = ray_color(r, &hittables);

            println!("{}", pixel_color);
        }
        eprint!("\rthere are {} lines remaining", IMAGE_HEIGHT - i);
        io::stderr().flush().unwrap();
    }
    eprintln!("\rDone!");
}

fn ray_color(mut r: Ray, hittables: &[Sphere]) -> Color {
    let sky_vec = Vec3::new(0., 1., 0.);
    // TODO: fix the bug that happens when r = (0, 0, 0)
    let unit_vec = r.direction().normalized();
    let a = 0.5 * (unit_vec.y() + 1.0);
    let col = (1. - a) * Vec3::new(1., 1., 1.) + a * Vec3::new(0.5, 0.7, 1.);
    let mut col = col.to_color();
    for i in 0..2 {
        let mut hit_record: Option<HitRecord> = None;
        for hittable in hittables {
            match hittable.hit(&r, 0., 10.) {
                None => continue,
                Some(HitRecord {
                    p,
                    normal,
                    t,
                    hit_object,
                }) => match hit_record {
                    None => {
                        hit_record = Some(HitRecord {
                            p,
                            normal,
                            t,
                            hit_object,
                        });
                    }
                    Some(HitRecord { t: t_min, .. }) => {
                        if t < t_min {
                            hit_record = Some(HitRecord {
                                p,
                                normal,
                                t,
                                hit_object,
                            });
                        }
                    }
                },
            }
        }
        match hit_record {
            None => {
                return col;
            }
            Some(HitRecord {
                p,
                normal,
                t,
                hit_object,
            }) => {
                col = col.mul(&hit_object.color);
                r = r.reflection(normal, p);
            }
        }
    }
    assert!(col.0 <= 1. && col.1 <= 1. && col.2 <= 1.);
    col
}
