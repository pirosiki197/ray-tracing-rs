use std::{f32, sync::Arc};

use glam::Vec3A;

use crate::{
    aabb::AABB, hittable::HitRecord, material::Material, onb::ONB, rand, ray::Ray, vec::Point3,
};

#[derive(Clone)]
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
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - c;

        if discriminant < 0.0 {
            return None;
        }

        let root = discriminant.sqrt();
        let temp = -half_b - root;
        if temp < t_max && temp > t_min {
            return Some((temp, self.hit_record(ray, temp)));
        }

        let temp = -half_b + root;
        if temp < t_max && temp > t_min {
            return Some((temp, self.hit_record(ray, temp)));
        }

        None
    }

    fn hit_test(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let oc = *ray.origin() - self.center;
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - c;

        if discriminant < 0.0 {
            return false;
        }

        let root = discriminant.sqrt();
        let temp = -half_b - root;
        if temp < t_max && temp > t_min {
            return true;
        }

        let temp = -half_b + root;
        if temp < t_max && temp > t_min {
            return true;
        }

        false
    }

    pub fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3A::new(self.radius, self.radius, self.radius),
            self.center + Vec3A::new(self.radius, self.radius, self.radius),
        ))
    }

    pub fn pdf_value(&self, origin: Point3, v: Vec3A) -> f32 {
        if !self.hit_test(&Ray::new(origin, v), 0.001, f32::INFINITY) {
            return 0.0;
        }

        let cos_theta_max =
            f32::sqrt(1.0 - self.radius * self.radius / (self.center - origin).length_squared());
        let solid_angle = 2.0 * f32::consts::PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    pub fn random(&self, origin: Point3) -> Vec3A {
        let direction = self.center - origin;
        let uvw = ONB::build_from_w(direction);
        uvw.local(random_to_sphere(self.radius, direction.length_squared()))
    }

    fn calculate_uv(p: Point3) -> (f32, f32) {
        let pi = f32::consts::PI;
        let phi = f32::atan2(p.z, p.x);
        let theta = f32::asin(p.y);
        let u = 1.0 - (phi + pi) / (2.0 * pi);
        let v = (theta + pi / 2.0) / pi;
        (u, v)
    }

    fn hit_record(&self, ray: &Ray, t: f32) -> HitRecord {
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let (u, v) = Sphere::calculate_uv(p);
        HitRecord::new(
            p,
            if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            self.material.clone(),
            u,
            v,
            front_face,
        )
    }
}

fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3A {
    let r1: f32 = rand::random();
    let r2: f32 = rand::random();
    let term = radius * radius / distance_squared;
    let z = if term < 0.01 {
        1.0 - r2 * 0.5 * term
    } else {
        1.0 + r2 * (f32::sqrt(1.0 - term) - 1.0)
    };

    let phi = 2.0 * f32::consts::PI * r1;
    let (s, c) = phi.sin_cos();
    let sin_theta = f32::sqrt(1.0 - z * z);
    let x = c * sin_theta;
    let y = s * sin_theta;

    Vec3A::new(x, y, z)
}
