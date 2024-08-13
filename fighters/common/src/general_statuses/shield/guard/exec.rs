// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuard_execStatus_common)]
unsafe fn sub_ftStatusUniqProcessGuard_execStatus_common(fighter: &mut L2CFighterCommon) {
    super::super::misc::sub_ftStatusUniqProcessGuardFunc_updateShield(fighter, false.into());
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuard_execStatus)]
unsafe fn ftStatusUniqProcessGuard_execStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    super::super::misc::sub_ftStatusUniqProcessGuardFunc_updateShield(fighter, false.into());
    L2CValue::I32(0)
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuard_execStop_common)]
unsafe fn sub_ftStatusUniqProcessGuard_execStop_common(fighter: &mut L2CFighterCommon) {
    super::super::guard_on::exec::ftStatusUniqProcessGuardOn_execStop(fighter);
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuard_execStop)]
unsafe fn ftStatusUniqProcessGuard_execStop(fighter: &mut L2CFighterCommon) -> L2CValue {
    super::super::guard_on::exec::ftStatusUniqProcessGuardOn_execStop(fighter)
}

pub fn install() {
    skyline::install_hooks!(
        sub_ftStatusUniqProcessGuard_execStatus_common,
        sub_ftStatusUniqProcessGuard_execStop_common,
        ftStatusUniqProcessGuard_execStatus,
        ftStatusUniqProcessGuard_execStop
    );
}
