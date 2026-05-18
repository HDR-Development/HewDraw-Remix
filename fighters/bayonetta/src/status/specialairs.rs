use super::*;

unsafe extern "C" fn special_air_s_x_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D

unsafe extern "C" fn special_air_s_d_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_S_DABK_COUNT);
    set_startup_speed(fighter);
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
    fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    if fighter.is_prev_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_d"), 5.0, 1.0, false, 0.0, false, false);
    } else { //removed qc input
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_d"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_air_s_d_main_loop as *const () as _))
}

unsafe extern "C" fn special_air_s_d_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // angling
    if fighter.motion_frame() < 8.0 { // should return true until hitboxes spawn
        VarModule::set_float(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_ABK_ANGLE, fighter.left_stick_x() * fighter.lr());
    } else {
        if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP) {
            fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());
            fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP);
        }
    }
    angling(fighter, true);
    bounce_check(fighter);
    //bullet_checks(fighter);
    bullet_movement(fighter);
    if (CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_air_check_fall_common().get_bool())
    || fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if AttackModule::is_attack(fighter.module_accessor, 0, false) {
            VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
        }
        if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING.into(), false.into());
        }
    } else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn special_air_s_x_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP) {
        fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());
        fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP);
    }
    set_lag(fighter); 
    0.into()
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U

unsafe extern "C" fn special_air_s_u_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    set_startup_speed(fighter);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_u"), 0.0, 1.0, false, 0.0, false, false);
    fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP);
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(special_air_s_u_main_loop as *const () as _))
}

unsafe extern "C" fn special_air_s_u_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //bullet_checks(fighter); // use to enable flag
    bullet_movement(fighter);
    angling(fighter, false);
    let frame = fighter.global_table[CURRENT_FRAME].get_i32();
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP) {
        if fighter.get_param_int("param_special_s", "ab_u_disable_landing_frame") <= frame {
            fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());
            fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP);
        }
    }
    if frame <= 9 {
        cache_input(fighter);
    }
    // downwards afterburner kick input
    if frame == 9 && fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S_U_TO_D) {
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D.into(), false.into());
    }
    if (CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_air_check_fall_common().get_bool())
    || fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK) {wall_check(fighter); }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    } else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT

unsafe extern "C" fn special_air_s_bounce_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_air_s_d_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    SlowModule::clear(fighter.module_accessor); // manually balanced faf
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_d_hit"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_air_s_bounce_main_loop as *const () as _))
}

unsafe extern "C" fn special_air_s_bounce_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // apply momentum at x fraame of anim
    if VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK) {
        // use normal gravity/drift values (no custom values for now)
        let ab_d_hit_after_speed_x = fighter.get_param_float("param_special_s", "ab_d_hit_after_speed_x");
        let ab_d_hit_after_speed_y = fighter.get_param_float("param_special_s", "ab_d_hit_after_speed_y");
        KineticModule::clear_speed_all(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        let lr = fighter.lr();
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, -ab_d_hit_after_speed_x*lr, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ab_d_hit_after_speed_y);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK);
    }
    // drift lock
    let prev_inflict_status = VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
    let hit_drift_lockout_frame = fighter.get_param_int("param_special_s", "ab_d_hit_after_stick_control_start_frame");
    let whiff_drift_lockout_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_s.whiff_drift_lockout_frame");
    if fighter.global_table[CURRENT_FRAME].get_i32() >= hit_drift_lockout_frame
    && prev_inflict_status & *COLLISION_KIND_MASK_HIT != 0 {
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    if prev_inflict_status & *COLLISION_KIND_MASK_HIT == 0 
    && fighter.global_table[CURRENT_FRAME].get_i32() == whiff_drift_lockout_frame {
        let whiff_drift_mul_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.whiff_drift_mul_x");
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, whiff_drift_mul_x);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    
    if (CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_air_check_fall_common().get_bool())
    || fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    } else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END

unsafe extern "C" fn special_air_s_wall_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR); // stall until bounce portion
    let motion = if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D {"special_air_s_d_wall_end"} else {"special_air_s_u_wall_end"};
    MotionModule::change_motion(fighter.module_accessor, Hash40::new(motion), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_air_s_bounce_main_loop as *const () as _))
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING

