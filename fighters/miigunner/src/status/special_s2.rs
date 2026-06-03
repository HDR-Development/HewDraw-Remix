use super::*;

pub unsafe extern "C" fn special_s2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let s2_stop_y = fighter.get_param_int("param_special_s", "s2_stop_y");
    fighter.set_int(s2_stop_y, *FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_WORK_INT_STOP_Y);
    let s2_no_bang = fighter.get_param_int("param_special_s", "s2_no_bang");
    fighter.set_int(s2_no_bang, *FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_WORK_INT_NO_BANG);
    fighter.off_flag(*FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_FLAG_MOT_CHANGE);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_special_s2(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_s2 as *const () as _));
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_05) - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_05) - 1);

    fighter.main_shift(special_s2_main_loop)
}

unsafe extern "C" fn sub_special_s2(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        fighter.dec_int(*FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_WORK_INT_STOP_Y);
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB) {
            fighter.dec_int(*FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_WORK_INT_NO_BANG);
        }
    }
    else {
        let stop_y = fighter.get_int(*FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_WORK_INT_STOP_Y);
        if stop_y >= 0 {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_FLAG_1) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB, false, -1);
            fighter.off_flag(*FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_FLAG_1);
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_s2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_FLAG_MOT_CHANGE) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_start"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_start"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_FLAG_MOT_CHANGE);
            }
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MIIGUNNER_SPECIAL_S2_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_FLAG_MOT_CHANGE) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_start"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_start"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_FLAG_MOT_CHANGE);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S2_LOOP.into(), false.into());
        return 0.into();
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        let no_bang = fighter.get_int(*FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_WORK_INT_NO_BANG);
        if no_bang <= 0 && ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB) {
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S2_END.into(), true.into());
        }
    }

    return 0.into();
}

pub unsafe extern "C" fn special_s2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_next = StatusModule::status_kind_next(fighter.module_accessor);
    if ![*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S2_LOOP, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S2_END].contains(&status_next) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB, ArticleOperationTarget(0));
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S2_LOOP, special_s2_end);
}