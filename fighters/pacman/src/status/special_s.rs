use super::*;
use globals::*;

// FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH

pub unsafe extern "C" fn special_s_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH)(fighter);
    // Disables super armor on sideB Power Pellet consumption
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
    ret
}

// FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN

pub unsafe extern "C" fn special_s_return_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN)(fighter);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let fall_x_stable_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.fall_x_stable_mul");
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * fall_x_stable_mul,
            0.0
        );
    }
    ret
}

// FIGHTER_STATUS_KIND_FALL_SPECIAL

pub unsafe extern "C" fn fall_special_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN {
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let fall_x_stable_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.fall_x_stable_mul");
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * fall_x_stable_mul,
            0.0
        );
    }
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_FALL_SPECIAL)(fighter)
}

pub fn install() {
    smashline::Agent::new("pacman")
        .status(Main, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH, special_s_dash_main)
        .status( Main,*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN, special_s_return_main)
        .status(Init, *FIGHTER_STATUS_KIND_FALL_SPECIAL, fall_special_init)
        .install();
}