unsafe extern "C" fn special_air_s_d_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("special_air_s_d_landing"));
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_air_s_d_landing"), true);
    let special_lag = fighter.get_float(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
    let motion_rate = if cancel_frame > 0.0 {cancel_frame/special_lag} else {end_frame/special_lag};
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_d_landing"), 0.0, motion_rate, false, 0.0, false, false);
    GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    var_reset(fighter); // reset special vars
    fighter.sub_shift_status_main(L2CValue::Ptr(special_air_s_d_landing_main_loop as *const () as _))
}

unsafe extern "C" fn special_air_s_d_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && (fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_MOTION, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_d_landing_edge"), 0.0, 1.0, false, 0.0, false, false);
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    } // 6f faf edge cancel
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    0.into()
}

// helpers

// using current up b stats
unsafe extern "C" fn set_startup_speed(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let y_speed = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let air_start_speed_mul_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.air_start_speed_mul_x");
    let air_start_speed_mul_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.air_start_speed_mul_y");
    let air_accel_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.air_accel_y");
    let air_max_speed_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.air_max_speed_y");
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, x_speed * air_start_speed_mul_x, 0.0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, y_speed * air_start_speed_mul_y);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_max_speed_y);
    0.into()
}

unsafe extern "C" fn bounce_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AttackModule::is_attack(fighter.module_accessor, 0, false) { // don't activate during bullet arts
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) {
            fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_D_FLAG_HIT);
            return 1.into();
        }
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
            if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
            }
            fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT.into(), false.into());
            return 1.into();
        }
    }
    wall_check(fighter);
    0.into()
}

unsafe extern "C" fn wall_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut touch_wall = false;
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
    } else {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
    }
    if touch_wall {
        if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && AttackModule::is_attack(fighter.module_accessor, 0, false) { //checks if hitbox cleared to prevent double dipping
            VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
        }
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn cache_input(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S_U_TO_D) {
        if fighter.is_button_on(Buttons::Attack | Buttons::Catch) 
        && !fighter.is_button_on(Buttons::CStickOn) 
        && !fighter.is_cat_flag(Cat1::AttackHi3 | Cat1::SpecialHi) {
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.75, 4, 4, 4, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S_U_TO_D);
            VarModule::set_float(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_ABK_ANGLE, -1.15); //angle forced down during dabk windup
        } else {
            VarModule::set_float(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_ABK_ANGLE, fighter.left_stick_y());
        } //angle if no dabk
    }
    0.into()
}

unsafe extern "C" fn angling(fighter: &mut L2CFighterCommon, dive: bool) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let facing = fighter.lr();
    let stick = VarModule::get_float(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_ABK_ANGLE);
    if dive {
        joint_rotator(fighter, frame, Hash40::new("rot"), Vector3f{x: -6.0*stick, y:0.0, z:0.0}, 1.0, 7.0, 25.0, 32.0);
        if frame > 7.0 && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK) {
            KineticModule::clear_speed_all(fighter.module_accessor);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
            let base = fighter.get_param_float("param_special_s", "ab_d_rotate");
            let speed = fighter.get_param_float("param_special_s", "ab_d_motion_speed_mul");
            let maxrot = 6.0;
            let angle = if facing < 0.0 {
                -base - stick *maxrot //l
            } else {
                base + stick *maxrot //r
            };
            let angle_rad = angle.to_radians();

            // have to manually calculate what the horizontal distance should be..
            let base_angle: f32 = 45.0;
            let base_angle_c: f32 = 90.0 - base_angle;
            let base_c = 113.14;
            let base_dist = (base_c*f32::sin(90.0_f32.to_radians()))/f32::sin(base_angle_c.to_radians());
            let angle_c: f32 = 90.0 - angle;
            let new_dist = (base_c*f32::sin(90.0_f32.to_radians()))/f32::sin(angle_c.to_radians());
            let ratio = new_dist / base_dist;
            let new_speed = ratio*speed;
            
            sv_kinetic_energy!(set_angle, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, angle_rad);
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, new_speed);
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK); //only angle once
        }
    } else {
        joint_rotator(fighter, frame, Hash40::new("rot"), Vector3f{x: -15.0*stick, y:0.0, z:0.0}, 1.0, 12.0, 31.0, 40.0);
        if frame > 11.0 && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK) {
            KineticModule::clear_speed_all(fighter.module_accessor);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
            let base = fighter.get_param_float("param_special_s", "ab_u_rotate"); // base seems to actually be 35* before falling??
            let speed = fighter.get_param_float("param_special_s", "ab_u_motion_speed_mul");
            let maxrot = 15.0;
            let angle = if facing < 0.0 {
                -base - stick *maxrot //l
            } else {
                base + stick *maxrot //r
            };
            let angle_rad = angle.to_radians();

            // have to manually calculate what the horizontal distance should be..
            let base_angle: f32 = 35.0;
            let base_angle_c: f32 = 90.0 - base_angle;
            let base_c = 63.0;
            let base_dist = (base_c*f32::sin(90.0_f32.to_radians()))/f32::sin(base_angle_c.to_radians());
            let angle_c: f32 = 90.0 - angle;
            let new_dist = (base_c*f32::sin(90.0_f32.to_radians()))/f32::sin(angle_c.to_radians());
            let ratio = new_dist / base_dist;
            let new_speed = ratio*speed;

            sv_kinetic_energy!(set_angle, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, angle_rad);
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, new_speed);
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK); //only angle once
        }
    }
    0.into()
}

