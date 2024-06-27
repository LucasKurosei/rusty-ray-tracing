use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {
    pub fn mul(&self, other: &Color) -> Color {
        Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Color(r, g, b) = *self;
        write!(
            f,
            "{} {} {}",
            (r * 255.999) as i32,
            (g * 255.999) as i32,
            (b * 255.999) as i32
        )
    }
}
