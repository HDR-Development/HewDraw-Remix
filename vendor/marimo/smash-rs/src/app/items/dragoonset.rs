use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app10dragoonset18calc_dragoonsight2Eff"]
        pub(super) fn calc_dragoonsight2(arg1: f32, arg2: f32);
    
        #[link_name = "_ZN3app10dragoonset34get_stage_dragoon_camera_rectangleEv"]
        pub(super) fn get_stage_dragoon_camera_rectangle() -> cpp::simd::Vector4;
    
    }
}

pub fn calc_dragoonsight2(arg1: f32, arg2: f32) {
    unsafe {
        impl_::calc_dragoonsight2(arg1, arg2)
    }
}

pub fn get_stage_dragoon_camera_rectangle() -> lib::Rect {
    unsafe {
        impl_::get_stage_dragoon_camera_rectangle().into()
    }
}

