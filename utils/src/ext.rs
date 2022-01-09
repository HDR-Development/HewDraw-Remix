use smash::phx::*;

pub trait Vec2Ext {
    fn new(x: f32, y: f32) -> Self where Self: Sized;
}

pub trait Vec3Ext {
    fn new(x: f32, y: f32, z: f32) -> Self where Self: Sized;
}

pub trait Vec4Ext {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self where Self: Sized;
}

impl Vec2Ext for Vector2f {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Vec3Ext for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

impl Vec4Ext for Vector4f {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x,
            y,
            z,
            w
        }
    }
}