use super::*;

// FIGHTER_STATUS_KIND_LANDING_LIGHT

unsafe extern "C" fn landing_light_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_LAG) {
        let lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.empty_landing");
        fighter.set_float(lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.set_status_kind_interrupt(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL);
        return 1.into();
    }
    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_LANDING_LIGHT)(fighter)
}

// FIGHTER_STATUS_KIND_LANDING

unsafe extern "C" fn landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_LAG) {
        let lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.empty_landing");
        fighter.set_float(lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.set_status_kind_interrupt(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL);
        return 1.into();
    }
    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_LANDING)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_LANDING_LIGHT, landing_light_pre);
    agent.status(Pre, *FIGHTER_STATUS_KIND_LANDING, landing_pre);
}