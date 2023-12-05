use super::*;
use std::convert::TryInto;

#[status_script(agent = "krool", status = FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, false, -1);
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, *WEAPON_KROOL_BACKPACK_STATUS_KIND_START, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        special_hi_change_motion(fighter, Hash40::new("special_hi_start"), true, false);
    }
    else {
        special_hi_change_motion(fighter, Hash40::new("special_air_hi_start"), false, false);
    }
    VarModule::set_int(fighter.object(), vars::krool::instance::SPECIAL_HI_FUEL, 0);
    special_hi_set_physics(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_hi_start_helper as *const () as _));
    }
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_KROOL_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_start_main_loop as *const () as _))

}

unsafe extern "C" fn special_hi_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || !fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let rise_min_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.rise_min_frame");
    if MotionModule::is_end(fighter.module_accessor) ||
    ((rise_min_frame..MotionModule::end_frame(fighter.module_accessor) as i32).contains(&fighter.status_frame())
        && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
        fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI.into(), false.into());
    }
    else {
        if fighter.status_frame() > rise_min_frame {
            if fighter.is_situation(*SITUATION_KIND_GROUND) && (fighter.status_frame() + rise_min_frame) % 15 == 0 {
                EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_ZX);
            }
            if fighter.status_frame() <= 50 {
                VarModule::inc_int(fighter.object(), vars::krool::instance::SPECIAL_HI_FUEL);
            }
        }
        if fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
            if fighter.is_situation(*SITUATION_KIND_AIR) {
                special_hi_change_motion(fighter, Hash40::new("special_air_hi_start"), false, true);
            }
        }
        else {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                special_hi_change_motion(fighter, Hash40::new("special_hi_start"), true, true);
            }
        }
    }

    return 0.into()
}

#[status_script(agent = "krool", status = FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, *WEAPON_KROOL_BACKPACK_STATUS_KIND_FLY, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    if fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
        special_hi_change_motion(fighter, Hash40::new("special_hi"), false, false);
    }
    else {
        special_hi_change_motion(fighter, Hash40::new("special_air_hi"), false, false);
    }
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
    special_hi_set_physics(fighter);
    WorkModule::set_float(fighter.module_accessor, 0.5, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOTION_2ND_LERP_RATE);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_hi_movement_helper as *const () as _));
    }
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_KROOL_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))

}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || !fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let param_brake_after_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_brake_after_frame"));
    let brake_after_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_HI_INT_BRAKE_AFTER_FRAME);
    if brake_after_frame > param_brake_after_frame {
        fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END.into(), false.into());
        return 0.into()
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        let start_air_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_start_air_frame"));
        if fighter.global_table[CURRENT_FRAME].get_i32() > start_air_frame {
            fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
        }
    }

    return 0.into()
}

#[status_script(agent = "krool", status = FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, *WEAPON_KROOL_BACKPACK_STATUS_KIND_TOP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_air_end"), 0.0, 1.0, false, 0.0, false, false);
    special_hi_lerp_motion(fighter, "special_hi_air_end_f", "special_hi_air_end_b");
    special_hi_set_physics(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_hi_movement_helper as *const () as _));
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_KROOL_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_end_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
        }
        else {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
            }
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || !fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    return 0.into()
}

#[status_script(agent = "krool", status = FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, *WEAPON_KROOL_BACKPACK_STATUS_KIND_FALL, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    special_hi_change_motion(fighter, Hash40::new("special_hi_fall"), false, true);
    special_hi_lerp_motion(fighter, "special_hi_fall_f", "special_hi_fall_b");
    special_hi_set_physics(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_hi_movement_helper as *const () as _));
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_KROOL_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_fall_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
        }
        else {
            let fall_special_spd_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_spd_y");
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            let mut speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            if speed_y <= -fall_special_spd_y {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            }
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || !fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    return 0.into()
}

