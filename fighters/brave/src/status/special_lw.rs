use super::*;

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::SPECIAL_LW_CSTICK_BUFFER);
    VarModule::set_float(fighter.battle_object, vars::brave::instance::SPECIAL_LW_CSTICK_BUFFER_DIR, 0.0);
    let brave = fighter.global_table[0x4].get_ptr() as *mut Fighter;
    if fighter.get_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_WINDOW_STATE) == *FIGHTER_BRAVE_COMMAND_WINDOW_STATE_CLOSE {
        FighterSpecializer_Brave::special_lw_close_window(brave, true, false, false);
    }
    FighterSpecializer_Brave::special_lw_open_command(brave);
    FighterSpecializer_Brave::special_lw_start_select_command(brave);
    fighter.sub_change_motion_by_situation(Hash40::new("special_lw_start").into(), Hash40::new("special_air_lw_start").into(), false.into());
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_lw").into());
    fighter.on_flag(*FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_DISABLE_SP_AUTO_RECOVER);

    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT.into(), true.into());
        return 1.into();
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        return 0.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_motion_inherit_frame_by_situation("special_lw_start", "special_air_lw_start", -1.0, 1.0, 0.0, false, false);
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    if !fighter.is_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_DECIDE) {
        if fighter.is_pad_flag(PadFlag::AttackTrigger)
        || fighter.is_pad_flag(PadFlag::SpecialTrigger) {
            // check if the selection was performed with the cstick and get the direction
            let cstick_x = ControlModule::get_stick_x(fighter.module_accessor);
            if cstick_x.abs() > 0.2 {
                VarModule::on_flag(fighter.battle_object, vars::brave::instance::SPECIAL_LW_CSTICK_BUFFER);
                VarModule::set_float(fighter.battle_object, vars::brave::instance::SPECIAL_LW_CSTICK_BUFFER_DIR, cstick_x.signum());
            }
            let brave = fighter.global_table[0x4].get_ptr() as *mut Fighter;
            FighterSpecializer_Brave::special_lw_cursor_decide(brave);
            fighter.off_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_GUARD_CANCEL);
            fighter.off_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_JUMP_CANCEL);
        }
        if !fighter.is_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_DECIDE) {
            special_lw_set_index(fighter);
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                fighter.on_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_GUARD_CANCEL);
            }
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                fighter.on_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_JUMP_CANCEL);
            }
        }
    }

    return 0.into();
}

// FUN_7100027810
unsafe extern "C" fn special_lw_set_index(fighter: &mut L2CFighterCommon) {
    if FighterControlModuleImpl::get_stick_button_trigger(fighter.module_accessor, *FIGHTER_CONTROLLER_STICK_BUTTON_UP) == 0
    && FighterControlModuleImpl::get_stick_button_trigger(fighter.module_accessor, *FIGHTER_CONTROLLER_STICK_BUTTON_DOWN) == 0 {
        return;
    }
    if FighterControlModuleImpl::get_stick_button_trigger(fighter.module_accessor, *FIGHTER_CONTROLLER_STICK_BUTTON_UP) == 1 {
        fighter.dec_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
        if fighter.get_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX) < 0 {
            if FighterControlModuleImpl::get_stick_button_trigger(fighter.module_accessor, *FIGHTER_CONTROLLER_STICK_BUTTON_UP) == 1 {
                fighter.set_int(*FIGHTER_BRAVE_SPECIAL_LW_COMMAND_LIST_MAX - 1, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
            }
            else {
                fighter.set_int(0, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
            }
        }
    }
    else {
        // if FighterControlModuleImpl::get_stick_button_repeat(fighter.module_accessor, *FIGHTER_CONTROLLER_STICK_BUTTON_UP) == 1 {
        //     fighter.dec_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
        //     if fighter.get_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX) < 0 {
        //         if FighterControlModuleImpl::get_stick_button_trigger(fighter.module_accessor, *FIGHTER_CONTROLLER_STICK_BUTTON_UP) == 1 {
        //             fighter.set_int(*FIGHTER_BRAVE_SPECIAL_LW_COMMAND_LIST_MAX - 1, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
        //         }
        //         else {
        //             fighter.set_int(0, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
        //         }
        //     }
        // }
        if FighterControlModuleImpl::get_stick_button_trigger(fighter.module_accessor, *FIGHTER_CONTROLLER_STICK_BUTTON_DOWN) == 1 {
        //|| FighterControlModuleImpl::get_stick_button_repeat(fighter.module_accessor, *FIGHTER_CONTROLLER_STICK_BUTTON_DOWN) == 1 {
            fighter.inc_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
            if fighter.get_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX) > *FIGHTER_BRAVE_SPECIAL_LW_COMMAND_LIST_MAX - 1 {
                if FighterControlModuleImpl::get_stick_button_trigger(fighter.module_accessor, *FIGHTER_CONTROLLER_STICK_BUTTON_DOWN) == 1 {
                    fighter.set_int(0, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
                }
                else {
                    fighter.set_int(*FIGHTER_BRAVE_SPECIAL_LW_COMMAND_LIST_MAX - 1, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
                }
            }
        }
    }
    let brave = fighter.global_table[0x4].get_ptr() as *mut Fighter;
    let select_index = fighter.get_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    FighterSpecializer_Brave::special_lw_select_index(brave, select_index);
}

unsafe extern "C" fn special_lw_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Pre, fighter, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START)(fighter);
    let mp = fighter.get_float(*FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP);
    VarModule::set_float(fighter.battle_object, vars::brave::status::SPECIAL_MENU_MP, mp);
    let mut start_turn = *FIGHTER_STATUS_ATTR_START_TURN as u32;
    let facing = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.battle_object, vars::brave::instance::SPECIAL_LW_CSTICK_BUFFER) {
        if facing.signum() != VarModule::get_float(fighter.battle_object, vars::brave::instance::SPECIAL_LW_CSTICK_BUFFER_DIR).signum() {
            start_turn = 0;
            PostureModule::reverse_lr(fighter.module_accessor);
        }
        VarModule::off_flag(fighter.battle_object, vars::brave::instance::SPECIAL_LW_CSTICK_BUFFER);
        VarModule::set_float(fighter.battle_object, vars::brave::instance::SPECIAL_LW_CSTICK_BUFFER_DIR, 0.0);
    }
    else {
        let c_stick_override = fighter.is_button_on(Buttons::CStickOverride);
        let cstick = if c_stick_override {
            ControlModule::get_stick_x(fighter.module_accessor)
        } else {
            ControlModule::get_sub_stick_x(fighter.module_accessor)
        };
        if cstick.abs() > 0.2 && facing.signum() != cstick.signum() {
            start_turn = 0;
            PostureModule::reverse_lr(fighter.module_accessor);
        }
    }
    let spell_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_DECIDE_COMMAND);
    let various_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_VARIOUS_KIND);
    let mask = if let Some(target) = smashline::api::get_target_function("lua2cpp_brave.nrs", 0x0398f0) {
        let get_special_lw_mask: fn(&mut L2CValue, L2CValue, L2CValue) = std::mem::transmute(target);
        let mask_l2c = &mut L2CValue::U64(0);
        get_special_lw_mask(mask_l2c, spell_kind.into(), various_kind.into());
        mask_l2c.get_u64()
    }
    else {
        0
    };
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask,
        start_turn,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    ret
}

