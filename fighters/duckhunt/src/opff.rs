// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn gunman_timer(fighter: &mut L2CFighterCommon) {
    let timer = VarModule::get_int(fighter.object(), vars::duckhunt::instance::SPECIAL_LW_GUNMAN_TIMER);
    if  timer != 0 {
        VarModule::set_int(fighter.object(), vars::duckhunt::instance::SPECIAL_LW_GUNMAN_TIMER, (timer-1));
    }
    if timer == 1 {
        gimmick_flash(fighter);
    }
}

pub extern "C" fn duckhunt_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        gunman_timer(fighter);
        fastfall_specials(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, duckhunt_frame_wrapper);
}