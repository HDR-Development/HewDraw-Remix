use super::*;

pub unsafe extern "C" fn special_hi_check_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    // Aerial Grappling Hook cancel
    if fighter.is_situation(*SITUATION_KIND_AIR) && !fighter.is_in_hitlag() {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }

    return 0.into();
}

// FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH

unsafe extern "C" fn special_hi2_rush_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_frame() == 1 {
        let curr_radians = fighter.get_float(*FIGHTER_JACK_STATUS_SPECIAL_HI2_FLOAT_SDIR);
        let rush_speed = fighter.get_param_float("param_special_hi", "rush_speed");
        let dir = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_STOP).signum();
        let speed_x = f32::cos(curr_radians) * rush_speed;
        let speed_y = f32::sin(curr_radians) * rush_speed;
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x.abs() * dir, speed_y);
    }
    smashline::original_status(Exec, fighter, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH)(fighter)
}

// FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END

unsafe extern "C" fn special_hi2_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Pre, fighter, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END)(fighter);

    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_speed_x_max_mul");
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * speed_x_max_mul,
        0.0
    );

    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_check_attack);
    agent.status(Exec, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH, special_hi2_rush_exec);
    agent.status(Pre, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, special_hi2_end_pre);
}