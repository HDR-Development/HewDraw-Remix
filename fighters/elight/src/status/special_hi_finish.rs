use super::*;


unsafe extern "C" fn special_hi_finish_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    0.into()
}


unsafe extern "C" fn special_hi_finish_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] change motion and enable energy so that we can drift
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    
    // [v] reset the control energy after we've enabled it to ensure that we don't have any left over drift impacting us
    {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    }

    // [v] set the stable speed to 0.0 (gets changed when FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL is true)
    {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
        app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    }

    // [v] get the motion speed multipliers depending on whether or not the move was started on the ground
    let speed_mul_x: f32;
    let speed_mul_y: f32;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_GROUND_START) {
        speed_mul_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x1e0c7f2c05);
        speed_mul_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x1ec0d52c9b);
    } else {
        speed_mul_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x1be165d7da);
        speed_mul_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x1bdc05fe6a);
    }

    // [v] set the obtained speed multipliers. speed_mul_2nd is different from speed_mul because it contains multipliers for both
    //      x and y, whereas speed_mul pertains to both
    {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul_x, speed_mul_y);
        app::sv_kinetic_energy::set_speed_mul_2nd(fighter.lua_state_agent);
    }

    fighter.main_shift(special_hi_finish_main_loop)
}

unsafe extern "C" fn special_hi_finish_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] check if we have grabbed a ledge
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    // [v] check if we have passed the landing frame and are on the ground, if so transition to special fall
    //      which will transition to the appropriate landing lag
    if fighter.global_table[globals::CURRENT_FRAME].get_i32() > fighter.get_param_int("param_special_hi", "can_landing_frame")
        && fighter.is_situation(*SITUATION_KIND_GROUND)
    {
        let interrupt_frame = fighter.get_param_int("param_special_hi", "interrupt_landing_fix_frame");
        fighter.set_float(interrupt_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }

    // [v] set our kinetic energy infos only once
    if fighter.is_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL) {
        fighter.off_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);

        let accel_mul = fighter.get_param_float("param_special_hi", "finish_accel_x_mul");
        let accel_max = fighter.get_param_float("param_special_hi", "finish_speed_x_max");

        {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_mul);
            app::sv_kinetic_energy::set_accel_x_mul(fighter.lua_state_agent);
        }

        {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_max);
            app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        }
    }

    // [v] check if you are fastfalling and if so zero out your motion's y movement and enable gravity
    //      to allow fast fall kinetic to take over
    if fighter.is_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_CHECK_DIVE)
        && { fighter.sub_air_check_dive(); fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) }
    {
        let speed_x = {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
            app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)
        };

        {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_x, 0.0);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        }

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    if !MotionModule::is_end(fighter.module_accessor) {
        return 0.into();
    }

    // [v] when the animation is over, set transition into special fall
    let fall_special_accel = fighter.get_param_float("param_special_hi", "fall_special_accel_x_mul");
    fighter.set_float(fall_special_accel, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_ACCEL_MUL);

    let fall_special_speed_max = fighter.get_param_float("param_special_hi", "fall_special_speed_x_max_mul");
    fighter.set_float(fall_special_speed_max, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);

    // [v] the landing fix frame is different depending on whether you used spreadbullet or not
    let landing_fix_frame;
    if fighter.is_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET) {
        landing_fix_frame = fighter.get_param_int("param_special_hi", "attack2_landing_fix_frame");
    } else {
        landing_fix_frame = fighter.get_param_int("param_special_hi", "attack1_landing_fix_frame");
    }

    fighter.set_float(landing_fix_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    0.into()
}


unsafe extern "C" fn special_hi_finish_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    smashline::Agent::new("elight")
        .status(Pre, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_FINISH, special_hi_finish_pre)
        .status(Main, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_FINISH, special_hi_finish_main)
        .status(End, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_FINISH, special_hi_finish_end)
        .install();
}