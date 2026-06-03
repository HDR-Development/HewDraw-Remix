use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return special_s_init_common(fighter);
}

pub unsafe extern "C" fn special_s_init_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int(*FIGHTER_RYU_STRENGTH_S, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    speed_x *= fighter.get_param_float("param_special_s", "speed_x_mul");
    speed_x += fighter.get_param_float("param_special_s", "add_speed_x") * PostureModule::lr(fighter.module_accessor);

    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    speed_y *= fighter.get_param_float("param_special_s", "speed_y_mul");

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        speed_y += fighter.get_param_float("param_special_s", "air_add_speed_y");

        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

        let air_accel_y = fighter.get_param_float("param_special_s", "air_accel_y");
        let air_max_speed_y = fighter.get_param_float("param_special_s", "air_max_speed_y");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_max_speed_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    } else {
        let ground_speed_limit = fighter.get_param_float("common", "ground_speed_limit");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_NONE, speed_x, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ground_speed_limit, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }

    if fighter.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        let command_power_mul = fighter.get_param_float("param_special_s", "command_power_mul");
        AttackModule::set_power_mul_status(fighter.module_accessor, command_power_mul);
    }

    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    return false.into();
}

pub unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_main_common(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s_main_sub_common as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop_common as *const () as _))
}

pub unsafe extern "C" fn special_s_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StopModule::is_stop(fighter.module_accessor)
    && false { // yea idk man
        fighter.inc_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    return false.into();
}

pub unsafe extern "C" fn special_s_main_sub_common(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        fighter.inc_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    return false.into();
}

unsafe extern "C" fn special_s_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if StatusModule::is_changing(fighter.module_accessor) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            return;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let air_accel_y = fighter.get_param_float("param_special_s", "air_accel_y");
        let air_max_speed_y = fighter.get_param_float("param_special_s", "air_max_speed_y");
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_max_speed_y);
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if StatusModule::is_changing(fighter.module_accessor) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            return;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        let ground_speed_limit = fighter.get_param_float("common", "ground_speed_limit");
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ground_speed_limit, 0.0);
    }
}

pub unsafe extern "C" fn special_s_main_loop_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        special_s_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP.into(), false.into());
    }
    return false.into();
}

pub unsafe extern "C" fn special_s_exec_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_situation(*SITUATION_KIND_GROUND)
    && KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) < 0.0 {
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(1.0, 0.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    return false.into();
}

// FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND

pub unsafe extern "C" fn special_s_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    return special_s_init_common(fighter);
}

pub unsafe extern "C" fn special_s_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_main_common(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s_main_sub_common as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop_common as *const () as _))
}

// FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP

pub unsafe extern "C" fn special_s_loop_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    fighter.set_int(situation_kind, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);

    // get loop num and speed hash locations conditionally
    let loop_count_hash;
    let speed_x_hash;
    let strength = fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        (loop_count_hash, speed_x_hash) = match strength {
            _ if strength == *FIGHTER_RYU_STRENGTH_W => ("air_loop_num_w", "air_speed_x_w"),
            _ if strength == *FIGHTER_RYU_STRENGTH_M => ("air_loop_num_m", "air_speed_x_m"),
            _ => ("air_loop_num_s", "air_speed_x_s")
        };
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        (loop_count_hash, speed_x_hash) = match strength {
            _ if strength == *FIGHTER_RYU_STRENGTH_W => ("loop_num_w", "speed_x_w"),
            _ if strength == *FIGHTER_RYU_STRENGTH_M => ("loop_num_m", "speed_x_m"),
            _ => ("loop_num_s", "speed_x_s")
        };
    }

    // use hashes to get loop num and speed_x
    let mut loop_num = fighter.get_param_int("param_special_s", loop_count_hash);
    if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::EX_SPECIAL_USED) { loop_num = 3; }
    fighter.set_int(loop_num, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
    let speed_x = if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::EX_SPECIAL_USED) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND { 1.8 } else { 1.5 }
    } else {
        fighter.get_param_float("param_special_s", speed_x_hash)
    };

    // init stop energy
    let lr = PostureModule::lr(fighter.module_accessor);
    let stop_type = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        ENERGY_STOP_RESET_TYPE_AIR
    } else {
        ENERGY_STOP_RESET_TYPE_NONE
    };
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, stop_type, speed_x * lr, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let ground_speed_limit = fighter.get_param_float("common", "ground_speed_limit");
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ground_speed_limit, 0.0);
    }
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let air_accel_y = fighter.get_param_float("param_special_s", "air_accel_y");
        let air_max_speed_y = fighter.get_param_float("param_special_s", "air_max_speed_y");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y,  0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_max_speed_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    if fighter.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        let command_power_mul = fighter.get_param_float("param_special_s", "command_power_mul");
        AttackModule::set_power_mul_status(fighter.module_accessor, command_power_mul);
    }

    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    return false.into();
}

