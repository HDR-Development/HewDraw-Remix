use super::*;

const END_TYPE_LANDING: i32 = 0;
const END_TYPE_GROUND: i32 = 1;
const END_TYPE_AIR: i32 = 2;

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);
    VarModule::set_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE, 0.0);
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("sjump_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_hold"), L2CValue::Hash40s("special_hi_hold_air"), false.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    special_hi_set_kinetic(fighter);
    
    fighter.main_shift(special_hi_main_loop)
}

unsafe extern "C" fn special_hi_set_kinetic(fighter: &mut L2CFighterCommon) {
    let start_speed_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_speed_x_mul");
    let start_speed_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_speed_y_mul");
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * start_speed_x_mul;
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * start_speed_y_mul;

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        let start_brake_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_brake_x");
        let start_brake_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_brake_y");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, start_brake_x, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else {
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.1, 0.0);
    }
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        fighter.change_status(statuses::samusd::SPECIAL_HI_RUSH.into(), false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_hold"), L2CValue::Hash40s("special_hi_hold_air"), true.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let gravity_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_gravity_frame");
    if fighter.motion_frame() >= gravity_frame {
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.015);
    }
    else {
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    }
    let lock_angle_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_lock_angle_frame");
    if !VarModule::is_flag(fighter.battle_object, vars::samusd::status::SPECIAL_HI_SET_ROT)
    && fighter.motion_frame() > lock_angle_frame {
        VarModule::on_flag(fighter.battle_object, vars::samusd::status::SPECIAL_HI_SET_ROT);
        special_hi_set_direction(fighter);
    }
    else if VarModule::is_flag(fighter.battle_object, vars::samusd::status::SPECIAL_HI_SET_ROT) {
        let angle_target = VarModule::get_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE);
        let end_frame = MotionModule::end_frame(fighter.module_accessor) - 1.0;
        let t = (frame - lock_angle_frame) / (end_frame - lock_angle_frame);
        use interpolation::Lerp;
        let angle = Lerp::lerp(&0.0, &angle_target, &(t * 0.5));
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f::new(-angle, 0.0, 0.0), MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    }
    
    return 0.into();
}

unsafe extern "C" fn special_hi_set_direction(fighter: &mut L2CFighterCommon) {
    VarModule::set_flag(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_AIR, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);

    // Get angle
    let mut lr: f32 = PostureModule::lr(fighter.module_accessor);
    let mut stick_x: f32 = ControlModule::get_stick_x(fighter.module_accessor);
    let mut stick_y: f32 = ControlModule::get_stick_y(fighter.module_accessor);

    if (stick_x * lr <= -0.1 && stick_x.abs() >= 0.2) {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        lr *= -1.0;
    }

    // If in deadzone, go up
    if (stick_x.abs() < 0.1 && stick_y.abs() < 0.1) {
        stick_x = 0.0;
        stick_y = 1.0;
    }
    // If on ground, and aiming the stick towards the ground, limit y to 0
    if (fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND && stick_y < 0.0) {
        stick_y = 0.0;
        if (stick_x.abs() < 0.1) { stick_x = lr; }
        stick_x = sv_math::vec2_normalize(stick_x, stick_y).x;
    }

    let normalized = sv_math::vec2_normalize(stick_x, stick_y);
    let arctangent = normalized.y.atan2(normalized.x.abs());
    let mut angle_degrees = arctangent.to_degrees();
    let mut flip = 0.0;
    if angle_degrees > 90.0 && angle_degrees < 270.0 {
        angle_degrees = (angle_degrees - 180.0);
        flip = 180.0;
    }

    VarModule::set_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE, angle_degrees);
}

unsafe extern "C" fn special_hi_rush_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0 as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn special_hi_rush_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut angle = VarModule::get_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE);
    let isGrounded = fighter.is_situation(*SITUATION_KIND_GROUND) && angle.abs() < 10.0;

    if isGrounded {
        if angle.abs() < 10.0 {
            angle = 0.0;
        }
        VarModule::set_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE, 0.0);
    }

    // Set speed
    let angle_radians = angle.to_radians();
    let rush_speed = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.rush_speed");
    let speed_x = angle_radians.cos() * rush_speed * PostureModule::lr(fighter.module_accessor);
    let speed_y = angle_radians.sin() * rush_speed;
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    // If in not the air and under a certain degree, clear gravity with no accel
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    special_hi_set_joint_rotation(fighter);

    return 0.into();
}

unsafe extern "C" fn special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //GroundModule::set_passable_check(fighter.module_accessor, true);
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    if VarModule::is_flag(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_AIR)
    || VarModule::get_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE) > 10.0 {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
    }
    else {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        special_hi_rush_substatus(fighter);
    }

    //fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(special_hi_rush_substatus as *const () as _));
    fighter.main_shift(special_hi_rush_main_loop)
}

