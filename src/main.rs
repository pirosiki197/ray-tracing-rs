use core::f32;
use std::rc::Rc;

use rand::Rng;
use ray_tracing::{
    camera::Camera,
    color,
    hittable::{Hittable, HittableList},
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    sphere::Sphere,
    vec::{Color, Point3, Vec3},
};

fn ray_color(ray: &Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }

    if let Some(rec) = world.hit(&ray, 0.001, f32::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material().scatter(&ray, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
    }

    let unit_direction = ray.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random_range(0.0..1.0);
            let center = Point3::new(
                a as f32 + 0.9 * rng.random_range(0.0..1.0),
                0.2,
                b as f32 + 0.9 * rng.random_range(0.0..1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() < 0.9 {
                continue;
            }

            if choose_mat < 0.8 {
                let albedo = Color::random() * Color::random();
                world.add(Rc::new(Sphere::new(
                    center,
                    0.2,
                    Rc::new(Lambertian::new(albedo)),
                )));
            } else if choose_mat < 0.95 {
                let albedo = Color::random_range(0.5..1.0);
                let fuzz = rng.random_range(0.0..0.5);
                world.add(Rc::new(Sphere::new(
                    center,
                    0.2,
                    Rc::new(Metal::new(albedo, fuzz)),
                )));
            } else {
                world.add(Rc::new(Sphere::new(
                    center,
                    0.2,
                    Rc::new(Dielectric::new(1.5)),
                )))
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() {
    let mut stdout = std::io::stdout();

    let aspect_ration = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ration) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let lookfrom = Point3::new(13.0, 2.0, 6.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        120.0,
        aspect_ration,
        0.1,
        10.0,
    );

    let world = random_scene();

    let mut rng = rand::rng();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::ZERO;
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rng.random_range(0.0..1.0)) / (image_width - 1) as f32;
                let v = (j as f32 + rng.random_range(0.0..1.0)) / (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            color::write_color(&mut stdout, pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}
