use super::*;

const SPECIAL_N2_CANCEL_TYPE_NONE: i32 = 0x0;
const SPECIAL_N2_CANCEL_TYPE_GROUND_JUMP: i32 = 0x1;
const SPECIAL_N2_CANCEL_TYPE_JUMP_AERIAL: i32 = 0x2;

unsafe extern "C" fn special_n2_common_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_n2_common_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let next_status = StatusModule::status_kind_next(fighter.module_accessor);
    let death_statuses =
        &[*FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY];
    let damage_statuses =
        &[*FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL];
    if death_statuses.contains(&next_status) || damage_statuses.contains(&next_status) {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT, 0);
    }
    SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_miifighter_special_n2_charge"), 0);

    return 0.into();
}

pub unsafe extern "C" fn special_n2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn special_n2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "special_n2.charge_frame");
    if VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT) >= charge_frame {
        fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH.into(), false.into());
        return 1.into();
    }
    special_n2_change_motion(fighter, Hash40::new("special_n2_start"), Hash40::new("special_air_n2_start"), false);
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x * 0.75, 0.0); // parameterize

    fighter.main_shift(special_n2_main_loop)
}

unsafe extern "C" fn special_n2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n2_change_motion(fighter, Hash40::new("special_n2_start"), Hash40::new("special_air_n2_start"), false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(statuses::miifighter::SPECIAL_N2_HOLD.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

pub unsafe extern "C" fn special_n2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn special_n2_change_motion(fighter: &mut L2CFighterCommon, ground_motion: Hash40, air_motion: Hash40, inherit: bool) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        if inherit {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, ground_motion, -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, ground_motion, 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_situation(SITUATION_KIND_AIR.into());
        if inherit {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, air_motion, -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, air_motion, 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn special_n2_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_n2_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n2_change_motion(fighter, Hash40::new("special_n2_hold"), Hash40::new("special_air_n2_hold"), false);
    fighter.enable_transition_term_many(&[
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT,
    ]);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);

    fighter.main_shift(special_n2_hold_main_loop)
}

unsafe extern "C" fn special_n2_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n2_change_motion(fighter, Hash40::new("special_n2_hold"), Hash40::new("special_air_n2_hold"), true);
    }
    if fighter.is_pad_flag(PadFlag::SpecialTrigger) || fighter.is_pad_flag(PadFlag::AttackTrigger) {
        fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH.into(), true.into());
        return 1.into();
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.sub_check_jump_in_charging().get_bool() {
            VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N2_CANCEL_TYPE, SPECIAL_N2_CANCEL_TYPE_GROUND_JUMP);
            fighter.change_status(statuses::miifighter::SPECIAL_N2_CANCEL.into(), true.into());
            return 1.into();
        }
        if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
            VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N2_CANCEL_TYPE, SPECIAL_N2_CANCEL_TYPE_NONE);
            fighter.change_status(statuses::miifighter::SPECIAL_N2_CANCEL.into(), true.into());
            return 1.into();
        }
    }
    else {
        if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
            VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N2_CANCEL_TYPE, SPECIAL_N2_CANCEL_TYPE_NONE);
            fighter.change_status(statuses::miifighter::SPECIAL_N2_CANCEL.into(), true.into());
            return 1.into();
        }
        if fighter.get_num_used_jumps() < fighter.get_jump_count_max()
        && fighter.sub_check_jump_in_charging().get_bool() {
            VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N2_CANCEL_TYPE, SPECIAL_N2_CANCEL_TYPE_JUMP_AERIAL);
            fighter.change_status(statuses::miifighter::SPECIAL_N2_JUMP_CANCEL.into(), true.into());
            return 1.into();
        }
    }
    if !fighter.global_table[IS_STOPPING].get_bool() {
        VarModule::inc_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT);
        let count = VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT);
        let charge_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "special_n2.charge_frame");
        if charge_frame <= count {
            EffectModule::req_common(fighter.module_accessor, Hash40::new("charge_max"), 0.0);
            app::FighterUtil::flash_eye_info(fighter.module_accessor);
            VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT, count);
            fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_MISS.into(), false.into());
            return 1.into();
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_n2_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n2_change_motion(fighter, Hash40::new("special_n2_end"), Hash40::new("special_air_n2_end"), false);
    EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 2, 12, -3, 0, 0, 0, 0.5, false, *EF_FLIP_AXIS_YZ);
    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_miifighter_special_n2_ready"), true, false, false, false, app::enSEType(0));
    ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);

    fighter.main_shift(special_n2_end_main_loop)
}

