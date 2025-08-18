use super::*;

// FIGHTER_STATUS_KIND_GUARD_ON

unsafe extern "C" fn guard_on_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOn()
}

// FIGHTER_STATUS_KIND_GUARD

unsafe extern "C" fn guard_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Guard()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, guard_on_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD, guard_main);
}
