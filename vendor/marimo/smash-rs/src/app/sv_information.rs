use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app14sv_information16damage_log_valueEP9lua_State"]
        pub(super) fn damage_log_value(state: *mut lua_State) -> i32;

        #[link_name = "_ZN3app14sv_information24dead_up_camera_hit_countEv"]
        pub(super) fn dead_up_camera_hit_count() -> u32;

        #[link_name = "_ZN3app14sv_information12dead_up_typeEv"]
        pub(super) fn dead_up_type() -> i32;

        #[link_name = "_ZN3app14sv_information15get_delta_frameEv"]
        pub(super) fn get_delta_frame() -> f32;

        #[link_name = "_ZN3app14sv_information27get_remaining_time_as_frameEv"]
        pub(super) fn get_remaining_time_as_frame() -> u32;

        #[link_name = "_ZN3app14sv_information22is_dead_up_force_crushEP9lua_State"]
        pub(super) fn is_dead_up_force_crush(state: *mut lua_State) -> bool;

        #[link_name = "_ZN3app14sv_information20is_disable_lock_downEv"]
        pub(super) fn is_disable_lock_down() -> bool;

        #[link_name = "_ZN3app14sv_information21is_enabled_time_limitEv"]
        pub(super) fn is_enabled_time_limit() -> bool;

        #[link_name = "_ZN3app14sv_information13is_flat_stageEv"]
        pub(super) fn is_flat_stage() -> bool;

        #[link_name = "_ZN3app14sv_information20is_master_hand_stageEv"]
        pub(super) fn is_master_hand_stage() -> bool;

        #[link_name = "_ZN3app14sv_information11is_ready_goEv"]
        pub(super) fn is_ready_go() -> bool;

        #[link_name = "_ZN3app14sv_information22is_stage_can_dead_backEv"]
        pub(super) fn is_stage_can_dead_back() -> bool;

        #[link_name = "_ZN3app14sv_information23is_stage_can_dead_frontEv"]
        pub(super) fn is_stage_can_dead_front() -> bool;

        #[link_name = "_ZN3app14sv_information11owner_handiEP9lua_State"]
        pub(super) fn owner_handi(state: *mut lua_State) -> f32;

        #[link_name = "_ZN3app14sv_information10owner_rankEP9lua_State"]
        pub(super) fn owner_rank(state: *mut lua_State) -> i32;

        #[link_name = "_ZN3app14sv_information14sleep_time_mulEP9lua_State"]
        pub(super) fn sleep_time_mul(state: *mut lua_State) -> f32;

        #[link_name = "_ZN3app14sv_information9slow_rateEv"]
        pub(super) fn slow_rate() -> f32;

        #[link_name = "_ZN3app14sv_information8stage_idEv"]
        pub(super) fn stage_id() -> i32;
    }
}

pub fn damage_log_value(state: &mut lua_State) -> i32 {
    unsafe {
        impl_::damage_log_value(state)
    }
}

pub fn dead_up_camera_hit_count() -> u32 {
    unsafe {
        impl_::dead_up_camera_hit_count()
    }
}

pub fn dead_up_type() -> i32 {
    unsafe {
        impl_::dead_up_type()
    }
}

pub fn get_delta_frame() -> f32 {
    unsafe {
        impl_::get_delta_frame()
    }
}

pub fn get_remaining_time_as_frame() -> u32 {
    unsafe {
        impl_::get_remaining_time_as_frame()
    }
}

pub fn is_dead_up_force_crush(state: &mut lua_State) -> bool {
    unsafe {
        impl_::is_dead_up_force_crush(state)
    }
}

pub fn is_disable_lock_down() -> bool {
    unsafe {
        impl_::is_disable_lock_down()
    }
}

pub fn is_enabled_time_limit() -> bool {
    unsafe {
        impl_::is_enabled_time_limit()
    }
}

pub fn is_flat_stage() -> bool {
    unsafe {
        impl_::is_flat_stage()
    }
}

pub fn is_master_hand_stage() -> bool {
    unsafe {
        impl_::is_master_hand_stage()
    }
}

pub fn is_ready_go() -> bool {
    unsafe {
        impl_::is_ready_go()
    }
}

pub fn is_stage_can_dead_back() -> bool {
    unsafe {
        impl_::is_stage_can_dead_back()
    }
}

pub fn is_stage_can_dead_front() -> bool {
    unsafe {
        impl_::is_stage_can_dead_front()
    }
}

pub fn owner_handi(state: &mut lua_State) -> f32 {
    unsafe {
        impl_::owner_handi(state)
    }
}

pub fn owner_rank(state: &mut lua_State) -> i32 {
    unsafe {
        impl_::owner_rank(state)
    }
}

pub fn sleep_time_mul(state: &mut lua_State) -> f32 {
    unsafe {
        impl_::sleep_time_mul(state)
    }
}

pub fn slow_rate() -> f32 {
    unsafe {
        impl_::slow_rate()
    }
}

pub fn stage_id() -> i32 {
    unsafe {
        impl_::stage_id()
    }
}
