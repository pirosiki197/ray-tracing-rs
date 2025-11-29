use std::sync::Arc;

use glam::Vec3A;
use indicatif::{ParallelProgressIterator, ProgressBar};
use ray_tracing::{
    bvh::BVHBranch,
    camera::Camera,
    color,
    hittable::{Geometry, HittableList},
    material::{Dielectric, DiffuseLight, Lambertian, Metal, ScatterEvent},
    pdf::{HittablePDF, MixturePDF, PDF},
    rand,
    ray::Ray,
    sphere::Sphere,
    texture::SolidTexture,
    vec::{Color, Point3, Vec3Ext},
};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

fn ray_color(ray: &Ray, world: &HittableList, lights: Arc<Geometry>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }

    let Some((_, rec)) = world.hit(ray, 0.001, f32::INFINITY) else {
        return Color::new(1.0, 0.5, 0.0);
    };

    let emitted = rec.material().emitted(rec.u(), rec.v(), rec.point());

    let Some(srec) = rec.material().scatter(ray, &rec) else {
        return emitted;
    };

    match srec.event {
        ScatterEvent::Specular(specular) => {
            srec.attenuation * ray_color(&specular, world, lights, depth - 1)
        }
        ScatterEvent::Diffuse(pdf) => {
            let light = HittablePDF::new(lights.clone(), rec.point());
            let p = MixturePDF::new(Arc::new(light), pdf);

            let scattered = Ray::new(rec.point(), p.generate());
            let pdf_value = p.value(scattered.direction());

            if pdf_value < 1e-16 {
                return emitted;
            }

            emitted
                + srec.attenuation
                    * rec.material().scattering_pdf(ray, &rec, &scattered)
                    * ray_color(&scattered, world, lights, depth - 1)
                    / pdf_value
        }
    }
}

fn random_scene() -> (HittableList, HittableList) {
    let mut world = HittableList::new();
    let ground_material = Arc::new(Lambertian::new(Arc::new(SolidTexture::new(0.5, 0.5, 0.5))));
    world.add(Geometry::Sphere(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut spheres: Vec<Geometry> = Vec::with_capacity(22 * 22);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rand::random();
            let center = Point3::new(
                a as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::random::<f32>(),
            );

            if (center - Vec3A::new(4.0, 0.2, 0.0)).length() < 0.9 {
                continue;
            }

            if choose_mat < 0.7 {
                let texture: SolidTexture =
                    (rand::random::<Color>() * rand::random::<Color>()).into();
                spheres.push(Geometry::Sphere(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Lambertian::new(Arc::new(texture))),
                )));
            } else if choose_mat < 0.95 {
                let albedo = Color::random_range(0.5..1.0);
                let fuzz = rand::random_range(0.0..0.5);
                spheres.push(Geometry::Sphere(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Metal::new(albedo, fuzz)),
                )));
            } else {
                spheres.push(Geometry::Sphere(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Dielectric::new(1.5)),
                )))
            }
        }
    }

    world.add(BVHBranch::build(spheres));

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Geometry::Sphere(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new(Arc::new(SolidTexture::new(
        0.25, 0.875, 0.8125,
    ))));
    world.add(Geometry::Sphere(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(Color::new(0.75, 0.75, 0.75), 0.0));
    world.add(Geometry::Sphere(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let sun_light = DiffuseLight::new(Arc::new(SolidTexture::new(10.0, 10.0, 10.0)));
    let sun = Sphere::new(Point3::new(100.0, 100.0, 100.0), 50.0, Arc::new(sun_light));

    world.add(Geometry::Sphere(sun.clone()));

    let mut lights = HittableList::new();
    lights.add(Geometry::Sphere(sun));

    (world, lights)
}

fn main() {
    let mut stdout = std::io::stdout();

    let aspect_ration = 16.0 / 9.0;
    let image_width = 1280;
    let image_height = (image_width as f32 / aspect_ration) as i32;
    let samples_per_pixel = 1000;
    let max_depth = 50;

    let lookfrom = Point3::new(13.0, 2.0, 6.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3A::new(0.0, 1.0, 0.0),
        120.0,
        aspect_ration,
        0.1,
        10.0,
    );

    let (world, lights) = random_scene();
    let lights = Arc::new(Geometry::List(lights));

    let pb = ProgressBar::new(image_height as u64);

    let mut pixels = vec![Color::ZERO; (image_width * image_height) as usize];
    pixels
        .par_chunks_mut(image_width as usize)
        .enumerate()
        .progress_with(pb)
        .for_each(|(row_idx, row_slice)| {
            let j = image_height - 1 - row_idx as i32;
            for (i, pixel) in row_slice.iter_mut().enumerate() {
                let mut pixel_color = Color::ZERO;
                for _ in 0..samples_per_pixel {
                    let u = (i as f32 + rand::random::<f32>()) / (image_width - 1) as f32;
                    let v = (j as f32 + rand::random::<f32>()) / (image_height - 1) as f32;
                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(&ray, &world, lights.clone(), max_depth);
                }
                *pixel = pixel_color;
            }
        });

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for pixel_color in pixels {
        color::write_color(&mut stdout, pixel_color, samples_per_pixel);
    }
    eprintln!("\nDone.");
}
