use std::sync::Arc;

use glam::Vec3A;

use crate::{
    aabb::AABB, bvh::BVHBranch, material::Material, ray::Ray, sphere::Sphere, vec::Point3,
};

pub struct HitRecord {
    p: Point3,
    normal: Vec3A,
    material: Arc<dyn Material>,
    u: f32,
    v: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3A,
        material: Arc<dyn Material>,
        u: f32,
        v: f32,
        front_face: bool,
    ) -> Self {
        HitRecord {
            p,
            normal,
            material,
            u,
            v,
            front_face,
        }
    }

    pub fn point(&self) -> Point3 {
        self.p
    }

    pub fn normal(&self) -> Vec3A {
        self.normal
    }

    pub fn material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    pub fn u(&self) -> f32 {
        self.u
    }

    pub fn v(&self) -> f32 {
        self.v
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub struct HittableList {
    objects: Vec<Geometry>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, obj: Geometry) {
        self.objects.push(obj);
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, HitRecord)> {
        let (t, hit_record) =
            self.objects
                .iter()
                .fold((t_max, None), |(closest_t, base_hit), obj| {
                    if let Some((t, rec)) = obj.hit(&ray, t_min, closest_t) {
                        (t, Some(rec))
                    } else {
                        (closest_t, base_hit)
                    }
                });
        hit_record.map(|rec| (t, rec))
    }

    pub fn bounding_box(&self) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut first_box = true;
        let mut res = AABB::new(Vec3A::ZERO, Vec3A::ZERO);
        for object in &self.objects {
            let bbox = object.bounding_box()?;
            res = if first_box {
                bbox
            } else {
                AABB::surrounding_box(&res, &bbox)
            };
            first_box = false;
        }

        Some(res)
    }
}

pub enum Geometry {
    Sphere(Sphere),
    Branch(Box<BVHBranch>),
}

impl Geometry {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, HitRecord)> {
        match self {
            Geometry::Sphere(s) => s.hit(ray, t_min, t_max),
            Geometry::Branch(n) => n.hit(ray, t_min, t_max),
        }
    }

    pub fn bounding_box(&self) -> Option<AABB> {
        match self {
            Geometry::Sphere(s) => s.bounding_box(),
            Geometry::Branch(n) => n.bounding_box(),
        }
    }
}
