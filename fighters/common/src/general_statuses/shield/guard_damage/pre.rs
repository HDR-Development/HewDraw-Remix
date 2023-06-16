// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_GuardDamage)]
unsafe fn status_pre_GuardDamage(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GUARD_ON,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_FLOAT,
        0,
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        true,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0,
    );
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hook!(status_pre_GuardDamage);
}
