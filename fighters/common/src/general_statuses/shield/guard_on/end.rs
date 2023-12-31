// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_status_end_guard_on_common)]
unsafe fn sub_status_end_guard_on_common(fighter: &mut L2CFighterCommon, arg: L2CValue) {
    if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_GUARD
        && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_GUARD_DAMAGE
    {
        effect!(
            fighter,
            MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
            Hash40::new("sys_shield"),
            true,
            true
        );
        effect!(
            fighter,
            MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
            Hash40::new("sys_shield_smoke"),
            true,
            true
        );
    } else if !arg.get_bool() {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_end_GuardOn)]
unsafe fn status_end_GuardOn(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_effect = WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT,
    );
    sub_status_end_guard_on_common(fighter, is_effect.into());
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hooks!(sub_status_end_guard_on_common, status_end_GuardOn);
}
