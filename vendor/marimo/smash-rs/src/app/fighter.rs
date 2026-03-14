use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app7fighter21find_fighter_entry_idEj"]
        pub(super) fn find_fighter_entry_id(object_id: u32) -> i32;
    
        #[link_name = "_ZN3app7fighter23get_fighter_entry_countEv"]
        pub(super) fn get_fighter_entry_count() -> u32;
    
        #[link_name = "_ZN3app7fighter20get_fighter_entry_idEj"]
        pub(super) fn get_fighter_entry_id(object_id: u32) -> i32;
    
        #[link_name = "_ZN3app7fighter20get_id_from_entry_idEi"]
        pub(super) fn get_id_from_entry_id(entry_id: i32) -> u32;
    
        #[link_name = "_ZN3app7fighter21get_joint_from_hit_noEi"]
        pub(super) fn get_joint_from_hit_no(hit_no: i32) -> phx::Hash40;
    
        #[link_name = "_ZN3app7fighter26is_absolutely_final_statusEv"]
        pub(super) fn is_absolutely_final_status() -> bool;
    
        #[link_name = "_ZN3app7fighter18is_fighter_enabledEj"]
        pub(super) fn is_fighter_enabled(object_id: u32) -> bool;
    
        #[link_name = "_ZN3app7fighter35is_final_status_or_stage_transitionEv"]
        pub(super) fn is_final_status_or_stage_transition() -> bool;
    
        #[link_name = "_ZN3app7fighter26is_final_status_or_standbyEv"]
        pub(super) fn is_final_status_or_standby() -> bool;
    
        #[link_name = "_ZN3app7fighter16is_have_on_mouthEj"]
        pub(super) fn is_have_on_mouth(object_id: u32) -> bool;
    
        #[link_name = "_ZN3app7fighter11is_ready_goEv"]
        pub(super) fn is_ready_go() -> bool;
    
        #[link_name = "_ZN3app7fighter14set_assistbeatEi"]
        pub(super) fn set_assistbeat(entry_id: i32);
    
        #[link_name = "_ZN3app7fighter9set_finalEP9lua_Statehj"]
        pub(super) fn set_final(state: *mut lua_State, use_team: bool, object_id: u32);
    
        #[link_name = "_ZN3app7fighter22set_opponent_team_slowEP9lua_Stateii"]
        pub(super) fn set_opponent_team_slow(state: *mut lua_State, magnitude: i32, frame: i32);
    
        #[link_name = "_ZN3app7fighter22set_opponent_team_stopEP9lua_Statei"]
        pub(super) fn set_opponent_team_stop(state: *mut lua_State, frames: i32);
    
        #[link_name = "_ZN3app7fighter14waist_joint_idEPKNS_26BattleObjectModuleAccessorE"]
        pub(super) fn waist_joint_id(module_accessor: *const app::BattleObjectModuleAccessor) -> phx::Hash40;
    
    }
}

pub fn find_fighter_entry_id(object_id: u32) -> i32 {
    unsafe {
        impl_::find_fighter_entry_id(object_id)
    }
}

pub fn get_fighter_entry_count() -> u32 {
    unsafe {
        impl_::get_fighter_entry_count()
    }
}

pub fn get_fighter_entry_id(object_id: u32) -> i32 {
    unsafe {
        impl_::get_fighter_entry_id(object_id)
    }
}

pub fn get_id_from_entry_id(entry_id: i32) -> u32 {
    unsafe {
        impl_::get_id_from_entry_id(entry_id)
    }
}

pub fn get_joint_from_hit_no(hit_no: i32) -> phx::Hash40 {
    unsafe {
        impl_::get_joint_from_hit_no(hit_no)
    }
}

pub fn is_absolutely_final_status() -> bool {
    unsafe {
        impl_::is_absolutely_final_status()
    }
}

pub fn is_fighter_enabled(object_id: u32) -> bool {
    unsafe {
        impl_::is_fighter_enabled(object_id)
    }
}

pub fn is_final_status_or_stage_transition() -> bool {
    unsafe {
        impl_::is_final_status_or_stage_transition()
    }
}

pub fn is_final_status_or_standby() -> bool {
    unsafe {
        impl_::is_final_status_or_standby()
    }
}

pub fn is_have_on_mouth(battle_object_id: u32) -> bool {
    unsafe {
        impl_::is_have_on_mouth(battle_object_id)
    }
}

pub fn is_ready_go() -> bool {
    unsafe {
        impl_::is_ready_go()
    }
}

pub fn set_assistbeat(entry_id: i32) {
    unsafe {
        impl_::set_assistbeat(entry_id)
    }
}

pub fn set_final(state: *mut lua_State, use_team: bool, object_id: u32) {
    unsafe {
        impl_::set_final(state, use_team, object_id)
    }
}

pub fn set_opponent_team_slow(state: *mut lua_State, magnitude: i32, frame: i32) {
    unsafe {
        impl_::set_opponent_team_slow(state, magnitude, frame)
    }
}

pub fn set_opponent_team_stop(state: *mut lua_State, frames: i32) {
    unsafe {
        impl_::set_opponent_team_stop(state, frames)
    }
}

pub fn waist_joint_id(module_accessor: &app::BattleObjectModuleAccessor) -> phx::Hash40 {
    unsafe {
        impl_::waist_joint_id(module_accessor)
    }
}

