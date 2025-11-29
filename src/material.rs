use std::{f32, sync::Arc};

use glam::Vec3A;

use crate::{
    hittable::HitRecord,
    pdf::{CosinePDF, PDF},
    rand,
    ray::Ray,
    texture::Texture,
    vec::{Color, Point3, Vec3Ext},
};

pub trait Material: Send + Sync {
    /// scatter calculates (color, ray_out, pdf).
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
    fn emitted(&self, u: f32, v: f32, p: Point3) -> Color {
        _ = u;
        _ = v;
        _ = p;
        Color::ZERO
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        _ = r_in;
        _ = rec;
        _ = scattered;
        0.0
    }
}

pub enum ScatterEvent {
    Specular(Ray),
    Diffuse(Arc<dyn PDF>),
}

pub struct ScatterRecord {
    pub event: ScatterEvent,
    pub attenuation: Color,
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
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.albedo.value(rec.u(), rec.v(), rec.point());
        let pdf = Arc::new(CosinePDF::new(rec.normal()));
        Some(ScatterRecord {
            attenuation,
            event: ScatterEvent::Diffuse(pdf),
        })
    }
    fn scattering_pdf(&self, _: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = rec.normal().dot(scattered.direction().normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / f32::consts::PI
        }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = r_in.direction().reflect(rec.normal());
        let specular_ray = Ray::new(
            rec.point(),
            reflected + self.fuzz * Vec3A::radom_in_unit_sphere(),
        );
        Some(ScatterRecord {
            event: ScatterEvent::Specular(specular_ray),
            attenuation: self.albedo,
        })
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face() {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.direction();
        let cos_theta = -unit_direction.dot(rec.normal()).min(1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = unit_direction.reflect(rec.normal());
            let scattered = Ray::new(rec.point(), reflected);
            return Some(ScatterRecord {
                event: ScatterEvent::Specular(scattered),
                attenuation,
            });
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if rand::random::<f32>() < reflect_prob {
            let reflected = unit_direction.reflect(rec.normal());
            let scattered = Ray::new(rec.point(), reflected);
            return Some(ScatterRecord {
                event: ScatterEvent::Specular(scattered),
                attenuation,
            });
        }

        let refracted = unit_direction.refract(rec.normal(), etai_over_etat);
        let scattered = Ray::new(rec.point(), refracted);
        Some(ScatterRecord {
            event: ScatterEvent::Specular(scattered),
            attenuation,
        })
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
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
