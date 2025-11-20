use core::f32;
use std::ops::Range;

use glam::Vec3A;
use rand::Rng;

pub type Point3 = Vec3A;
pub type Color = Vec3A;

pub trait Vec3Ext {
    fn random_in_unit_disk() -> Self;
    fn random_unit() -> Self;
    fn radom_in_unit_sphere() -> Self;
    fn random_range(r: Range<f32>) -> Self;
}

impl Vec3Ext for Vec3A {
    fn random_range(range: Range<f32>) -> Self {
        Vec3A::new(
            rand::random_range(range.clone()),
            rand::random_range(range.clone()),
            rand::random_range(range.clone()),
        )
    }
    fn radom_in_unit_sphere() -> Self {
        loop {
            let v = Vec3A::new(
                rand::random_range(-1.0..1.0),
                rand::random_range(-1.0..1.0),
                rand::random_range(-1.0..1.0),
            );
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    fn random_unit() -> Self {
        let mut rng = rand::rng();
        let a: f32 = rng.random_range(0.0..2.0 * f32::consts::PI);
        let z: f32 = rng.random_range(-1.0..1.0);
        let r: f32 = (1.0 - z * z).sqrt();
        Vec3A::new(r * a.cos(), r * a.sin(), z)
    }

    fn random_in_unit_disk() -> Self {
        loop {
            let v = Vec3A::new(
                rand::random_range(-1.0..1.0),
                rand::random_range(-1.0..1.0),
                0.0,
            );
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }
}
