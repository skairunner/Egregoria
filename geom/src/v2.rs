use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub};

#[derive(Copy, Clone, PartialEq, Default, Serialize, Deserialize, Debug)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::iter::Sum for Vec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut z = Vec2::ZERO;
        for x in iter {
            z += x;
        }
        z
    }
}

impl<'a> std::iter::Sum<&'a Vec2> for Vec2 {
    fn sum<I: Iterator<Item = &'a Vec2>>(iter: I) -> Self {
        let mut z = Vec2::ZERO;
        for &x in iter {
            z += x;
        }
        z
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl From<Vec2> for [f32; 2] {
    fn from(v: Vec2) -> Self {
        [v.x, v.y]
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(v: [f32; 2]) -> Self {
        Self { x: v[0], y: v[1] }
    }
}

impl From<Vec2> for mint::Point2<f32> {
    fn from(v: Vec2) -> Self {
        mint::Point2 { x: v.x, y: v.y }
    }
}

impl From<mint::Point2<f32>> for Vec2 {
    fn from(v: mint::Point2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Vec2> for mint::Vector2<f32> {
    fn from(v: Vec2) -> Self {
        mint::Vector2 { x: v.x, y: v.y }
    }
}

impl From<mint::Vector2<f32>> for Vec2 {
    fn from(v: mint::Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl Vec2 {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const UNIT_X: Self = Self { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Self = Self { x: 0.0, y: 1.0 };

    #[inline]
    pub fn perpendicular(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    #[inline]
    pub fn magnitude(self) -> f32 {
        self.magnitude2().sqrt()
    }

    #[inline]
    pub fn magnitude2(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline]
    pub fn perp_dot(self, rhs: Self) -> f32 {
        self.dot(rhs.perpendicular())
    }

    #[inline]
    pub fn distance2(self, rhs: Self) -> f32 {
        (self - rhs).magnitude2()
    }

    #[inline]
    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).magnitude()
    }

    #[inline]
    pub fn cossin_angle(self, other: Vec2) -> Vec2 {
        let s = self.normalize();
        let o = other.normalize();
        s * o - s * o.perpendicular()
    }

    #[inline]
    pub fn angle(self, other: Vec2) -> f32 {
        f32::atan2(Self::perp_dot(self, other), Self::dot(self, other))
    }

    #[inline]
    pub fn try_normalize(self) -> Option<Vec2> {
        let m = self.magnitude();
        if m > std::f32::EPSILON {
            Some(self / m)
        } else {
            None
        }
    }

    #[inline]
    pub fn normalize(self) -> Vec2 {
        let m = self.magnitude();
        self / m
    }

    #[inline]
    pub fn normalize_to(self, v: f32) -> Vec2 {
        let m = self.magnitude();
        self * (v / m)
    }

    #[inline]
    pub fn dir_dist(self) -> Option<(Vec2, f32)> {
        let m = self.magnitude();
        if m > 0.0 {
            Some((self / m, m))
        } else {
            None
        }
    }

    #[inline]
    pub fn min(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    #[inline]
    pub fn max(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    #[inline]
    pub fn cap_magnitude(self, max: f32) -> Vec2 {
        let m = self.magnitude();
        if m > max {
            self * max / m
        } else {
            self
        }
    }

    #[inline]
    pub fn approx_eq(self, other: Vec2) -> bool {
        let m = self.distance2(other);
        m < std::f32::EPSILON
    }

    #[inline]
    pub fn rotated_by(self, cossin: Vec2) -> Self {
        self.x * cossin - self.y * cossin.perpendicular()
    }
}
