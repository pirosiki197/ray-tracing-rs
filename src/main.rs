use ray_tracing::{color, vec::Color};

fn main() {
    let mut stdout = std::io::stdout();

    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.25,
            );
            color::write_color(&mut stdout, pixel_color);
        }
    }
    eprintln!("\nDone.");
}
