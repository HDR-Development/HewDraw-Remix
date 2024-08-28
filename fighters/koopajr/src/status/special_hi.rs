use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_HI

pub unsafe extern "C" fn special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::mul_speed(fighter.module_accessor, &Vector3f::zero(), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi);
}