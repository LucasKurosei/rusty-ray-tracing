use crate::color::Color;
use crate::math_obj::{Point, Ray, Sphere, Vec3};
use math_obj::{HitRecord, Hittable};

use std::f32::consts::PI;
use std::io::{self, Write};

pub mod color;
pub mod math_obj;

const IMAGE_HEIGHT: i32 = 256;
const IMAGE_WIDTH: i32 = 256;
const ASPECT_RATIO: f32 = (IMAGE_HEIGHT as f32) / (IMAGE_WIDTH as f32);
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT / ASPECT_RATIO;
const SAMPLE_PER_PIXEL_SQRT: i32 = 3;
const SAMPLE_PER_PIXEL: i32 = SAMPLE_PER_PIXEL_SQRT * SAMPLE_PER_PIXEL_SQRT;

fn main() {
    // vp is short for viewport
    let vp_v = Vec3::new(0., -VIEWPORT_HEIGHT as f32, 0.) / IMAGE_HEIGHT as f32;
    let vp_u = Vec3::new(VIEWPORT_WIDTH as f32, 0., 0.) / IMAGE_WIDTH as f32;
    let focal_point = Vec3::new(0., 0., 1.0);
    let camera_origin = Point::new(0., 0., 0.);
    let pixel00_loc =
        Point::new(-VIEWPORT_WIDTH * 0.5, VIEWPORT_HEIGHT * 0.5, 0.) + focal_point + camera_origin;
    let centers = vec![Vec3::new(0., 1., 3.), Vec3::new(1., 0., 2.)];
    let mut hittables = Vec::new();
    for i in 0..2 {
        hittables.push(Sphere {
            center: centers[i],
            radius: 0.5,
            color: centers[i].abs().normalized().to_color(),
        });
    }
    let floor = Sphere {
        center: Vec3::new(0., -500.5, 0.),
        radius: 500.,
        color: Color(0.7, 0.7, 0.7),
    };
    hittables.push(floor);
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("256");
    let rand_directions = fibonacci_sphere(SAMPLE_PER_PIXEL as usize);
    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + i as f32 * vp_v + j as f32 * vp_u;
            // TODO: correct for centering the pixel
            let mut color_vec = Vec3::new(0., 0., 0.);
            for i in 0..SAMPLE_PER_PIXEL_SQRT {
                for j in 0..SAMPLE_PER_PIXEL_SQRT {
                    // TODO: fix the bug that happens when r = (0, 0, 0)
                    let direction = pixel_center
                        + vp_u * (i as f32) / SAMPLE_PER_PIXEL_SQRT as f32
                        + vp_v * (j as f32) / SAMPLE_PER_PIXEL_SQRT as f32;
                    let unit_vec = direction.normalized();
                    let a = 0.5 * (unit_vec.y() + 1.0);
                    let col = (1. - a) * Vec3::new(1., 1., 1.) + a * Vec3::new(0.4, 0.6, 1.);
                    let col = col.to_color();
                    let r = Ray::new(camera_origin, direction, col);
                    color_vec = color_vec + ray_color(r, &hittables, 3, &rand_directions).to_vec3();
                }
            }
            let pixel_color = (color_vec / SAMPLE_PER_PIXEL as f32).to_color();

            println!("{}", pixel_color);
        }
        eprint!("\rthere are {} lines remaining", IMAGE_HEIGHT - i);
        io::stderr().flush().unwrap();
    }
    eprintln!("\rDone!");
}
fn fibonacci_sphere(samples: usize) -> Vec<Vec3> {
    let mut points = Vec::new();
    let phi = PI * ((5.0 as f32).sqrt() - 1.0); // golden angle in radians

    for i in 0..samples {
        let y = 1.0 - (i as f32 / (samples - 1) as f32) * 2.0; // y goes from 1 to -1
        let radius = (1.0 - y * y).sqrt(); // radius at y

        let theta = phi * i as f32; // golden angle increment

        let x = theta.cos() * radius;
        let z = theta.sin() * radius;

        points.push(Vec3::new(x, y, z) * 0.5);
    }

    points
}
fn fetch_hittable(r: &Ray, hittables: &[Sphere]) -> Option<HitRecord> {
    let mut maybe_hit_record: Option<HitRecord> = None;
    for hittable in hittables {
        match hittable.hit(&r, 0.001, 10.) {
            None => continue,
            Some(new_hit_record) => match maybe_hit_record {
                None => {
                    maybe_hit_record = Some(new_hit_record);
                }
                Some(HitRecord { t: t_min, .. }) => {
                    if new_hit_record.t < t_min {
                        maybe_hit_record = Some(new_hit_record);
                    }
                }
            },
        }
    }
    maybe_hit_record
}

fn ray_color(r: Ray, hittables: &[Sphere], max_rebounds: i32, rand_directions: &[Vec3]) -> Color {
    let mut col = r.color;
    match fetch_hittable(&r, hittables) {
        None => {
            return col;
        }
        Some(hit_record) => {
            col = col.mul(&hit_record.hit_object.color);
            if max_rebounds > 0 {
                let mut col_vec = Vec3::new(0., 0., 0.);
                for i in 0..SAMPLE_PER_PIXEL {
                    let rand_direction = rand_directions[i as usize];
                    let reflected_ray =
                        Ray::new(hit_record.p, hit_record.normal + rand_direction, col);
                    col_vec = col_vec
                        + ray_color(reflected_ray, hittables, max_rebounds - 1, rand_directions)
                            .to_vec3();
                }
                col_vec = col_vec / SAMPLE_PER_PIXEL as f32;
                col_vec = col_vec * 0.9;
                col = col.mul(&col_vec.to_color());
            }
        }
    }
    assert!(col.0 <= 1. && col.1 <= 1. && col.2 <= 1.);
    col
}
