use glam::Vec3A;

use crate::vec::Point3;

pub struct Ray {
    orig: Point3,
    dir: Vec3A,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3A) -> Self {
        Self {
            orig: origin,
            dir: direction.normalize(),
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn direction(&self) -> Vec3A {
        self.dir
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
}
