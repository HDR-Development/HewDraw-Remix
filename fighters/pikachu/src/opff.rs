// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

unsafe fn skull_bash_edge_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END) {
        if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    fastfall_specials(fighter);
    skull_bash_edge_cancel(fighter);
}

pub extern "C" fn pikachu_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pikachu_frame(fighter);
    }
}

pub unsafe fn pikachu_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pikachu_frame_wrapper);
}
