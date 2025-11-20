use std::sync::Arc;

use crate::{
    aabb::AABB,
    material::Material,
    ray::Ray,
    vec::{Point3, Vec3},
};

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    material: Arc<dyn Material>,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, material: Arc<dyn Material>, front_face: bool) -> Self {
        HitRecord {
            p,
            normal,
            material,
            front_face,
        }
    }

    pub fn point(&self) -> &Point3 {
        &self.p
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, HitRecord)>;
    fn bounding_box(&self) -> Option<AABB> {
        None
    }
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, HitRecord)> {
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

    fn bounding_box(&self) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut first_box = true;
        let mut res = AABB::new(Vec3::ZERO, Vec3::ZERO);
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
