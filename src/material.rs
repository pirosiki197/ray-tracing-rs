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

#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl Material {
    /// scatter calculates (color, ray_out, pdf).
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian(m) => m.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
            Material::Dielectric(m) => m.scatter(r_in, rec),
            Material::DiffuseLight(m) => m.scatter(r_in, rec),
        }
    }
    pub fn emitted(&self, u: f32, v: f32, p: Point3) -> Color {
        match self {
            Material::Lambertian(m) => m.emitted(u, v, p),
            Material::Metal(m) => m.emitted(u, v, p),
            Material::Dielectric(m) => m.emitted(u, v, p),
            Material::DiffuseLight(m) => m.emitted(u, v, p),
        }
    }
    pub fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        match self {
            Material::Lambertian(m) => m.scattering_pdf(r_in, rec, scattered),
            Material::Metal(m) => m.scattering_pdf(r_in, rec, scattered),
            Material::Dielectric(m) => m.scattering_pdf(r_in, rec, scattered),
            Material::DiffuseLight(m) => m.scattering_pdf(r_in, rec, scattered),
        }
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

#[derive(Clone)]
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.albedo.value(rec.u(), rec.v(), rec.point());
        let pdf = Arc::new(CosinePDF::new(rec.normal()));
        Some(ScatterRecord {
            attenuation,
            event: ScatterEvent::Diffuse(pdf),
        })
    }

    pub fn scattering_pdf(&self, _: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = rec.normal().dot(scattered.direction().normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / f32::consts::PI
        }
    }

    pub fn emitted(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        Color::ZERO
    }
}

#[derive(Clone, Copy)]
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

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
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

    pub fn emitted(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        Color::ZERO
    }

    pub fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
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

    pub fn emitted(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        Color::ZERO
    }

    pub fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }
}

fn schlick(cos: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    pub fn emitted(&self, u: f32, v: f32, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }

    pub fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }
}