unsafe extern "C" fn special_hi_rush_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rush_frame = VarModule::get_int(fighter.battle_object, vars::samusd::status::SPECIAL_HI_RUSH_FRAME);
    VarModule::inc_int(fighter.battle_object, vars::samusd::status::SPECIAL_HI_RUSH_FRAME);
    // let rush_pass_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.rush_pass_frame");
    // if rush_pass_frame < rush_frame {
    //     GroundModule::set_passable_check(fighter.module_accessor, false);
    // }

    return 0.into();
}

unsafe extern "C" fn special_hi_rush_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    let rush_frame = VarModule::get_int(fighter.battle_object, vars::samusd::status::SPECIAL_HI_RUSH_FRAME);
    let rush_max_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.rush_max_frame");
    if rush_frame > rush_max_frame {
        special_hi_find_end_type(fighter);
        fighter.change_status(statuses::samusd::SPECIAL_HI_END.into(), false.into());
        return 0.into();
    }
    if !fighter.global_table[IS_STOPPING].get_bool() {
        special_hi_rush_substatus(fighter);
        let to_return = special_hi_rush_handle_bound(fighter);
        if to_return == 1 {
            return to_return.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            VarModule::on_flag(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_BRAKE.into());
    }
    
    return 0.into();
}

unsafe extern "C" fn special_hi_rush_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_hi_set_joint_rotation(fighter);

    let angle = VarModule::get_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE);

    let rush_frame = VarModule::get_int(fighter.battle_object, vars::samusd::status::SPECIAL_HI_RUSH_FRAME);
    let rush_brake_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.rush_brake_frame");
    if rush_frame <= rush_brake_frame {
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        return 0.into();
    }
    else {
        let dir = angle.to_radians();
        let rush_brake = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.rush_brake");
        let mut deccel_x = (dir.cos() * rush_brake).abs();
        let mut deccel_y = (dir.sin() * rush_brake).abs();

        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, deccel_x, deccel_y);
    }
    
    return 0.into();
}

unsafe extern "C" fn special_hi_rush_execstop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_hi_set_joint_rotation(fighter);
    return 0.into();
}

unsafe extern "C" fn special_hi_rush_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("samusd_cshot_bullet"), false, false);
    EFFECT_OFF_KIND(fighter, Hash40::new("samusd_dash_attack"), false, false);
    return 0.into();
}

unsafe extern "C" fn special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0 as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn special_hi_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let end_type = VarModule::get_int(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_END_TYPE);
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let reset_type_x = if end_type != END_TYPE_AIR { *ENERGY_CONTROLLER_RESET_TYPE_MOVE_GROUND } else { *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST };
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, reset_type_x, 0.0, 0.0, 0.0, 0.0, 0.0);
    if end_type != END_TYPE_AIR {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        if end_type == END_TYPE_LANDING {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            let limit_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.end_land_limit_speed_x");
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, limit_speed_x, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            let brake_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.end_ground_brake_speed_x");
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
            sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, brake_speed_x, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        }
        VarModule::set_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE, 0.0);
        VarModule::off_flag(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_AIR);
    }
    else {
        let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);

        let speed_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.end_speed_y_mul");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y * speed_y_mul, 0.0, 0.0, 0.0);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let accel_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.end_accel_y_mul");
        let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * accel_y_mul);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);

        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let fall_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.end_fall_x_mul");
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * fall_x_mul, 0.0);

        special_hi_end_enable_control(fighter);
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("sjump_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    
    let end_type = VarModule::get_int(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_END_TYPE);
    let angle = VarModule::get_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE);
    VarModule::set_float(fighter.battle_object, vars::samusd::status::SPECIAL_HI_RUSH_BRAKE, angle);

    let motion = match end_type {
        END_TYPE_GROUND => Hash40::new("special_hi_landing_f"),
        END_TYPE_LANDING => Hash40::new("special_hi_landing_lw"),
        END_TYPE_AIR => Hash40::new("special_hi_fall"),
        _ => Hash40::new("special_hi_fall"),
    };
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    
    fighter.main_shift(special_hi_end_main_loop)
}

unsafe extern "C" fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    fighter.sub_air_check_dive();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let next_status = match VarModule::get_int(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_END_TYPE) {
            END_TYPE_GROUND => *FIGHTER_STATUS_KIND_WAIT,
            END_TYPE_LANDING => *FIGHTER_STATUS_KIND_WAIT,
            END_TYPE_AIR => {
                let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("sjump_landing_frame"));
                WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME); 
                *FIGHTER_STATUS_KIND_FALL_SPECIAL
            },
            _ => {
                let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("sjump_landing_frame"));
                WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME); 
                *FIGHTER_STATUS_KIND_FALL_SPECIAL
            },
        };
        fighter.change_status(next_status.into(), false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("samusd_dash_attack"), true, true);
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("sjump_landing_frame"));
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME); 
        }
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_FALL, false);
    }

    return 0.into();
}

