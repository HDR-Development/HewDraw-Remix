use super::*;

unsafe extern "C" fn throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_THROW)(fighter);

    if fighter.is_motion(Hash40::new("throw_b")) { SET_STANCE(fighter, 0, false); }
    else if fighter.is_motion(Hash40::new("throw_lw")) { SET_STANCE(fighter, 1, false); }
    else if fighter.is_motion(Hash40::new("throw_f")) { SET_STANCE(fighter, 2, false); }

    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_THROW, throw_main);
}