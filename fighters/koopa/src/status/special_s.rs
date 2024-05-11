use super::*;

pub const SPECIAL_S_KIND_F: i32 = 1;
pub const SPECIAL_S_KIND_B: i32 = 2;
pub const SPECIAL_S_KIND_HI: i32 = 3;
pub const SPECIAL_S_KIND_LW: i32 = 4;

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_situation_change_helper(fighter, true);

    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_situation_change_helper(fighter: &mut L2CFighterCommon, param_2: bool) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        if !param_2 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_catch"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_catch"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        if !param_2 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_air_catch"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_air_catch"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() 
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into()
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into()
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        special_s_situation_change_helper(fighter, false);
    }

    return 0.into()
}

unsafe extern "C" fn special_s_situation_helper(fighter: &mut L2CFighterCommon, is_start: bool) {
    let motion_g;
    let motion_a;
    let throw_input = VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE);

    if throw_input == SPECIAL_S_KIND_LW {
        motion_g = Hash40::new("special_s_throwlw");
        motion_a = Hash40::new("special_air_s_throwlw");
    }
    else if throw_input == SPECIAL_S_KIND_HI {
        motion_g = Hash40::new("special_s_squat");
        motion_a = Hash40::new("special_air_s_squat");
    }
    else if throw_input == SPECIAL_S_KIND_F {
        motion_g = Hash40::new("special_s_throwf");
        motion_a = Hash40::new("special_air_s_throwf");
    }
    else if throw_input == SPECIAL_S_KIND_B {
        motion_g = Hash40::new("special_s_throwb");
        motion_a = Hash40::new("special_air_s_throwb");
    }
    //Should be unused
    else {
        motion_g = Hash40::new("special_s_squat");
        motion_a = Hash40::new("special_air_s_squat");
    }
    fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), (!is_start).into());

    let correct = if fighter.is_situation(*SITUATION_KIND_GROUND) {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(correct));

    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    if throw_input == SPECIAL_S_KIND_LW && fighter.is_situation(*SITUATION_KIND_AIR) {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); 
    }
    else {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    if is_start {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_x_mul = if throw_input == 0 { 1.0 } else { 1.0 };
        let speed_y_mul = if throw_input == 0 { 0.5 } else { 1.0 };
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            speed_x * speed_x_mul,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y * speed_y_mul,
            0.0,
            0.0,
            0.0
        );
    }

    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.1,
        0.0
    );
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -air_accel_y//*0.5
    ); 
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_accel_y_stable//*0.01
    );
    
}

unsafe extern "C" fn special_s_kinetic_exec(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) ||
    VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE) == SPECIAL_S_KIND_HI ||
    (fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE) == SPECIAL_S_KIND_LW) {
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); 
        }
        if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL); 
        }
    }
    //Falling
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_CAPTURE) {
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);

        let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                speed_y,
                0.0, 0.0, 0.0
            );
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y * 0.425
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y_stable
            );
        }
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
                0.0, 0.0, 0.0, 0.0, 0.0
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_accel_x //* 0.5
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_speed_x_stable, //* 0.75,
                0.0
            );
        }
    }
    //Throwing
    else {
        sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        // let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        // let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        // if speed_y <= 0.0 {
        //     sv_kinetic_energy!(
        //         set_accel,
        //         fighter,
        //         FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        //         0.0
        //     );
        //     sv_kinetic_energy!(
        //         set_speed,
        //         fighter,
        //         FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        //         0.0
        //     );
        // }
    }
}

unsafe extern "C" fn special_s_squat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE, 0);
    WorkModule::set_float(fighter.module_accessor, PostureModule::pos_y(fighter.module_accessor), *FIGHTER_KOOPA_STATUS_SPECIAL_S_WORK_FLOAT_START_Y);
    ControlModule::reset_trigger(fighter.module_accessor);
    special_s_situation_helper(fighter, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_squat_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_squat_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_frame() >= 10 {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {   // TEMPORARY UNTIL CRASHES ARE FIXED
            if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW != 0 {
                VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE, SPECIAL_S_KIND_LW);
                fighter.change_status(FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING.into(),false.into());
                return 0.into();
            }
        }
        // Uncommenting this will allow early transition into Flying Slam
        // else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI != 0 {
        //     VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE, SPECIAL_S_KIND_HI);
        //     fighter.change_status(FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP.into(),false.into());
        //     return 0.into();
        // }
        else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B != 0 {
            VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE, SPECIAL_S_KIND_B);
            fighter.change_status(FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING.into(),false.into());
            return 0.into();
        }
        else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_F != 0 {
            VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE, SPECIAL_S_KIND_F);
            fighter.change_status(FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING.into(),false.into());
            return 0.into();
        }
    }
    
    if MotionModule::is_end(fighter.module_accessor) {
        let throw_Lw = ControlModule::get_stick_y(fighter.module_accessor) < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw4_stick_y"));
        let throw_Hi = ControlModule::get_stick_y(fighter.module_accessor) > WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi4_stick_y"));

        let mut throw_input = SPECIAL_S_KIND_F;    // default to fthrow
        let mut next_status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING;
        if throw_Lw {
            throw_input = if fighter.is_situation(*SITUATION_KIND_GROUND) { SPECIAL_S_KIND_LW } else { SPECIAL_S_KIND_HI };
            next_status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING } else { FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP };
        }
        else if throw_Hi {
            throw_input = SPECIAL_S_KIND_HI;
            next_status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP;
        }
        else {
            throw_input = if (PostureModule::lr(fighter.module_accessor).signum() == ControlModule::get_stick_x(fighter.module_accessor).signum()
            || ControlModule::get_stick_x(fighter.module_accessor).abs() < 0.2) { SPECIAL_S_KIND_F } else { SPECIAL_S_KIND_B };
        }

        VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE, throw_input);
        fighter.change_status(next_status.into(),false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_s_situation_helper(fighter,false);
    }

    0.into()
}

