use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use std::{cmp::Ordering, sync::Arc};

pub struct BVH {
    root: Arc<BVHNode>,
}

impl BVH {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        let root = Arc::new(BVHNode::new(objects));
        Self { root }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, HitRecord)> {
        self.root.hit(&ray, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.root.bx.clone())
    }
}

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bx: AABB,
}

impl BVHNode {
    pub fn new(mut objects: Vec<Arc<dyn Hittable>>) -> Self {
        let axis = rand::random_range(0..=2);

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => unreachable!(),
        };

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        match objects.len() {
            1 => {
                left = objects[0].clone();
                right = objects[0].clone();
            }
            2 => {
                let first = objects[0].clone();
                let second = objects[1].clone();
                match comparator(first.as_ref(), second.as_ref()) {
                    Ordering::Less | Ordering::Equal => {
                        left = first;
                        right = second;
                    }
                    Ordering::Greater => {
                        left = second;
                        right = first;
                    }
                }
            }
            _ => {
                objects.sort_by(|a, b| comparator(a.as_ref(), b.as_ref()));
                let mid = objects.len() / 2;
                let first_half = objects.split_off(mid);
                let second_half = objects;
                left = Arc::new(BVHNode::new(first_half));
                right = Arc::new(BVHNode::new(second_half));
            }
        }
        let box_left = left.bounding_box().unwrap();
        let box_right = right.bounding_box().unwrap();

        Self {
            left: left,
            right: right,
            bx: AABB::surrounding_box(&box_left, &box_right),
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, HitRecord)> {
        if !self.bx.hit(&ray, t_min, t_max) {
            return None;
        }

        let left_rec = self.left.hit(&ray, t_min, t_max);
        let right_rec = self.right.hit(
            &ray,
            t_min,
            left_rec.as_ref().map(|(t, _)| *t).unwrap_or(t_max),
        );

        right_rec.or(left_rec)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bx.clone())
    }
}

fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}

fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> std::cmp::Ordering {
    let box_a = a.bounding_box().unwrap();
    let box_b = b.bounding_box().unwrap();

    box_a.min().as_array()[axis].total_cmp(&box_b.min().as_array()[axis])
}
