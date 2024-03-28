use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn init_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::plizardon::instance::DISABLE_SPECIAL_S);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, init_special_s);
}
