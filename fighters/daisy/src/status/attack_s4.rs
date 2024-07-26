use super::*;

// removes the varying weapons from forward smash, using only the tennis racket

// FIGHTER_STATUS_KIND_ATTACK_S4_HOLD

unsafe extern "C" fn attack_s4_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if PostureModule::lr(fighter.boma()) == 1.0 {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketmflip"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketm"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("panm"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("clubm"), false);
    } else {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketm"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketmflip"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("panmflip"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("clubmflip"), false);
    }
    0.into()
}

unsafe extern "C" fn attack_s4_hold_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketmflip"), false);
    ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketm"), false);
    smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD)(fighter)
}

// FIGHTER_STATUS_KIND_ATTACK_S4

unsafe extern "C" fn attack_s4_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if PostureModule::lr(fighter.boma()) == 1.0 {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketmflip"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketm"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("panm"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("clubm"), false);
    } else {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketm"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketmflip"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("panmflip"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("clubmflip"), false);
    }
    0.into()
}

unsafe extern "C" fn attack_s4_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketmflip"), false);
    ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketm"), false);
    0.into()
}


pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_hold_exec);
    agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_hold_exit);

    agent.status(Init, *FIGHTER_STATUS_KIND_ATTACK_S4, attack_s4_init);
    agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_S4, attack_s4_exit);
}