// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn gunman_timer(fighter: &mut L2CFighterCommon) {
    if VarModule::countdown_int(fighter.battle_object, vars::duckhunt::instance::SPECIAL_LW_GUNMAN_TIMER, 0) {
        gimmick_flash(fighter);
    }
}

unsafe fn fall_special_ledgegrab_box(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_FALL_SPECIAL) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 4);
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    gunman_timer(fighter);
}

pub unsafe extern "C" fn duckhunt_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    duckhunt_frame(fighter);
}

pub unsafe fn duckhunt_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter);
    }
}

pub unsafe extern "C" fn duckhunt_frame_wrapper_exec(fighter: &mut L2CFighterCommon) {
    fall_special_ledgegrab_box(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, duckhunt_frame_wrapper);
    agent.on_line(Exec, duckhunt_frame_wrapper_exec);
}