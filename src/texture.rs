use crate::vec::{Color, Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: Point3) -> Color;
}

pub struct SolidTexture(Color);

impl SolidTexture {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Color::new(r, g, b))
    }
}

impl Texture for SolidTexture {
    fn value(&self, _: f32, _: f32, _: Point3) -> Color {
        self.0
    }
}

impl From<Color> for SolidTexture {
    fn from(value: Color) -> Self {
        Self(value)
    }
}
