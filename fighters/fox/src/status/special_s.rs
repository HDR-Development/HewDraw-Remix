use super::*;
use globals::*;

pub unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STEP_START, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP_PREV);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        special_s_air_mot(fighter);
    }
    else {
        special_s_ground_mot(fighter);
    }
    let illusion_rush_speed_mul = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("param_special_s"),
        hash40("illusion_rush_speed_mul")
    );
    let illusion_rush_speed_mul_power_up = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("param_special_s"),
        hash40("illusion_rush_speed_mul_power_up")
    );
    WorkModule::set_float(
        fighter.module_accessor,
        illusion_rush_speed_mul * illusion_rush_speed_mul_power_up,
        *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(special_s_substatus as *const () as _));
	fighter.main_shift(special_s_main_loop)
}

pub unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let mut cont = false;
    if !StatusModule::is_changing(fighter.module_accessor) {
        let is_end = MotionModule::is_end(fighter.module_accessor);
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if step == *FIGHTER_FOX_ILLUSION_STEP_FORCE_END {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STEP_END, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
                cont = true;
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_END {
                if is_end
                && situation == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
                    return 0.into();
                }
                if situation == *SITUATION_KIND_GROUND {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                    MotionModule::change_motion(
                        fighter.module_accessor,
                        Hash40::new("special_s_landing"),
                        0.0,
                        1.0,
                        false,
                        0.0,
                        false,
                        false
                    );
                    return 0.into();
                }
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_RUSH
            || step == *FIGHTER_FOX_ILLUSION_STEP_START {
                if situation == *SITUATION_KIND_GROUND
                || is_end {
                    cont = true;
                }
            }
        }
        else {
            if step == *FIGHTER_FOX_ILLUSION_STEP_FORCE_END {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STEP_END, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
                cont = true;
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_END {
                if situation == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
                    return 0.into();
                }
                if is_end
                && situation == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
                    return 0.into();
                }
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_RUSH
            || step == *FIGHTER_FOX_ILLUSION_STEP_START {
                if situation == *SITUATION_KIND_AIR
                || is_end {
                    cont = true;
                }
            }
        }
    }
    if cont {
        if !StatusModule::is_changing(fighter.module_accessor) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END)
            && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD_TO_END) {
                if MotionModule::is_end(fighter.module_accessor) {
                    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
                }
            }
        }
        let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        if situation != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if step == *FIGHTER_FOX_ILLUSION_STEP_END {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                special_s_change_mot(fighter, Hash40::new("special_air_s_end"));
                special_s_air_control(fighter);
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_RUSH {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_S as u32);
                special_s_change_mot(fighter, Hash40::new("special_air_s"));
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS.into());
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_START {
                special_s_air_mot(fighter);
            }
        }
        else {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
            if step == *FIGHTER_FOX_ILLUSION_STEP_END {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                special_s_change_mot(fighter, Hash40::new("special_s_end"));
                special_s_air_control(fighter);
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_RUSH {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                sv_kinetic_energy!(
                    friction_off,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_MOTION
                );
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                let motion_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
                sv_kinetic_energy!(
                    set_speed_mul,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_MOTION,
                    motion_mul
                );
                special_s_change_mot(fighter, Hash40::new("special_s"));
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_START {
                special_s_ground_mot(fighter);
            }
        }
    }
    0.into()
}

pub unsafe extern "C" fn special_s_change_mot(fighter: &mut L2CFighterCommon, mot: Hash40) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion(
            fighter.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
}

pub unsafe extern "C" fn special_s_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    0.into()
}

pub unsafe extern "C" fn special_s_ground_mot(fighter: &mut L2CFighterCommon) {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_s_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new("special_s_start"),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
}

pub unsafe extern "C" fn special_s_air_mot(fighter: &mut L2CFighterCommon) {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_s_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new("special_air_s_start"),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
}

