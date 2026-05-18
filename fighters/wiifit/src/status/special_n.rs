use super::*;

unsafe extern "C" fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    fighter.sub_change_motion_by_situation(Hash40::new("special_n_hold").into(), Hash40::new("special_air_n_hold").into(), false.into());
    let charge_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), 0x1b44a04660);
    MotionModule::set_rate(fighter.module_accessor, charge_speed_mul * 3.0);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_SUNBULLET, false, -1);
    fighter.on_flag(*FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOLD_TO_SHOOT);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x254dd9906d));
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);

    fighter.main_shift(special_n_hold_main_loop)
}

unsafe extern "C" fn special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_WIIFIT_STATUS_SPECIAL_N_FLAG_SUN_BULLET_ABSORBED) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }
    let frame = MotionModule::frame(fighter.module_accessor);
    let ratio = fighter.get_float(*FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO);
    let end_frame_ratio = ratio * MotionModule::end_frame(fighter.module_accessor);
    let charge_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), 0x1b44a04660);
    let mut reapply_charge_ratio = false;
    if end_frame_ratio <= frame {
        MotionModule::set_rate(fighter.module_accessor, charge_speed_mul);
        reapply_charge_ratio = true;
    }
    let mut some_float_ass_float = 3.0;
    if reapply_charge_ratio {
        some_float_ass_float = 1.0;
    }
    else {
        if end_frame_ratio < frame + some_float_ass_float {
            MotionModule::set_rate(fighter.module_accessor, frame + some_float_ass_float - end_frame_ratio);
        }
    }
    some_float_ass_float *= charge_speed_mul;
    if reapply_charge_ratio {
        let charge_level_ratio = frame / MotionModule::end_frame(fighter.module_accessor);
        fighter.set_float(charge_level_ratio, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO);
        if 1.0 <= charge_level_ratio {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_wiifit_whistle"), true, false, false, false, enSEType(0));
            SoundModule::set_play_inhivit(fighter.module_accessor, Hash40::new("se_wiifit_whistle"), 0x1e);
            FighterUtil::flash_eye_info(fighter.module_accessor);
            fighter.change_status(FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_END.into(), false.into());
            return 0.into();
        }
    }
    if fighter.is_pad_flag(PadFlag::SpecialTrigger | PadFlag::AttackTrigger) {
        fighter.change_status(FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_SHOOT.into(), true.into());
        return 1.into();
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.sub_check_jump_in_charging().get_bool() {
            fighter.set_int(*FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
            return 1.into();
        }
        if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
            fighter.set_int(*FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
            return 1.into();
        }
    }
    else {
        if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
            fighter.set_int(*FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
            return 1.into();
        }
        if fighter.sub_check_jump_in_charging().get_bool() {
            fighter.set_int(*FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_AIR_JUMP_AERIAL, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_JUMP_CANCEL.into(), true.into());
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        fighter.change_motion_inherit_frame_by_situation("special_n_hold", "special_air_n_hold", -1.0, 1.0, 0.0, false, false);
    }
    if !fighter.global_table[IS_STOPPING].get_bool() {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);  //l50
            let energy_type = if fighter.is_situation(*SITUATION_KIND_GROUND) { ENERGY_STOP_RESET_TYPE_GROUND } else { ENERGY_STOP_RESET_TYPE_AIR };
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, energy_type, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_HOLD, special_n_hold_main);
}