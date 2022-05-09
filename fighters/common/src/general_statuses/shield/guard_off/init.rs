// status imports
use super::*;
use globals::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS,
    symbol = "_ZN7lua2cpp16L2CFighterCommon42sub_ftStatusUniqProcessGuardOff_initStatusEv")]
unsafe fn ftStatusUniqProcessGuardOff_initStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if app::FighterUtil::is_valid_just_shield(fighter.module_accessor) {
        ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, app::ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
        ShieldModule::set_shield_type(fighter.module_accessor, app::ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
        ReflectorModule::set_status(fighter.module_accessor, 0, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        super::super::fighter_status_guard::set_just_shield_scale(fighter);
        let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
        ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
    }
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        ftStatusUniqProcessGuardOff_initStatus
    );
}