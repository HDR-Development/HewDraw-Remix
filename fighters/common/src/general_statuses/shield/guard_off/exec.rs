// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOff_execStatus)]
unsafe fn ftStatusUniqProcessGuardOff_execStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    super::super::fighter_status_guard::set_just_shield_scale(fighter);
    L2CValue::I32(0)
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOff_execStop)]
unsafe fn ftStatusUniqProcessGuardOff_execStop(fighter: &mut L2CFighterCommon) -> L2CValue {
    super::super::fighter_status_guard::set_just_shield_scale(fighter);
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hooks!(
        ftStatusUniqProcessGuardOff_execStatus,
        ftStatusUniqProcessGuardOff_execStop
    );
}