// Enable kinetics, smooth degree and joint rotation towards 0.0
unsafe extern "C" fn special_hi_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL
    && !fighter.is_situation(*SITUATION_KIND_GROUND)
    && VarModule::get_int(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_END_TYPE) == END_TYPE_AIR {
        special_hi_end_enable_control(fighter);
    }
    let mut angle = VarModule::get_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE);
    let initial_angle = VarModule::get_float(fighter.battle_object, vars::samusd::status::SPECIAL_HI_RUSH_BRAKE);
    if initial_angle > 0.0 {
        angle -= 4.0;
        VarModule::set_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE, angle.max(0.0));
        special_hi_set_joint_rotation(fighter);
    }
    else if initial_angle < 0.0 {
        angle += 4.0;
        VarModule::set_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE, angle.min(0.0));
        special_hi_set_joint_rotation(fighter);
    }

    return 0.into();
}

// HELPER FUNCTIONS

unsafe extern "C" fn special_hi_set_joint_rotation(fighter: &mut L2CFighterCommon) {
    let angle = VarModule::get_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE);
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f::new(-angle, 0.0, 0.0), MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
}

unsafe extern "C" fn special_hi_rush_handle_bound(fighter: &mut L2CFighterCommon) -> i32 {
    // Check for landing
    let rush_frame = VarModule::get_int(fighter.battle_object, vars::samusd::status::SPECIAL_HI_RUSH_FRAME);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.is_situation(*SITUATION_KIND_GROUND) 
    && VarModule::is_flag(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_AIR)
    && (speed_y <= 0.5 || rush_frame > 6) {
        let normal_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let touch_normal_angle = sv_math::vec2_angle(normal_x, normal_y, speed_x, speed_y);
        let rush_impact_angle = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.rush_impact_angle");
        let adjusted_angle_rad: f32 = (rush_impact_angle + 90.0).to_radians();
        // If angle is too steep, then land. Otherwise, continue forward
        if adjusted_angle_rad < touch_normal_angle {
            special_hi_find_end_type(fighter);
            fighter.change_status(statuses::samusd::SPECIAL_HI_END.into(), false.into());
            return 1.into();
        }
        else {
            let speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let lr = PostureModule::lr(fighter.module_accessor);
            VarModule::set_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed.abs() * lr, -0.02);
            VarModule::off_flag(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_AIR);
            StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);
        }
    }

    return 0.into();
}

unsafe fn special_hi_find_end_type(fighter: &mut L2CFighterCommon) {
    let angle = VarModule::get_float(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_RUSH_ANGLE);
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::set_int(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_END_TYPE, END_TYPE_AIR);
    }
    else if angle.abs() < 20.0 {
        VarModule::set_int(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_END_TYPE, END_TYPE_GROUND);
    }
    else {
        if angle > 0.0 {
            // account for ending the rush just above a platform when rising
            VarModule::set_int(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_END_TYPE, END_TYPE_AIR);
        }
        else {
            VarModule::set_int(fighter.battle_object, vars::samusd::instance::SPECIAL_HI_END_TYPE, END_TYPE_LANDING);
        }
    }
}

unsafe extern "C" fn special_hi_end_enable_control(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR { return };

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let stable_speed_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.end_stable_speed_x_mul");
    let accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.end_accel_x_mul");
    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul * accel_x_mul);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add * accel_x_mul);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * stable_speed_x_mul, 0.0);
    WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_ACCEL_MUL);
    WorkModule::set_float(fighter.module_accessor, stable_speed_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
}

unsafe extern "C" fn stub(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut smashline::Agent) {    
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, stub);
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, stub);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, stub);

    agent.status(Init, statuses::samusd::SPECIAL_HI_RUSH, special_hi_rush_init);
    agent.status(Pre, statuses::samusd::SPECIAL_HI_RUSH, special_hi_rush_pre);
    agent.status(Main, statuses::samusd::SPECIAL_HI_RUSH, special_hi_rush_main);
    agent.status(Exec, statuses::samusd::SPECIAL_HI_RUSH, special_hi_rush_exec);
    agent.status(ExecStop, statuses::samusd::SPECIAL_HI_RUSH, special_hi_rush_execstop);
    agent.status(End, statuses::samusd::SPECIAL_HI_RUSH, special_hi_rush_end);
    agent.status(Exit, statuses::samusd::SPECIAL_HI_RUSH, special_hi_rush_end);

    agent.status(Init, statuses::samusd::SPECIAL_HI_END, special_hi_end_init);
    agent.status(Pre, statuses::samusd::SPECIAL_HI_END, special_hi_end_pre);
    agent.status(Main, statuses::samusd::SPECIAL_HI_END, special_hi_end_main);
    agent.status(Exec, statuses::samusd::SPECIAL_HI_END, special_hi_end_exec);
    agent.status(ExecStop, statuses::samusd::SPECIAL_HI_END, special_hi_rush_execstop);
    agent.status(End, statuses::samusd::SPECIAL_HI_END, stub);
}