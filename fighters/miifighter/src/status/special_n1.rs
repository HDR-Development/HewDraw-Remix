use super::*;

pub unsafe extern "C" fn special_n1_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        special_n_change_motion(fighter, Hash40::new("special_n1"));
    }
    else if fighter.is_situation(*SITUATION_KIND_AIR) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        special_n_change_motion(fighter, Hash40::new("special_air_n1"));
    }
    // don't ask why I have to hardcode these
    VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_ANGLE, 45.0);
    VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_SPEED, 2.25);

    fighter.main_shift(special_n1_main_loop)
}

unsafe extern "C" fn special_n1_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            special_n_change_motion(fighter, Hash40::new("special_n1"));
        }
        else if fighter.is_situation(*SITUATION_KIND_AIR) {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            special_n_change_motion(fighter, Hash40::new("special_air_n1"));
        }
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
    fighter.sub_air_check_dive();
    let charge = VarModule::get_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_CHARGE) as f32;
    let angle = 45.0 - charge * 0.75;
    if VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_START_HOLD) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::inc_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_CHARGE);
            if charge == 1.0 {
                MotionModule::set_rate(fighter.module_accessor, 0.5);
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 2, 12, -3, 0, 0, 0, 0.3, false, *EF_FLIP_AXIS_YZ);
            }
            if charge == 10.0 {
                fighter.change_motion_inherit_frame_keep_rate_by_situation("special_n1_bowl", "special_air_n1_bowl", -1.0, 1.0, 0.0);
            }
        }
        else {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            VarModule::on_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_END_HOLD);
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_END_HOLD) {
        VarModule::off_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_START_HOLD);
        VarModule::off_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_END_HOLD);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        let bowl_speed_ground = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "special_n1.bowl_speed_ground");
        let throw_speed = if fighter.is_situation(*SITUATION_KIND_GROUND) { bowl_speed_ground } else { bowl_speed_ground - (charge * 0.025) };   // 2.25-2.75
        VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_ANGLE, angle);
        VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_SPEED, throw_speed);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT } else { FIGHTER_STATUS_KIND_FALL };
        fighter.change_status(status.into(), false.into());
    }

    return 0.into();
}

unsafe fn special_n_change_motion(fighter: &mut L2CFighterCommon, motion: Hash40) {
    if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_IRONBALL_FLAG_FIRST) {
        let _motion = if fighter.is_motion(Hash40::new("special_n1_bowl")) {
            let bowl_speed_air = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "special_n1.bowl_speed_air");
            VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_SPEED, bowl_speed_air);
            Hash40::new("special_air_n1_bowl")
        }
        else if fighter.is_motion(Hash40::new("special_air_n1_bowl")) {
            let bowl_speed_ground = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "special_n1.bowl_speed_ground");
            VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_SPEED, bowl_speed_ground);
            Hash40::new("special_n1_bowl")
        }
        else { motion };
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, _motion, -1.0, 1.0, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_IRONBALL_FLAG_FIRST);
    }
}

pub unsafe extern "C" fn special_n1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MIIFIGHTER_GENERATE_ARTICLE_IRONBALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    
    return 0.into();
}