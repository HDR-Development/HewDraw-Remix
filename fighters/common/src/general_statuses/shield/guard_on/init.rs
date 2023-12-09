// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_initStatus_common)]
unsafe fn sub_ftStatusUniqProcessGuardOn_initStatus_common(fighter: &mut L2CFighterCommon) {
    ShieldModule::set_status(
        fighter.module_accessor,
        *FIGHTER_SHIELD_KIND_GUARD,
        app::ShieldStatus(*SHIELD_STATUS_NORMAL),
        0,
    );
    let hit_stop_mul =
        WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
    let recovery_frame = WorkModule::get_param_int(
        fighter.module_accessor,
        hash40("common"),
        hash40("guard_off_disable_shield_recovery"),
    );
    WorkModule::set_int(
        fighter.module_accessor,
        recovery_frame,
        *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_SHIELD_RECOVERY_FRAME,
    );
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_initStatus)]
unsafe fn ftStatusUniqProcessGuardOn_initStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuardOn_initStatus_common(fighter);
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hooks!(
        sub_ftStatusUniqProcessGuardOn_initStatus_common,
        ftStatusUniqProcessGuardOn_initStatus
    );
}
