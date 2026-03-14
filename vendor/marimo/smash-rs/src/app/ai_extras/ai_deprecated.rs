use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app13ai_deprecated8range_inEP9lua_Stateff"]
        pub(super) fn range_in(state: *mut lua_State, range_min: f32, range_max: f32) -> bool;
    
        #[link_name = "_ZN3app13ai_deprecated11distance_inEP9lua_Stateff"]
        pub(super) fn distance_in(state: *mut lua_State, distance_min: f32, distance_max: f32) -> bool;
    
        #[link_name = "_ZN3app13ai_deprecated8other_inEP9lua_Stateff"]
        pub(super) fn other_in(state: *mut lua_State, other_min: f32, other_max: f32) -> bool;
    
        #[link_name = "_ZN3app13ai_deprecated17get_target_vectorEP9lua_Statef"]
        pub(super) fn get_target_vector(state: *mut lua_State, scale: f32) -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app13ai_deprecated13get_safe_fallEP9lua_State"]
        pub(super) fn get_safe_fall(state: *mut lua_State) -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app13ai_deprecated21get_safe_fall_currentEP9lua_State"]
        pub(super) fn get_safe_fall_current(state: *mut lua_State) -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app13ai_deprecated18find_meteor_targetEP9lua_State"]
        pub(super) fn find_meteor_target(state: *mut lua_State) -> bool;
    
    }
}

pub fn range_in(state: *mut lua_State, range_min: f32, range_max: f32) -> bool {
    unsafe {
        impl_::range_in(state, range_min, range_max)
    }
}

pub fn distance_in(state: *mut lua_State, distance_min: f32, distance_max: f32) -> bool {
    unsafe {
        impl_::distance_in(state, distance_min, distance_max)
    }
}

pub fn other_in(state: *mut lua_State, other_min: f32, other_max: f32) -> bool {
    unsafe {
        impl_::other_in(state, other_min, other_max)
    }
}

pub fn get_target_vector(state: *mut lua_State, scale: f32) -> phx::Vec2 {
    unsafe {
        impl_::get_target_vector(state, scale).into()
    }
}

pub fn get_safe_fall(state: *mut lua_State) -> phx::Vec2 {
    unsafe {
        impl_::get_safe_fall(state).into()
    }
}

pub fn get_safe_fall_current(state: *mut lua_State) -> phx::Vec2 {
    unsafe {
        impl_::get_safe_fall_current(state).into()
    }
}

pub fn find_meteor_target(state: *mut lua_State) -> bool {
    unsafe {
        impl_::find_meteor_target(state)
    }
}

