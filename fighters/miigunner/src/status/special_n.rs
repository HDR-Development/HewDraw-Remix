use super::*;

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
        let motion = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
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
        let status = fighter.get_status_by_situation(FIGHTER_STATUS_KIND_WAIT, FIGHTER_STATUS_KIND_FALL);
        fighter.change_status(status, false.into());
        return 0.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK, *GROUND_CORRECT_KIND_AIR);
        if fighter.is_motion(Hash40::new("special_n1_fire")) && fighter.is_motion(Hash40::new("special_air_n1_fire")) {
            let motion = fighter.get_hash_by_situation(Hash40::new("special_n1_fire"), Hash40::new("special_air_n1_fire"));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, motion, -1.0, 1.0, 0.0);
        }
        else if fighter.is_motion(Hash40::new("special_n1_fire_max")) && fighter.is_motion(Hash40::new("special_air_n1_fire_max")) {
            let motion = fighter.get_hash_by_situation(Hash40::new("special_n1_fire_max"), Hash40::new("special_air_n1_fire_max"));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, motion, -1.0, 1.0, 0.0);
        }
        else {
            let motion = fighter.get_hash_by_situation(Hash40::new("special_n1_neon"), Hash40::new("special_air_n1_neon"));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, motion, -1.0, 1.0, 0.0);
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE, special_n1_fire_main);
}