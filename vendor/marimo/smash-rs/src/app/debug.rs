use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app5debug17add_debug_commandEP9lua_StatePKci"]
        pub(super) fn add_debug_command(state: *mut lua_State, name: *const u8, arg: i32);
    
        #[link_name = "_ZN3app5debug9draw_lineEP9lua_StateRKN3phx8Vector2fES6_i"]
        pub(super) fn draw_line(state: *mut lua_State, pos_a: *const phx::Vector2f, pos_b: *const phx::Vector2f, frames: i32);
    
        #[link_name = "_ZN3app5debug9log_fatalEPKc"]
        pub(super) fn log_fatal(text: *const u8);
    
        #[link_name = "_ZN3app5debug8log_infoEPKc"]
        pub(super) fn log_info(text: *const u8);
    
        #[link_name = "_ZN3app5debug14set_draw_colorEP9lua_Stateffff"]
        pub(super) fn set_draw_color(state: *mut lua_State, r: f32, g: f32, b: f32, a: f32);
    
    }
}

pub fn add_debug_command(state: *mut lua_State, name: &str, arg: i32) {
    unsafe {
        impl_::add_debug_command(state, c_str!(name), arg)
    }
}

pub fn draw_line(state: *mut lua_State, pos_a: &phx::Vector2f, pos_b: &phx::Vector2f, frames: i32) {
    unsafe {
        impl_::draw_line(state, pos_a, pos_b, frames)
    }
}

pub fn log_fatal(text: &str) {
    unsafe {
        impl_::log_fatal(c_str!(text))
    }
}

pub fn log_info(text: &str) {
    unsafe {
        impl_::log_info(c_str!(text))
    }
}

pub fn set_draw_color(state: *mut lua_State, r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        impl_::set_draw_color(state, r, g, b, a)
    }
}

