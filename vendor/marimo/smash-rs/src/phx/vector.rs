use crate::*;
use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl Vector2f {
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn mag(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.mag()
    }
}

impl From<lib::L2CValue> for Vector2f {
    fn from(val: lib::L2CValue) -> Self {
        Self {
            x: val["x"].get(),
            y: val["y"].get(),
        }
    }
}

impl From<&lib::L2CValue> for Vector2f {
    fn from(val: &lib::L2CValue) -> Self {
        Self {
            x: val["x"].get(),
            y: val["y"].get(),
        }
    }
}

impl Into<lib::L2CValue> for Vector2f {
    fn into(self) -> lib::L2CValue {
        let mut value = lib::L2CValue::new_table();
        value["x"] = self.x.into();
        value["y"] = self.y.into();
        value
    }
}

impl fmt::Display for Vector2f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "<x: {}, y: {}>", self.x, self.y)
        } else {
            write!(f, "<{}, {}>", self.x, self.y)
        }
    }
}

impl From<cpp::simd::Vector2> for Vector2f {
    fn from(other: cpp::simd::Vector2) -> Self {
        Self {
            x: other.x(),
            y: other.y(),
        }
    }
}

impl Into<cpp::simd::Vector2> for Vector2f {
    fn into(self) -> cpp::simd::Vector2 {
        cpp::simd::Vector2 {
            vec: [self.x, self.y]
        }
    }
}

impl Mul<f32> for Vector2f {
    type Output = Vector2f;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vector2f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vector2f {
    type Output = Vector2f;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vector2f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Add<Vector2f> for Vector2f {
    type Output = Vector2f;

    fn add(self, rhs: Vector2f) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector2f> for Vector2f {
    fn add_assign(&mut self, rhs: Vector2f) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2f> for Vector2f {
    type Output = Vector2f;

    fn sub(self, rhs: Vector2f) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector2f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Vector2f {
    type Output = Vector2f;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn mag(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.mag()
    }
}

impl From<lib::L2CValue> for Vector3f {
    fn from(val: lib::L2CValue) -> Self {
        Self {
            x: val["x"].get(),
            y: val["y"].get(),
            z: val["z"].get(),
        }
    }
}

impl From<&lib::L2CValue> for Vector3f {
    fn from(val: &lib::L2CValue) -> Self {
        Self {
            x: val["x"].get(),
            y: val["y"].get(),
            z: val["z"].get(),
        }
    }
}

impl Into<lib::L2CValue> for Vector3f {
    fn into(self) -> lib::L2CValue {
        let mut value = lib::L2CValue::new_table();
        value["x"] = self.x.into();
        value["y"] = self.y.into();
        value["z"] = self.z.into();
        value
    }
}

impl From<cpp::simd::Vector3> for Vector3f {
    fn from(other: cpp::simd::Vector3) -> Self {
        Self {
            x: other.x(),
            y: other.y(),
            z: other.z(),
        }
    }
}

impl Into<cpp::simd::Vector3> for Vector3f {
    fn into(self) -> cpp::simd::Vector3 {
        cpp::simd::Vector3 {
            vec: [self.x, self.y, self.z]
        }
    }
}

impl fmt::Display for Vector3f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "<x: {}, y: {}, z: {}>", self.x, self.y, self.z)
        } else {
            write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
        }
    }
}

impl Mul<f32> for Vector3f {
    type Output = Vector3f;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vector3f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vector3f {
    type Output = Vector3f;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vector3f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Add<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn add(self, rhs: Vector3f) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vector3f> for Vector3f {
    fn add_assign(&mut self, rhs: Vector3f) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn sub(self, rhs: Vector3f) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.y - rhs.z,
        }
    }
}

impl SubAssign for Vector3f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vector3f {
    type Output = Vector3f;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<lib::L2CValue> for Vector4f {
    fn from(val: lib::L2CValue) -> Self {
        Self {
            x: val["x"].get(),
            y: val["y"].get(),
            z: val["z"].get(),
            w: val["w"].get(),
        }
    }
}

impl From<&lib::L2CValue> for Vector4f {
    fn from(val: &lib::L2CValue) -> Self {
        Self {
            x: val["x"].get(),
            y: val["y"].get(),
            z: val["z"].get(),
            w: val["w"].get(),
        }
    }
}

impl Into<lib::L2CValue> for Vector4f {
    fn into(self) -> lib::L2CValue {
        let mut value = lib::L2CValue::new_table();
        value["x"] = self.x.into();
        value["y"] = self.y.into();
        value["z"] = self.z.into();
        value["w"] = self.w.into();
        value
    }
}

impl From<cpp::simd::Vector4> for Vector4f {
    fn from(other: cpp::simd::Vector4) -> Self {
        Self {
            x: other.x(),
            y: other.y(),
            z: other.z(),
            w: other.w(),
        }
    }
}

impl Into<cpp::simd::Vector4> for Vector4f {
    fn into(self) -> cpp::simd::Vector4 {
        cpp::simd::Vector4 {
            vec: [self.x, self.y, self.z, self.w]
        }
    }
}

impl Vector4f {
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn mag(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.mag()
    }
}

impl fmt::Display for Vector4f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "<x: {}, y: {}, z: {}, w: {}>",
                self.x, self.y, self.z, self.w
            )
        } else {
            write!(f, "<{}, {}, {}, {}>", self.x, self.y, self.z, self.w)
        }
    }
}

impl Mul<f32> for Vector4f {
    type Output = Vector4f;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl MulAssign<f32> for Vector4f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Div<f32> for Vector4f {
    type Output = Vector4f;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl DivAssign<f32> for Vector4f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl Add<Vector4f> for Vector4f {
    type Output = Vector4f;

    fn add(self, rhs: Vector4f) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign<Vector4f> for Vector4f {
    fn add_assign(&mut self, rhs: Vector4f) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub<Vector4f> for Vector4f {
    type Output = Vector4f;

    fn sub(self, rhs: Vector4f) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.y - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl SubAssign for Vector4f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl Neg for Vector4f {
    type Output = Vector4f;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

pub type Vec2 = Vector2f;
pub type Vec3 = Vector3f;
pub type Vec4 = Vector4f;
