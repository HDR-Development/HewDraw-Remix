use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_N

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    // air stall wasnt working
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        let special_n_attack_speed_y = fighter.get_param_float("param_special_n", "special_n_attack_speed_y");
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_n_attack_speed_y);
        let special_n_attack_stable_y = fighter.get_param_float("param_special_n", "special_n_attack_stable_y");
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_n_attack_stable_y);
        let special_n_attack_accel_y = fighter.get_param_float("param_special_n", "special_n_attack_accel_y");
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_n_attack_accel_y);
    }
    ret
}

// FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT

unsafe extern "C" fn special_n_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT)(fighter);
    // set gravity limits to match first stage of move
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        let special_n_attack_stable_y = fighter.get_param_float("param_special_n", "special_n_attack_stable_y");
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_n_attack_stable_y);
        let special_n_attack_accel_y = fighter.get_param_float("param_special_n", "special_n_attack_accel_y");
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_n_attack_accel_y);
    }
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT, special_n_hit_main);
}
