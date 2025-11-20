use std::sync::Arc;

use glam::Vec3A;

use crate::{aabb::AABB, hittable::HitRecord, material::Material, ray::Ray, vec::Point3};

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<(f32, HitRecord)> {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let root = discriminant.sqrt();
        let temp = (-half_b - root) / a;
        if temp < t_max && temp > t_min {
            return Some((temp, self.hit_record(&ray, temp)));
        }

        let temp = (-half_b + root) / a;
        if temp < t_max && temp > t_min {
            return Some((temp, self.hit_record(&ray, temp)));
        }

        None
    }

    pub fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3A::new(self.radius, self.radius, self.radius),
            self.center + Vec3A::new(self.radius, self.radius, self.radius),
        ))
    }
}

impl Sphere {
    fn hit_record(&self, ray: &Ray, t: f32) -> HitRecord {
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        HitRecord::new(
            p,
            if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            self.material.clone(),
            front_face,
        )
    }
}
