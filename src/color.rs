use std::io::Write;

use crate::vec::Color;

pub fn write_color(w: &mut impl Write, mut pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f32;
    pixel_color *= scale;
    // gamma correction
    pixel_color = pixel_color.map(f32::sqrt);

    let sanitize = |v: f32| if v.is_nan() { 0.0 } else { v }.clamp(0.0, 0.999);

    let r = 256.0 * sanitize(pixel_color.x);
    let g = 256.0 * sanitize(pixel_color.y);
    let b = 256.0 * sanitize(pixel_color.z);

    write!(w, "{} {} {}\n", r as u8, g as u8, b as u8).unwrap();
}
