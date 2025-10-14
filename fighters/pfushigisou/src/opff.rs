// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn special_lw_track(boma: &mut BattleObjectModuleAccessor) {
    if !LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
        return;
    }
    let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
    let object = utils::util::get_battle_object_from_id(parent_id);
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && !boma.is_button_on(Buttons::SpecialAll) {
        VarModule::off_flag(object, vars::ptrainer::instance::SPECIAL_LW_BACKWARDS_SWITCH);
    }
    if is_training_mode() && !sv_information::is_ready_go() {
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, *PLEDGE_STATE_NONE);
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 0);
        VarModule::off_flag(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER);
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER, 0);
    }
    let pledge = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE) as i32;
    if pledge == *PLEDGE_STATE_NONE 
    && (sv_information::is_ready_go() || boma.status_frame() >= 1)
    && !boma.is_status_one_of(&[
        *FIGHTER_POKEMON_STATUS_KIND_SPECIAL_LW_OUT,
        *FIGHTER_POKEMON_STATUS_KIND_SPECIAL_LW_STANDBY
    ]) {
        let entry_id = boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        utils::ui::UiManager::set_ptrainer_meter_enable(entry_id, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    special_lw_track(boma);
    fastfall_specials(fighter);
}

pub extern "C" fn pfushigisou_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pfushigisou_frame(fighter)
    }
}

pub unsafe fn pfushigisou_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pfushigisou_frame_wrapper);
}
