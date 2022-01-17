// status imports
use super::*;
use globals::*;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon30sub_status_end_guard_on_commonEN3lib8L2CValueE")]
unsafe fn sub_status_end_guard_on_common(fighter: &mut L2CFighterCommon, arg: L2CValue) {
    if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_GUARD && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0xafae75f05), true, true);
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0x10da0b43c8), true, true);
    } else if !arg.get_bool() {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
    }
}

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_ON, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon18status_end_GuardOnEv")]
unsafe fn status_end_GuardOn(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_effect = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT);
    sub_status_end_guard_on_common(fighter, is_effect.into());
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        status_end_GuardOn
    );

    install_hooks!(
        sub_status_end_guard_on_common
    );
}