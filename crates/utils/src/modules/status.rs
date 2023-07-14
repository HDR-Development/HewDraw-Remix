use smash::app::BattleObject;
use smash::phx::Hash40;
use crate::ext::StatusInfo;

extern "Rust" {
    #[link_name = "CustomStatusManager__add_new_common_status_script"]
    fn CustomStatusManager__add_new_common_status_script(id: i32, info: StatusInfo) -> bool;

    #[link_name = "CustomStatusManager__add_new_agent_status_script"]
    fn CustomStatusManager__add_new_agent_status_script(agent: Hash40, id: i32, info: StatusInfo) -> bool;

    #[link_name = "CustomStatusModule__get_agent_status_kind"]
    fn CustomStatusModule__get_agent_status_kind(object: *mut BattleObject, id: i32) -> i32;

    #[link_name = "CustomStatusModule__get_common_status_kind"]
    fn CustomStatusModule__get_common_status_kind(object: *mut BattleObject, id: i32) -> i32;
}

pub mod CustomStatusManager {
    use super::*;
    
    pub fn add_new_common_status_script(id: i32, info: StatusInfo) -> bool {
        unsafe {
            CustomStatusManager__add_new_common_status_script(id, info)
        }
    }

    pub fn add_new_agent_status_script(agent: Hash40, id: i32, info: StatusInfo) -> bool {
        unsafe {
            CustomStatusManager__add_new_agent_status_script(agent, id, info)
        }
    }
}

pub mod CustomStatusModule {
    use super::*;
    
    pub fn get_agent_status_kind(object: *mut BattleObject, id: i32) -> i32 {
        unsafe {
            CustomStatusModule__get_agent_status_kind(object, id)
        }
    }

    pub fn get_common_status_kind(object: *mut BattleObject, id: i32) -> i32 {
        unsafe {
            CustomStatusModule__get_common_status_kind(object, id)
        }
    }
}