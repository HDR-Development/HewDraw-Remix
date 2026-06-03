use super::*;

unsafe extern "C" fn attack_stand_1_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_3);
    1.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1, attack_stand_1_pre);
}