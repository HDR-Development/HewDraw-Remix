// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_status_end_GuardDamage)]
unsafe fn status_end_GuardDamage(fighter: &mut L2CFighterCommon) -> L2CValue {
    super::super::guard::end::status_end_Guard(fighter);
    effect!(
        fighter,
        MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
        Hash40::new("sys_shield_damage"),
        true,
        true
    );
    effect!(
        fighter,
        MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
        Hash40::new("sys_shield_damage2"),
        true,
        true
    );
    effect!(
        fighter,
        MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
        Hash40::new("sys_shield_damage3"),
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
    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD_SLOW_WHOLE,
    ) && 0 < SlowModule::whole_frame(fighter.module_accessor)
    {
        SlowModule::clear_whole(fighter.module_accessor);
        WorkModule::off_flag(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD_SLOW_WHOLE,
        );
    }
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hooks!(status_end_GuardDamage);
}
