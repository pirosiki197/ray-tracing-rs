use glam::Vec3A;

pub struct ONB {
    pub u: Vec3A,
    pub v: Vec3A,
    pub w: Vec3A,
}

impl ONB {
    pub fn build_from_w(n: Vec3A) -> Self {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 {
            Vec3A::new(0.0, 1.0, 0.0)
        } else {
            Vec3A::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).normalize();
        let u = w.cross(v);
        Self { u, v, w }
    }

    pub fn local(&self, a: Vec3A) -> Vec3A {
        a.x * self.u + a.y * self.v + a.z * self.w
    }
}
