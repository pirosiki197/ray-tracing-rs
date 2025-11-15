use std::rc::Rc;

use crate::{
    ray::Ray,
    vec::{Point3, Vec3},
};

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f32, front_face: bool) -> Self {
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (_, hit_record) =
            self.objects
                .iter()
                .fold((t_max, None), |(closest_t, base_hit), obj| {
                    if let Some(rec) = obj.hit(&ray, t_min, closest_t) {
                        (rec.t, Some(rec))
                    } else {
                        (closest_t, base_hit)
                    }
                });
        hit_record
    }
}
