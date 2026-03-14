use crate::*;

mod impl_ {
    use crate::*;
    extern "C" {
        #[link_name = "_ZN3app8ai_debug20set_mode_action_nameEP9lua_StatePKc"]
        pub(super) fn set_mode_action_name(state: *mut lua_State, action_name: *const skyline::libc::c_char);
    
        #[link_name = "_ZN3app8ai_debug13draw_line_redERKN3phx8Vector2fES4_i"]
        pub(super) fn draw_line_red(line_a: *const phx::Vector2f, line_b: *const phx::Vector2f, index: i32);
    
        #[link_name = "_ZN3app8ai_debug15draw_line_greenERKN3phx8Vector2fES4_i"]
        pub(super) fn draw_line_green(line_a: *const phx::Vector2f, line_b: *const phx::Vector2f, index: i32);
    
        #[link_name = "_ZN3app8ai_debug15draw_circle_redERKN3phx8Vector2fEfi"]
        pub(super) fn draw_circle_red(center: *const phx::Vector2f, radius: f32, index: i32);
    
        #[link_name = "_ZN3app8ai_debug17draw_circle_greenERKN3phx8Vector2fEfi"]
        pub(super) fn draw_circle_green(center: *const phx::Vector2f, radius: f32, index: i32);
    }
}

pub fn set_mode_action_name(state: *mut lua_State, action_name: &str) {
    unsafe {
        impl_::set_mode_action_name(state, [action_name, "\0"].concat().as_bytes().as_ptr())
    }
}

pub fn draw_line_red(line_a: &phx::Vector2f, line_b: &phx::Vector2f, index: i32) {
    unsafe {
        impl_::draw_line_red(line_a, line_b, index)
    }
}

pub fn draw_line_green(line_a: &phx::Vector2f, line_b: &phx::Vector2f, index: i32) {
    unsafe {
        impl_::draw_line_green(line_a, line_b, index)
    }
}

pub fn draw_circle_red(center: &phx::Vector2f, radius: f32, index: i32) {
    unsafe {
        impl_::draw_circle_red(center, radius, index)
    }
}

pub fn draw_circle_green(center: &phx::Vector2f, radius: f32, index: i32) {
    unsafe {
        impl_::draw_circle_green(center, radius, index)
    }
}