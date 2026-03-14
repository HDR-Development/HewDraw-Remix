use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app5devil14IT_MOVE_CAMERAERNS_18ItemModuleAccessorEbff"]
        pub(super) fn IT_MOVE_CAMERA(module_accessor: *mut app::ItemModuleAccessor, is_add: bool, x: f32, y: f32);
    
        #[link_name = "_ZN3app5devil23get_camera_devil_offsetEv"]
        pub(super) fn get_camera_devil_offset() -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app5devil24get_stage_devil_positionEv"]
        pub(super) fn get_stage_devil_position() -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app5devil20is_camera_mode_meleeEv"]
        pub(super) fn is_camera_mode_melee() -> bool;
    
        #[link_name = "_ZN3app5devil25set_camera_devil_fix_modeEb"]
        pub(super) fn set_camera_devil_fix_mode(is_fixed: bool);
    
        #[link_name = "_ZN3app5devil33set_camera_devil_interpolate_rateEf"]
        pub(super) fn set_camera_devil_interpolate_rate(rate: f32);
    
        #[link_name = "_ZN3app5devil21set_camera_devil_modeEb"]
        pub(super) fn set_camera_devil_mode(mode: bool);
    
        #[link_name = "_ZN3app5devil23set_camera_devil_offsetEff"]
        pub(super) fn set_camera_devil_offset(x: f32, y: f32);
    
    }
}

#[allow(non_snake_case)]
pub fn IT_MOVE_CAMERA(module_accessor: &mut app::ItemModuleAccessor, is_add: bool, x: f32, y: f32) {
    unsafe {
        impl_::IT_MOVE_CAMERA(module_accessor, is_add, x, y)
    }
}

pub fn get_camera_devil_offset() -> phx::Vector2f {
    unsafe {
        impl_::get_camera_devil_offset().into()
    }
}

pub fn get_stage_devil_position() -> phx::Vector2f {
    unsafe {
        impl_::get_stage_devil_position().into()
    }
}

pub fn is_camera_mode_melee() -> bool {
    unsafe {
        impl_::is_camera_mode_melee()
    }
}

pub fn set_camera_devil_fix_mode(is_fixed: bool) {
    unsafe {
        impl_::set_camera_devil_fix_mode(is_fixed)
    }
}

pub fn set_camera_devil_interpolate_rate(rate: f32) {
    unsafe {
        impl_::set_camera_devil_interpolate_rate(rate)
    }
}

pub fn set_camera_devil_mode(mode: bool) {
    unsafe {
        impl_::set_camera_devil_mode(mode)
    }
}

pub fn set_camera_devil_offset(x: f32, y: f32) {
    unsafe {
        impl_::set_camera_devil_offset(x, y)
    }
}

