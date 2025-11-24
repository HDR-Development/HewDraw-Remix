use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON)  as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    
    0.into()
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    sv_kinetic_energy!(clear_speed_ex, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.set_int(1, *FIGHTER_PEACH_STATUS_SPECIAL_S_WORK_INT_ENABLE_UNIQ);
    if !StopModule::is_stop(fighter.module_accessor) {
        special_s_check_flick(fighter);
    }
    VarModule::off_flag(fighter.battle_object, vars::daisy::instance::SPECIAL_S_GROUND_START);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        VarModule::on_flag(fighter.battle_object, vars::daisy::instance::SPECIAL_S_GROUND_START);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(special_s_check_flick as *const () as _));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_S_START);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_S_AIR_START);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP.into(), false.into());
    }

    return 0.into();
}

unsafe extern "C" fn special_s_check_flick(fighter: &mut L2CFighterCommon) -> L2CValue {
    let flick_x = ControlModule::get_flick_no_reset_x(fighter.module_accessor);
    if flick_x <= fighter.get_param_int("common", "special_smash_flick_x") {
        fighter.on_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_FLAG_FLICK_START);
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x20cbc92683));
        fighter.push_lua_stack(&mut L2CValue::I32(1));
        fighter.push_lua_stack(&mut L2CValue::I32(*FIGHTER_LOG_DATA_INT_HAJIKI_NUM));
        app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.set_int(0, *FIGHTER_PEACH_STATUS_SPECIAL_S_WORK_INT_ENABLE_UNIQ);
    }

    return 0.into();
}

// FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP

unsafe extern "C" fn special_s_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_S_JUMP,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::daisy::instance::DISABLE_SPECIAL_S);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_jump"), 0.0, 1.0, false, 0.0, false, false);
    fighter.main_shift(special_s_jump_main_loop)
}

unsafe extern "C" fn special_s_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) || fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::unable_energy_all(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);

        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x, speed_y, 0.0, 0.0, 0.0);
        let dec_accel_x = fighter.get_param_float("param_special_s", "special_s_jump_dec_accel_x");
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, dec_accel_x, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_stable_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
        let speed_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.end_speed_y_mul");
        let accel_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.end_accel_y_mul");
        let stable_speed_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.end_stable_speed_y_mul");
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y * accel_y_mul);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * accel_y_mul);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_stable_y * stable_speed_y_mul);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let control_speed_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.end_control_speed_x_mul");
        let control_accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.end_control_accel_x_mul");
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * control_speed_x_mul, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, control_accel_x_mul);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_s_away_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_s_away_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s_end") as i64, *FIGHTER_PEACH_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_end") as i64, *FIGHTER_PEACH_STATUS_WORK_INT_MOT_AIR_KIND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    fighter.on_flag(*FIGHTER_PEACH_MOTION_TRANSITION_TERM_ID_MOT_END);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(0.75, 1.0, 1.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ID_GROUND);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    //sub_special_s_away_end(fighter);
    
    fighter.main_shift(special_s_away_end_main_loop)
}

// unsafe extern "C" fn sub_special_s_away_end(fighter: &mut L2CFighterCommon) {
//     if fighter.is_situation(*SITUATION_KIND_GROUND) {
//         if !fighter.is_motion(Hash40::new("special_air_s_end")) {
//             return;
//         }
//         if fighter.motion_frame() < 10.0 {
//             MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);
//         }
//         else {
//             sub_sub_special_s_away_end(fighter);
//         }
//         GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
//         KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
//     }
//     else {
//         if !fighter.global_table[IS_STOPPING].get_bool() {
//             return;
//         }
//         if !fighter.is_motion(Hash40::new("special_s_end")) {
//             return;
//         }
//         KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
//         MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false);
//         GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
//         fighter.off_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_DONE_CONTROLLER_MOVE);
//     }
// }

// unsafe extern "C" fn sub_sub_special_s_away_end(fighter: &mut L2CFighterCommon) {
//     if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT) {
//         if fighter.is_situation(*SITUATION_KIND_GROUND) {
//             let speed_y_stable_frame = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_SPEED_Y_STABLE_FRAME);
//             let landing_heavy_frame = fighter.get_param_int("landing_heavy_frame", "0");
//             if speed_y_stable_frame < landing_heavy_frame {
//                 fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), false.into());
//                 return;
//             }
//         }
//     }
//     if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
//         return;
//     }
//     if fighter.is_situation(*SITUATION_KIND_AIR) {
//         return;
//     }
//     fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
//     return;
// }

unsafe extern "C" fn special_s_away_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            if fighter.status_frame() <= 20 {
                WorkModule::set_float(fighter.module_accessor, 30.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 1.into();
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    if fighter.is_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_START_CONTROLLER_MOVE) {
        if !fighter.is_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_DONE_CONTROLLER_MOVE) {
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            let control_accel_x = fighter.get_param_float("param_special_s", "special_air_s_end_control_accel_x");
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, control_accel_x);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
            fighter.on_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_DONE_CONTROLLER_MOVE);
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);

    agent.status(Pre, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_pre);
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_main);

    agent.status(Pre, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END, special_s_away_end_pre);
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END, special_s_away_end_main);
}