#[derive(Clone, Copy)]
#[repr(simd)]
pub struct Vector2 {
    pub vec: [f32; 2]
}

impl Vector2 {
    pub fn x(self) -> f32 {
        unsafe {
            core::intrinsics::simd::simd_extract(self, 0)
        }
    }
    pub fn y(self) -> f32 {
        unsafe {
            core::intrinsics::simd::simd_extract(self, 1)
        }
    }
}

#[derive(Clone, Copy)]
#[repr(simd)]
pub struct Vector3 {
    pub vec: [f32; 3]
}

impl Vector3 {
    pub fn x(self) -> f32 {
        unsafe {
            core::intrinsics::simd::simd_extract(self, 0)
        }
    }
    pub fn y(self) -> f32 {
        unsafe {
            core::intrinsics::simd::simd_extract(self, 1)
        }
    }
    pub fn z(self) -> f32 {
        unsafe {
            core::intrinsics::simd::simd_extract(self, 2)
        }
    }
}

#[derive(Clone, Copy)]
#[repr(simd)]
pub struct Vector4 {
    pub vec: [f32; 4]
}

impl Vector4 {
    pub fn x(self) -> f32 {
        unsafe {
            core::intrinsics::simd::simd_extract(self, 0)
        }
    }
    pub fn y(self) -> f32 {
        unsafe {
            core::intrinsics::simd::simd_extract(self, 1)
        }
    }
    pub fn z(self) -> f32 {
        unsafe {
            core::intrinsics::simd::simd_extract(self, 2)
        }
    }
    pub fn w(self) -> f32 {
        unsafe {
            core::intrinsics::simd::simd_extract(self, 3)
        }
    }
}
