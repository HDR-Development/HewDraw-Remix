use super::*;

pub unsafe extern "C" fn special_lw3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn special_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_MIISWORDSMAN_STATUS_RAPID_SLASH_FLAG_CHARGE_MAX);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_03) - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_03) - 1);
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    fighter.change_motion_by_situation("special_n3_start", "special_air_n3_start", 0.0, 1.0, false, 0.0, false, false);

    fighter.main_shift(special_lw3_main_loop)
}

pub unsafe extern "C" fn special_lw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_motion_inherit_frame_by_situation("special_n3_start", "special_air_n3_start", -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            let turn_stick_x = fighter.get_param_float("common", "turn_stick_x");
            if fighter.stick_x() <= turn_stick_x {
                fighter.on_flag(*FIGHTER_MIISWORDSMAN_STATUS_RAPID_SLASH_FLAG_TURN);
            }
            fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_LOOP.into(), true.into());
        }
    }

    return 0.into();
}

pub unsafe extern "C" fn special_lw3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn special_lw3_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    fighter.change_motion_by_situation("special_n3_end", "special_air_n3_end", 0.0, 1.0, false, 0.0, false, false);

    fighter.main_shift(special_lw3_end_main_loop)
}

unsafe extern "C" fn special_lw3_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let (ground_motion, air_motion) = if fighter.is_motion_one_of(&[Hash40::new("special_n3_end"), Hash40::new("special_air_n3_end")])
            { ("special_n3_end", "special_air_n3_end") } else { ("special_lw3_end2", "special_air_lw3_end2") };
        fighter.change_motion_inherit_frame_by_situation(ground_motion, air_motion, -1.0, 1.0, 0.0, false, false);
        if fighter.is_flag(*FIGHTER_MIISWORDSMAN_STATUS_RAPID_SLASH_FLAG_TURN) {
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.0);
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW3_CHECK_INPUT)
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW3_CHECK_INPUT);
        fighter.change_motion_by_situation("special_lw3_end2", "special_air_lw3_end2", 0.0, 1.0, false, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }
    if fighter.is_motion_one_of(&[Hash40::new("special_lw3_end2"), Hash40::new("special_air_lw3_end2")]) {
        if let Some(func_ptr) = smashline::api::get_target_function("lua2cpp_miiswordsman.nrs", 0x3a8e0) {
            let apply_charge_muls: fn(&mut L2CFighterCommon) = std::mem::transmute(func_ptr);
            apply_charge_muls(fighter);
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_lw3_end_max_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_motion_by_situation("special_n3_end_max", "special_air_n3_end_max", 0.0, 1.0, false, 0.0, false, false);

    fighter.main_shift(special_lw3_end_main_loop)
}

unsafe extern "C" fn special_lw3_end_max_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let (ground_motion, air_motion) = if fighter.is_motion_one_of(&[Hash40::new("special_n3_end_max"), Hash40::new("special_air_n3_end_max")])
            { ("special_n3_end_max", "special_air_n3_end_max") } else { ("special_lw3_end2_max", "special_air_lw3_end2_max") };
        fighter.change_motion_inherit_frame_by_situation(ground_motion, air_motion, -1.0, 1.0, 0.0, false, false);
        if fighter.is_flag(*FIGHTER_MIISWORDSMAN_STATUS_RAPID_SLASH_FLAG_TURN) {
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.0);
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW3_CHECK_INPUT)
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW3_CHECK_INPUT);
        fighter.change_motion_by_situation("special_lw3_end2_max", "special_air_lw3_end2_max", 0.0, 1.0, false, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }
    if fighter.is_motion_one_of(&[Hash40::new("special_lw3_end2_max"), Hash40::new("special_air_lw3_end2_max")]) {
        if let Some(func_ptr) = smashline::api::get_target_function("lua2cpp_miiswordsman.nrs", 0x3a8e0) {
            let apply_charge_muls: fn(&mut L2CFighterCommon) = std::mem::transmute(func_ptr);
            apply_charge_muls(fighter);
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END, special_lw3_end_main);
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END_MAX, special_lw3_end_max_main);
}