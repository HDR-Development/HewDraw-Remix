use super::*;
use globals::*;

// This file contains code for dash grab

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_CatchDash_common,
            status_end_CatchDash,
            bind_address_call_status_end_CatchDash,
            status_CatchAttack_Main
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_CatchDash_common)]
unsafe fn status_pre_CatchDash_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    JostleModule::set_overlap_rate_mul(fighter.module_accessor, 6.666);  // 0.3 (base overlap rate) * 6.666 = 2.0 overlap rate
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CatchDash)]
unsafe fn status_end_CatchDash(fighter: &mut L2CFighterCommon) -> L2CValue {
    JostleModule::set_overlap_rate_mul(fighter.module_accessor, 1.0);  // reset to 0.3 overlap rate
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_bind_address_call_status_end_CatchDash)]
unsafe fn bind_address_call_status_end_CatchDash(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_CatchDash();
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CatchAttack_Main)]
unsafe fn status_CatchAttack_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
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