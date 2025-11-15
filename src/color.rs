use std::io::Write;

use crate::vec::Color;

pub fn write_color(w: &mut impl Write, mut pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f32;
    pixel_color *= scale;
    // gamma correction
    pixel_color = pixel_color.sqrt();

    let r = 256.0 * pixel_color.x().clamp(0.0, 0.999);
    let g = 256.0 * pixel_color.y().clamp(0.0, 0.999);
    let b = 256.0 * pixel_color.z().clamp(0.0, 0.999);

    write!(w, "{} {} {}\n", r as u8, g as u8, b as u8).unwrap();
}
