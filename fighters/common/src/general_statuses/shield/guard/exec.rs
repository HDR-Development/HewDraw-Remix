// status imports
use super::*;
use globals::*;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon46sub_ftStatusUniqProcessGuard_execStatus_commonEv")]
unsafe fn sub_ftStatusUniqProcessGuard_execStatus_common(fighter: &mut L2CFighterCommon) {
    super::super::misc::sub_ftStatusUniqProcessGuardFunc_updateShield(fighter, false.into());
}

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS,
    symbol = "_ZN7lua2cpp16L2CFighterCommon39sub_ftStatusUniqProcessGuard_execStatusEv")]
unsafe fn ftStatusUniqProcessGuard_execStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    super::super::misc::sub_ftStatusUniqProcessGuardFunc_updateShield(fighter, false.into());
    L2CValue::I32(0)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon44sub_ftStatusUniqProcessGuard_execStop_commonEv")]
unsafe fn sub_ftStatusUniqProcessGuard_execStop_common(fighter: &mut L2CFighterCommon) {
    super::super::guard_on::exec::ftStatusUniqProcessGuardOn_execStop(fighter);
}

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP,
    symbol = "_ZN7lua2cpp16L2CFighterCommon37sub_ftStatusUniqProcessGuard_execStopEv")]
unsafe fn ftStatusUniqProcessGuard_execStop(fighter: &mut L2CFighterCommon) -> L2CValue {
    super::super::guard_on::exec::ftStatusUniqProcessGuardOn_execStop(fighter)
}

pub fn install() {
    install_status_scripts!(
        ftStatusUniqProcessGuard_execStatus,
        ftStatusUniqProcessGuard_execStop
    );

    install_hooks!(
        sub_ftStatusUniqProcessGuard_execStatus_common,
        sub_ftStatusUniqProcessGuard_execStop_common
    );
}