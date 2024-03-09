// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuard_initStatus_common)]
unsafe fn sub_ftStatusUniqProcessGuard_initStatus_common(fighter: &mut L2CFighterCommon) {
    ShieldModule::set_status(
        fighter.module_accessor,
        *FIGHTER_SHIELD_KIND_GUARD,
        app::ShieldStatus(*SHIELD_STATUS_NORMAL),
        0
    );
    let hit_stop_mul = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("common"),
        0x20d241cd64
    );
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuard_initStatus)]
unsafe fn ftStatusUniqProcessGuard_initStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuard_initStatus_common(fighter);
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hooks!(
        ftStatusUniqProcessGuard_initStatus,
        sub_ftStatusUniqProcessGuard_initStatus_common
    );
}
