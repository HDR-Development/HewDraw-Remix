use super::*;

// FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND

pub unsafe extern "C" fn special_s_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Init, fighter, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND)(fighter)
}

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
}

// FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP

pub unsafe extern "C" fn special_s_loop_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        MeterModule::drain_direct(fighter.battle_object, 2.0 * MeterModule::meter_per_level(fighter.battle_object));
    }
    smashline::original_status(Init, fighter, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, special_s_command_init);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, special_s_loop_init);
}