//FUN_7100023a20
unsafe extern "C" fn special_hi_start_helper(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let mut speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        let square_y = speed_y * speed_y;
        if square_y < 0.0 {
            let move_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
            WorkModule::set_float(fighter.module_accessor, -(move_y.abs()), *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
        }
        else {
            let move_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
            WorkModule::set_float(fighter.module_accessor, move_y.abs(), *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
        }
    }

    return 0.into()
}

// FUN_710001dba0
unsafe extern "C" fn special_hi_change_motion(fighter: &mut L2CFighterCommon, motion: Hash40, ground: bool, inherit: bool) {
    if ground {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if inherit {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    
}

// FUN_710001ea30
unsafe extern "C" fn special_hi_set_physics(fighter: &mut L2CFighterCommon) {
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START) {
        let start_mul_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_start_mul_spd_x"));
        let start_air_mul_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_start_air_mul_spd_x"));
        let start_brake_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_brake_x");
        let start_limit_spd_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_limit_spd_x");
        let start_air_mul_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_start_air_mul_spd_y"));
        let start_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_start_max_spd_y"));
        let start_stable_spd_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_stable_spd_y");
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        speed_y *= start_air_mul_spd_y;
        speed_y = speed_y.clamp(-start_max_spd_y, start_max_spd_y);
        let start_acl_y = if speed_y <= 0.0 { 1.0 } else { -1.0 };
        speed_x *= if fighter.is_situation(*SITUATION_KIND_GROUND) { start_mul_spd_x } else { start_air_mul_spd_x };

        sv_kinetic_energy!(reset_energy, fighter, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, start_acl_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, start_max_spd_y);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, start_stable_spd_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

        sv_kinetic_energy!(reset_energy, fighter, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, start_brake_x, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, start_limit_spd_x, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI) {
        let fly_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_acl_y"));
        let fly_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_max_spd_y"));
        let charge_frames = VarModule::get_int(fighter.object(), vars::krool::instance::SPECIAL_HI_FUEL) as f32;
        let fly_charge_min_spd_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fly_charge_min_spd_x");
        let fly_charge_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fly_charge_x_mul");
        let mut fly_limit_spd_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fly_limit_spd_x");
        fly_limit_spd_x *= PostureModule::lr(fighter.module_accessor);
        let fly_charge_min_spd_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fly_charge_min_spd_y");
        let fly_charge_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fly_charge_y_mul");

        // accounts for 50 max charge frames
        let calc_charge_x = (fly_charge_min_spd_x + (charge_frames * fly_charge_x_mul)) * PostureModule::lr(fighter.module_accessor);
        let calc_charge_y = fly_charge_min_spd_y + (charge_frames * fly_charge_y_mul);
        // max x: 0.7, max y: 3.0

        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        sv_kinetic_energy!(reset_energy, fighter, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, calc_charge_y);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -fly_acl_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fly_max_spd_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

        sv_kinetic_energy!(reset_energy, fighter, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, calc_charge_x, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, fly_limit_spd_x, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END) {
        let max_sum_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_max_sum_spd_y"));
        let fall_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_acl_y"));
        let fall_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_y"));
        let fall_max_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_x"));
        let fall_stick_mul_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_stick_mul_max_spd_y")); //unused
        let fall_brake_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_brake_x");
        let mut fall_stable_spd_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_stable_spd_x");
        fall_stable_spd_x *= PostureModule::lr(fighter.module_accessor);
        let fall_limit_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_limit_spd_x");
        if speed_y >= max_sum_spd_y { speed_y = max_sum_spd_y; }
        speed_x = speed_x.clamp(-fall_max_spd_x, fall_max_spd_x);

        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        sv_kinetic_energy!(reset_energy, fighter, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -fall_acl_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -fall_max_spd_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, fall_brake_x, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, fall_limit_speed_x, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, fall_stable_spd_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL) {
        let fall_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_acl_y"));
        let fall_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_y"));

        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        sv_kinetic_energy!(reset_energy, fighter, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -fall_acl_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fall_max_spd_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

        sv_kinetic_energy!(reset_energy, fighter, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 2.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }

}

//FUN_71000210d0
unsafe extern "C" fn special_hi_movement_helper(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        let gravity_minus = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_acl_y_speed_minus"));   //0x2041e0d192
        let acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_acl_y"));   //0x141a28d9bf
        let touch_max_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_touch_max_spd_x"));   //0x1eff745701
        let max_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_max_spd_x"));   //0x187c1ef75f
        let stick_mul_max_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_stick_mul_max_spd_x"));   //0x22c9b49999
        let stick_mul_acl_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_stick_mul_acl_spd_y"));   //0x228933e5bb
        let brake_movement_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_brake_movement_y")); //0x1ffbac0abc
        let fly_brake_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_brake_y"));   //0x16a2f66223
        let fall_max_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_x")); //0x19ecf9d8dc
        let fall_stick_mul_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_stick_mul_max_spd_y")); //0x2388140c3b
        let fall_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_y")); //0x199bfee84a
        let unknown_param = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x1f32a7c5bf);
        let mut stick_mul_x_delta;  //l160
        let mut stick_mul_acl_y_delta; //l170
        let mut stick_mul_fall_y_delta;   //l180

        if stick_mul_max_spd_x <= 1.0 {
            stick_mul_x_delta = 1.0;
        }
        else {
            stick_mul_x_delta = (stick_mul_max_spd_x - 1.0) * fighter.stick_x() + 1.0;
        }
        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == (*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL | *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END) {
            stick_mul_x_delta = 1.0;
        }
        if stick_mul_acl_spd_y >= 1.0 {
            stick_mul_acl_y_delta = 1.0;
        }
        else {
            stick_mul_acl_y_delta = 1.0 - (1.0 - stick_mul_acl_spd_y) * fighter.stick_x();
        }
        if fall_stick_mul_max_spd_y <= 1.0 {
            stick_mul_fall_y_delta = 1.0;
        }
        else {
            stick_mul_fall_y_delta = 1.0 + (fall_stick_mul_max_spd_y - 1.0) * fighter.stick_x();
        }

        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, fall_max_spd_x * stick_mul_x_delta, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fall_max_spd_y * stick_mul_fall_y_delta);
        }
        else {
            if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
                sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, touch_max_spd_x * stick_mul_x_delta, 0.0);
            }
            else {
                sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, max_spd_x * stick_mul_x_delta, 0.0);
            }
            let brake_after_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_HI_INT_BRAKE_AFTER_FRAME);
            if brake_after_frame >= 0 {
                sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -fly_brake_y);
                WorkModule::inc_int(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_HI_INT_BRAKE_AFTER_FRAME);
            }
            let movement_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
            let brake_mul = brake_movement_y * 10.0;
            if brake_mul > movement_y {
                sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, acl_y * stick_mul_acl_y_delta);
            }
            else {
                sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -fly_brake_y);
                WorkModule::inc_int(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_HI_INT_BRAKE_AFTER_FRAME);
            }
            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int((*FIGHTER_KINETIC_ENERGY_ID_GRAVITY).try_into().unwrap()));
            let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            if speed_y < 0.0 {
                sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_minus);
            }
        }

        special_hi_lean_physics(fighter);

        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int((*FIGHTER_KINETIC_ENERGY_ID_GRAVITY).try_into().unwrap()));
        let mut movement_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
        if app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent) <= 0.0 {
            movement_y *= -1.0;
        }
        WorkModule::set_float(fighter.module_accessor, movement_y, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);  // unsure about this

        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
            special_hi_lerp_motion(fighter, "special_hi_f", "special_hi_b");
        }
        else if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END {
            special_hi_lerp_motion(fighter, "special_hi_air_end_f", "special_hi_air_end_b");
        }
        else if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL {
            special_hi_lerp_motion(fighter, "special_hi_fall_f", "special_hi_fall_b");
        }
    }

    return 0.into()
}

