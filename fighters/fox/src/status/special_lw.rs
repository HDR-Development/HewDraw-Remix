use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_LW

unsafe extern "C" fn special_lw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        // Returning here allows for running shine
        return 0.into();
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING) {
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
    } else {
        let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y.min(0.0) * 0.2
        );
    }
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

pub unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING) {
        let stop_y_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("reflector_air_stop_y_frame"));
        WorkModule::set_int(fighter.module_accessor, stop_y_frame, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    special_lw_motion_helper(fighter);
    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Allows for jump cancel on frame 4 in game
    if fighter.global_table[CURRENT_FRAME].get_i32() < 3 {
        fighter.clear_commands(Cat1::Jump | Cat1::JumpButton | Cat1::AttackHi4); 
    } else if !fighter.is_in_hitlag() && fighter.check_jump_cancel(false, false) {
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_req(*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, false);
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_lw_motion_helper(fighter);
    }

    0.into()
}

unsafe extern "C" fn special_lw_motion_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start_l"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);
        }
        else {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start_l"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
            }
        }
        fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {    
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start_l"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);
        }
        else {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start_l"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
            }
        }
        fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_GROUND), 1.into());
    }
}

unsafe extern "C" fn special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
        return false.into();
    }
    let stop_y_frame = fighter.get_param_int("param_special_lw", "reflector_air_stop_y_frame");
    if stop_y_frame != 0 {
        let work_stop_y_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if work_stop_y_frame - 1 <= 0 {
            let mut reflector_air_accel_y = if fighter.is_flag(*FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING) {
                // fighter.get_param_float("air_accel_y", "") / 2.0
                fighter.get_param_float("param_special_lw", "reflector_air_accel_y") * 1.2
            } else {
                fighter.get_param_float("param_special_lw", "reflector_air_accel_y")
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
        WorkModule::set_int(fighter.module_accessor, work_stop_y_frame - 1, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    return false.into();
}

pub unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ![*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        WorkModule::set_flag(fighter.module_accessor, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING);
    }
    0.into()
}

unsafe extern "C" fn special_lw_loop_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
    if fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) <= turn_stick_x {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }

    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
        return false.into();
    }
    let stop_y_frame = fighter.get_param_int("param_special_lw", "reflector_air_stop_y_frame");
    if stop_y_frame != 0 {
        let work_stop_y_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if work_stop_y_frame - 1 <= 0 {
            let mut reflector_air_accel_y = if fighter.is_flag(*FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING) {
                // fighter.get_param_float("air_accel_y", "") / 2.0
                fighter.get_param_float("param_special_lw", "reflector_air_accel_y") * 1.2
            } else {
                fighter.get_param_float("param_special_lw", "reflector_air_accel_y")
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
        WorkModule::set_int(fighter.module_accessor, work_stop_y_frame - 1, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    return false.into();
}

pub unsafe extern "C" fn special_lw_loop_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ![*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        WorkModule::set_flag(fighter.module_accessor, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING);
    }
    0.into()
}

pub unsafe extern "C" fn special_lw_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);
    fighter.on_flag(*FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_HIT_TO_RESTART);
    special_lw_hit_motion_helper(fighter);
    fighter.main_shift(special_lw_hit_main_loop)
}

unsafe extern "C" fn special_lw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.check_jump_cancel(false, false) {
            return 0.into();
        }
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END.into(), false.into()); 
            return 0.into();
        }
        fighter.change_status_req(*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, false);
        return 0.into();
    }
    
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_lw_hit_motion_helper(fighter);
    }

    return 0.into();
}

unsafe extern "C" fn special_lw_hit_motion_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit_l"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);
        }
        else {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_hit_l"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_hit"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {    
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit_l"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);
        }
        else {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_hit_l"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_hit"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);

    agent.status(Exec, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, special_lw_loop_exec);
    agent.status(End, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, special_lw_loop_end);

    agent.status(Exec, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, special_lw_exec);

    agent.status(Main, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, special_lw_hit_main);
    agent.status(Exec, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, special_lw_exec);
}