unsafe extern "C" fn special_n2_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
          return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n2_change_motion(fighter, Hash40::new("special_n2_end"), Hash40::new("special_air_n2_end"), true);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }

    return 0.into();
}

pub unsafe extern "C" fn special_n2_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut start_attr = 0;
    let charge_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "special_n2.charge_frame");
    if VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT) >= charge_frame {
        fighter.sub_status_pre_SpecialNCommon();
        start_attr = *FIGHTER_STATUS_ATTR_START_TURN as u32;
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        start_attr,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_n2_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        fighter.set_situation(SITUATION_KIND_AIR.into());
    }
    let count = VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT);
    let charge_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "special_n2.charge_frame");
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n2_attack"), 0.0, 1.0, false, 0.0, false, false);
    if charge_frame > count {
        let motion_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "special_n2.min_motion_mul");
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, motion_mul);
    }
    else if fighter.is_situation(*SITUATION_KIND_AIR) {
        let motion_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "special_n2.air_motion_mul");
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, motion_mul);
    }
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02) + -1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02) + -1);
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));

    fighter.main_shift(special_n2_attack_main_loop)
}

unsafe extern "C" fn special_n2_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n2_landing"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_n2_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT, 0);
    return 0.into()
}

unsafe extern "C" fn special_n2_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n2_change_motion(fighter, Hash40::new("special_n2_end"), Hash40::new("special_air_n2_end"), false);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    
    fighter.main_shift(special_n2_cancel_main_loop)
}

unsafe extern "C" fn special_n2_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_check_charge_cancel_jump_mini_attack();
    fighter.sub_air_check_dive();
    let cancel_type = VarModule::get_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N2_CANCEL_TYPE);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if cancel_type == SPECIAL_N2_CANCEL_TYPE_GROUND_JUMP {
                FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
            }
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n2_change_motion(fighter, Hash40::new("special_n2_end"), Hash40::new("special_air_n2_end"), true);
        if !StatusModule::is_changing(fighter.module_accessor) {
            VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N2_CANCEL_TYPE, SPECIAL_N2_CANCEL_TYPE_NONE);
        }
    }
    let mut shift_cancel_status = false;
    if cancel_type != SPECIAL_N2_CANCEL_TYPE_NONE {
        if MotionModule::is_end(fighter.module_accessor)
        || CancelModule::is_enable_cancel(fighter.module_accessor) {
            shift_cancel_status = true;
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            shift_cancel_status = true;
        }
    }
    if shift_cancel_status {
        match VarModule::get_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N2_CANCEL_TYPE) {
            SPECIAL_N2_CANCEL_TYPE_GROUND_JUMP => fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), false.into()),
            SPECIAL_N2_CANCEL_TYPE_NONE => { fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false); },
            _ => {},
        }
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
          return 1.into();
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_n2_jump_cancel_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_n2_jump_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n2_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n2_end"), 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.main_shift(special_n2_jump_cancel_main_loop)
}

unsafe extern "C" fn special_n2_jump_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if VarModule::get_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N2_CANCEL_TYPE) == SPECIAL_N2_CANCEL_TYPE_JUMP_AERIAL {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_n_jump_cancel_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::miifighter::SPECIAL_N2_HOLD, special_n2_common_pre);
    agent.status(Main, statuses::miifighter::SPECIAL_N2_HOLD, special_n2_hold_main);
    agent.status(End, statuses::miifighter::SPECIAL_N2_HOLD, special_n2_common_end);

    agent.status(Pre, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_MISS, special_n2_common_pre);
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_MISS, special_n2_end_main);
    agent.status(End, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_MISS, special_n2_common_end);

    agent.status(Pre, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH, special_n2_attack_pre);
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH, special_n2_attack_main);
    agent.status(End, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH, special_n2_attack_end);

    agent.status(Pre, statuses::miifighter::SPECIAL_N2_CANCEL, special_n2_common_pre);
    agent.status(Main, statuses::miifighter::SPECIAL_N2_CANCEL, special_n2_cancel_main);
    agent.status(End, statuses::miifighter::SPECIAL_N2_CANCEL, special_n2_common_end);

    agent.status(Pre, statuses::miifighter::SPECIAL_N2_JUMP_CANCEL, special_n2_common_pre);
    agent.status(Main, statuses::miifighter::SPECIAL_N2_JUMP_CANCEL, special_n2_jump_cancel_main);
    agent.status(End, statuses::miifighter::SPECIAL_N2_JUMP_CANCEL, special_n2_common_end);
}