unsafe extern "C" fn special_hi_lean_physics(fighter: &mut L2CFighterCommon) {
    let mul_max_acl_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_mul_max_acl_x"));
    let mul_stick_x = fighter.stick_x() * mul_max_acl_x;    //l190
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    if mul_stick_x != 0.0 {
        let mut calc_mul_x = mul_stick_x;
        if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI)
        || fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END)
        || fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL) {
            // restrict min speed based on charge
            let fly_lean_min_acl_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fly_lean_min_acl_x");
            if mul_stick_x * PostureModule::lr(fighter.module_accessor) < fly_lean_min_acl_x {
                calc_mul_x = fly_lean_min_acl_x * PostureModule::lr(fighter.module_accessor);
                let charge_frames = VarModule::get_int(fighter.object(), vars::krool::instance::SPECIAL_HI_FUEL) as f32;
                let fly_lean_min_spd_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fly_lean_min_spd_x");
                let fly_lean_charge_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fly_lean_charge_x_mul");
                let calc_charge_fly_lean = fly_lean_min_spd_x + (charge_frames * fly_lean_charge_x_mul);
                sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, calc_charge_fly_lean, 0.0);
            }
        }
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, calc_mul_x, 0.0);
    }
    
    if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI) {
        let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if (-0.1..0.1).contains(&(fighter.stick_x() * PostureModule::lr(fighter.module_accessor))) {
            let fly_no_lean_spd_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fly_no_lean_spd_y");
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y - fly_no_lean_spd_y);
        }
        else if fighter.stick_x() * PostureModule::lr(fighter.module_accessor) > 0.1 {
            let fly_lean_f_spd_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fly_lean_f_spd_y");
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y - fly_lean_f_spd_y);
        }
    }

}

