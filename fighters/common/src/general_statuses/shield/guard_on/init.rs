// status imports
use super::*;
use globals::*;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon48sub_ftStatusUniqProcessGuardOn_initStatus_commonEv")]
unsafe fn sub_ftStatusUniqProcessGuardOn_initStatus_common(fighter: &mut L2CFighterCommon) {
    // Original
    ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
    // Additions
    if FighterUtil::is_valid_just_shield(fighter.module_accessor) {
        let shield_just_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_just_frame")) as f32;
        let just_shield_check_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("just_shield_check_frame"), 0);
        let just_frame = (shield_just_frame * just_shield_check_frame + 0.5) as i32;
        WorkModule::set_int(fighter.module_accessor, just_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
        ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        fighter.FighterStatusGuard__set_just_shield_scale();
    }
    // Also Original, but moved down
    let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
    let recovery_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_disable_shield_recovery"));
    WorkModule::set_int(fighter.module_accessor, recovery_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_SHIELD_RECOVERY_FRAME);
}

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_ON, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS,
    symbol = "_ZN7lua2cpp16L2CFighterCommon41sub_ftStatusUniqProcessGuardOn_initStatusEv")]
unsafe fn ftStatusUniqProcessGuardOn_initStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuardOn_initStatus_common(fighter);
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        ftStatusUniqProcessGuardOn_initStatus
    );

    install_hooks!(
        sub_ftStatusUniqProcessGuardOn_initStatus_common
    );
}