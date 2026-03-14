use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app4darz9can_existEv"]
        pub(super) fn can_exist() -> bool;
    
        #[link_name = "_ZN3app4darz20get_look_at_rotationERKN3phx8Vector3fES4_S4_"]
        pub(super) fn get_look_at_rotation(arg1: *const phx::Vector3f, arg2: *const phx::Vector3f, arg3: *const phx::Vector3f) -> cpp::simd::Vector3;
    
    }
}

pub fn can_exist() -> bool {
    unsafe {
        impl_::can_exist()
    }
}

pub fn get_look_at_rotation(arg1: &phx::Vector3f, arg2: &phx::Vector3f, arg3: &phx::Vector3f) -> phx::Vector3f {
    unsafe {
        impl_::get_look_at_rotation(arg1, arg2, arg3).into()
    }
}