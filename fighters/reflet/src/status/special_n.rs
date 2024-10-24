use super::*;

// FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START

pub unsafe extern "C" fn special_n_tron_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_float(fighter.battle_object, vars::reflet::instance::SPECIAL_N_CHARGE, fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) as f32);
    let ret = smashline::original_status(Main, fighter, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START)(fighter);
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    }
    ret
}

// FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD

pub unsafe extern "C" fn special_n_tron_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD)(fighter);
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_BIND);
    }
    ret
}

// FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END

pub unsafe extern "C" fn special_n_tron_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END)(fighter);
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        let gravity_mul = fighter.get_param_float("param_special_n", "special_n_air_invoke_fall_speed_mul");
        let air_accel_y = fighter.get_param_float("air_accel_y", "");
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * gravity_mul);
    }
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START, special_n_tron_start_main);
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD, special_n_tron_hold_main);
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END, special_n_tron_end_main);
}