use super::*;

unsafe extern "C" fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int(-1, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
    fighter.sub_change_motion_by_situation(Hash40::new("special_n_hold").into(), Hash40::new("special_air_n_hold").into(), false.into());
    let hold_frame_l = fighter.get_param_int("param_special_n", "hold_frame_l");
    let hold_frame = fighter.get_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    let rate = MotionModule::end_frame(fighter.module_accessor) / hold_frame_l as f32;
    MotionModule::set_rate(fighter.module_accessor, rate);
    MotionModule::set_frame(fighter.module_accessor, rate * hold_frame as f32, true);
    let hold_frame_m = fighter.get_param_int("param_special_n", "hold_frame_m");
    if hold_frame >= hold_frame_m {
        fighter.on_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_START_M);
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);

    fighter.main_shift(special_n_hold_main_loop)
}

unsafe extern "C" fn special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    fighter.sub_change_motion_by_situation(Hash40::new("special_n_hold").into(), Hash40::new("special_air_n_hold").into(), true.into());
    fighter.sub_exec_special_start_common_kinetic_setting(Hash40::new("special_n").into());
    if fighter.is_pad_flag(PadFlag::SpecialTrigger | PadFlag::AttackTrigger) {
        fighter.on_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_TO_SHOOT);
    }
    if fighter.is_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_TO_SHOOT) {
        fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        return 1.into();
    }
    let hold_frame_l = fighter.get_param_int("param_special_n", "hold_frame_l");
    let hold_frame = fighter.get_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    if hold_frame <= hold_frame_l {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
                fighter.set_int(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
                fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                return 1.into();
            }
            if fighter.sub_check_jump_in_charging().get_bool() {
                fighter.set_int(*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
                fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                return 1.into();
            }
        }
        else {
            if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
                fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
                fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                return 1.into();
            }
            if fighter.sub_check_jump_in_charging().get_bool() {
                fighter.set_int(*FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
                fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_JUMP_CANCEL.into(), false.into());
                return 1.into();
            }
        }
    }
    else {
        fighter.on_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_CANCEL_L);
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_specialnholdmax"), -1);
        app::FighterUtil::flash_eye_info(fighter.module_accessor);
        fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_HOLD, special_n_hold_main);
}