use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::WAS_CANCEL);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    wall_bounce(fighter);
    var_reset(fighter);
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.motion_frame() < 44.0 {
            fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, statuses::bayonetta::SPECIAL_S_EDGE, false);
            return 1.into();
        }
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if fighter.is_in_hitlag() {special_s_slow_hit(fighter); }
    kick_checks(fighter);
    0.into()
}

unsafe extern "C" fn kick_checks(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_frame = fighter.motion_frame();
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT) {
        // manual heelslide kick on jab/ftilt or nb/sb input, buffered heel slide legacy input on last frame if input is held
        if check_input(fighter)
        && !fighter.is_in_hitlag() {
            if (fighter.is_cat_flag(Cat1::SpecialN | Cat1::SpecialS | Cat1::AttackN | Cat1::AttackS3)
            && motion_frame > 19.0 
            && AttackModule::is_attack(fighter.module_accessor, 0, false))
            || (hold_check(fighter)
            && VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK))
            && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::ATTACK_INVALID_COMBO_INPUT)
            {
                EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.75, 4, 4, 4, 0, 0, 0, false); // flash on manual activation to match dabk
                LAST_EFFECT_SET_RATE(fighter, 0.5);
                GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                fighter.change_status(statuses::bayonetta::SPECIAL_S_KICK.into(), true.into())
            } 
        }
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK);
    }
    if motion_frame > 35.0 {EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_heelslide_burst"), false, false); } //fx
    0.into()
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_KICK

unsafe extern "C" fn special_s_edge_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK
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

unsafe extern "C" fn special_s_edge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame: i32 = fighter.global_table[PREV_STATUS_FRAME].get_i32() - 15;
    let start_frame = (frame as f32/10.0).round();
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_edge"), start_frame, 1.0, false, 0.0, false, false);
    // calc speed
    let air_accel_y = fighter.get_param_float("air_accel_y", "");
    let mut speed = (1.125 - (frame as f32/23.0)).max(0.00125);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y / 1.25);
    //sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.03);
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed);
    // force late hit
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10, 0, 20, 0, false);
    if AttackModule::is_attack(fighter.module_accessor, 0, false) { // force late-hit hitbox
        ATTACK(fighter, 0, 0, Hash40::new("footr"), 7.0, 65, 15, 0, 55, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_kick_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_edge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_heelslide_burst"), false, false);
    0.into()
}

unsafe extern "C" fn special_s_edge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if fighter.is_in_hitlag() {special_s_slow_hit(fighter); }
    0.into()
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_KICK

unsafe extern "C" fn special_s_kick_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK
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

unsafe extern "C" fn special_s_kick_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mul_x = fighter.get_param_float("param_special_s", "hs_shooting_speed_mul_x");
    let shield_x = fighter.get_param_float("param_special_s", "guard_speed_mul_x");
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_hold_end"), 0.0, 1.0, false, 0.0, false, false);
    let frame = fighter.global_table[PREV_STATUS_FRAME].get_i32() - 20;
    let mut speed = 1.15 - (0.015 * frame as f32); //instant kick = 1.15, last second kick ~ 0.89
    let prev_inflict_status = VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
    if prev_inflict_status & *COLLISION_KIND_MASK_SHIELD != 0 {
        speed=speed*shield_x;
    } // shield-kick starts with cut speed
     else if prev_inflict_status & *COLLISION_KIND_MASK_HIT != 0 {
        speed=speed*mul_x;
    } // less speed if part 1 hit, shouldnt be able to use on whiff but anyways
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_kick_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_kick_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_FORBID);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    0.into()
}

unsafe extern "C" fn special_s_kick_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if fighter.is_in_hitlag() {special_s_slow_hit(fighter); }
    0.into()
}

unsafe extern "C" fn special_s_slow_hit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mul_x = fighter.get_param_float("param_special_s", "hs_shooting_speed_mul_x");
    let shield_x = fighter.get_param_float("param_special_s", "guard_speed_mul_x");
    fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT);
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, shield_x);
        VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::WAS_CANCEL);
    } else if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, mul_x);
        fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_BEFORE_GUARD);
    }
    0.into()
}

unsafe extern "C" fn wall_bounce(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK) { //wall bounce
        let mut touch_wall = false;
        if PostureModule::lr(fighter.module_accessor) > 0.0 {
            touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
        } else {
            touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
        }
        if touch_wall {fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END.into(), false.into()); }
    }
    0.into()
}

unsafe extern "C" fn var_reset(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_float(0.0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
    fighter.set_int(0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT);
    fighter.set_int(0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_REUSE_FRAME);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    fighter.set_int(0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AFTER_ACTION);
    return 0.into();
}

unsafe extern "C" fn check_input(fighter: &mut L2CFighterCommon) -> bool {
    let special_stick_y = fighter.get_param_float("common", "special_stick_y");
    let lr = fighter.lr();
    // f1 of input try to filter non jab/ftilt inputs
    if fighter.is_button_trigger(Buttons::Attack | Buttons::Special) {
        if fighter.is_stick_backward()
        || fighter.stick_y().abs() > special_stick_y {
            fighter.clear_commands(Cat1::AttackN); 
            fighter.clear_commands(Cat1::AttackS3);
            fighter.clear_commands(Cat1::SpecialN); 
            fighter.clear_commands(Cat1::SpecialS);
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::ATTACK_INVALID_COMBO_INPUT);
            return false.into();
        }
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::status::ATTACK_INVALID_COMBO_INPUT);
    }
    // hold input restriction
    if fighter.is_stick_backward()
    || fighter.left_stick_y().abs() > special_stick_y
    || fighter.right_stick_y().abs() > special_stick_y {
        VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::ATTACK_INVALID_COMBO_INPUT);
        return false.into();
    }
    // again but also filter grab
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)
    || fighter.global_table[CMD_CAT1].get_i32() & (
        *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH
    ) == 1 {
        return false.into();
    }
    true.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);

    agent.status(Pre, statuses::bayonetta::SPECIAL_S_EDGE, special_s_edge_pre);
    agent.status(Main, statuses::bayonetta::SPECIAL_S_EDGE, special_s_edge_main);
    agent.status(End, statuses::bayonetta::SPECIAL_S_EDGE, special_s_edge_end);

    agent.status(Pre, statuses::bayonetta::SPECIAL_S_KICK, special_s_kick_pre);
    agent.status(Main, statuses::bayonetta::SPECIAL_S_KICK, special_s_kick_main);
    agent.status(End, statuses::bayonetta::SPECIAL_S_KICK, special_s_kick_end);
}