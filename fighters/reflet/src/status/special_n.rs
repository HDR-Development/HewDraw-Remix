use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_N

pub unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    fighter.set_int(*FIGHTER_REFLET_MAGIC_KIND_THUNDER, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    fighter.set_int64(hash40("special_n_start") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
    fighter.set_int64(hash40("special_air_n_start") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    fighter.set_int(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
    fighter.set_int(*FIGHTER_KINETIC_TYPE_AIR_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
    fighter.set_int(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
    fighter.set_int(*GROUND_CORRECT_KIND_AIR, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 10);
    fighter.main_shift(special_n_main_loop)
}

pub unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD.into(), false.into())
        }
        if fighter.is_button_trigger(Buttons::Special | Buttons::Attack) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        } // buffered release
    }
    mot_handler(fighter)
}

// FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD

pub unsafe extern "C" fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // set vars
    fighter.set_int64(hash40("special_n_hold") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
    fighter.set_int64(hash40("special_air_n_hold") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    fighter.set_int(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
    fighter.set_int(*FIGHTER_KINETIC_TYPE_AIR_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
    fighter.set_int(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
    fighter.set_int(*GROUND_CORRECT_KIND_AIR, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);
    fighter.set_int(0, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
    fighter.enable_transition_term_many(&[
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR
    ]);
    let stage = fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
    let mut init_count = 0; // set charge frame depending on last reached charge stage, fx
    if stage == *FIGHTER_REFLET_MAGIC_KIND_GIGA_THUNDER {
        init_count = fighter.get_param_float("param_special_n", "special_n_giga_thunder_shoot_time") as i32;
        charge_handler(fighter, hash40("reflet_specialn_hold3"), *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_EFFECT_HANDLE3, *FIGHTER_REFLET_MAGIC_KIND_GIGA_THUNDER);
    } else if stage == *FIGHTER_REFLET_MAGIC_KIND_EL_THUNDER {
        init_count = fighter.get_param_float("param_special_n", "special_n_el_thunder_shoot_time") as i32;
        charge_handler(fighter, hash40("reflet_specialn_hold2"), *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_EFFECT_HANDLE2, *FIGHTER_REFLET_MAGIC_KIND_EL_THUNDER);
    } else {
        charge_handler(fighter, hash40("reflet_specialn_hold"), *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_EFFECT_HANDLE, *FIGHTER_REFLET_MAGIC_KIND_THUNDER);
    }
    // set book and charge frame
    fighter.set_int(init_count, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_COUNT);
    app::FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut app::FighterModuleAccessor, stage);
    air_charge_stall(fighter);
    fighter.main_shift(special_n_hold_main_loop)
}

pub unsafe extern "C" fn special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_button_trigger(Buttons::Special | Buttons::Attack) 
    || VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) { // buffered fire
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        fighter.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT.into(), true.into())
    } else if cancel_check(fighter) {//cancel
        fighter.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into())
    }
    // charge frame
    fighter.inc_int(*FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_COUNT);
    if !CHECK_MAGIC(fighter) {
        fighter.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT.into(), true.into()) // fail status
    }
    let hold_frame = fighter.get_int(*FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_COUNT) as f32;
    let stage = fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
    let el_frame = fighter.get_param_float("param_special_n", "special_n_el_thunder_shoot_time");
    let arc_frame = fighter.get_param_float("param_special_n", "special_n_giga_thunder_shoot_time");
    let thoron_frame = fighter.get_param_float("param_special_n", "special_n_tron_shoot_time");
    // set vars&fx by stage
    if stage >= *FIGHTER_REFLET_MAGIC_KIND_GIGA_THUNDER {
        if hold_frame >= thoron_frame {
            fighter.set_int(*FIGHTER_REFLET_MAGIC_KIND_TRON, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
            fighter.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into())
        }//thoron fx moved to end stat
    } else if stage == *FIGHTER_REFLET_MAGIC_KIND_EL_THUNDER {
        if hold_frame >= arc_frame {
            fighter.set_int(*FIGHTER_REFLET_MAGIC_KIND_GIGA_THUNDER, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
            app::FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_THUNDER);
            eff_clear(fighter, hash40("reflet_specialn_hold2"));
            charge_handler(fighter, hash40("reflet_specialn_hold3"), *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_EFFECT_HANDLE3, *FIGHTER_REFLET_MAGIC_KIND_GIGA_THUNDER);

        }
    } else {
        if hold_frame >= el_frame {
            fighter.set_int(*FIGHTER_REFLET_MAGIC_KIND_EL_THUNDER, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
            app::FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_THUNDER);
            eff_clear(fighter, hash40("reflet_specialn_hold"));
            charge_handler(fighter, hash40("reflet_specialn_hold2"), *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_EFFECT_HANDLE2, *FIGHTER_REFLET_MAGIC_KIND_EL_THUNDER);
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {air_charge_stall(fighter); }
    mot_handler(fighter); // set mot/physics by var
    // mot rate
    let motion_rate = fighter.get_float(*FIGHTER_REFLET_INSTANCE_WORK_ID_SPECIAL_N_CHARGE_RATE);
    MotionModule::set_rate(fighter.module_accessor, motion_rate);
    0.into()
}

unsafe extern "C" fn special_n_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND) == *FIGHTER_REFLET_MAGIC_KIND_TRON {
        app::FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_TRON);
        EffectModule::req_common(fighter.module_accessor, Hash40::new("charge_max"), 0.0);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("reflet_specialn_max"), Hash40::new("handl"), &Vector3f{x: 1.0, y: 2.0, z: 0.0}, &Vector3f::zero(), 1.0, false, 0, 0, -1, 0, 0, false, false) as u32;
        app::FighterUtil::flash_eye_info(fighter.module_accessor);
    }
    eff_clear(fighter, hash40("reflet_specialn_hold3"));
    eff_clear(fighter, hash40("reflet_specialn_hold2"));
    eff_clear(fighter, hash40("reflet_specialn_hold"));
    0.into() // kill vfx on exit
}

