use std::io::Write;

use crate::vec::Color;

pub fn write_color(w: &mut impl Write, pixel_color: Color) {
    write!(
        w,
        "{} {} {}\n",
        (255.999 * pixel_color.x()) as u8,
        (255.999 * pixel_color.y()) as u8,
        (255.999 * pixel_color.z()) as u8
    )
    .unwrap();
}
