use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    0.into()
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND

pub unsafe extern "C" fn special_s_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    0.into()
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B

pub unsafe extern "C" fn special_b_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    0.into()
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND

pub unsafe extern "C" fn special_b_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, special_s_command_init);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, special_b_init);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, special_b_command_init);
}