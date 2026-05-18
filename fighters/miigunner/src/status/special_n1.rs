use super::*;

pub unsafe extern "C" fn special_n1_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n1_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n1_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    VarModule::on_flag(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);

    fighter.main_shift(special_n1_start_main_loop)
}

pub unsafe extern "C" fn special_n1_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        VarModule::off_flag(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let charge_count = fighter.get_int(*FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
        let cshot_charge_frame = fighter.get_param_float("param_special_n", "n1_cshot_charge_frame");
        if charge_count as f32 >= cshot_charge_frame {
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_HOLD.into(), false.into());
        }
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n1_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n1_start"), -1.0, 1.0, 0.0, false, false);
        }
    }

    return 0.into();
}

pub unsafe extern "C" fn special_n1_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK, *GROUND_CORRECT_KIND_AIR);
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    fighter.change_motion_by_situation("special_n1_hold", "special_air_n1_hold", 0.0, 1.0, false, 0.0, false, false);
    fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);

    fighter.main_shift(special_n1_hold_main_loop)
}

pub unsafe extern "C" fn special_n1_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_pad_flag(PadFlag::SpecialTrigger)
    || fighter.is_pad_flag(PadFlag::AttackTrigger) {
        fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE.into(), false.into());
        return 1.into();
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
            fighter.set_int(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL.into(), true.into());
            return 0.into();
        }
        if fighter.sub_check_jump_in_charging_for_cancel_status((*FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS).into()).get_bool() {
            fighter.set_int(*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL.into(), true.into());
            return 0.into();
        }
    }
    else {
        if fighter.is_cat_flag(Cat1::AirEscape) {
            fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL.into(), true.into());
            return 0.into();
        }
        if fighter.sub_check_jump_in_charging_for_cancel_status((*FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS).into()).get_bool() {
            fighter.set_int(*FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL.into(), true.into());
            return 0.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK, *GROUND_CORRECT_KIND_AIR);
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        fighter.change_motion_inherit_frame_by_situation("special_n1_hold", "special_air_n1_hold", -1.0, 1.0, 0.0, false, false);
    }
    if !fighter.global_table[IS_STOPPING].get_bool() {
        fighter.inc_int(*FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
        let charge_count = fighter.get_int(*FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
        let n1_cshot_charge_frame = fighter.get_param_float("param_special_n", "n1_cshot_charge_frame");
        if charge_count as f32 >= n1_cshot_charge_frame {
            fighter.set_int(n1_cshot_charge_frame as i32, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL.into(), false.into());
            return 1.into();
        }
        else {
            if fighter.is_pad_flag(PadFlag::SpecialTrigger)
            || fighter.is_pad_flag(PadFlag::AttackTrigger) {
                fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE.into(), false.into());
                return 1.into();
            }
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x322bb60c04), charge_count as f32 / n1_cshot_charge_frame);
        }
    }
    else {
        let charge_count = fighter.get_int(*FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
        let n1_cshot_charge_frame = fighter.get_param_float("param_special_n", "n1_cshot_charge_frame");
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x322bb60c04), charge_count as f32 / n1_cshot_charge_frame);
    }

    return 0.into();
}

pub unsafe extern "C" fn special_n1_fire_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_count = fighter.get_int(*FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_COUNT);
    let n1_cshot_charge_frame = fighter.get_param_float("param_special_n", "n1_cshot_charge_frame");
    if (charge_count as f32) < n1_cshot_charge_frame {
        let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {
            Hash40::new("special_n1_fire")
        }
        else {
            Hash40::new("special_air_n1_fire")
        };
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        let motion = if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                Hash40::new("special_n1_neon")
            }
            else {
                Hash40::new("special_air_n1_neon")
            }
        }
        else {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                Hash40::new("special_n1_fire_max")
            }
            else {
                Hash40::new("special_air_n1_fire_max")
            }
        };
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    fighter.set_int(0, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 as i32 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);

    fighter.main_shift(special_n1_fire_main_loop)
}

pub unsafe extern "C" fn special_n1_fire_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK, *GROUND_CORRECT_KIND_AIR);
            if (fighter.is_motion(Hash40::new("special_n1_fire")) || fighter.is_motion(Hash40::new("special_air_n1_fire"))) {
                fighter.change_motion_inherit_frame_keep_rate_by_situation("special_n1_fire", "special_air_n1_fire", -1.0, 1.0, 0.0);
            }
            else if (fighter.is_motion(Hash40::new("special_n1_fire_max")) || fighter.is_motion(Hash40::new("special_air_n1_fire_max"))) {
                fighter.change_motion_inherit_frame_keep_rate_by_situation("special_n1_fire_max", "special_air_n1_fire_max", -1.0, 1.0, 0.0);
            }
            else {
                fighter.change_motion_inherit_frame_keep_rate_by_situation("special_n1_neon", "special_air_n1_neon", -1.0, 1.0, 0.0);
            }
        }
        if fighter.is_motion_one_of(&[Hash40::new("special_n1_neon"), Hash40::new("special_air_n1_neon")]) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            && !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::SPECIAL_N1_CLEAR_CRIT) {
                VarModule::on_flag(fighter.battle_object, vars::miigunner::status::SPECIAL_N1_CLEAR_CRIT);
                SlowModule::set_whole(fighter.module_accessor, 4, 5);
                EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), false, true, true);
            }
        }
    }

    return 0.into();
}

pub unsafe extern "C" fn special_n1_fire_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW3_CLEAR_CRIT) {
        SlowModule::clear_whole(fighter.module_accessor);
        CameraModule::reset_all(fighter.module_accessor);
        EffectModule::remove_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), 0);
    }
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_START, special_n1_start_main);

    agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_HOLD, special_n1_hold_main);

    agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE, special_n1_fire_main);
    agent.status(End, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE, special_n1_fire_end);
}