// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_GuardOn)]
unsafe fn status_pre_GuardOn(fighter: &mut L2CFighterCommon) -> L2CValue {
    // VarModule::set_flag(
    //     // disables cstick buffered rolls if cstick was already held when entering shield for the first time
    //     fighter.battle_object,
    //     vars::common::instance::DISABLE_CSTICK_BUFFER_ROLL_OOS,
    //     fighter.is_button_on(Buttons::CStickOn)
    // );
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GUARD_ON,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hook!(status_pre_GuardOn);
}
