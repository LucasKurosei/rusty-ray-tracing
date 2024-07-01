use crate::color::Color;
use crate::math_obj::{Point, Ray, Sphere, Vec3};
use math_obj::{HitRecord, Hittable, RayTracingTexture};
use rand::Rng;

use std::f32::consts::PI;
use std::io::{self, Write};
use std::vec;

pub mod color;
pub mod math_obj;

const IMAGE_HEIGHT: i32 = 256;
const IMAGE_WIDTH: i32 = 256;
const ASPECT_RATIO: f32 = (IMAGE_HEIGHT as f32) / (IMAGE_WIDTH as f32);
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT / ASPECT_RATIO;
const SAMPLE_PER_PIXEL_SQRT: i32 = 4;
const SAMPLE_PER_PIXEL: i32 = SAMPLE_PER_PIXEL_SQRT * SAMPLE_PER_PIXEL_SQRT;

fn main() {
    // vp is short for viewport
    let vp_v = Vec3::new(0., -VIEWPORT_HEIGHT as f32, 0.) / IMAGE_HEIGHT as f32;
    let vp_u = Vec3::new(VIEWPORT_WIDTH as f32, 0., 0.) / IMAGE_WIDTH as f32;
    let focal_point = Vec3::new(0., 0., 1.0);
    let camera_origin = Point::new(0., 0., 0.);
    let pixel00_loc =
        Point::new(-VIEWPORT_WIDTH * 0.5, VIEWPORT_HEIGHT * 0.5, 0.) + focal_point + camera_origin;

    // default scene configuration
    let centers = vec![Vec3::new(0., 1., 3.), Vec3::new(1., 0., 2.), Vec3::new(0., -500.5, 0.), Vec3::new(0.25, -0.25, 1.25), Vec3::new(-1.125, -0.36, 1.75)];
    let radiuses = vec!(0.5, 0.5, 500., 0.25, 0.125);
    let colors = vec!(Color(0.5, 0., 1.), Color(1., 1., 1.), Color(0.7, 0.7, 0.7), Color(1., 1., 1.), Color(0.95, 0.2, 0.2));
    let mut matt_texture = RayTracingTexture{color: Color(1., 1., 1.), scatter_ray: matt_texture_scatter_ray};
    let mut hittables = Vec::new();
    for i in 0..5 {
        matt_texture.color = colors[i];
        hittables.push(Sphere {
            center: centers[i],
            radius: radiuses[i],
            texture: matt_texture
        });
    }
    hittables[1].texture.scatter_ray = metal_texture_scatter_ray;
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("256");
    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + i as f32 * vp_v + j as f32 * vp_u;
            // TODO: correct for centering the pixel
            let mut color_vec = Vec3::new(0., 0., 0.);
            for k in 0..SAMPLE_PER_PIXEL_SQRT {
                for l in 0..SAMPLE_PER_PIXEL_SQRT {
                    // TODO: fix the bug that happens when r = (0, 0, 0)
                    let direction = pixel_center
                        + vp_u * (k as f32) / SAMPLE_PER_PIXEL_SQRT as f32
                        + vp_v * (l as f32) / SAMPLE_PER_PIXEL_SQRT as f32;
                    let col = Color(1., 1., 1.);
                    let r = Ray::new(camera_origin, direction, col, SAMPLE_PER_PIXEL);
                    color_vec = color_vec + ray_color(r, &hittables, 4).to_vec3();
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
        match hittable.hit(&r, 0.001, 100000.) {
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

fn matt_texture_scatter_ray(r: &Ray, at: &Point, normal: &Vec3) -> Vec<Ray> {
    let mut rng_thread = rand::thread_rng();
    let mut reflected_rays = Vec::with_capacity(r.scatter_potential.try_into().unwrap());
    for _ in 0..r.scatter_potential {
        loop {
            let rand_direction = Vec3::new(rng_thread.gen_range(-1.0..1.0), rng_thread.gen_range(-1.0..1.0), rng_thread.gen_range(-1.0..1.0));
            if rand_direction.norm_squared()<1.0 {
                let reflected_ray =
                    Ray::new(*at, *normal + 0.5*rand_direction, Color(1., 1., 1.), 1);
                reflected_rays.push(reflected_ray);
                break;
            }
        }
    }
    reflected_rays
}

fn metal_texture_scatter_ray(r: &Ray, at: &Point, normal: &Vec3) -> Vec<Ray> {
    if r.scatter_potential<1 {
        return Vec::new()
    }
    vec![r.reflection(*normal, *at, Color(1., 1., 1.))]
}

fn ray_color(mut r: Ray, hittables: &[Sphere], max_rebounds: i32) -> Color {
    let sky_normal = Vec3::new(0., 1., 0.);
    match fetch_hittable(&r, hittables) {
        None => {
            let cos_theta = r.direction().normalized().dot(&sky_normal);
            if cos_theta > 0.0 {
                return r.color
            } else {
                return Color(0., 0., 0.)
            }
        }
        Some(hit_record) => {
            r.color = r.color * hit_record.hit_object.texture.color;
            let mut col_vec = Vec3::new(0., 0., 0.);
            let scattered_rays = (hit_record.hit_object.texture.scatter_ray)(&r, &hit_record.p, &hit_record.normal);
            let scatter_num = scattered_rays.len() as f32;
            if scatter_num > 0. {
                for mut ray in scattered_rays {
                    if max_rebounds <= 0 {
                        ray.scatter_potential = 0;
                    }
                    col_vec = col_vec + ray_color(ray, hittables, max_rebounds - 1).to_vec3();
                }
                col_vec = col_vec / scatter_num;
            }
            r.color = r.color * col_vec.to_color();
        }
    }
    assert!(r.color.0 <= 1. && r.color.1 <= 1. && r.color.2 <= 1.);
    r.color
}
