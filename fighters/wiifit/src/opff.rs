// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn track_effect(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_LW_EFFECT_ON)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_INT_SPECIAL_LW_WAZA_EFFECTIVE_FRAME) <= 0 {
        VarModule::off_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_LW_EFFECT_ON);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_BREATH,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_FAILURE
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
    fastfall_specials(fighter);
    track_effect(fighter);
}

pub extern "C" fn wiifit_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		wiifit_frame(fighter)
    }
}

pub unsafe fn wiifit_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, wiifit_frame_wrapper);
}