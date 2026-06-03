use super::*;

unsafe extern "C" fn special_lw_success_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_LW_EFFECT_ON);

    return smashline::original_status(Main, fighter, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS)(fighter);
}

unsafe extern "C" fn special_lw_failure_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_LW_EFFECT_ON);

    return smashline::original_status(Main, fighter, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_FAILURE)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS, special_lw_success_main);
    agent.status(Main, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_FAILURE, special_lw_failure_main);
}