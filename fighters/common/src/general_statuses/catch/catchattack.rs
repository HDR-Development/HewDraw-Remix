use super::*;
use globals::*;

// This file contains code for pummels

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_CatchAttack_Main
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CatchAttack_Main)]
unsafe fn status_CatchAttack_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor)
    || CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.CatchCont().get_bool() {
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_CATCH_WAIT, false);
        }
    }
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_CATCH_JUMP, false);
    }
    0.into()
}