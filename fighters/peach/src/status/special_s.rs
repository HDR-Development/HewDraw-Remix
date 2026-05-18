use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
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

    return 0.into();
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
    VarModule::on_flag(fighter.battle_object, vars::common::status::DISABLE_ECB_SHIFT);
    fighter.change_motion_by_situation("special_s_start", "special_air_s_start", 0.0, 1.0, false, 0.0, false, false);
    special_s_start_momentum(fighter, 1.0);
    fighter.set_int(1, *FIGHTER_PEACH_STATUS_SPECIAL_S_WORK_INT_ENABLE_UNIQ);

    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP.into(), false.into());
    }
    if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0
    && GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        KineticModule::clear_speed_all(fighter.module_accessor);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    return 0.into();
}

unsafe fn special_s_start_momentum(fighter: &mut L2CFighterCommon, mul: f32) {
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_y = fighter.get_param_float("param_special_s", "special_s_start_speed_y");
    let stable_y = fighter.get_param_float("param_special_s", "special_s_jump_stable_y");
    let max_y = fighter.get_param_float("air_speed_y_stable", "");
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, stable_y);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, (speed_y+start_y).clamp(-max_y, max_y * mul));
}

unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    0.into()
}


// FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP

unsafe extern "C" fn special_s_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::peach::instance::DISABLE_SPECIAL_S);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_jump"), 0.0, 1.0, false, 0.0, false, false);

    fighter.main_shift(special_s_jump_main_loop)
}

unsafe extern "C" fn special_s_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_jump_momentum(fighter);
    hit_check(fighter);
    wall_check(fighter);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) || fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END.into(), false.into());
        return 1.into();
    }
    
    return 0.into();
}

// rapidly decaying speed, drift to change distance
unsafe extern "C" fn special_s_jump_momentum(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = fighter.lr();
    let start_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_start_x");
    let min_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_min_x");
    let brake_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_brake_x");
    let accel_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_accel_x");
    let start_y = fighter.get_param_float("param_special_s", "special_s_jump_speed_y");
    let stable_y = fighter.get_param_float("param_special_s", "special_s_jump_stable_y");
    let jump_max_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_max_y");
    if StatusModule::is_changing(fighter.module_accessor) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_x, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_x, 0.0);
        sv_kinetic_energy!(set_accel_x_add, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0);
        sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_x*lr, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, stable_y);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, (speed_y+start_y).clamp(-jump_max_y, jump_max_y));
        return 1.into();
    }
    // speed cannot go below minimum, cannot exceed starting value due to higher brake value
    let add_speed = (fighter.left_stick_x() * lr * accel_x) - brake_x;
    let mut new_speed = (add_speed*lr) + speed_x;
    new_speed = if lr > 0.0 {new_speed.clamp(min_x, start_x)} else {new_speed.clamp(-start_x, -min_x)};
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed, 0.0);
    0.into()
}

// hit check
unsafe extern "C" fn hit_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    // bounce on-hit
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END.into(), false.into());
        return 1.into();
    }
    // end on-shield
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END.into(), false.into());
        return 1.into();
    }
    // kill hitbox if minimum speed
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let min_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_min_x");
    let lr = fighter.lr();
    if speed_x.abs() <= min_x
    && AttackModule::is_attack(fighter.module_accessor, 0, false) {
        AttackModule::clear_all(fighter.module_accessor);
        EFFECT_OFF_KIND(fighter, Hash40::new("peach_bomber_jamp"), false, false);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
    }
    0.into()
}

// wall bounce if moving forward
unsafe extern "C" fn wall_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut touch_wall = false;
    if fighter.lr() > 0.0 {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
    } else {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
    }
    if touch_wall
    && speed_x.abs() >= 0.0 {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END.into(), true.into());
    }
    0.into()
}

// FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END

unsafe extern "C" fn special_s_away_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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

    return 0.into();
}

unsafe extern "C" fn special_s_away_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_motion_by_situation("special_s_end", "special_air_s_end", 1.0, 1.0, false, 0.0, false, false);
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
    special_s_end_momentum(fighter);
    
    fighter.main_shift(special_s_away_end_main_loop)
}

unsafe extern "C" fn special_s_away_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // wall bounce first 4 frames
    let end_wall_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_s.end_wall_frame");
    if fighter.status_frame() < end_wall_frame {
        wall_check(fighter);
    }
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
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        // if returning to fall anim, cancel into heavy landing
        if fighter.is_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_DONE_CONTROLLER_MOVE) {
            // special landing lag if not actionable
            let end_landing_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.end_landing_frame");
            let mut lag = Some(end_landing_frame);
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                lag = None;
            }
            fighter.check_land_cancel(lag);
            return 1.into();
        }
        // cancel into full lag ground slide if still in jump pose, ledge cancel
        fighter.change_status_by_situation(*FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END, *FIGHTER_STATUS_KIND_FALL, false);
    }
    // enable drift frame
    if fighter.is_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_START_CONTROLLER_MOVE) {
        let control_accel_x = fighter.get_param_float("param_special_s", "special_air_s_end_control_accel_x");
        sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, control_accel_x);
        // uncap speed frame (transition better to fall?)
        if fighter.is_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_DONE_CONTROLLER_MOVE) {
            let max_x = fighter.get_param_float("air_speed_x_stable", "");
            let max_y = fighter.get_param_float("air_speed_y_stable", "");
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, max_x, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, max_y);
        }
        fighter.off_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_START_CONTROLLER_MOVE);
    }

    return 0.into();
}

// prevent excessive speed transfer? drift limits
unsafe fn special_s_end_momentum(fighter: &mut L2CFighterCommon) {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = fighter.lr();
    let mut end_landing_x_min = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.end_landing_x_min");
    let end_landing_x_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.end_landing_x_max");
    let end_x_stable = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.end_x_stable");
    let end_y_stable = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.end_y_stable");
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        // a2g speed transfer, clamped values if landed during dash to prevent broken looking speed
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        let mut new_speed = if lr > 0.0 {speed_x.clamp(end_landing_x_min, end_landing_x_max)} else {speed_x.clamp(-end_landing_x_max, -end_landing_x_min)};
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, new_speed);
        return;
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, end_x_stable, 0.0);
    sv_kinetic_energy!(set_accel_x_add, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0);
    sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, end_y_stable);
}

unsafe extern "C" fn special_s_hit_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END)(fighter);
    let prev_inflict_status = VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
    if prev_inflict_status & *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY != 0 {
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
        let speed_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.shield_hit_end_speed_x_mul");
        let accel_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.shield_hit_end_accel_mul");
        
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * speed_mul, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul * accel_mul);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add * accel_mul);
    }

    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);

    agent.status(Pre, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_pre);
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_main);

    agent.status(Pre, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END, special_s_away_end_pre);
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END, special_s_away_end_main);
}