unsafe extern "C" fn special_s_squat_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_kinetic_exec(fighter);
    0.into()
}

unsafe extern "C" fn special_s_ejected(fighter: &mut L2CFighterCommon) {
    let no_change = fighter.global_table[0xB] == FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL;
    WorkModule::set_flag(fighter.module_accessor, no_change,*FIGHTER_KOOPA_INSTANCE_WORK_ID_FLAG_THROW_NO_CHANGE);
    if fighter.global_table[0xB] != FIGHTER_STATUS_KIND_DEAD {
        CatchModule::set_send_cut_event(fighter.module_accessor, true);
        CatchModule::catch_cut(fighter.module_accessor, false,false);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KOOPA_INSTANCE_WORK_ID_FLAG_THROW_NO_CHANGE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x31dbed6513), Hash40::new("koopa_dived"), Hash40::new("invalid"));
    }
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_CAPTURE);
}

unsafe extern "C" fn special_s_squat_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xB] != FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP
    && fighter.global_table[0xB] != FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL
    && fighter.global_table[0xB] != FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_CAPTURE) {
            special_s_ejected(fighter);
        }
    }

    0.into()
}

unsafe extern "C" fn special_s_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let toReturn = return smashline::original_status(Init, fighter, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP)(fighter);
    
    let certain_death = GroundModule::ray_check(
        fighter.module_accessor, 
        &smash::phx::Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor) }, 
        &Vector2f{ x: 0.0, y: -999.0 }, true
    ) != 1;
    VarModule::set_flag(fighter.battle_object, vars::koopa::status::SPECIAL_S_ABOVE_BLASTZONE, certain_death);

    toReturn
}

unsafe extern "C" fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // let motion;
    // if VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE) == SPECIAL_S_KIND_HI {
    //     motion = Hash40::new("special_s_jump");
    // }
    // else {
    //     motion = Hash40::new("special_s_throw_lw_jump");
    // }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_jump"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    fighter.main_shift(special_s_jump_main_loop)
}

unsafe extern "C" fn special_s_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL.into(), false.into());
    }

    return 0.into()
}

unsafe extern "C" fn special_s_jump_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); /*{
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)
    };*/

    let speed_y = {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent)
    };
    let speed_y_motion = {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent)
    };
    if LinkModule::is_linked(fighter.module_accessor, *LINK_NO_CAPTURE) {
        let captured_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
        WorkModule::set_int(fighter.module_accessor, captured_id as i32, *FIGHTER_KOOPA_STATUS_SPECIAL_S_INT_CAPTURED_TASK_ID);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x32a70c6b67), captured_id);
        //Assuming captured_id != 0x50000000 and we are not giga bowser, influence stop speed based on damage
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if !VarModule::is_flag(fighter.battle_object,vars::koopa::status::SPECIAL_S_ABOVE_BLASTZONE) {
        return 0.into();
    }

    let current_dir = speed_x.signum();
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let speed_dir = pos_x.signum()*-1.0;
    let new_speed_x = if speed_y_motion <= 0.1 {0.0} else {speed_y_motion.abs()*speed_dir*0.75};
    
    if new_speed_x.abs() > 0.0 {
        let motion_vec = Vector3f{x: new_speed_x, y: 0.0, z: 0.0};
        KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else{ 
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }

    0.into()
}

unsafe extern "C" fn special_s_fall_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    return smashline::original_status(Init, fighter, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL)(fighter);
}

unsafe extern "C" fn special_s_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dead_offset_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_fall_check_dead_offset_y"));
    WorkModule::set_float(fighter.module_accessor, dead_offset_y, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHECK_DEAD_OFFSET_Y);
    // let motion;
    // if VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE) == SPECIAL_S_KIND_HI {
    //     motion = Hash40::new("special_s_fall");
    // }
    // else {
    //     motion = Hash40::new("special_s_throw_lw_fall");
    // }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_fall"), 0.0, 1.0, false, 0.0, false, false);
    fighter.main_shift(special_s_fall_main_loop)
}

