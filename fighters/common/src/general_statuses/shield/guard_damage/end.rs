// status imports
use super::*;
use globals::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon22status_end_GuardDamageEv")]
unsafe fn status_end_GuardDamage(fighter: &mut L2CFighterCommon) -> L2CValue {
    super::super::guard::end::status_end_Guard(fighter);
    effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0x113434cb66), true, true);
    effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0x12be304eab), true, true);
    effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0x12c9377e3d), true, true);
    effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0x10da0b43c8), true, true);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD_SLOW_WHOLE) 
        && 0 < SlowModule::whole_frame(fighter.module_accessor) {
        SlowModule::clear_whole(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD_SLOW_WHOLE);
    }
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        status_end_GuardDamage
    );
}