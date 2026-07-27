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
    VarModule::set_int(fighter.battle_object, vars::brave::instance::MENU_ICON_EFFECT_HANDLE, -1);
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
            let handle = VarModule::get_int(fighter.battle_object, vars::brave::instance::MENU_ICON_EFFECT_HANDLE);
            set_icon_wobble(fighter, handle);
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
unsafe extern "C" fn special_lw_set_index(fighter: &mut L2CFighterCommon) -> bool {
    if FighterControlModuleImpl::get_stick_button_trigger(fighter.module_accessor, *FIGHTER_CONTROLLER_STICK_BUTTON_UP) == 0
    && FighterControlModuleImpl::get_stick_button_trigger(fighter.module_accessor, *FIGHTER_CONTROLLER_STICK_BUTTON_DOWN) == 0 {
        return false;
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
    let command = FighterSpecializer_Brave::get_special_lw_command_from_index(brave, select_index);
    set_command_overhead_effect(fighter, command);

    return true;
}

unsafe extern "C" fn set_command_overhead_effect(fighter: &mut L2CFighterCommon, command: i32) {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("brave_command_attack"), false, false);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("brave_command_magic"), false, false);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("brave_command_support"), false, false);
    let base_hash = "brave_command_";
    let eff_hash = match command {
        0 => format!("{}{}", base_hash, "support"),     // Heal
        1 => format!("{}{}", base_hash, "attack"),      // Sizz
        2 => format!("{}{}", base_hash, "attack"),      // Sizzle
        3 => format!("{}{}", base_hash, "attack"),      // Bang
        4 => format!("{}{}", base_hash, "attack"),      // Kaboom
        5 => format!("{}{}", base_hash, "magic"),       // Whack
        6 => format!("{}{}", base_hash, "magic"),       // Thwack
        7 => format!("{}{}", base_hash, "magic"),       // Magic Burst
        8 => format!("{}{}", base_hash, "magic"),       // Kamikazee
        9 => format!("{}{}", base_hash, "magic"),       // Kaclang
        10 => format!("{}{}", base_hash, "support"),    // Acceleratle
        11 => format!("{}{}", base_hash, "support"),    // Oomph
        12 => format!("{}{}", base_hash, "support"),    // Bounce
        13 => format!("{}{}", base_hash, "magic"),      // Snooze
        14 => format!("{}{}", base_hash, "support"),    // Hocus Pocus
        15 => format!("{}{}", base_hash, "nozoom"),     // Zoom (unused)
        16 => format!("{}{}", base_hash, "attack"),     // Flame Slash
        17 => format!("{}{}", base_hash, "attack"),     // Kacrackle Slash
        18 => format!("{}{}", base_hash, "magic"),      // Metal Slash
        19 => format!("{}{}", base_hash, "attack"),     // Hatchet Man
        20 => format!("{}{}", base_hash, "support"),    // Psyche Up
        _ => format!("{}{}", base_hash, "attack")
    };
    let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new(eff_hash.as_str()), Hash40::new("top"), &Vector3f::new(0.0, 20.0, 2.0), &Vector3f::zero(), 0.35, false, 0, 0, 0, 0, 0, false, false);
    EffectModule::set_rate(fighter.module_accessor, handle as u32, 0.000001);
    EffectModule::set_alpha(fighter.module_accessor, handle as u32, 2.0);
    VarModule::set_int(fighter.battle_object, vars::brave::instance::MENU_ICON_EFFECT_HANDLE, handle as i32);
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

unsafe extern "C" fn special_lw_select_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int(0, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_SELECT_INPUT_COUNT);
    fighter.set_int(-1, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS);
    fighter.change_motion_by_situation("special_lw_select", "special_air_lw_select", 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_select") as i64, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_SELECT_MOTION);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_select") as i64, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_SELECT_MOTION_AIR);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);

    fighter.main_shift(special_lw_select_main_loop)
}

