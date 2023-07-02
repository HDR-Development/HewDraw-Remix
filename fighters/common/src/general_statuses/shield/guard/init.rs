// status imports
use super::*;
use globals::*;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon46sub_ftStatusUniqProcessGuard_initStatus_commonEv")]
unsafe fn sub_ftStatusUniqProcessGuard_initStatus_common(fighter: &mut L2CFighterCommon) {
    ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, app::ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
    let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
}

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS,
    symbol = "_ZN7lua2cpp16L2CFighterCommon39sub_ftStatusUniqProcessGuard_initStatusEv")]
unsafe fn ftStatusUniqProcessGuard_initStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuard_initStatus_common(fighter);
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        ftStatusUniqProcessGuard_initStatus
    );

    install_hooks!(
        sub_ftStatusUniqProcessGuard_initStatus_common
    );
}