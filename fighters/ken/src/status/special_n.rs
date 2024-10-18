use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_N

pub unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::MAGIC_SERIES_CANCEL) {
        fighter.set_status_kind_interrupt(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND);
        return 1.into();
    }
    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
}

// FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND

pub unsafe extern "C" fn special_n_command_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::MAGIC_SERIES_CANCEL) {
        fighter.set_status_kind_interrupt(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND);
        return 1.into();
    }
    smashline::original_status(Pre, fighter, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre);
    agent.status(Pre, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND, special_n_command_pre);
}