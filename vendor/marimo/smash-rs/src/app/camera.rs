use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app6camera15check_dead_areaERKN3phx8Vector3fE"]
        pub(super) fn check_dead_area(pos: *const phx::Vector3f) -> bool;
    
        #[link_name = "_ZN3app6camera22get_camera_limit_rangeEv"]
        pub(super) fn get_camera_limit_range() -> cpp::simd::Vector4;
    
        #[link_name = "_ZN3app6camera16get_camera_rangeEv"]
        pub(super) fn get_camera_range() -> cpp::simd::Vector4;
    
        #[link_name = "_ZN3app6camera13get_dead_areaEv"]
        pub(super) fn get_dead_area() -> cpp::simd::Vector4;
    
        #[link_name = "_ZN3app6camera13set_auto_zoomEb"]
        pub(super) fn set_auto_zoom(auto_zoom: bool);
    
        #[link_name = "_ZN3app6camera10target_posEv"]
        pub(super) fn target_pos() -> cpp::simd::Vector3;
    
    }
}

pub fn check_dead_area(pos: &phx::Vector3f) -> bool {
    unsafe {
        impl_::check_dead_area(pos)
    }
}

pub fn get_camera_limit_range() -> lib::Rect {
    unsafe {
        impl_::get_camera_limit_range().into()
    }
}

pub fn get_camera_range() -> lib::Rect {
    unsafe {
        impl_::get_camera_range().into()
    }
}

pub fn get_dead_area() -> lib::Rect {
    unsafe {
        impl_::get_dead_area().into()
    }
}

pub fn set_auto_zoom(auto_zoom: bool) {
    unsafe {
        impl_::set_auto_zoom(auto_zoom)
    }
}

pub fn target_pos() -> phx::Vec3 {
    unsafe {
        impl_::target_pos().into()
    }
}

