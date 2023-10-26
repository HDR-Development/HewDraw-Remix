// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOff_initStatus)]
unsafe fn ftStatusUniqProcessGuardOff_initStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if app::FighterUtil::is_valid_just_shield(fighter.module_accessor)
        && VarModule::is_flag(
            fighter.object(),
            vars::common::instance::IS_PARRY_FOR_GUARD_OFF,
        )
    {
        ShieldModule::set_status(
            fighter.module_accessor,
            *FIGHTER_SHIELD_KIND_GUARD,
            app::ShieldStatus(*SHIELD_STATUS_NORMAL),
            0,
        );
        ShieldModule::set_shield_type(
            fighter.module_accessor,
            app::ShieldType(*SHIELD_TYPE_JUST_SHIELD),
            *FIGHTER_SHIELD_KIND_GUARD,
            0,
        );
        ReflectorModule::set_status(
            fighter.module_accessor,
            0,
            app::ShieldStatus(*SHIELD_STATUS_NORMAL),
            *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD,
        );
        super::super::fighter_status_guard::set_just_shield_scale(fighter);
        let hit_stop_mul =
            WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
        ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
    }
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hook!(ftStatusUniqProcessGuardOff_initStatus);
}
