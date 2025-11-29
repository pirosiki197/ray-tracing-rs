use std::{f32, sync::Arc};

use glam::Vec3A;

use crate::{
    hittable::Geometry,
    onb::ONB,
    rand,
    vec::{Point3, Vec3Ext},
};

pub trait PDF {
    fn value(&self, direction: Vec3A) -> f32;
    fn generate(&self) -> Vec3A;
}

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vec3A) -> Self {
        Self {
            uvw: ONB::build_from_w(w),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3A) -> f32 {
        let cosine = direction.normalize().dot(self.uvw.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / f32::consts::PI
        }
    }

    fn generate(&self) -> Vec3A {
        self.uvw.local(Vec3A::random_cosine_direction())
    }
}

pub struct HittablePDF {
    geometry: Arc<Geometry>,
    origin: Point3,
}

impl HittablePDF {
    pub fn new(geometry: Arc<Geometry>, origin: Point3) -> Self {
        Self { geometry, origin }
    }
}

impl PDF for HittablePDF {
    fn value(&self, direction: Vec3A) -> f32 {
        self.geometry.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3A {
        self.geometry.random(self.origin)
    }
}

pub struct MixturePDF {
    p: [Arc<dyn PDF>; 2],
}

impl MixturePDF {
    pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: Vec3A) -> f32 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3A {
        if rand::random() {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
