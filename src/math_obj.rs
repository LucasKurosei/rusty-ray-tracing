use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::color::Color;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(f32, f32, f32);
pub type Point = Vec3;

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
// implement multiplication and division too
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
// implement negation
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}
// implement scalar multiplication and division
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
// implement scalar multiplication and division for f32
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.0, self.1, self.2)
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }
    pub fn x(&self) -> f32 {
        self.0
    }
    // create similar getters for the y and z values
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }
    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
    pub fn mul(&self, other: &Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
    pub fn div(&self, other: &Vec3) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
    pub fn abs(&self) -> Vec3 {
        Vec3(self.0.abs(), self.1.abs(), self.2.abs())
    }
    pub fn norm_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
    pub fn normalized(self) -> Self {
        self / self.norm()
    }
    pub fn to_color(self) -> Color {
        assert!(self.0 <= 1. && self.1 <= 1. && self.2 <= 1.);
        Color(self.0, self.1, self.2)
    }
}

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn origin(&self) -> Point {
        self.origin
    }
    pub fn reflection(&self, normal: Vec3, origin: Point) -> Ray {
        let normal_component = normal.dot(&self.direction());
        Ray {
            origin,
            direction: self.direction - 2. * normal_component * normal
        }
    }
}

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
    pub hit_object: Sphere,
}
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub color: Color,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().norm_squared();
        let h = r.direction().dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;
        let delta = h * h - a * c;
        let t = if delta < 0. {
            return None;
        } else {
            (h - delta.sqrt()) / a
        };
        if t < t_min || t > t_max {
            return None;
        }

        let hit_at = r.at(t);
        let normal = hit_at - self.center;
        let normal = normal/self.radius;

        let hit_record = HitRecord {
            p: hit_at,
            normal,
            t,
            hit_object: *self,
        };

        Some(hit_record)
    }
}
