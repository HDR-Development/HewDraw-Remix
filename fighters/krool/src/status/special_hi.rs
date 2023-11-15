use super::*;

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
        || !fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let rise_min_frame = 12; // add new param rise_min_frame
    if MotionModule::is_end(fighter.module_accessor) ||
    ((rise_min_frame..MotionModule::end_frame(fighter.module_accessor) as i32).contains(&fighter.status_frame())
        && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
        fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI.into(), false.into());
    }
    else {
        VarModule::inc_int(fighter.object(), vars::krool::instance::SPECIAL_HI_FUEL);
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
        let start_brake_x = 0.04;   // add new param
        let start_limit_spd_x = 1.3;    // add new param
        let start_air_mul_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_start_air_mul_spd_y"));
        let start_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_start_max_spd_y"));
        let start_stable_spd_y = 0.2;   // add new param
        let mut start_acl_y = 0.03;
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        speed_y *= start_air_mul_spd_y;
        speed_y = speed_y.clamp(-start_max_spd_y, start_max_spd_y);
        start_acl_y *= if speed_y <= 0.0 { 1.0 } else { -1.0 };    // add new param start_acl_y
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
        let fly_brake_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fly_brake_y"));   //unused
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        if speed_y > fly_max_spd_y { speed_y = fly_max_spd_y; }

        sv_kinetic_energy!(reset_energy, fighter, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fly_acl_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fly_max_spd_y);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

        sv_kinetic_energy!(reset_energy, fighter, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END) {
        let max_sum_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_max_sum_spd_y"));
        let fall_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_acl_y"));
        let fall_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_y"));
        let fall_max_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_x"));
        let fall_stick_mul_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_stick_mul_max_spd_y")); //unused
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        if speed_y >= max_sum_spd_y { speed_y = max_sum_spd_y; }
        speed_x = speed_x.clamp(-fall_max_spd_x, fall_max_spd_x);

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
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else if fighter.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL) {
        let fall_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_acl_y"));
        let fall_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_y"));
        let fall_stick_mul_max_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_stick_mul_max_spd_y")); //unused
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
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }

}

pub fn install() {
    smashline::install_status_scripts!(
        special_hi_start_main,
    );
}