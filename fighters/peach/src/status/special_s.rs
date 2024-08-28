use super::*;

// FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP

unsafe extern "C" fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::daisy::instance::DISABLE_SPECIAL_S);
    smashline::original_status(Main, fighter, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_main);
}
