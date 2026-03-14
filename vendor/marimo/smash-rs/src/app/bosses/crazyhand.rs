use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app9crazyhand16get_camera_rangeEv"]
        pub(super) fn get_camera_range() -> cpp::simd::Vector4;
    
        #[link_name = "_ZN3app9crazyhand14get_dead_rangeEv"]
        pub(super) fn get_dead_range() -> cpp::simd::Vector4;
    
        #[link_name = "_ZN3app9crazyhand25get_shrinked_camera_rangeEv"]
        pub(super) fn get_shrinked_camera_range() -> cpp::simd::Vector4;
    
        #[link_name = "_ZN3app9crazyhand23get_shrinked_dead_rangeEv"]
        pub(super) fn get_shrinked_dead_range() -> cpp::simd::Vector4;
    
        #[link_name = "_ZN3app9crazyhand13revert_cameraEv"]
        pub(super) fn revert_camera();
    
        #[link_name = "_ZN3app9crazyhand16set_camera_rangeERKN3phx8Vector4fE"]
        pub(super) fn set_camera_range(range: *const lib::Rect);
    
        #[link_name = "_ZN3app9crazyhand14set_dead_rangeERKN3phx8Vector4fE"]
        pub(super) fn set_dead_range(range: *const lib::Rect);
    
        #[link_name = "_ZN3app9crazyhand14start_zoom_outEiRKN3phx8Vector4fE"]
        pub(super) fn start_zoom_out(arg: i32, range: *const lib::Rect);
    
    }
}

pub fn get_camera_range() -> lib::Rect {
    unsafe {
        impl_::get_camera_range().into()
    }
}

pub fn get_dead_range() -> lib::Rect {
    unsafe {
        impl_::get_dead_range().into()
    }
}

pub fn get_shrinked_camera_range() -> lib::Rect {
    unsafe {
        impl_::get_shrinked_camera_range().into()
    }
}

pub fn get_shrinked_dead_range() -> lib::Rect {
    unsafe {
        impl_::get_shrinked_dead_range().into()
    }
}

pub fn revert_camera() {
    unsafe {
        impl_::revert_camera()
    }
}

pub fn set_camera_range(range: &lib::Rect) {
    unsafe {
        impl_::set_camera_range(range)
    }
}

pub fn set_dead_range(range: &lib::Rect) {
    unsafe {
        impl_::set_dead_range(range)
    }
}

pub fn start_zoom_out(arg: i32, range: &lib::Rect) {
    unsafe {
        impl_::start_zoom_out(arg, range)
    }
}