//FUN_710001e090
unsafe extern "C" fn special_hi_lerp_motion(fighter: &mut L2CFighterCommon, motion1: &str, motion2: &str) {
    let mut lerp_rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOTION_2ND_LERP_RATE);    //l80
    //println!("stick_x: {}, pre-lerp: {}", fighter.stick_x(), lerp_rate);
    if (-0.1..0.1).contains(&fighter.stick_x()) {
        if lerp_rate >= 0.5 {
            lerp_rate = (lerp_rate - 0.05).clamp(0.5, 1.0);
        }
        else {
            lerp_rate = (lerp_rate + 0.05).clamp(0.0, 0.5);
        }
    }
    else {
        let stick_lr = fighter.stick_x() * PostureModule::lr(fighter.module_accessor);
        let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
        //println!("{} >? {}", stick_lr, turn_stick_x);
        if stick_lr > turn_stick_x {
            lerp_rate = (lerp_rate - 0.05).clamp(0.0, 1.0);
        }
        else {
            lerp_rate = (lerp_rate + 0.05).clamp(0.0, 1.0);
        }
    }
    //println!("stick_x: {}, post-lerp: {}", fighter.stick_x(), lerp_rate);
    WorkModule::set_float(fighter.module_accessor, lerp_rate, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOTION_2ND_LERP_RATE);

    let mut motion_kind = MotionModule::motion_kind_2nd(fighter.module_accessor);    //l90
    //println!("starting motion: {}", motion_kind);
    let mut hash_motion;
    let mut adjusted_lerp;  //la0
    if lerp_rate >= 0.5 {
        //println!("expected motion: {}", motion2);
        motion_kind = hash40(motion2);
        hash_motion = Hash40::new(motion2);
        adjusted_lerp = (lerp_rate - 0.5) * 2.0;
    }
    else {
        //println!("expected motion: {}", motion1);
        motion_kind = hash40(motion1);
        hash_motion = Hash40::new(motion1);
        adjusted_lerp = (lerp_rate * 2.0) - 1.0;
    }
    if MotionModule::motion_kind_2nd(fighter.module_accessor) != motion_kind {
        let frame = MotionModule::frame(fighter.module_accessor);
        let rate = MotionModule::rate(fighter.module_accessor);
        MotionModule::add_motion_2nd(fighter.module_accessor, hash_motion, frame, rate, true, adjusted_lerp);
    }
    MotionModule::set_weight(fighter.module_accessor, 1.0 - adjusted_lerp, true);

}

pub fn install() {
    smashline::install_status_scripts!(
        special_hi_start_main,
        special_hi_main,
        special_hi_end_main,
        special_hi_fall_main,
    );
}