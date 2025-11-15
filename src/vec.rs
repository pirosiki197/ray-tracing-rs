use core::f32;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub},
};

use rand::Rng;
use wide::f32x4;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3(f32x4);

impl Vec3 {
    pub const ZERO: Vec3 = Vec3(f32x4::ZERO);

    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self(f32x4::new([e0, e1, e2, 0.0]))
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self(f32x4::new([rng.random(), rng.random(), rng.random(), 0.0]))
    }

    pub fn random_range(range: Range<f32>) -> Self {
        let mut rng = rand::rng();
        Self(f32x4::new([
            rng.random_range(range.clone()),
            rng.random_range(range.clone()),
            rng.random_range(range.clone()),
            0.0,
        ]))
    }

    pub fn radom_in_unit_sphere() -> Self {
        loop {
            let v = Vec3::random_range(-1.0..1.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit() -> Self {
        let mut rng = rand::rng();
        let a: f32 = rng.random_range(0.0..2.0 * f32::consts::PI);
        let z: f32 = rng.random_range(-1.0..1.0);
        let r: f32 = (1.0 - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn radom_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                rand::random_range(-1.0..1.0),
                rand::random_range(-1.0..1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn x(&self) -> f32 {
        self.0.as_array()[0]
    }

    pub fn y(&self) -> f32 {
        self.0.as_array()[1]
    }

    pub fn z(&self) -> f32 {
        self.0.as_array()[2]
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        let muled = *self * *rhs;
        muled.x() + muled.y() + muled.z()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        let a_yzx = f32x4::new([self.y(), self.z(), self.x(), 0.0]);
        let a_zxy = f32x4::new([self.z(), self.x(), self.y(), 0.0]);
        let b_yzx = f32x4::new([rhs.y(), rhs.z(), rhs.x(), 0.0]);
        let b_zxy = f32x4::new([rhs.z(), rhs.x(), rhs.y(), 0.0]);
        Vec3(a_yzx * b_zxy - a_zxy * b_yzx)
    }

    pub fn reflect(&self, n: &Self) -> Self {
        *self - 2.0 * self.dot(n) * *n
    }

    pub fn refract(&self, n: &Self, etai_over_etat: f32) -> Self {
        let cos_theta = -self.dot(n);
        let r_out_parallel = etai_over_etat * (*self + cos_theta * *n);
        let r_out_perp = -f32::sqrt(1.0 - r_out_parallel.length_squared()) * *n;
        r_out_parallel + r_out_perp
    }

    pub fn sqrt(&self) -> Self {
        Self(self.0.sqrt())
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        let squared = *self * *self;
        squared.x() + squared.y() + squared.z()
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3(-self.0)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= f32x4::splat(rhs);
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= f32x4::splat(rhs);
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}
