use super::*;

unsafe extern "C" fn special_hi2_rush_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    agent.status(Pre, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, special_hi2_rush_end_pre);
}