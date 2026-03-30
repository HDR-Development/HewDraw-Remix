// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn specialhi_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_status(*FIGHTER_STATUS_KIND_CLIFF_CATCH) {
        VarModule::off_flag(fighter.battle_object, vars::master::instance::SPECIAL_HI_CATCH_USED);
    }
}

// Allows Byleth to grab ledge after upB tether ledgegrab boxes have cleared
unsafe fn up_special_whiff_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_AIR_LASSO_FAIL) {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        fighter.sub_transition_group_check_air_cliff();
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        // *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        // *FIGHTER_STATUS_KIND_SPECIAL_LW,
        // *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_HOLD,
        // *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_TURN,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_MAX_SHOOT,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT_DASH,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_WALL_JUMP,
        // *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_TURN,
        // *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_HIT,
        // *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_CANCEL
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

unsafe fn special_lw_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_in_hitlag()
    || StatusModule::is_changing(fighter.module_accessor)
    || !fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
    ]) {
        return;
    }

    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    && VarModule::is_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_ENABLE_CANCEL) {
        let terms = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON,
        ];
        fighter.enable_transition_term_many(&terms);
        let ret = fighter.sub_transition_group_check_air_special().get_bool()
            || fighter.sub_transition_group_check_ground_special().get_bool()
            || fighter.sub_transition_group_check_air_jump_aerial().get_bool();
        fighter.unable_transition_term_many(&terms);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    specialhi_reset(fighter);
    up_special_whiff_ledgegrab(fighter);
    fastfall_specials(fighter);
    special_lw_cancel(fighter);
}

pub extern "C" fn master_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		master_frame(fighter)
    }
}

pub unsafe fn master_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, master_frame_wrapper);
}