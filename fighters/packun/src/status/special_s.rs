use super::*;

unsafe extern "C" fn special_s_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_charge"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        CORRECT(fighter, *GROUND_CORRECT_KIND_AIR);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_charge"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_n_charge as *const () as _));
    fighter.main_shift(special_s_charge_main_loop)
}

unsafe extern "C" fn sub_special_n_charge(fighter: &mut L2CFighterCommon, param: L2CValue) -> L2CValue {
    if param.get_bool() {
        fighter.inc_int(*FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT);
    }

    return 0.into();
}

unsafe extern "C" fn special_s_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let count = fighter.get_int(*FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT);
    let charge_max_frame = fighter.get_param_int("param_special_s", "charge_max_frame");
    if count >= charge_max_frame {
        fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }
    if fighter.is_pad_flag(PadFlag::SpecialTrigger) {
        fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT.into(), false.into());
        return 0.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_charge"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 0 {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            }
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_charge"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
            if fighter.sub_check_command_guard().get_bool() {
                fighter.set_int(*FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_GROUND_GUARD, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE);
                fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL.into(), true.into());
                return 1.into();
            }
        }
        if fighter.sub_check_jump_in_charging().get_bool() {
            fighter.set_int(*FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_GROUND_JUMP, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL.into(), true.into());
            return 1.into();
        }
    }
    else {
        if fighter.is_cat_flag(Cat1::AirEscape) {
            fighter.set_int(*FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_NONE, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL.into(), true.into());
            return 1.into();
        }
        if fighter.sub_check_jump_in_charging().get_bool() {
            fighter.set_int(*FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_AIR_JUMP_AERIAL, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_JUMP_CANCEL.into(), true.into());
            return 1.into();
        }
    }
    if !fighter.global_table[IS_STOPPING].get_bool() {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_s_charge_set_kinetic(fighter);
        }
    }
    stance_head(fighter);
    
    return 0.into();
}

unsafe extern "C" fn special_s_charge_set_kinetic(fighter: &mut L2CFighterCommon) {
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let reset_type = if fighter.is_situation(*SITUATION_KIND_GROUND) { ENERGY_STOP_RESET_TYPE_GROUND } else { ENERGY_STOP_RESET_TYPE_AIR };
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, reset_type, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
}

unsafe extern "C" fn special_s_shoot_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        if !(VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 0
        && fighter.get_int(*FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) < 60) {    // do not apply physics to uncharged Fiery Breath
            let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            let start_air_speed_x_mul = fighter.get_param_float("param_special_s", "start_air_speed_x_mul");
            let start_air_speed_y = fighter.get_param_float("param_special_s", "start_air_speed_y");
            let accel_y = fighter.get_param_float("param_special_s", "accel_y");
            
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x * start_air_speed_x_mul, 0.0);
            if !fighter.is_flag(*FIGHTER_PACKUN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_LANDING) {
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, start_air_speed_y);
            }
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);

            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
            let stable_speed_x = fighter.get_param_float("air_speed_x_stable", "");
            let facing = PostureModule::lr(fighter.module_accessor);
            let speed_x = sum_speed_x.abs().min(stable_speed_x) * facing;
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x * 0.5, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, stable_speed_x * 0.5, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, stable_speed_x * 0.5, 0.0);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.03);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }

    EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
    let handle = fighter.get_int(*FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_CHARGE_MAX_EFFECT_HANDLE);
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_EFFECT_REMOVE, handle);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);

    return 0.into();
}

unsafe extern "C" fn special_s_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        let motion = if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 
            { Hash40::new("special_s_shoot_s") } else { Hash40::new("special_s_shoot") };
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        CORRECT(fighter, *GROUND_CORRECT_KIND_AIR);
        let motion = if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 
            { Hash40::new("special_air_s_shoot_s") } else { Hash40::new("special_air_s_shoot") };
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.main_shift(special_s_shoot_main_loop)
}

unsafe extern "C" fn special_s_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
          return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                let motion = if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 
                    { Hash40::new("special_s_shoot_s") } else { Hash40::new("special_s_shoot") };
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC_DONE);
            }
            else {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 0 {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                }
                let motion = if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 
                    { Hash40::new("special_air_s_shoot_s") } else { Hash40::new("special_air_s_shoot") };
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
                if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC) {
                    // let accel_y = -1.0 * WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("accel_y"));
                    // sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, accel_y);

                    let control_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("control_accel_x"));
                    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
                    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, control_accel_x);
                    sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0);
                    KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);

                    let air_accel_y = -(WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0));
                    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                    let brake_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                    let limit_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_y_limit"));
                    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y);
                    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
                    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, brake_y);
                    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, limit_speed_y);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC_DONE);
                }
            }
        }
    }
    if !fighter.global_table[IS_STOPPING].get_bool() {
        special_s_shoot_helper(fighter);
    }
    stance_head(fighter);
    
    return 0.into();
}

unsafe fn special_s_shoot_helper(fighter: &mut L2CFighterCommon) {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            let sum_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let mut stop_type = ENERGY_STOP_RESET_TYPE_NONE;
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                stop_type = ENERGY_STOP_RESET_TYPE_GROUND;
            }
            else {
                stop_type = ENERGY_STOP_RESET_TYPE_AIR;
            }
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, stop_type, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_x, 0.0);
            KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
}

unsafe extern "C" fn special_s_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    stance_head(fighter);
    return 0.into();
}

unsafe fn stance_head(fighter: &mut L2CFighterCommon) {
    match VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) {
        0 => {
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), true);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), false);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), false);
        },
        1 => {
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), true);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), false);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), false);
        },
        2 => {
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), true);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), false);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), false);
        },
        _ => {}
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CHARGE, special_s_charge_main);

    agent.status(Init, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT, special_s_shoot_init);
    agent.status(Main, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT, special_s_shoot_main);

    agent.status(Exec, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL, special_s_end_exec);
    agent.status(Exec, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_JUMP_CANCEL, special_s_end_exec);
    agent.status(Exec, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_END, special_s_end_exec);
}