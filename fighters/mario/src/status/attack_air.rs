use super::*;

unsafe extern "C" fn attack_air_check_attack(fighter: &mut L2CFighterCommon, param_1: &L2CValue, param_2: &L2CValue) -> L2CValue {
    if fighter.is_motion(Hash40::new("attack_air_lw")) {
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let add_speed = -0.15 * speed_y + 0.2;
        KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(0.0, add_speed, 0.0));
    }
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_check_attack);
}
