use super::*;

unsafe extern "C" fn catch_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Catch()
}

unsafe extern "C" fn catch_dash_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchDash()
}

unsafe extern "C" fn catch_turn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchTurn()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_CATCH, catch_end);

    agent.status(End, *FIGHTER_STATUS_KIND_CATCH_DASH, catch_dash_end);

    agent.status(End, *FIGHTER_STATUS_KIND_CATCH_TURN, catch_turn_end);
}