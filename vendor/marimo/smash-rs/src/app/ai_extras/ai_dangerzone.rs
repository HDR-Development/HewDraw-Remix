use crate::*;

mod impl_ {
    use crate::*;
    
    extern "C" {
        #[link_name = "_ZN3app13ai_dangerzone8is_existEv"]
        pub(super) fn is_exist() -> bool;
    
        #[link_name = "_ZN3app13ai_dangerzone18check_line_segmentEP9lua_StateRKN3phx8Vector2fES6_"]
        pub(super) fn check_line_segment(state: *mut lua_State, line_a: *const phx::Vector2f, line_b: *const phx::Vector2f) -> bool;
    
        #[link_name = "_ZN3app13ai_dangerzone10center_posEi"]
        pub(super) fn center_pos(index: i32) -> cpp::simd::Vector4;
    
        #[link_name = "_ZN3app13ai_dangerzone5widthEi"]
        pub(super) fn width(index: i32) -> f32;
    }
}

pub fn is_exist() -> bool {
    unsafe {
        impl_::is_exist()
    }
}

pub fn check_line_segment(state: *mut lua_State, line_a: &phx::Vector2f, line_b: &phx::Vector2f) -> bool {
    unsafe {
        impl_::check_line_segment(state, line_a, line_b)
    }
}

pub fn center_pos(index: i32) -> phx::Vec4 {
    unsafe {
        impl_::center_pos(index).into()
    }
}

pub fn width(index: i32) -> f32 {
    unsafe {
        impl_::width(index)
    }
}
