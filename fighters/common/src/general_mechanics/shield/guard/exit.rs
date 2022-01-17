// status imports
use super::*;
use globals::*;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon46sub_ftStatusUniqProcessGuard_exitStatus_commonEv")]
unsafe fn sub_ftStatusUniqProcessGuard_exitStatus_common(fighter: &mut L2CFighterCommon) {
    ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, app::ShieldStatus(*SHIELD_STATUS_NONE), 0);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, 1.0);
}

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS,
    symbol = "_ZN7lua2cpp16L2CFighterCommon39sub_ftStatusUniqProcessGuard_exitStatusEv")]
unsafe fn ftStatusUniqProcessGuard_exitStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuard_exitStatus_common(fighter);
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        ftStatusUniqProcessGuard_exitStatus
    );

    install_hooks!(
        sub_ftStatusUniqProcessGuard_exitStatus_common
    );
}