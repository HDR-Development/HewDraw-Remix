use super::*;
use globals::*;

// FIGHTER_STATUS_KIND_SPECIAL_S //

pub unsafe extern "C" fn init_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    0.into()
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND //

pub unsafe extern "C" fn init_special_s_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    0.into()
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B //

pub unsafe extern "C" fn init_special_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    0.into()
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND //

pub unsafe extern "C" fn init_special_b_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, init_special_s);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, init_special_s_command);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, init_special_b);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, init_special_b_command);
}