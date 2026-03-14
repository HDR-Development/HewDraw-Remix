use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app5stage11back_line_zEv"]
        pub(super) fn back_line_z() -> f32;

        #[link_name = "_ZN3app5stage24calc_offset_with_gravityERKN3phx8Vector2fERKNS1_8Vector3fE"]
        pub(super) fn calc_offset_with_gravity(offset: *const phx::Vector2f, position: *const phx::Vector3f) -> cpp::simd::Vector2;

        #[link_name = "_ZN3app5stage20get_gravity_positionEv"]
        pub(super) fn get_gravity_position() -> cpp::simd::Vector3;

        #[link_name = "_ZN3app5stage18get_smashball_rectEv"]
        pub(super) fn get_smashball_rect() -> cpp::simd::Vector4;

        #[link_name = "_ZN3app5stage12get_stage_idEv"]
        pub(super) fn get_stage_id() -> i32;

        #[link_name = "_ZN3app5stage24get_stage_position_deltaEv"]
        pub(super) fn get_stage_position_delta() -> cpp::simd::Vector3;

        #[link_name = "_ZN3app5stage31is_exist_item_generate_positionEv"]
        pub(super) fn is_exist_item_generate_position() -> bool;

        #[link_name = "_ZN3app5stage13is_flat_stageEv"]
        pub(super) fn is_flat_stage() -> bool;

        #[link_name = "_ZN3app5stage26is_looking_at_stage_centerEP9lua_State"]
        pub(super) fn is_looking_at_stage_center(state: *mut lua_State) -> bool;

        #[link_name = "_ZN3app5stage17is_normal_gravityEv"]
        pub(super) fn is_normal_gravity() -> bool;

        #[link_name = "_ZN3app5stage32is_the_stage_needs_render_offsetEv"]
        pub(super) fn is_the_stage_needs_render_offset() -> bool;

        #[link_name = "_ZN3app5stage22item_generate_positionEv"]
        pub(super) fn item_generate_position() -> cpp::simd::Vector3;

        #[link_name = "_ZN3app5stage30item_generate_position_in_rectEffffRN3phx8Vector3fE"]
        pub(super) fn item_generate_position_in_rect(arg1: f32, arg2: f32, arg3: f32, arg4: f32, arg5: *const phx::Vector3f) -> cpp::simd::Vector3;

        #[link_name = "_ZN3app5stage30item_generate_position_nearestERKN3phx8Vector3fEi"]
        pub(super) fn item_generate_position_nearest(arg1: *const phx::Vector3f, arg2: i32) -> cpp::simd::Vector3;

        #[link_name = "_ZN3app5stage18lr_to_stage_centerEP9lua_State"]
        pub(super) fn lr_to_stage_center(state: *mut lua_State) -> f32;
    }
}

pub fn back_line_z() -> f32 {
    unsafe {
        impl_::back_line_z()
    }
}

pub fn calc_offset_with_gravity(offset: &phx::Vector2f, position: &phx::Vector3f) -> phx::Vec2 {
    unsafe {
        impl_::calc_offset_with_gravity(offset, position).into()
    }
}

pub fn get_gravity_position() -> phx::Vector3f {
    unsafe {
        impl_::get_gravity_position().into()
    }
}

pub fn get_smashball_rect() -> lib::Rect {
    unsafe {
        impl_::get_smashball_rect().into()
    }
}

pub fn get_stage_id() -> i32 {
    unsafe {
        impl_::get_stage_id()
    }
}

pub fn get_stage_position_delta() -> phx::Vector3f {
    unsafe {
        impl_::get_stage_position_delta().into()
    }
}

pub fn is_exist_item_generate_position() -> bool {
    unsafe {
        impl_::is_exist_item_generate_position()
    }
}

pub fn is_flat_stage() -> bool {
    unsafe {
        impl_::is_flat_stage()
    }
}

pub fn is_looking_at_stage_center(state: &mut lua_State) -> bool {
    unsafe {
        impl_::is_looking_at_stage_center(state)
    }
}

pub fn is_normal_gravity() -> bool {
    unsafe {
        impl_::is_normal_gravity()
    }
}

pub fn is_the_stage_needs_render_offset() -> bool {
    unsafe {
        impl_::is_the_stage_needs_render_offset()
    }
}

pub fn item_generate_position() -> phx::Vector3f {
    unsafe {
        impl_::item_generate_position().into()
    }
}

pub fn item_generate_position_in_rect(arg1: f32, arg2: f32, arg3: f32, arg4: f32, arg5: &phx::Vector3f) -> phx::Vector3f {
    unsafe {
        impl_::item_generate_position_in_rect(arg1, arg2, arg3, arg4, arg5).into()
    }
}

pub fn item_generate_position_nearest(arg1: &phx::Vector3f, arg2: i32) -> phx::Vector3f {
    unsafe {
        impl_::item_generate_position_nearest(arg1, arg2).into()
    }
}

pub fn lr_to_stage_center(state: &mut lua_State) -> f32 {
    unsafe {
        impl_::lr_to_stage_center(state)
    }
}
