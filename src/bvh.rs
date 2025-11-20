use crate::{
    aabb::AABB,
    hittable::{Geometry, HitRecord},
    ray::Ray,
};
use std::cmp::Ordering;

pub struct BVHBranch {
    left: Geometry,
    right: Geometry,
    bx: AABB,
}

impl BVHBranch {
    pub fn build(mut objects: Vec<Geometry>) -> Geometry {
        let axis = rand::random_range(0..=2);

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => unreachable!(),
        };

        match objects.len() {
            1 => objects.pop().unwrap(),
            2 => {
                let second = objects.pop().unwrap();
                let first = objects.pop().unwrap();
                match comparator(&first, &second) {
                    Ordering::Less | Ordering::Equal => BVHBranch::create_branch(first, second),
                    Ordering::Greater => BVHBranch::create_branch(second, first),
                }
            }
            _ => {
                objects.sort_by(|a, b| comparator(a, b));
                let mid = objects.len() / 2;
                let first_half = objects.split_off(mid);
                let second_half = objects;
                BVHBranch::create_branch(
                    BVHBranch::build(first_half),
                    BVHBranch::build(second_half),
                )
            }
        }
    }

    fn create_branch(left: Geometry, right: Geometry) -> Geometry {
        let box_left = left.bounding_box().unwrap();
        let box_right = right.bounding_box().unwrap();

        Geometry::Branch(Box::new(Self {
            left,
            right,
            bx: AABB::surrounding_box(&box_left, &box_right),
        }))
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, HitRecord)> {
        if !self.bx.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);

        let t_max_for_right = hit_left.as_ref().map_or(t_max, |(t, _)| *t);
        let hit_right = self.right.hit(ray, t_min, t_max_for_right);

        hit_right.or(hit_left)
    }

    pub fn bounding_box(&self) -> Option<AABB> {
        Some(self.bx.clone())
    }
}

fn box_x_compare(a: &Geometry, b: &Geometry) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Geometry, b: &Geometry) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Geometry, b: &Geometry) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}

fn box_compare(a: &Geometry, b: &Geometry, axis: usize) -> std::cmp::Ordering {
    let box_a = a.bounding_box().unwrap();
    let box_b = b.bounding_box().unwrap();

    box_a.min().as_array()[axis].total_cmp(&box_b.min().as_array()[axis])
}
