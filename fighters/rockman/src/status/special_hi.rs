use super::*;


// FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP

pub unsafe extern "C" fn special_hi_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::rockman::instance::SPECIAL_HI_ENABLE_FREEFALL);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_LAG);

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_end);
}