use super::*;

unsafe extern "C" fn demon_walk_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Walk()
}

unsafe extern "C" fn demon_walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Walk()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_WALK, demon_walk_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_WALK, demon_walk_main);
}