use crate::math_obj::Vec3;
use std::fmt;   
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.0, self.1, self.2)
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Color {
        Color(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Color(r, g, b) = *self;
        write!(
            f,
            "{} {} {}",
            (r.sqrt() * 255.999) as i32,
            (g.sqrt() * 255.999) as i32,
            (b.sqrt() * 255.999) as i32
        )
    }
}
