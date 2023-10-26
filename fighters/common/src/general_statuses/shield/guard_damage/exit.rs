// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_exitStatus_common)]
unsafe fn sub_ftStatusUniqProcessGuardDamage_exitStatus_common(fighter: &mut L2CFighterCommon) {
    ShieldModule::set_status(
        fighter.module_accessor,
        *FIGHTER_SHIELD_KIND_GUARD,
        app::ShieldStatus(*SHIELD_STATUS_NONE),
        0,
    );
    let shield_type =
        app::FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32())
            as i32;
    ShieldModule::set_shield_type(
        fighter.module_accessor,
        app::ShieldType(shield_type),
        *FIGHTER_SHIELD_KIND_GUARD,
        0,
    );
    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD,
    ) {
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU,
        ) {
            HitModule::set_whole(
                fighter.module_accessor,
                app::HitStatus(*HIT_STATUS_NORMAL),
                0,
            );
            WorkModule::off_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU,
            );
        }

        EffectModule::remove_common(fighter.module_accessor, Hash40::new("just_shield"));
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE,
        ) && !WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL,
        ) {
            ModelModule::disable_gold_eye(fighter.module_accessor);
        }
        WorkModule::off_flag(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE,
        );

        ControlModule::set_command_life_extend(fighter.module_accessor, 0);
        InputModule::disable_persist(fighter.battle_object);
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_exitStatus)]
unsafe fn ftStatusUniqProcessGuardDamage_exitStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuardDamage_exitStatus_common(fighter);
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hooks!(sub_ftStatusUniqProcessGuardDamage_exitStatus_common, ftStatusUniqProcessGuardDamage_exitStatus);
}
