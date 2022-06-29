use super::*;
use globals::*;

// This file contains code for ledgehogging

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_end_CliffCatchMove,
            status_end_CliffCatch,
            status_end_CliffWait,
            status_end_CliffAttack,
            status_end_CliffClimb,
            status_end_CliffEscape,
            status_end_CliffJump1,
            status_end_CliffJump2,
            status_end_CliffJump3
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffCatchMove)]
unsafe fn status_end_CliffCatchMove(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CLIFF_CATCH {
        VarModule::set_vec3(fighter.object(), vars::common::instance::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffCatch)]
unsafe fn status_end_CliffCatch(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CLIFF_WAIT {
        VarModule::set_vec3(fighter.object(), vars::common::instance::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffWait)]
unsafe fn status_end_CliffWait(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ![*FIGHTER_STATUS_KIND_CLIFF_ATTACK,
        *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        VarModule::set_vec3(fighter.object(), vars::common::instance::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffAttack)]
unsafe fn status_end_CliffAttack(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_vec3(fighter.object(), vars::common::instance::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffClimb)]
unsafe fn status_end_CliffClimb(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_vec3(fighter.object(), vars::common::instance::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffEscape)]
unsafe fn status_end_CliffEscape(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_vec3(fighter.object(), vars::common::instance::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffJump1)]
unsafe fn status_end_CliffJump1(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CLIFF_JUMP2 {
        VarModule::set_vec3(fighter.object(), vars::common::instance::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffJump2)]
unsafe fn status_end_CliffJump2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CLIFF_JUMP3 {
        VarModule::set_vec3(fighter.object(), vars::common::instance::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffJump3)]
unsafe fn status_end_CliffJump3(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_vec3(fighter.object(), vars::common::instance::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
    call_original!(fighter)
}