use std::sync::Arc;

use glam::Vec3A;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    texture::Texture,
    vec::{Color, Point3, Vec3Ext},
};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, u: f32, v: f32, p: Point3) -> Color {
        _ = u;
        _ = v;
        _ = p;
        Color::ZERO
    }
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal() + Vec3A::random_unit();
        let scattered = Ray::new(rec.point(), scatter_direction);
        let attenuation = self.albedo.value(rec.u(), rec.v(), rec.point());
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal());
        let scattered = Ray::new(
            rec.point(),
            reflected + self.fuzz * Vec3A::radom_in_unit_sphere(),
        );
        Some((self.albedo, scattered))
    }
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face() {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.direction().normalize();
        let cos_theta = -unit_direction.dot(rec.normal()).min(1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = unit_direction.reflect(rec.normal());
            let scattered = Ray::new(rec.point(), reflected);
            return Some((attenuation, scattered));
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if rand::random::<f32>() < reflect_prob {
            let reflected = unit_direction.reflect(rec.normal());
            let scattered = Ray::new(rec.point(), reflected);
            return Some((attenuation, scattered));
        }

        let refracted = unit_direction.refract(rec.normal(), etai_over_etat);
        let scattered = Ray::new(rec.point(), refracted);
        Some((attenuation, scattered))
    }
}

fn schlick(cos: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
