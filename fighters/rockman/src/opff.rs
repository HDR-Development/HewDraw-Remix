// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


// upB freefalls after one use per airtime
unsafe fn up_special_freefall(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP) {
        if fighter.is_situation(*SITUATION_KIND_AIR)
        && !StatusModule::is_changing(fighter.module_accessor)
        && VarModule::is_flag(fighter.battle_object, vars::rockman::instance::SPECIAL_HI_ENABLE_FREEFALL) {
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
                *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
    up_special_freefall(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn rockman_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		rockman_frame(fighter)
    }
}

pub unsafe fn rockman_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, rockman_frame_wrapper);
}