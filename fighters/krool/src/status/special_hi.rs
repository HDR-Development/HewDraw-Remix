use super::*;

#[status_script(agent = "krool", status = FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, false, -1);
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, *WEAPON_KROOL_BACKPACK_STATUS_KIND_START, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    if fighter.is_situation(*SITUATION_KIND_GROUND) { special_hi_change_motion(fighter, true, true); } else { special_hi_change_motion(fighter, false, true); }
    special_hi_physics(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let mut speed_x = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        let mut speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        let square_x = speed_x * speed_x;
        let square_y = speed_y * speed_y;
        let high_on_potenuse = (square_x + square_y).sqrt();
        if square_y < 0.0 {
            let move_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
            WorkModule::set_float(fighter.module_accessor, -(move_y.abs()), *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
        }
        else {
            let move_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
            WorkModule::set_float(fighter.module_accessor, move_y.abs(), *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
        }
    }
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_KROOL_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_start_main_loop as *const () as _))

}

unsafe extern "C" fn special_hi_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI.into(), false.into());
        return 1.into()
    }
    if fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) { special_hi_change_motion(fighter, true, false); }
        else if fighter.is_situation(*SITUATION_KIND_AIR) { special_hi_change_motion(fighter, false, false); }
    }

    return 0.into()
}

unsafe extern "C" fn special_hi_physics(fighter: &mut L2CFighterCommon) {
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_mul_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_start_mul_spd_x"));
    let start_air_mul_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_start_air_mul_spd_x"));
    let start_air_mul_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_start_air_mul_spd_y"));
    let fly_start_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_start_max_spd_y"));
    let fly_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_acl_y"));
    let fly_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_max_spd_y"));
    let fly_brake_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_brake_y"));   //unused
    let max_sum_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_max_sum_spd_y"));
    let fall_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_acl_y"));
    let fall_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_y"));
    let fall_max_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_x"));
    let fall_stick_mul_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_stick_mul_max_spd_y")); //unused

    if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START) {
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        speed_y *= start_air_mul_spd_y;
        if speed_y < -fly_start_max_spd_y { speed_y = -fly_start_max_spd_y; }

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fly_start_max_spd_y);
        app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            speed_x *= start_mul_spd_x;
        }
        else {
            speed_x *= start_air_mul_spd_x;
        }

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI) {
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        if speed_y > fly_max_spd_y { speed_y = fly_max_spd_y; }

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fly_acl_y);
        app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fly_max_spd_y);
        app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END) {
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        if speed_y >= max_sum_spd_y { speed_y = max_sum_spd_y; }

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -fall_acl_y);
        app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fall_max_spd_y);
        app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

        // 1c0 = speed_x, 1d0 = -fall_max_spd_x, 1e0 = fall_max_spd_x
        speed_x = speed_x.clamp(-fall_max_spd_x, fall_max_spd_x);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL) {
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -fall_acl_y);
        app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fall_max_spd_y);
        app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }

}

//func(Hash40::new("special_hi_start"), Hash40::new("special_air_hi_start"), false);
unsafe extern "C" fn special_hi_change_motion(fighter: &mut L2CFighterCommon, ground: bool, inherit: bool) {
    if ground {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if inherit {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if inherit {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    
}

pub fn install() {
    smashline::install_status_scripts!(
        special_hi_start_main,
    );
}