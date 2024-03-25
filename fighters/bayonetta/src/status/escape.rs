use super::*;
use globals::*;

 

// FIGHTER_STATUS_KIND_ESCAPE_F //

unsafe extern "C" fn escape_f_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN {
        fighter.sub_status_end_EscaleFB();
    }
    0.into()
}

// FIGHTER_STATUS_KIND_ESCAPE_B //

unsafe extern "C" fn escape_b_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN {
        fighter.sub_status_end_EscaleFB();
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ESCAPE_F, escape_f_end);
    agent.status(End, *FIGHTER_STATUS_KIND_ESCAPE_B, escape_b_end);
}
