use super::*;

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, air_speed_x_stable * 0.4, 0.0);
    special_n_drift(fighter, 0.8, 0.5);
    ret
}

unsafe extern "C" fn special_n_open_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_OPEN_WAIT)(fighter);
    special_n_drift(fighter, 0.8, 0.5);
    ret
}

unsafe extern "C" fn special_n_bite_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE)(fighter);
    special_n_drift(fighter, 0.5, 0.5);
    ret
}

unsafe extern "C" fn special_n_drift(fighter: &mut L2CFighterCommon, speed_mul: f32, accel_mul: f32) {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
        
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * speed_mul, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul * accel_mul);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add * accel_mul); 
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_OPEN_WAIT, special_n_open_wait_main);
    agent.status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE, special_n_bite_main);
}