unsafe extern "C" fn special_lw_select_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_AUTO_CANCEL) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.set_int(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS);
            fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CANCEL.into(), false.into());
        }
        else {
            fighter.set_int(*FIGHTER_STATUS_KIND_FALL, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS);
            fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CANCEL.into(), false.into());
        }
        return 1.into();
    }
    if fighter.is_pad_flag(PadFlag::AttackTrigger) || fighter.is_pad_flag(PadFlag::SpecialTrigger)
    || fighter.is_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_DECIDE) {
        let brave = fighter.global_table[0x4].get_ptr() as *mut Fighter;
        if !fighter.is_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_DECIDE) {
            FighterSpecializer_Brave::special_lw_cursor_decide(brave);
        }
        let select_index = fighter.get_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
        let command = FighterSpecializer_Brave::get_special_lw_command_from_index(brave, select_index);
        let sp_cost = FighterSpecializer_Brave::get_special_lw_command_sp_cost(fighter.module_accessor, FighterBraveSpecialLwCommand{ _address: command as u8 }, true);
        if brave_special_check_sp_set_flag(fighter, sp_cost, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_SUCCESS_SP) {
            FighterSpecializer_Brave::special_lw_decide_command(brave, FighterBraveSpecialLwCommand{ _address: command as u8 }, select_index);
            fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FAILURE.into(), true.into());
        }
        return 1.into();
    }
    if fighter.is_pad_flag(PadFlag::GuardTrigger)
    || fighter.is_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_GUARD_CANCEL) {
        fighter.off_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_GUARD_CANCEL);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.set_int(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS);
            fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CANCEL.into(), false.into());
        }
        else {
            if !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                fighter.set_int(*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS);
                fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CANCEL.into(), false.into());
            }
        }

        return 1.into();
    }
    if fighter.is_cat_flag(Cat1::JumpButton)
    || fighter.is_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_JUMP_CANCEL) {
        fighter.off_flag(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_JUMP_CANCEL);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON) {
                fighter.set_int(*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS);
                fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CANCEL.into(), false.into());
                return 1.into();
            }
        }
        else {
            if fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) {
                    fighter.set_int(*FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS);
                    fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CANCEL.into(), false.into());
                    return 1.into();
                }
            }
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_select") as i64, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_SELECT_MOTION);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_landing"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_SELECT_MOTION_AIR);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, true, 6.0, false, false);
        }
    }
    fighter.sub_exec_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());
    if special_lw_set_index(fighter) {
        if fighter.is_motion_one_of(&[Hash40::new("special_lw_select"), Hash40::new("special_air_lw_select")]) {
            if fighter.get_int(*FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_SELECT_INPUT_COUNT) <= 0 {
                if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_SELECT_MOTION) == hash40("special_lw_select") {
                    fighter.change_motion_by_situation("special_lw_select2", "special_air_lw_select2", 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_select2") as i64, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_SELECT_MOTION);
                    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_select2") as i64, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_SELECT_MOTION_AIR);
                }
            }
        }
    }
    let handle = VarModule::get_int(fighter.battle_object, vars::brave::instance::MENU_ICON_EFFECT_HANDLE);
    set_icon_wobble(fighter, handle);
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_select"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_select") as i64, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_SELECT_MOTION);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_select"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_select") as i64, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_SELECT_MOTION_AIR);
        }
    }

    return 0.into();
}

unsafe extern "C" fn brave_special_check_sp_set_flag(fighter: &mut L2CFighterCommon, sp_cost: f32, success: i32) -> bool {
    let sp = fighter.get_float(*FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP);
    let max_sp = fighter.get_float(*FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_MAX_SP);
    if sp_cost <= sp {
        fighter.on_flag(success);
        return true;
    }
    fighter.off_flag(success);
    return false;
}

unsafe extern "C" fn set_icon_wobble(fighter: &mut L2CFighterCommon, handle: i32) {
    if handle == -1 { return; }
    let frame = fighter.status_frame();
    let facing = fighter.lr();
    // oscillate up and down slightly
    let offset = ((frame as f32 * 0.125) / std::f32::consts::PI).cos();
    EffectModule::set_pos(fighter.module_accessor, handle as u32, &Vector3f::new(5.0 * facing, 20.0 + offset, 2.0));
}

unsafe extern "C" fn special_lw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let interrupt = StatusModule::status_kind_next(fighter.module_accessor);
    if interrupt == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT {
        return 0.into();
    }

    let handle = VarModule::get_int(fighter.battle_object, vars::brave::instance::MENU_ICON_EFFECT_HANDLE);
    if interrupt == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START
    || interrupt == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL_START {
        EffectModule::set_rate(fighter.module_accessor, handle as u32, 1.0);
    }
    else {
        EffectModule::kill(fighter.module_accessor, handle as u32, true, true);
    }
    VarModule::set_int(fighter.battle_object, vars::brave::instance::MENU_ICON_EFFECT_HANDLE, -1);

    return 0.into();
}

unsafe extern "C" fn special_lw_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START)(fighter);
    // persist rng
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::PERSIST_RNG);
    fighter.set_int(0, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    VarModule::set_int(fighter.battle_object, vars::brave::instance::CURSOR_SLOT, 0);
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
    // persist rng
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::PERSIST_RNG);
    fighter.set_int(0, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    VarModule::set_int(fighter.battle_object, vars::brave::instance::CURSOR_SLOT, 0);
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
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_exit);

    agent.status(Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, special_lw_start_pre);
    agent.status(Main, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, special_lw_start_main);
    agent.status(End, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, special_lw_start_end);

    agent.status(Main, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT, special_lw_select_main);
    agent.status(Exit, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT, special_lw_exit);

    agent.status(Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL_START, special_lw_steel_start_pre);
    agent.status(Main, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL_START, special_lw_steel_start_main);

    agent.status(Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FAILURE, special_lw_failure_pre);
}
