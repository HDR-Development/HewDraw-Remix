use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app4doll22DOLL_HIT_DATA_OFFSET_1Ev"]
        pub(super) fn DOLL_HIT_DATA_OFFSET_1() -> cpp::simd::Vector3;
    
        #[link_name = "_ZN3app4doll22DOLL_HIT_DATA_OFFSET_2Ev"]
        pub(super) fn DOLL_HIT_DATA_OFFSET_2() -> cpp::simd::Vector3;
    
        #[link_name = "_ZN3app4doll18DOLL_HIT_DATA_SIZEEv"]
        pub(super) fn DOLL_HIT_DATA_SIZE() -> f32;
    
        #[link_name = "_ZN3app4doll9DOLL_LIFEEv"]
        pub(super) fn DOLL_LIFE() -> f32;
    
        #[link_name = "_ZN3app4doll17DOLL_LIFE_DEC_MULEv"]
        pub(super) fn DOLL_LIFE_DEC_MUL() -> f32;
    
        #[link_name = "_ZN3app4doll17DOLL_MAX_DEGREE_YEv"]
        pub(super) fn DOLL_MAX_DEGREE_Y() -> f32;
    
        #[link_name = "_ZN3app4doll17DOLL_MIN_DEGREE_YEv"]
        pub(super) fn DOLL_MIN_DEGREE_Y() -> f32;
    
        #[link_name = "_ZN3app4doll14DOLL_POWER_MAXEv"]
        pub(super) fn DOLL_POWER_MAX() -> f32;
    
        #[link_name = "_ZN3app4doll14DOLL_POWER_MINEv"]
        pub(super) fn DOLL_POWER_MIN() -> f32;
    
        #[link_name = "_ZN3app4doll17DOLL_REACTION_MULEv"]
        pub(super) fn DOLL_REACTION_MUL() -> f32;
    
        #[link_name = "_ZN3app4doll18DOLL_ROT_SPEED_MAXEv"]
        pub(super) fn DOLL_ROT_SPEED_MAX() -> f32;
    
        #[link_name = "_ZN3app4doll18DOLL_ROT_SPEED_MINEv"]
        pub(super) fn DOLL_ROT_SPEED_MIN() -> f32;
    
        #[link_name = "_ZN3app4doll16DOLL_ROT_SPEED_ZEv"]
        pub(super) fn DOLL_ROT_SPEED_Z() -> f32;
    
        #[link_name = "_ZN3app4doll16DOLL_ROT_Z_RATIOEv"]
        pub(super) fn DOLL_ROT_Z_RATIO() -> f32;
    
        #[link_name = "_ZN3app4doll15DOLL_SHAPE_TYPEEv"]
        pub(super) fn DOLL_SHAPE_TYPE() -> i32;
    
        #[link_name = "_ZN3app4doll18DOLL_SMASH_ACCEL_YEv"]
        pub(super) fn DOLL_SMASH_ACCEL_Y() -> f32;
    
        #[link_name = "_ZN3app4doll22DOLL_SMASH_MAX_SPEED_XEv"]
        pub(super) fn DOLL_SMASH_MAX_SPEED_X() -> f32;
    
        #[link_name = "_ZN3app4doll22DOLL_SMASH_MAX_SPEED_YEv"]
        pub(super) fn DOLL_SMASH_MAX_SPEED_Y() -> f32;
    
        #[link_name = "_ZN3app4doll12DOLL_SPEED_YEv"]
        pub(super) fn DOLL_SPEED_Y() -> f32;
    
    }
}

pub fn DOLL_HIT_DATA_OFFSET_1() -> cpp::simd::Vector3 {
    unsafe {
        impl_::DOLL_HIT_DATA_OFFSET_1()
    }
}

pub fn DOLL_HIT_DATA_OFFSET_2() -> cpp::simd::Vector3 {
    unsafe {
        impl_::DOLL_HIT_DATA_OFFSET_2()
    }
}

pub fn DOLL_HIT_DATA_SIZE() -> f32 {
    unsafe {
        impl_::DOLL_HIT_DATA_SIZE()
    }
}

pub fn DOLL_LIFE() -> f32 {
    unsafe {
        impl_::DOLL_LIFE()
    }
}

pub fn DOLL_LIFE_DEC_MUL() -> f32 {
    unsafe {
        impl_::DOLL_LIFE_DEC_MUL()
    }
}

pub fn DOLL_MAX_DEGREE_Y() -> f32 {
    unsafe {
        impl_::DOLL_MAX_DEGREE_Y()
    }
}

pub fn DOLL_MIN_DEGREE_Y() -> f32 {
    unsafe {
        impl_::DOLL_MIN_DEGREE_Y()
    }
}

pub fn DOLL_POWER_MAX() -> f32 {
    unsafe {
        impl_::DOLL_POWER_MAX()
    }
}

pub fn DOLL_POWER_MIN() -> f32 {
    unsafe {
        impl_::DOLL_POWER_MIN()
    }
}

pub fn DOLL_REACTION_MUL() -> f32 {
    unsafe {
        impl_::DOLL_REACTION_MUL()
    }
}

pub fn DOLL_ROT_SPEED_MAX() -> f32 {
    unsafe {
        impl_::DOLL_ROT_SPEED_MAX()
    }
}

pub fn DOLL_ROT_SPEED_MIN() -> f32 {
    unsafe {
        impl_::DOLL_ROT_SPEED_MIN()
    }
}

pub fn DOLL_ROT_SPEED_Z() -> f32 {
    unsafe {
        impl_::DOLL_ROT_SPEED_Z()
    }
}

pub fn DOLL_ROT_Z_RATIO() -> f32 {
    unsafe {
        impl_::DOLL_ROT_Z_RATIO()
    }
}

pub fn DOLL_SHAPE_TYPE() -> i32 {
    unsafe {
        impl_::DOLL_SHAPE_TYPE()
    }
}

pub fn DOLL_SMASH_ACCEL_Y() -> f32 {
    unsafe {
        impl_::DOLL_SMASH_ACCEL_Y()
    }
}

pub fn DOLL_SMASH_MAX_SPEED_X() -> f32 {
    unsafe {
        impl_::DOLL_SMASH_MAX_SPEED_X()
    }
}

pub fn DOLL_SMASH_MAX_SPEED_Y() -> f32 {
    unsafe {
        impl_::DOLL_SMASH_MAX_SPEED_Y()
    }
}

pub fn DOLL_SPEED_Y() -> f32 {
    unsafe {
        impl_::DOLL_SPEED_Y()
    }
}

