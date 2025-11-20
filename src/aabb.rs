use crate::{ray::Ray, vec::Point3};

#[derive(Clone)]
pub struct AABB {
    min: Point3,
    max: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self { min: a, max: b }
    }

    pub fn min(&self) -> Point3 {
        self.min
    }

    pub fn max(&self) -> Point3 {
        self.max
    }

    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        let t0 = (self.min - ray.origin()) / ray.direction();
        let t1 = (self.max - ray.origin()) / ray.direction();

        let tmin_vec = t0.min(t1);
        let tmax_vec = t0.max(t1);

        let t_enter = tmin_vec.max_element().max(tmin);
        let t_exit = tmax_vec.min_element().min(tmax);

        t_enter < t_exit
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Point3::new(
            f32::min(box0.min.x, box1.min.x),
            f32::min(box0.min.y, box1.min.y),
            f32::min(box0.min.z, box1.min.z),
        );
        let big = Point3::new(
            f32::max(box0.max.x, box1.max.x),
            f32::max(box0.max.y, box1.max.y),
            f32::max(box0.max.z, box1.max.z),
        );
        AABB::new(small, big)
    }
}