unsafe extern "C" fn special_lw_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START)(fighter);
    // refund MP during a special roll
    if VarModule::is_flag(fighter.battle_object, vars::brave::instance::SPECIAL_MENU) {
        let mp = VarModule::get_float(fighter.battle_object, vars::brave::status::SPECIAL_MENU_MP);
        let mut brave_fighter = app::Fighter{battle_object: *(fighter.battle_object)};
        FighterSpecializer_Brave::set_sp(&mut brave_fighter, mp, false);
        VarModule::off_flag(fighter.battle_object, vars::brave::instance::SPECIAL_MENU);
    }

    ret
}

unsafe extern "C" fn special_lw_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let brave = fighter.global_table[0x4].get_ptr() as *mut Fighter;
    FighterSpecializer_Brave::special_lw_close_window(brave, true, false, false);
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::PERSIST_RNG);
    fighter.set_int(0, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    VarModule::set_int(fighter.battle_object, vars::brave::instance::CURSOR_SLOT, 0);
    return smashline::original_status(End, fighter, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START)(fighter);
}

unsafe extern "C" fn special_lw_steel_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let facing = PostureModule::lr(fighter.module_accessor);
    let c_stick_override = fighter.is_button_on(Buttons::CStickOverride);
    let cstick = if c_stick_override {
        ControlModule::get_stick_x(fighter.module_accessor)
    } else {
        ControlModule::get_sub_stick_x(fighter.module_accessor)
    };
    if cstick.abs() > 0.2 && facing.signum() != cstick.signum() {
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    StatusModule::init_settings(fighter.module_accessor,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    let mp = fighter.get_float(*FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP);
    VarModule::set_float(fighter.battle_object, vars::brave::status::SPECIAL_MENU_MP, mp);

    return 0.into()
}

unsafe extern "C" fn special_lw_steel_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL_START)(fighter);
    // refund MP during a special roll
    if VarModule::is_flag(fighter.battle_object, vars::brave::instance::SPECIAL_MENU) {
        let mp = VarModule::get_float(fighter.battle_object, vars::brave::status::SPECIAL_MENU_MP);
        let mut brave_fighter = app::Fighter{battle_object: *(fighter.battle_object)};
        FighterSpecializer_Brave::set_sp(&mut brave_fighter, mp, false);
        VarModule::off_flag(fighter.battle_object, vars::brave::instance::SPECIAL_MENU);
    }

    ret
}

unsafe extern "C" fn special_lw_failure_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);

    agent.status(Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, special_lw_start_pre);
    agent.status(Main, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, special_lw_start_main);
    agent.status(End, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, special_lw_start_end);

    agent.status(Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL_START, special_lw_steel_start_pre);
    agent.status(Main, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL_START, special_lw_steel_start_main);

    agent.status(Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FAILURE, special_lw_failure_pre);
}