unsafe extern "C" fn special_s_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_s_mot_name = if fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION) != *SITUATION_KIND_GROUND {
        "special_air_s"
    } else {
        "special_s"
    };
    MotionModule::change_motion(fighter.module_accessor, Hash40::new(special_s_mot_name), 0.0, 1.0, false, 0.0, false, false);

    let ryu_tatsumaki_wind = if !MotionModule::is_flip(fighter.module_accessor) {
        hash40("ryu_tatsumaki_wind_r")
    } else {
        hash40("ryu_tatsumaki_wind_l")
    };
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_EFFECT_REQUEST_FOLLOW, ryu_tatsumaki_wind, hash40("rot"), 0.0, 1.5, 0.0, 0.0, 0.0, 0.0, 1.0, false, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP, 0, -1);
    sv_module_access::effect(fighter.lua_state_agent);

    let spineffect = fighter.pop_lua_stack(1).get_u32();
    fighter.set_int(spineffect as i32, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_EFFECT_HANDLE);

    let wind_alpha_param = if fighter.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        "wind_alpha"
    } else {
        "command_wind_alpha"
    };
    let alpha = fighter.get_param_float("param_special_s", "command_wind_alpha") * 0.01;
    EffectModule::set_alpha(fighter.module_accessor, spineffect, alpha);

    if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::EX_SPECIAL_USED) {
        MeterModule::drain_direct(fighter.battle_object, 2.0 * MeterModule::meter_per_level(fighter.battle_object));
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_loop_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_loop_situation_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let start_situation = fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
        if start_situation != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        }
    }
}

unsafe extern "C" fn special_s_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_s_loop_situation_helper(fighter);
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.dec_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        if !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::EX_SPECIAL_USED)
        && !fighter.is_button_on(Buttons::SpecialAll) {
            fighter.set_int(0, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        }

        let loop_count = fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        if loop_count > 0 {
            let special_s_mot_name = if fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION) != *SITUATION_KIND_GROUND {
                "special_air_s"
            } else {
                "special_s"
            };
            MotionModule::change_motion(fighter.module_accessor, Hash40::new(special_s_mot_name), 0.0, 1.0, false, 0.0, false, false);
        } else {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
    }

    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    fighter.set_flag(situation_kind == *SITUATION_KIND_GROUND, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND);
    return false.into();
}

pub unsafe extern "C" fn special_s_loop_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    // reduce speed on hitting shield
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY) {
        let strength = fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
        let speed_x = if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::EX_SPECIAL_USED) {
                0.0
            } else if strength == *FIGHTER_RYU_STRENGTH_S {
                fighter.get_param_float("param_special_s", "speed_x_s")
            } else if strength == *FIGHTER_RYU_STRENGTH_M {
                fighter.get_param_float("param_special_s", "speed_x_m")
            } else {
                fighter.get_param_float("param_special_s", "speed_x_w")
            }
        } else {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::EX_SPECIAL_USED) {
                0.0
            } else if strength == *FIGHTER_RYU_STRENGTH_S {
                fighter.get_param_float("param_special_s", "air_speed_x_s")
            } else if strength == *FIGHTER_RYU_STRENGTH_M {
                fighter.get_param_float("param_special_s", "air_speed_x_m")
            } else {
                fighter.get_param_float("param_special_s", "air_speed_x_w")
            }
        };
        let lr = PostureModule::lr(fighter.module_accessor);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, lr * speed_x * 0.6, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    }
    if !fighter.is_situation(*SITUATION_KIND_GROUND)
    && KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) < 0.0 {
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(1.0, 0.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exec_common);

    agent.status(Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, special_s_command_init);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, special_s_command_main);
    agent.status(Exec, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, special_s_exec_common);

    agent.status(Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, special_s_loop_init);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, special_s_loop_main);
    agent.status(Exec, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, special_s_loop_exec);
}
