use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION |
            *FIGHTER_STATUS_ATTR_START_TURN
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    
    return 0.into();
}

pub unsafe extern "C" fn special_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_SP_BRAKE);
    fighter.off_flag(*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_CLIFF_CHECK);
    fighter.off_flag(*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_WALL_CHECK);
    fighter.off_flag(*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_FALL_ONOFF);
    fighter.off_flag(*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_LANDING_FALL_SPECIAL);
    let hit_reduct_count = fighter.get_param_int("param_special_lw", "lw3_hit_reduct_count");
    fighter.set_int(hit_reduct_count, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_REDUCTION_LEFT);
    fighter.set_float(1.0, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLOAT_JET_STUB_SPEED_COEFFICIENT);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw3"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else {
        fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_12) - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_12) - 1);
    // Restore double jump
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
        fighter.dec_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }

    fighter.main_shift(special_s3_main_loop)
}

pub unsafe extern "C" fn special_s3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn special_s3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END.into(), false.into());
        return 0.into();
    }

    // Skip to end on shield
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY)
    && !fighter.is_in_hitlag() {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END.into(), false.into());
        return 0.into();
    }

    let start_situation = fighter.get_int(*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION);
    if start_situation == *SITUATION_KIND_GROUND {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            else {
                // grounded edge cancel
                if fighter.is_flag(*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_CLIFF_CHECK) {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    return 0.into();
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                    fighter.set_situation(SITUATION_KIND_AIR.into());
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                }
            }
        }
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY) {
            fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END.into(), false.into());
            return 1.into();
        }

        return 0.into();
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND)
    && GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END.into(), false.into());
        return 0.into();
    }

    return 0.into();
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END

pub unsafe extern "C" fn special_s3_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION);
    
    if start_situation == *SITUATION_KIND_AIR
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        KineticModule::unable_energy_all(fighter.module_accessor);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR_BRAKE,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    } else if start_situation == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_GROUND,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );

        // Reduce speed on shield
        let prev_inflict_status = VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
        if prev_inflict_status & *COLLISION_KIND_MASK_SHIELD != 0 || prev_inflict_status & *COLLISION_KIND_MASK_PARRY != 0 {
            let shield_hit_end_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s3.shield_hit_end_speed_x");
            let lr = PostureModule::lr(fighter.module_accessor);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                shield_hit_end_speed_x * lr,
                0.0
            );
        }

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }

    0.into()
}

unsafe extern "C" fn special_s3_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) != *SITUATION_KIND_GROUND {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3_end_air"), 0.0, 1.0, false, 0.0, false, false);
            if StopModule::is_stop(fighter.module_accessor) {
                special_s3_end_substatus(fighter, false.into());
            }
            fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s3_end_substatus as *const () as _));
            fighter.main_shift(special_s3_end_main_loop_air)
        }
        else {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                // ending air thrust on the ground
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(0.8, 1.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_STOP);
                let rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_end_air_to_ground_rate"));
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3_end"), 0.0, rate, false, 0.0, false, false);
                fighter.main_shift(special_s3_end_main_loop_ground)
            }
            else {
                // ending air thrust in air
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                let dive_speed_y = fighter.get_param_float("dive_speed_y", "");
                sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, dive_speed_y);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3_end_air"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_INHERIT_FALL);
                WorkModule::unable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_LANDING_WAIT);
                WorkModule::unable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_LANDING_FALL);
                fighter.main_shift(special_s3_end_main_loop)
            }
        }
    }
    else {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            // ending grounded thrust on the ground
            GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            let rate = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"), hash40("lw3_end_ground_to_ground_rate"));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw3_end"), 0.0, rate, false, 0.0, false, false);

            // Reduce speed on shield
            let prev_inflict_status = VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
            if prev_inflict_status & *COLLISION_KIND_MASK_SHIELD != 0 || prev_inflict_status & *COLLISION_KIND_MASK_PARRY != 0 {
                let shield_hit_end_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s3.shield_hit_end_speed_x");
                let lr = PostureModule::lr(fighter.module_accessor);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, shield_hit_end_speed_x * lr, 0.0);
            }

            fighter.main_shift(special_s3_end_main_loop_ground)
        }
        else {
            // ending grounded thrust in the air
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(0.0, 0.5, 0.0));
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3_end_air"), 0.0, 1.0, false, 0.0, false, false);
            fighter.main_shift(special_s3_end_main_loop_air)
        }
    }
}

unsafe extern "C" fn special_s3_end_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if !param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_FALL_ONOFF) {
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_FALL);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_MOTION_AIR);
        }
    }
}

unsafe extern "C" fn special_s3_end_main_loop_ground(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    
    return 0.into();
}

unsafe extern "C" fn special_s3_end_main_loop_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) == *SITUATION_KIND_GROUND {
        if fighter.status_frame() == 20 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_s3_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_INHERIT_FALL) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_landing_frame"));
            WorkModule::set_float(fighter.module_accessor,landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 0.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 0.into();
            }
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END, special_s3_end_main);
}