unsafe extern "C" fn special_s_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING.into(), false.into());
        return 1.into()
    }
    if fighter.kind() != *FIGHTER_KIND_KOOPAG {
        let capture_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_INT_CAPTURE_FRAME);
        let capture_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_capture_frame_max"));
        if capture_frame > capture_frame_max {
            fighter.change_status(FIGHTER_STATUS_KIND_CAPTURE_JUMP.into(), false.into());
            return 1.into()
        }
    }

    return 0.into()
}

unsafe extern "C" fn special_s_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE) == SPECIAL_S_KIND_HI {
        return smashline::original_status(Pre, fighter, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING)(fighter);
    }
    let kinetic = KineticModule::get_kinetic_type(fighter.module_accessor);
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        kinetic,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_s_landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE) == SPECIAL_S_KIND_HI {
        return smashline::original_status(Init, fighter, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING)(fighter);
    }
    KineticModule::clear_speed_all(fighter.module_accessor);
    //Unable energies
    if LinkModule::is_linked(fighter.module_accessor, *LINK_NO_CAPTURE){
        if VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE) == -2 {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x329eb012b6), Hash40::new("koopa_dived"), Hash40::new("koopa_dived"));
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    0.into()
}

unsafe extern "C" fn special_s_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let throw_input = VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE);

    if throw_input == SPECIAL_S_KIND_HI {
        return smashline::original_status(Main, fighter, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING)(fighter);
    }

    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);

        let motion_share = WorkModule::get_param_int(fighter.module_accessor, 0xad2ee25eu64, 0x7d88ea0u64);
        //let throw_motion = if !throw_B {36603360558 as u64} else {36554879287 as u64}; //39642420386 lw 41418534085 hi 36603360558 f 36554879287 b
        let throw_motion =
            if throw_input == SPECIAL_S_KIND_LW { 39642420386 } else
            if throw_input == SPECIAL_S_KIND_HI { 41418534085 } else
            if throw_input == SPECIAL_S_KIND_F { 36603360558 } else
            { 36554879287 };    // SPECIAL_S_KIND_B
        let throw_rate = if throw_input == SPECIAL_S_KIND_B { 1.0 } else { 1.7 };
        
        let mut share_type = 0;
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
            share_type = *BODY_TYPE_MOTION_DX;
        }
        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
            share_type = *BODY_TYPE_MOTION_GIRL;
        }
        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
            share_type = *BODY_TYPE_MOTION_BIG;
        }
        if share_type > 0 {
            FighterMotionModuleImpl::add_body_type_hash(
                capture_boma,
                Hash40::new_raw(throw_motion),
                *BODY_TYPE_MOTION_DX
            );
        }
        MotionModule::change_motion(capture_boma, Hash40::new_raw(throw_motion), 0.0, throw_rate, false, 0.0, false, false);
    }
    special_s_situation_helper(fighter, true);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_landing_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        let new_status = if fighter.is_situation(*SITUATION_KIND_GROUND) {FIGHTER_STATUS_KIND_WAIT} else {FIGHTER_STATUS_KIND_FALL};
        fighter.change_status(new_status.into(), false.into());
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE) != SPECIAL_S_KIND_HI {
            special_s_situation_helper(fighter, false);
        }
    }

    0.into()
}

unsafe extern "C" fn special_s_landing_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_S_THROW_TYPE) == SPECIAL_S_KIND_HI {
        return smashline::original_status(Exec, fighter, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING)(fighter);
    }
    special_s_kinetic_exec(fighter);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_CAPTURE) {
            let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x31dbed6513), Hash40::new("throw"), Hash40::new("invalid"));
            if capture_id != 0x50000000 {
                AttackModule::hit_absolute_joint(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,capture_id as u32,Hash40::new("throw"), 0, 0);
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_CAPTURE);
            CameraModule::reset_all(fighter.module_accessor);
            
            //unable energies, then do hop energy
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            special_s_kinetic_exec(fighter);
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);

    agent.status(Main, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_SQUAT, special_s_squat_main);
    agent.status(Exec, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_SQUAT, special_s_squat_exec);
    agent.status(Exit, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_SQUAT, special_s_squat_exit);

    agent.status(Init, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_init);
    agent.status(Main, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_main);
    agent.status(Exec, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_exec);

    agent.status(Init, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL, special_s_fall_init);
    agent.status(Main, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL, special_s_fall_main);

    agent.status(Pre, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING, special_s_landing_pre);
    agent.status(Init, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING, special_s_landing_init);
    agent.status(Main, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING, special_s_landing_main);
    agent.status(Exec, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING, special_s_landing_exec);
}