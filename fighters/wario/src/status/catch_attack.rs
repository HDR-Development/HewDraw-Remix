use super::*;

// FIGHTER_STATUS_KIND_CATCH_ATTACK

// Force opponent rotation

unsafe extern "C" fn catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {

    let boma = fighter.boma();

    let mut vec =Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let offset = ModelModule::joint_global_rotation(fighter.module_accessor,Hash40::new("throw"),&mut vec,false);
    let rot = Vector3f{x: vec.x, y: 0.0, z: 0.0};
    PostureModule::set_rot(
        boma.get_grabbed_opponent_boma(),
        &rot,
        0
    );
    return false.into();
}

// Reset opponent rotation

unsafe extern "C" fn catch_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.boma();

    PostureModule::set_rot(
        boma.get_grabbed_opponent_boma(),
        &Vector3f::zero(),
        0
    );

    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_CATCH_ATTACK)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_end);
}
