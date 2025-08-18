use super::*;

// stubs toad removal from various end statuses. neccesary to prevent jank with the talking flower

unsafe extern "C" fn throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Throw();
    0.into()
}

unsafe extern "C" fn catch_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchAttack();
    0.into()
}

unsafe extern "C" fn catch_cut_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchCut();
    0.into()
}

unsafe extern "C" fn catch_dash_pull_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchDashPull();
    0.into()
}

unsafe extern "C" fn catch_pull_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchPull();
    0.into()
}

unsafe extern "C" fn catch_wait_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchWait();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_THROW, throw_end);

    agent.status(End, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_end);
    agent.status(End, *FIGHTER_STATUS_KIND_CATCH_CUT, catch_cut_end);
    agent.status(End, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, catch_dash_pull_end);
    agent.status(End, *FIGHTER_STATUS_KIND_CATCH_PULL, catch_pull_end);
    agent.status(End, *FIGHTER_STATUS_KIND_CATCH_WAIT, catch_wait_end);
}