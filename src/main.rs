use core::f32;
use std::rc::Rc;

use ray_tracing::{
    camera::Camera,
    color,
    hittable::{Hittable, HittableList},
    ray::Ray,
    sphere::Sphere,
    vec::{Color, Point3},
};

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    if let Some(rec) = world.hit(&ray, 0.0, f32::INFINITY) {
        return 0.5 * (*rec.normal() + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let mut stdout = std::io::stdout();

    let aspect_ration = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ration) as i32;
    let samples_per_pixel = 100;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let camera = Camera::new();

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::ZERO;
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rand::random_range(0.0..1.0)) / (image_width - 1) as f32;
                let v = (j as f32 + rand::random_range(0.0..1.0)) / (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }
            color::write_color(&mut stdout, pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}
