use super::*;

pub unsafe extern "C" fn special_lw2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        VarModule::on_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW2_CHECK_HOLD);
    }

    if VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW2_CHECK_HOLD) {
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }

    return 0.into();
}

unsafe extern "C" fn special_lw2_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW2_CHECK_HOLD);
    original_status(Main, fighter, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START)(fighter)
}

pub fn install (agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START, special_lw2_start_main);
}