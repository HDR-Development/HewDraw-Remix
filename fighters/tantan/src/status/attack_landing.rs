use super::*;

// ARMS land, prevents ARMDashing

unsafe extern "C" fn tantan_attack_landing_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.motion_frame() < 2.0 {
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.5);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING_LIGHT, tantan_attack_landing_exec);
}
