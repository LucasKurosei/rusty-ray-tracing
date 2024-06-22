use std::fmt;

pub struct Color(pub f32, pub f32, pub f32);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Color(r, g, b) = *self;
        write!(f, "{} {} {}", (r*255.999) as i32, (g*255.999) as i32, (b*255.999) as i32)
    } 
}