unsafe extern "C" fn bullet_movement(fighter: &mut L2CFighterCommon) -> L2CValue { //was like 400 lines
    let dabk = fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D);
    if VarModule::get_int(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_BULLET_STAGE) == *FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT {
        if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) != *FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT {
            if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) == *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING {
                // this runs the first time it shoots
                //VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_BULLET_ARTS_COUNT);
                let speed = VarModule::get_vec2(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_MOTION_XY);
                let x_reset = speed.x;
                let y_reset = speed.y;
                let initial_x = if dabk { 0.8 } else {fighter.get_param_float("param_special_s", "ab_u_shooting_speed_x_mul")};
                let initial_y = if dabk { 0.2 } else {fighter.get_param_float("param_special_s", "ab_u_shooting_speed_y_mul")};
                let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::KineticEnergy;
                let mut motion_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut app::KineticEnergy;
                //motion to stop energy
                lua_bind::KineticEnergy::reset_energy(stop_energy as _, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f { x: x_reset * initial_x, y: 0.0 }, &Vector3f::zero(), fighter.module_accessor);
                lua_bind::KineticEnergyNormal::set_accel(stop_energy as *mut app::KineticEnergyNormal, &Vector2f { x: 0.0, y: 0.0 });
                lua_bind::KineticEnergyNormal::set_brake(stop_energy as *mut app::KineticEnergyNormal, &Vector2f { x: fighter.get_param_float("param_special_s", "ab_u_shooting_brake_speed_x"), y: 0.0 });
                lua_bind::KineticEnergyNormal::set_stable_speed(stop_energy as *mut app::KineticEnergyNormal, &Vector2f { x: fighter.get_param_float("param_special_s", "ab_u_shooting_stable_speed_x"), y: 0.0 });
                lua_bind::KineticEnergyNormal::set_limit_speed(stop_energy as *mut app::KineticEnergyNormal, &Vector2f { x: -1.0, y: -1.0 });
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                //motion to gravity
                lua_bind::KineticEnergy::reset_energy(gravity_energy as _, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f { x: 0.0, y: y_reset * initial_y + 0.125 }, &Vector3f::zero(), fighter.module_accessor);
                lua_bind::FighterKineticEnergyGravity::set_accel(gravity_energy as *mut app::FighterKineticEnergyGravity, -fighter.get_param_float("param_special_s", "ab_u_shooting_accel_y"));
                lua_bind::FighterKineticEnergyGravity::set_stable_speed(gravity_energy as *mut app::FighterKineticEnergyGravity, fighter.get_param_float("param_special_s", "ab_u_shooting_max_speed_y"));
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
                VarModule::set_int(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_BULLET_STAGE, *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING)
            }
        } else { //save motion to vars
            let speed = Vector2f{
                x: KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN),
                y: KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            };
            VarModule::set_vec2(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_MOTION_XY, Vector2f{x: speed.x, y: speed.y});
        }
    } else if VarModule::get_int(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_BULLET_STAGE) == *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING { // shooting start
        if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) != *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING {
            let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
            let speed = Vector2f{
                x: lua_bind::KineticEnergy::get_speed_x(stop_energy),
                y: lua_bind::KineticEnergy::get_speed_y(stop_energy)
            };
            let x_cap = fighter.get_param_float("param_special_s", "ab_u_shooting_stable_speed_x");
            if speed.x.abs() <= x_cap {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                VarModule::set_int(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_BULLET_STAGE, *FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END)
            }
        }
    }
    0.into()
}