unsafe extern "C" fn eff_clear(fighter: &mut L2CFighterCommon, eff_hash: u64) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(eff_hash), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    0.into()
}

unsafe extern "C" fn charge_handler(fighter: &mut L2CFighterCommon, eff_hash: u64, eff_handle: i32, thunder_kind: i32) -> L2CValue { // 1, 3, 4
    let el_frame = fighter.get_param_float("param_special_n", "special_n_el_thunder_shoot_time");
    let arc_frame = fighter.get_param_float("param_special_n", "special_n_giga_thunder_shoot_time");
    let thoron_frame = fighter.get_param_float("param_special_n", "special_n_tron_shoot_time");
    if fighter.kind() == *FIGHTER_KIND_KIRBY {
        let eff_pos = &Vector3f{x: -2.5, y: -1.2, z: -1.0};
        if fighter.lr() > 0.0 {
            let eff_pos = &Vector3f{x: -2.0, y: -1.2, z: 1.5};
        }
        let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new_raw(eff_hash), Hash40::new("handr"), eff_pos, &Vector3f::zero(), 1.0, false, 0, 0, -1, 0, 0, false, false) as u32;
        fighter.set_int(effect as i32, eff_handle);
    } else {
        let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new_raw(eff_hash), Hash40::new("handl"), &Vector3f{x: 1.0, y: 2.0, z: 0.0}, &Vector3f::zero(), 1.0, false, 0, 0, -1, 0, 0, false, false) as u32;
        fighter.set_int(effect as i32, eff_handle);
    }
    match thunder_kind {
        _ if thunder_kind == *FIGHTER_REFLET_MAGIC_KIND_THUNDER => fighter.set_float(el_frame/64.0, *FIGHTER_REFLET_INSTANCE_WORK_ID_SPECIAL_N_CHARGE_RATE),
        _ if thunder_kind == *FIGHTER_REFLET_MAGIC_KIND_EL_THUNDER => fighter.set_float((arc_frame - el_frame)/64.0, *FIGHTER_REFLET_INSTANCE_WORK_ID_SPECIAL_N_CHARGE_RATE),
        _ if thunder_kind == *FIGHTER_REFLET_MAGIC_KIND_GIGA_THUNDER => fighter.set_float((thoron_frame - arc_frame)/64.0, *FIGHTER_REFLET_INSTANCE_WORK_ID_SPECIAL_N_CHARGE_RATE),
        _ => fighter.set_float(1.0, *FIGHTER_REFLET_INSTANCE_WORK_ID_SPECIAL_N_CHARGE_RATE),
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false); // ?
    }
    0.into()
}

unsafe extern "C" fn cancel_check(fighter: &mut L2CFighterCommon) -> bool {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.sub_check_jump_in_charging().get_bool() {
            fighter.set_int(*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
            return true.into()
        }
        else if fighter.sub_check_command_guard().get_bool() {
            fighter.set_int(0, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
            return true.into()
        }
        return false.into()
    }
    if fighter.get_num_used_jumps() < fighter.get_jump_count_max()
    && fighter.sub_check_jump_in_charging().get_bool() {
        fighter.set_int(*FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
        return true.into();
    }
    if fighter.is_cat_flag(Cat1::AirEscape) {
        fighter.set_int(0, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
        return true.into()
    }
    false.into()
}

pub unsafe extern "C" fn air_charge_stall(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        let air_accel_y = fighter.get_param_float("air_accel_y", "");
        let special_n_air_invoke_fall_speed_mul = fighter.get_param_float("param_special_n", "special_n_air_invoke_fall_speed_mul");
        let special_n_air_invoke_speed_y_limit = fighter.get_param_float("param_special_n", "special_n_air_invoke_speed_y_limit");
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * special_n_air_invoke_fall_speed_mul);
        let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if -special_n_air_invoke_speed_y_limit > speed_y {
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_n_air_invoke_speed_y_limit);
        }
    }
    0.into()
}

// FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START

pub unsafe extern "C" fn special_n_tron_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_float(fighter.battle_object, vars::reflet::instance::SPECIAL_N_CHARGE, fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) as f32);
    let ret = smashline::original_status(Main, fighter, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START)(fighter);
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    }
    ret
}

// FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD

pub unsafe extern "C" fn special_n_tron_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD)(fighter);
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::clear_speed_all(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_BIND);
    }
    ret
}

// FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END

pub unsafe extern "C" fn special_n_tron_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END)(fighter);
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_BIND);
    }
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD, special_n_hold_main);
    agent.status(End, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD, special_n_hold_end);
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START, special_n_tron_start_main);
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD, special_n_tron_hold_main);
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END, special_n_tron_end_main);
}