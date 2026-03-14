use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app8ai_stage24calc_offset_with_gravityERKN3phx8Vector2fERKNS1_8Vector3fE"]
        pub(super) fn calc_offset_with_gravity(offset: *const phx::Vector2f, position: *const phx::Vector3f) -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app8ai_stage10current_idEv"]
        pub(super) fn current_id() -> i32;
    
        #[link_name = "_ZN3app8ai_stage7is_1on1Ev"]
        pub(super) fn is_1on1() -> bool;
    
        #[link_name = "_ZN3app8ai_stage10is_gimmickEv"]
        pub(super) fn is_gimmick() -> bool;
    
        #[link_name = "_ZN3app8ai_stage10is_gravityEv"]
        pub(super) fn is_gravity() -> bool;
    
        #[link_name = "_ZN3app8ai_stage21is_in_norfair_capsuleEP9lua_State"]
        pub(super) fn is_in_norfair_capsule(state: *mut lua_State) -> bool;
    
        #[link_name = "_ZN3app8ai_stage16is_in_transitionEP9lua_State"]
        pub(super) fn is_in_transition(state: *mut lua_State) -> bool;
    
        #[link_name = "_ZN3app8ai_stage8is_smallEv"]
        pub(super) fn is_small() -> bool;
    
        #[link_name = "_ZN3app8ai_stage9is_smoothEv"]
        pub(super) fn is_smooth() -> bool;
    
        #[link_name = "_ZN3app8ai_stage8scroll_xEv"]
        pub(super) fn scroll_x() -> f32;
    
    }
}

pub fn calc_offset_with_gravity(offset: &phx::Vector2f, position: &phx::Vector3f) -> phx::Vec2 {
    unsafe {
        impl_::calc_offset_with_gravity(offset, position).into()
    }
}

pub fn current_id() -> i32 {
    unsafe {
        impl_::current_id()
    }
}

pub fn is_1on1() -> bool {
    unsafe {
        impl_::is_1on1()
    }
}

pub fn is_gimmick() -> bool {
    unsafe {
        impl_::is_gimmick()
    }
}

pub fn is_gravity() -> bool {
    unsafe {
        impl_::is_gravity()
    }
}

pub fn is_in_norfair_capsule(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_in_norfair_capsule(state)
    }
}

pub fn is_in_transition(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_in_transition(state)
    }
}

pub fn is_small() -> bool {
    unsafe {
        impl_::is_small()
    }
}

pub fn is_smooth() -> bool {
    unsafe {
        impl_::is_smooth()
    }
}

pub fn scroll_x() -> f32 {
    unsafe {
        impl_::scroll_x()
    }
}

