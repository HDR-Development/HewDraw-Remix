use super::*;

pub unsafe extern "C" fn special_s1_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_SHIELD_HIT);
    fighter.off_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT);
    fighter.off_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT_CHECK_ONOFF);
    fighter.off_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_CLIFF_FALL_ONOFF);
    fighter.off_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_GRAVITY_ONOFF);
    let s1_speed_coeff = fighter.get_param_float("param_special_s", "s1_speed_coeff");
    fighter.set_float(s1_speed_coeff, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s1_start"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        if !StopModule::is_stop(fighter.module_accessor) {
            sub_special_s1_ground(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_s1_ground as *const () as _));
    }
    else {
        fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1_start"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        if !StopModule::is_stop(fighter.module_accessor) {
            sub_special_s1_air(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_s1_air as *const () as _));
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04) + -1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04) + -1);

    fighter.main_shift(special_s1_main_loop)
}

unsafe extern "C" fn sub_special_s1_ground(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_CLIFF_FALL_ONOFF) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }

    return 0.into();
}

unsafe extern "C" fn sub_special_s1_air(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_GRAVITY_ONOFF) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_s1_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END.into(), true.into());
    }
    if fighter.get_int(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION) == *SITUATION_KIND_GROUND {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 1.into();
            }
            else {
                if !fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT) {
                    if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_SHIELD_HIT) {
                        app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
                        fighter.off_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_SHIELD_HIT);
                    }
                    if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_MOTION_SPEED_MUL) {
                        fighter.off_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_MOTION_SPEED_MUL);
                        let s1_start_motion_speed_mul = fighter.get_param_float("param_special_s", "s1_start_motion_speed_mul");
                        sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, s1_start_motion_speed_mul);
                    }
                    return 0.into();
                }
                else {
                    fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END.into(), false.into());
                    return 1.into();
                }
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
        else {
            if fighter.is_situation(*SITUATION_KIND_GROUND)
            && fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING) {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
            else {
                if !fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT) {
                    if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_SHIELD_HIT) {
                        app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
                        fighter.off_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_SHIELD_HIT);
                    }
                    if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_MOTION_SPEED_MUL) {
                        fighter.off_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_MOTION_SPEED_MUL);
                        let s1_start_motion_speed_mul = fighter.get_param_float("param_special_s", "s1_start_motion_speed_mul");
                        sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, s1_start_motion_speed_mul);
                    }
                    return 0.into();
                }
                fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END.into(), false.into());
                return 1.into();
            }
        }
    }
    
    return 0.into();
}

pub unsafe extern "C" fn special_s1_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.get_int(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION) == *SITUATION_KIND_GROUND {
        fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s1_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    let s1_reaction_mul_min_damage = fighter.get_param_float("param_special_s", "s1_reaction_mul_min_damage");
    let s1_reaction_mul_max_damage = fighter.get_param_float("param_special_s", "s1_reaction_mul_max_damage");
    let s1_reaction_mul_max_add = fighter.get_param_float("param_special_s", "s1_reaction_mul_max_add");
    let s1_reaction_mul_valid_time = fighter.get_param_float("param_special_s", "s1_reaction_mul_valid_time");
    let s1_reaction_mul_recover_time = fighter.get_param_float("param_special_s", "s1_reaction_mul_recover_time");
    let recover_frames = s1_reaction_mul_recover_time * 60.0;
    let mul_valid_frame = fighter.get_int(*FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_100KICK_REACTION_MUL_VALID_FRAME);
    let mul_recover_frame = fighter.get_int(*FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_100KICK_REACTION_MUL_RECOVER_FRAME);
    let mut damage_base_calc = 0.0;
    if mul_valid_frame >= 0 {
        if mul_recover_frame > 0 {
            if recover_frames > 0.0 {
                damage_base_calc = s1_reaction_mul_max_add * ((recover_frames - mul_recover_frame as f32) / recover_frames);
            }
        }
    }
    let damage = DamageModule::damage(fighter.module_accessor, 0);
    let mut damage_mul = 0.0;
    if damage > 0.0 {
        let damage_clamp = damage.clamp(s1_reaction_mul_min_damage, s1_reaction_mul_max_damage);
        damage_mul = (damage_clamp - s1_reaction_mul_min_damage) / damage_clamp;
    }
    AttackModule::set_reaction_mul_3rd(fighter.module_accessor, 1.0 + damage_base_calc * damage_mul);
    fighter.set_int((s1_reaction_mul_valid_time * 60.0) as i32, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_100KICK_REACTION_MUL_VALID_FRAME);
    fighter.set_int(recover_frames as i32, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_100KICK_REACTION_MUL_VALID_FRAME);
    fighter.off_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING);

    fighter.main_shift(special_s1_end_main_loop)
}

pub unsafe extern "C" fn special_s1_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
    if fighter.get_int(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION) == *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
            return 1.into();
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            let status = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING)
                { FIGHTER_STATUS_KIND_FALL } else { FIGHTER_STATUS_KIND_FALL_SPECIAL };
            fighter.change_status(status.into(), false.into());
            return 1.into();
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            let status = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING)
                { FIGHTER_STATUS_KIND_LANDING } else { FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL };
            fighter.change_status(status.into(), false.into());
            return 1.into();
        }
    }
    fighter.sub_off_passive_opponent(FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_HIT_OBJECT_ID.into(), FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_HIT_OBJECT_NUM.into(), true.into());

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END, special_s1_end_main);
}