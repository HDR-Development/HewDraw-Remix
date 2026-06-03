use super::*;

pub unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, vars::ness::instance::SPECIAL_LW_DISABLE_STALL) {
        let stop_y_time = fighter.get_param_int("param_special_lw", "stop_y_time");
        VarModule::set_int(fighter.battle_object, vars::ness::status::SPECIAL_LW_STOP_Y_FRAME, stop_y_time);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }
    else {
        VarModule::set_int(fighter.battle_object, vars::ness::status::SPECIAL_LW_STOP_Y_FRAME, 0);
        let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y.min(0.0) * 0.33
        );
    }
    VarModule::off_flag(fighter.battle_object, vars::ness::instance::SPECIAL_LW_DISABLE_JC);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

unsafe extern "C" fn special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
        return false.into();
    }

    let stop_y_time = fighter.get_param_int("param_special_lw", "stop_y_time");
    if stop_y_time != 0 {
        let work_stop_y_frame = VarModule::get_int(fighter.battle_object, vars::ness::status::SPECIAL_LW_STOP_Y_FRAME);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if work_stop_y_frame - 1 <= 0 {
            let mut reflector_air_accel_y = if VarModule::is_flag(fighter.battle_object, vars::ness::instance::SPECIAL_LW_DISABLE_STALL) {
                // fighter.get_param_float("air_accel_y", "") / 2.0
                fighter.get_param_float("param_special_lw", "accel_y") * 1.67
            } else {
                fighter.get_param_float("param_special_lw", "accel_y")
            };
            let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                speed_y.min(0.0)
            );
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -reflector_air_accel_y
            );
        }
        else {
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
        }
        VarModule::set_int(fighter.battle_object, vars::ness::status::SPECIAL_LW_STOP_Y_FRAME, work_stop_y_frame - 1);
    }
    return false.into();
}

unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD
    && StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT
    && StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END {
        VarModule::set_flag(fighter.battle_object, vars::ness::instance::SPECIAL_LW_DISABLE_STALL, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
    }
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

unsafe extern "C" fn special_lw_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
    if fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) <= turn_stick_x {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }

    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
        return false.into();
    }

    let stop_y_time = fighter.get_param_int("param_special_lw", "stop_y_time");
    if stop_y_time != 0 {
        let work_stop_y_frame = VarModule::get_int(fighter.battle_object, vars::ness::status::SPECIAL_LW_STOP_Y_FRAME);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if work_stop_y_frame - 1 <= 0 {
            let mut reflector_air_accel_y = if VarModule::is_flag(fighter.battle_object, vars::ness::instance::SPECIAL_LW_DISABLE_STALL) {
                fighter.get_param_float("param_special_lw", "accel_y") * 1.67
            } else {
                fighter.get_param_float("param_special_lw", "accel_y")
            };
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -reflector_air_accel_y
            );
        }
        else {
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
        }
        VarModule::set_int(fighter.battle_object, vars::ness::status::SPECIAL_LW_STOP_Y_FRAME, work_stop_y_frame - 1);
    }
    return false.into();
}

unsafe extern "C" fn special_lw_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END
    && StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT {
        VarModule::set_flag(fighter.battle_object, vars::ness::instance::SPECIAL_LW_DISABLE_STALL, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
    }
    smashline::original_status(End, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD)(fighter)
}


unsafe extern "C" fn special_lw_hit_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END
    && StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD {
        VarModule::set_flag(fighter.battle_object, vars::ness::instance::SPECIAL_LW_DISABLE_STALL, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
    }
    smashline::original_status(End, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT)(fighter)
}

unsafe extern "C" fn special_lw_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_flag(fighter.battle_object, vars::ness::instance::SPECIAL_LW_DISABLE_STALL, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
    smashline::original_status(End, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_exec);
    agent.status(End,  *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);
    agent.status(Exec, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, special_lw_hold_exec);
    agent.status(End,  *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, special_lw_hold_end);
    agent.status(Exec, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, special_lw_exec);
    agent.status(End,  *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, special_lw_hit_end);
    agent.status(Exec, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, special_lw_exec);
    agent.status(End,  *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, special_lw_end_end);
}