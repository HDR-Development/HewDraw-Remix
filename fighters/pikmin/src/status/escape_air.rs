use super::*;
use globals::*;

pub unsafe extern "C" fn escape_air_end(agent: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(agent.battle_object, vars::pikmin::instance::SPECIAL_HI_CANCEL_ESCAPE_AIR);
    agent.status_end_EscapeAir()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ESCAPE_AIR, escape_air_end);
}
