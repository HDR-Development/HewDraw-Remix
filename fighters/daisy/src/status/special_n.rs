use super::*;

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
        special_n_physics(fighter, false, false);
    }
    
    fighter.main_shift(special_n_main_loop)
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !fighter.is_motion(Hash40::new("special_n_attack")) {
        if fighter.status_frame() == 6 {
            if fighter.is_stick_backward() {
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
                if fighter.is_situation(*SITUATION_KIND_AIR) {
                    special_n_physics(fighter, false, true);
                }
            }
        }
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            let dive_start_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_n.dive_start_frame");
            let dive_end_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_n.dive_end_frame");
            if fighter.status_frame() == dive_start_frame {
                special_n_physics(fighter, true, false);
            }
            else if fighter.status_frame() == dive_end_frame {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
            else if (dive_start_frame..dive_end_frame).contains(&fighter.status_frame()) {
                fighter.check_wall_jump_cancel();
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_motion(Hash40::new("special_n_attack")) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
        else {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_attack"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 1.into();
            }
        }
        
        return 0.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            if fighter.status_frame() < ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_n.dive_end_frame") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_attack"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {  // temporary until air anim is created (will probably just make autocancel)
                WorkModule::set_float(fighter.module_accessor, 20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
            // MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
            // special_n_physics(fighter, true);
        }
    }
    
    return 0.into();
}

unsafe extern "C" fn special_n_physics(fighter: &mut L2CFighterCommon, dive: bool, turn: bool) {
    if dive {
        let dive_speed_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.dive_speed_y");
        let dive_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.dive_speed_x");

        sv_kinetic_energy!(reset_energy, fighter, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, dive_speed_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, dive_speed_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

        let facing = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(reset_energy, fighter, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, dive_speed_x * facing, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, dive_speed_x * facing, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else {
        if !turn {
            let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let limit_speed_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_limit_speed_y");
            let start_speed_y = speed_y.clamp(1.0, limit_speed_y);
            let accel_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_accel_y");
            let stable_speed_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_stable_speed_y");

            sv_kinetic_energy!(reset_energy, fighter, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, start_speed_y);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, accel_y);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, limit_speed_y);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, stable_speed_y);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }

        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let facing = PostureModule::lr(fighter.module_accessor) * if turn { -1.0 } else { 1.0 };
        let brake_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_brake_speed_x");
        let limit_speed_x_turn_add = if turn { ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_limit_speed_x_turn_add") } else { 0.0 };
        let limit_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_limit_speed_x") + limit_speed_x_turn_add;
        let start_speed_x_turn_add = if turn { ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_start_speed_x_turn_add") } else { 0.0 };
        let start_speed_x_base = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_start_speed_x") + start_speed_x_turn_add;
        let start_speed_x = speed_x.abs().clamp(0.6, start_speed_x_base) * facing;
        let stable_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_stable_speed_x");

        sv_kinetic_energy!(reset_energy, fighter, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, start_speed_x, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, brake_speed_x, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, limit_speed_x * facing, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, stable_speed_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

        // let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        // let control_x_stable_speed = air_speed_x_stable * (ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_control_x_mul") - 0.25);
        // sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, control_x_stable_speed, 0.0);
        // KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn special_n_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::daisy::status::CRYSTAL_ACTIVE) {
        PLAY_SE(fighter, Hash40::new("se_common_freeze"));
        EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), -10, 1, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.3, 1.0, 0.8);
        EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), 10, 1, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.3, 1.0, 0.8);
        EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.3, 1.0, 0.8);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_end);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exit);
}