unsafe fn joint_rotator(fighter: &mut L2CFighterCommon, frame: f32, joint: Hash40, rotation_amount: Vector3f, start_frame: f32, bend_frame: f32, return_frame: f32, straight_frame: f32) {
    let lua_state = fighter.lua_state_agent;
    let max_rotation = rotation_amount;
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    if frame >= start_frame && frame < return_frame {
        // this has to be called every frame, or you snap back to the normal joint angle
        // interpolate to the respective waist bend angle
        let calc_x_rotate = max_rotation.x * (frame / (bend_frame - start_frame));
        let calc_y_rotate = max_rotation.y * (frame / (bend_frame - start_frame));
        let calc_z_rotate = max_rotation.z * (frame / (bend_frame - start_frame));
        let mut x_rotation = 0.0;
        let mut y_rotation = 0.0;
        let mut z_rotation = 0.0;
        if max_rotation.x < 0.0 {
            x_rotation = calc_x_rotate.clamp(max_rotation.x, 0.0);
        }
        else {
            x_rotation = calc_x_rotate.clamp(0.0, max_rotation.x);
        }
        if max_rotation.y < 0.0 {
            y_rotation = calc_y_rotate.clamp(max_rotation.y, 0.0);
        }
        else {
            y_rotation = calc_y_rotate.clamp(0.0, max_rotation.y);
        }
        if max_rotation.z < 0.0 { 
            z_rotation = calc_z_rotate.clamp(max_rotation.z, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_rotation.z);
        }
        rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(fighter.module_accessor, joint, &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    } else if frame >= return_frame && frame < straight_frame {
        // linear interpolate back to normal
        let calc_x_rotate = max_rotation.x *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let calc_y_rotate = max_rotation.y *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let calc_z_rotate = max_rotation.z *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let mut x_rotation = 0.0;
        let mut y_rotation = 0.0;
        let mut z_rotation = 0.0;
        if max_rotation.x < 0.0 {
            x_rotation = calc_x_rotate.clamp(max_rotation.x, 0.0);
        }
        else {
            x_rotation = calc_x_rotate.clamp(0.0, max_rotation.x);
        }
        if max_rotation.y < 0.0 {
            y_rotation = calc_y_rotate.clamp(max_rotation.y, 0.0);
        }
        else {
            y_rotation = calc_y_rotate.clamp(0.0, max_rotation.y);
        }
        if max_rotation.z < 0.0 { 
            z_rotation = calc_z_rotate.clamp(max_rotation.z, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_rotation.z);
        }
        rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(fighter.module_accessor, joint, &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, special_air_s_x_pre);
    agent.status(Main, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, special_air_s_d_main);
    agent.status(End, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, special_air_s_x_end);

    agent.status(Pre, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, special_air_s_x_pre);
    agent.status(Main, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, special_air_s_u_main);
    agent.status(End, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, special_air_s_x_end);

    agent.status(Pre, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT, special_air_s_bounce_pre);
    agent.status(Main, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT, special_air_s_d_hit_main);
    agent.status(Pre, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END, special_air_s_bounce_pre);
    agent.status(Main, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END, special_air_s_wall_end_main);

    agent.status(Main, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING, special_air_s_d_landing_main);
}
