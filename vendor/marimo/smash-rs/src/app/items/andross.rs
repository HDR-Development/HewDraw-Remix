use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app7andross26get_stage_andross_positionEv"]
        pub(super) fn get_stage_andross_position() -> cpp::simd::Vector3;
    }
}

pub fn get_stage_andross_position() -> phx::Vec3 {
    unsafe {
        impl_::get_stage_andross_position().into()
    }
}