pub unsafe extern "C" fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
	let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let set = if situation != *SITUATION_KIND_AIR {
        WorkModule::off_flag
    }
    else {
        WorkModule::on_flag
    };
    set(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_ILLUSION_LANDING);
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    let prev_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP_PREV);
    if step != prev_step {
        special_s_handle_step(fighter);
    }
    if step == *FIGHTER_FOX_ILLUSION_STEP_START {
        if situation == *SITUATION_KIND_AIR {
            let stop_y_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
            if stop_y_frame == 0 {
                let illusion_stop_y_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_stop_y_frame"));
                if illusion_stop_y_frame != 0 {
                    sv_kinetic_energy!(
                        reset_energy,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        0,
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        0.0
                    );
                    let illusion_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_accel_y"));
                    sv_kinetic_energy!(
                        set_accel,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        -illusion_accel_y
                    );
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                }
            }
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END) {
            return 0.into();
        }
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
        special_s_handle_step(fighter);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STEP_FORCE_END, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    }
    else if step == *FIGHTER_FOX_ILLUSION_STEP_RUSH {
        // Illusion Shorten implementation
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END);
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD) {
            return 0.into();
        }

        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
        special_s_handle_step(fighter);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STEP_FORCE_END, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD_TO_END);
        }
    }
    else if step == *FIGHTER_FOX_ILLUSION_STEP_END {
        if situation == *SITUATION_KIND_AIR {
            // Fix friction if the value is, for some reason, incorrect.
            let illusion_end_air_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_air_brake_x"));
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let brake = sv_kinetic_energy::get_brake_x(fighter.lua_state_agent);
            if brake != illusion_end_air_brake_x {
                sv_kinetic_energy!(
                    set_brake,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    illusion_end_air_brake_x,
                    0.0
                );
            }
            let stop_y_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
            if stop_y_frame == 0 {
                sv_kinetic_energy!(
                    reset_energy,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                );
                let illusion_end_air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_air_accel_y"));
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    -illusion_end_air_accel_y
                );
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
        special_s_air_control(fighter);
    }
    0.into()
}

pub unsafe extern "C" fn special_s_handle_step(fighter: &mut L2CFighterCommon) {
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::set_int(fighter.module_accessor, step, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP_PREV);
    if step == *FIGHTER_FOX_ILLUSION_STEP_START {
        let illusion_stop_y_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_stop_y_frame"));
        WorkModule::set_int(fighter.module_accessor, illusion_stop_y_frame, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let illusion_start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_start_x_mul"));
        if situation != *SITUATION_KIND_AIR {
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                speed_x * illusion_start_x_mul,
                0.0
            );
        }
        else {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                6,
                speed_x * illusion_start_x_mul,
                0.0,
                0.0,
                0.0,
                0.0
            );
            let illusion_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_accel_x"));
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                illusion_accel_x,
                0.0
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            if illusion_stop_y_frame != 0 {
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
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    }
    else if step == *FIGHTER_FOX_ILLUSION_STEP_END {
        let illusion_end_air_stop_y_frame = WorkModule::get_param_int(
            fighter.module_accessor,
            hash40("param_special_s"),
            hash40("illusion_end_air_stop_y_frame")
        );
        WorkModule::set_int(
            fighter.module_accessor,
            illusion_end_air_stop_y_frame,
            *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME
        );
        if situation != *SITUATION_KIND_AIR {
            let illusion_end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_brake_x"));
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                illusion_end_brake_x,
                0.0
            );
            let end_speed_param = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD) {
                hash40("illusion_shield_hit_end_speed_x")
            }
            else {
                hash40("illusion_end_speed_x")
            };
            let end_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), end_speed_param);
            let lr = PostureModule::lr(fighter.module_accessor);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                end_speed * lr,
                0.0
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        else {
            let illusion_end_air_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_air_brake_x"));
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                illusion_end_air_brake_x,
                0.0
            );
            let end_speed_param = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD) {
                hash40("illusion_shield_hit_end_air_speed_x")
            }
            else {
                hash40("illusion_end_air_speed_x")
            };
            let end_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), end_speed_param);
            let lr = PostureModule::lr(fighter.module_accessor);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                end_speed * lr,
                0.0
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
        }
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            10.0,
            10.0
        );
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    }
}

pub unsafe extern "C" fn special_s_air_control(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_AIR_CONTROL) {
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
        let illusion_end_control_air_speed_x_mul = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("param_special_s"),
            hash40("illusion_end_control_air_speed_x_mul")
        );
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            fighter,
            air_accel_x_mul * illusion_end_control_air_speed_x_mul
        );
        sv_kinetic_energy!(
            controller_set_accel_x_add,
            fighter,
            air_accel_x_add * illusion_end_control_air_speed_x_mul
        );
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let illusion_end_control_air_speed_x_stable = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("param_special_s"),
            hash40("illusion_end_control_air_speed_x_stable")
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * illusion_end_control_air_speed_x_stable,
            0.0
        );
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_AIR_CONTROL);
    }
}

pub fn install() {
    smashline::Agent::new("fox")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exec)
        .install();
}
