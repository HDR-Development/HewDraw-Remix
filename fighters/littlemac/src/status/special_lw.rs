use super::*;
use globals::*;

unsafe extern "C" fn special_lw_old_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START);
    return 1.into()
}

unsafe extern "C" fn special_lw_hit_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into()
}

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
    let sum_spd_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);    //l70
    let sum_spd_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);    //l80
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let charge_motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_max_charge_motion_rate_"));
    MotionModule::change_motion(
        fighter.module_accessor,
        if fighter.is_situation(*SITUATION_KIND_GROUND) { Hash40::new("special_n_start") } else { Hash40::new("special_air_n_start") },
        0.0, charge_motion_rate, false, 0.0, false, false
    );
    let reset_type = if fighter.is_situation(*SITUATION_KIND_GROUND) { ENERGY_STOP_RESET_TYPE_GROUND } else { ENERGY_STOP_RESET_TYPE_AIR };
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, reset_type, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_spd_x, 0.0);
    let facing = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -0.01 * facing, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, sum_spd_y, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    let reac_mul_power = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), 0x22bac6d06f);
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode { _address: *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER as u8 }, -1.0, reac_mul_power, -1);
    if !StopModule::is_stop(fighter.module_accessor) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_INT_CHARGE_FRAME);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_lw_substatus as *const () as _));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);

    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if (fighter.is_prev_situation(*SITUATION_KIND_GROUND) && fighter.is_situation(*SITUATION_KIND_AIR))
    || (fighter.is_prev_situation(*SITUATION_KIND_AIR) && fighter.is_situation(*SITUATION_KIND_GROUND)) {
        KineticModule::clear_speed_all(fighter.module_accessor);
    }
    let max_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_max_charge_frame"));
    let frame = MotionModule::frame(fighter.module_accessor);
    let a0 = (1.0 / max_charge_frame as f32);
    WorkModule::set_float(fighter.module_accessor, a0, *FIGHTER_LITTLEMAC_STATUS_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_CHECK_DASH) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_DASH_RESERVE);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_CHECK_DASH);
        }
    }
    if MotionModule::frame(fighter.module_accessor) <= max_charge_frame as f32 - 1.0 {
        if (fighter.global_table[CURRENT_FRAME].get_f32() < max_charge_frame as f32 - 1.0) {
            if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
                if fighter.is_situation(*SITUATION_KIND_GROUND) {
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0);
                    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
                    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                }
            }
            else {
                if fighter.is_situation(*SITUATION_KIND_AIR) {
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0);
                    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                }
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_DASH_RESERVE) {
                let active_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_active_start_frame"));
                if active_start_frame <= MotionModule::frame(fighter.module_accessor) {
                    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode { _address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8 }, -1.0, -1.0, -1);
                    let facing = PostureModule::lr(fighter.module_accessor);
                    let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
                    if fighter.stick_x() * facing > turn_stick_x {
                        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH.into(), false.into());
                    }
                    else {
                        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH_TURN.into(), false.into());
                    }
                    return 1.into()
                }
            }
            if fighter.status_frame() >= 10 {
                if fighter.is_situation(*SITUATION_KIND_GROUND) {
                    if fighter.is_cat_flag(Cat2::StickEscape) {
                        VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_LW_CANCEL_TYPE, vars::littlemac::SPECIAL_LW_CANCEL_TYPE_ESCAPE);
                        fighter.change_status(statuses::littlemac::SPECIAL_LW_CANCEL.into(), true.into());
                    }
                    else if fighter.is_cat_flag(Cat2::StickEscapeF) {
                        VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_LW_CANCEL_TYPE, vars::littlemac::SPECIAL_LW_CANCEL_TYPE_ESCAPE_F);
                        fighter.change_status(statuses::littlemac::SPECIAL_LW_CANCEL.into(), true.into());
                    }
                    else if fighter.is_cat_flag(Cat2::StickEscapeB) {
                        VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_LW_CANCEL_TYPE, vars::littlemac::SPECIAL_LW_CANCEL_TYPE_ESCAPE_B);
                        fighter.change_status(statuses::littlemac::SPECIAL_LW_CANCEL.into(), true.into());
                    }
                    else if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump) && fighter.sub_check_button_frick().get_bool())) {
                        VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_LW_CANCEL_TYPE, vars::littlemac::SPECIAL_LW_CANCEL_TYPE_GROUND_JUMP);
                        fighter.change_status(statuses::littlemac::SPECIAL_LW_CANCEL.into(), true.into());
                    }
                    if fighter.sub_check_command_guard().get_bool() {
                        VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_LW_CANCEL_TYPE, vars::littlemac::SPECIAL_LW_CANCEL_TYPE_GUARD);
                        fighter.change_status(statuses::littlemac::SPECIAL_LW_CANCEL.into(), true.into());
                        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
                    }
                }
                else if fighter.is_situation(*SITUATION_KIND_AIR) {
                    if fighter.is_cat_flag(Cat1::AirEscape)  {
                        VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_LW_CANCEL_TYPE, vars::littlemac::SPECIAL_LW_CANCEL_TYPE_ESCAPE_AIR);
                        fighter.change_status(statuses::littlemac::SPECIAL_LW_CANCEL.into(), true.into());
                        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
                    }
                    else if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump)))
                    && fighter.get_num_used_jumps() < fighter.get_jump_count_max()
                    {
                        VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_LW_CANCEL_TYPE, vars::littlemac::SPECIAL_LW_CANCEL_TYPE_JUMP_AERIAL);
                        fighter.change_status(statuses::littlemac::SPECIAL_LW_CANCEL_JUMP.into(), true.into());
                    }
                }
            }
        }
    }
    else {
        let facing = PostureModule::lr(fighter.module_accessor);
        let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x")); //l80
        if fighter.stick_x() * facing > turn_stick_x {
            fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_MAX_DASH.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_MAX_DASH_TURN.into(), false.into());
        }
        return 1.into()
    }

    return 0.into()
}

unsafe extern "C" fn special_lw_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_INT_CHARGE_FRAME);
    }
    return 0.into()
}

pub fn install() {
    smashline::Agent::new("littlemac")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_old_pre)
        .status(Init, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT, special_lw_hit_init)
        .status(Pre, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START, special_lw_pre)
        .status(Main, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START, special_lw_main)
        .install();
}