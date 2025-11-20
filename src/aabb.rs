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
        let min = self.min.as_array();
        let max = self.max.as_array();
        let origin = ray.origin().as_array();
        let direction = ray.direction().as_array();
        for a in 0..3 {
            let t0 = f32::min(
                (min[a] - origin[a]) / direction[a],
                (max[a] - origin[a]) / direction[a],
            );
            let t1 = f32::max(
                (min[a] - origin[a]) / direction[a],
                (max[a] - origin[a]) / direction[a],
            );
            let tmin = f32::max(t0, tmin);
            let tmax = f32::min(t1, tmax);
            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Point3::new(
            f32::min(box0.min.x(), box1.min.x()),
            f32::min(box0.min.y(), box1.min.y()),
            f32::min(box0.min.z(), box1.min.z()),
        );
        let big = Point3::new(
            f32::max(box0.max.x(), box1.max.x()),
            f32::max(box0.max.y(), box1.max.y()),
            f32::max(box0.max.z(), box1.max.z()),
        );
        AABB::new(small, big)
    }
}
