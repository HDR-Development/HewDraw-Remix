use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_LW

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_lw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        // Returning here allows for running shine
        return 0.into();
    }

    if !VarModule::is_flag(fighter.battle_object, vars::falco::instance::SPECIAL_LW_DISABLE_STALL) {
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

    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let reflector_air_start_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.reflector_air_start_x_mul");

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x * reflector_air_start_x_mul, 0.0);
    app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);

    0.into()
}

pub unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, vars::falco::instance::SPECIAL_LW_DISABLE_STALL) {
        let stop_y_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_lw.reflector_air_stop_y_frame");
        VarModule::set_int(fighter.battle_object, vars::falco::status::SPECIAL_LW_STOP_Y_FRAME, stop_y_frame);
    }
    else {
        VarModule::set_int(fighter.battle_object, vars::falco::status::SPECIAL_LW_STOP_Y_FRAME, 0);
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y
            );
        }
    }
    special_lw_motion_helper(fighter);
    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CURRENT_FRAME].get_i32() > 2  // Allows for jump cancel on frame 4 in game
    && !fighter.is_in_hitlag()
    && fighter.check_jump_cancel(false, false) {
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(statuses::falco::SPECIAL_LW_LOOP.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn special_lw_motion_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe extern "C" fn special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
        return 0.into();
    }
    let stop_y_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_lw.reflector_air_stop_y_frame");
    if stop_y_frame != 0 {
        let work_stop_y_frame = VarModule::get_int(fighter.battle_object, vars::falco::status::SPECIAL_LW_STOP_Y_FRAME);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if work_stop_y_frame - 1 <= 0 {
            let mut reflector_air_accel_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.reflector_air_accel_y");
            if VarModule::is_flag(fighter.battle_object, vars::falco::instance::SPECIAL_LW_DISABLE_STALL) {
                reflector_air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            }
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
        VarModule::set_int(fighter.battle_object, vars::falco::status::SPECIAL_LW_STOP_Y_FRAME, work_stop_y_frame - 1);
    }
    0.into()
}

unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != statuses::falco::SPECIAL_LW_LOOP {
        VarModule::set_flag(fighter.battle_object, vars::falco::instance::SPECIAL_LW_DISABLE_STALL, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
    }
    0.into()
}

// SPECIAL_LW_LOOP

unsafe extern "C" fn special_lw_loop_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0xc000,
        0x80,
        0,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_lw_loop_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(
        fighter,
        MA_MSC_SHIELD_SET_STATUS,
        COLLISION_KIND_REFLECTOR,
        FIGHTER_FALCO_REFLECTOR_KIND_REFLECTOR,
        SHIELD_STATUS_NORMAL,
        FIGHTER_REFLECTOR_GROUP_EXTEND
    );
    smash::app::sv_module_access::shield(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);

    0.into()
}

unsafe extern "C" fn special_lw_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_loop"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.main_shift(special_lw_loop_main_loop)
}

unsafe extern "C" fn special_lw_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    if fighter.check_jump_cancel(false, false) {
        return 0.into();
    }

    let reflector_init_keep_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_lw.reflector_init_keep_frame");
    if fighter.global_table[CURRENT_FRAME].get_i32() > reflector_init_keep_frame {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(statuses::falco::SPECIAL_LW_END.into(), false.into()); 
            return 0.into();
        }
    }
    if !VarModule::is_flag(fighter.battle_object, vars::falco::status::SET_ATTACK) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    else {
        let reflector_attack_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_lw.reflector_attack_frame");
        if MotionModule::frame(fighter.module_accessor) >= reflector_attack_frame as f32 {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x2ad70b3640));
            app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(0);

            VarModule::on_flag(fighter.battle_object, vars::falco::status::SET_ATTACK);
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_loop"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_loop"), -1.0, 1.0, 0.0, false, false);
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_loop_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != statuses::falco::SPECIAL_LW_END {
        VarModule::set_flag(fighter.battle_object, vars::falco::instance::SPECIAL_LW_DISABLE_STALL, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
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
        return 0.into();
    }
    let stop_y_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_lw.reflector_air_stop_y_frame");
    if stop_y_frame != 0 {
        let work_stop_y_frame = VarModule::get_int(fighter.battle_object, vars::falco::status::SPECIAL_LW_STOP_Y_FRAME);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if work_stop_y_frame - 1 <= 0 {
            let mut reflector_air_accel_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.reflector_air_accel_y");
            if VarModule::is_flag(fighter.battle_object, vars::falco::instance::SPECIAL_LW_DISABLE_STALL) {
                reflector_air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            }
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
        VarModule::set_int(fighter.battle_object, vars::falco::status::SPECIAL_LW_STOP_Y_FRAME, work_stop_y_frame - 1);
    }
    0.into()
}

// SPECIAL_LW_END

unsafe extern "C" fn special_lw_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0xc000,
        0x80,
        0,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_lw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_WAIT);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.main_shift(special_lw_end_main_loop)
}

unsafe extern "C" fn special_lw_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    if fighter.check_jump_cancel(false, false) {
        return 0.into();
    }

    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (!fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_WAIT)
        && MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_FALL)
        && MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
            return 0.into();
        }
        if !StatusModule::is_changing(fighter.module_accessor)
        && StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_WAIT);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_FALL);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_end"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_WAIT);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_FALL);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_end"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_flag(fighter.battle_object, vars::falco::instance::SPECIAL_LW_DISABLE_STALL, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
    0.into()
}

unsafe extern "C" fn special_lw_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
        return 0.into();
    }
    let stop_y_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_lw.reflector_air_stop_y_frame");
    if stop_y_frame != 0 {
        let work_stop_y_frame = VarModule::get_int(fighter.battle_object, vars::falco::status::SPECIAL_LW_STOP_Y_FRAME);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if work_stop_y_frame - 1 <= 0 {
            let mut reflector_air_accel_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.reflector_air_accel_y");
            if VarModule::is_flag(fighter.battle_object, vars::falco::instance::SPECIAL_LW_DISABLE_STALL) {
                reflector_air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            }
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
        VarModule::set_int(fighter.battle_object, vars::falco::status::SPECIAL_LW_STOP_Y_FRAME, work_stop_y_frame - 1);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
        agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
        agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_init);
        agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
        agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_exec);
        agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);

        agent.status(Pre, statuses::falco::SPECIAL_LW_LOOP, special_lw_loop_pre);
        agent.status(Init, statuses::falco::SPECIAL_LW_LOOP, special_lw_loop_init);
        agent.status(Main, statuses::falco::SPECIAL_LW_LOOP, special_lw_loop_main);
        agent.status(Exec, statuses::falco::SPECIAL_LW_LOOP, special_lw_loop_exec);
        agent.status(End, statuses::falco::SPECIAL_LW_LOOP, special_lw_loop_end);
        agent.status(Pre, statuses::falco::SPECIAL_LW_END, special_lw_end_pre);
        agent.status(Main, statuses::falco::SPECIAL_LW_END, special_lw_end_main);
        agent.status(Exec, statuses::falco::SPECIAL_LW_END, special_lw_end_exec);
        agent.status(End, statuses::falco::SPECIAL_LW_END, special_lw_end_end);
}
