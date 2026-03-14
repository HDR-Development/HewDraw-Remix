use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app12dragoonsight17calc_dragoonsightEbff"]
        pub(super) fn calc_dragoonsight(arg1: bool, arg2: f32, arg3: f32) -> cpp::simd::Vector4;
    
        #[link_name = "_ZN3app12dragoonsight39get_rotation_with_calc_camera_directionEv"]
        pub(super) fn get_rotation_with_calc_camera_direction() -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app12dragoonsight23set_dragoonsight_effectERNS_18ItemModuleAccessorEb"]
        pub(super) fn set_dragoonsight_effect(module_accessor: *mut app::ItemModuleAccessor, arg: bool);
    
    }
}

pub fn calc_dragoonsight(arg1: bool, arg2: f32, arg3: f32) -> phx::Vector4f {
    unsafe {
        impl_::calc_dragoonsight(arg1, arg2, arg3).into()
    }
}

pub fn get_rotation_with_calc_camera_direction() -> phx::Vector2f {
    unsafe {
        impl_::get_rotation_with_calc_camera_direction().into()
    }
}

pub fn set_dragoonsight_effect(module_accessor: &mut app::ItemModuleAccessor, arg: bool) {
    unsafe {
        impl_::set_dragoonsight_effect(module_accessor, arg)
    }
}

