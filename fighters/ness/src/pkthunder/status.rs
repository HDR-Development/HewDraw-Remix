use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);
// status script import

// WEAPON_NESS_PK_THUNDER_STATUS_KIND_MOVE

unsafe extern "C" fn move_exec(weapon: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(weapon.object(), vars::ness::status::THUNDER_LOOSE) {
        let parent_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID);
        let parent_object = get_battle_object_from_id(parent_id as u32);
        if !parent_object.is_null()
        && sv_battle_object::kind(parent_id as u32) == *FIGHTER_KIND_NESS
        && StatusModule::status_kind((*parent_object).module_accessor) != *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD {
            VarModule::on_flag(weapon.object(), vars::ness::status::THUNDER_LOOSE);
            MotionModule::change_motion_force_inherit_frame(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, 1.0);
            return 0.into();
        }
        smashline::original_status(Exec, weapon, *WEAPON_NESS_PK_THUNDER_STATUS_KIND_MOVE)(weapon);
    }
    0.into() 
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *WEAPON_NESS_PK_THUNDER_STATUS_KIND_MOVE, move_exec);
}