// status imports
use super::*;
use globals::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS,
    symbol = "_ZN7lua2cpp16L2CFighterCommon42sub_ftStatusUniqProcessGuardOff_execStatusEv")]
unsafe fn ftStatusUniqProcessGuardOff_execStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    super::super::fighter_status_guard::set_just_shield_scale(fighter);
    L2CValue::I32(0)
}
#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS,
    symbol = "_ZN7lua2cpp16L2CFighterCommon40sub_ftStatusUniqProcessGuardOff_execStopEv")]
unsafe fn ftStatusUniqProcessGuardOff_execStop(fighter: &mut L2CFighterCommon) -> L2CValue {
    super::super::fighter_status_guard::set_just_shield_scale(fighter);
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        ftStatusUniqProcessGuardOff_execStatus,
        ftStatusUniqProcessGuardOff_execStop
    );
}