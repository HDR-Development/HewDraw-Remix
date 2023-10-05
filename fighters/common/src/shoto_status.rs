use super::*;
use globals::*;

// FIGHTER_STATUS_KIND_TURN_DASH //

/*pub unsafe fn fgc_pre_turn(fighter: &mut L2CFighterCommon) {
    app::FighterSpecializer_Dolly::update_opponent_lr_1on1(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN_DASH);
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr != 0.0 {
        if PostureModule::lr(fighter.module_accessor) == 0.0 {
            if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_TURN {
                StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_DASH_BACK);
                1.into();
            }
        }
    }
    fighter.status_pre_TurnDash();
    0.into()
}*/

#[no_mangle]
pub unsafe extern "Rust" fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("dash_b"), 0.0, 1.0, false, 0.0, false, false);
    fighter.status_DashCommon();
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
    GroundModule::set_reverse_direction(fighter.module_accessor, true, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(fgc_dashback_main_loop as *const () as _))
}

unsafe extern "C" fn fgc_dashback_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let run_accel_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_add"), 0);
    let run_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_mul"), 0);
    let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
    let dash_speed: f32 = WorkModule::get_param_float(fighter.module_accessor, hash40("dash_speed"), 0);
    let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);
    let stick_x = fighter.global_table[STICK_X].get_f32();

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), 1.into());
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool()
        && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
            sv_module_access::item(fighter.lua_state_agent);
            if fighter.pop_lua_stack(1).get_bool()
            && ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                return 1.into();
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE)
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() == false {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 1.into();
        }
    }
    // original
    // if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
    //     if WorkMoudle::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)
    //     && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F != 0 {
    //         fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
    //         return 1.into();
    //     }
    //     if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
    //         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE) {
    //             fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
    //             return 1.into();
    //         }
    //     }
    // }
    // new

    if fighter.sub_transition_group_check_ground_guard().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_special_command().get_bool()
    || fighter.sub_transition_group_check_ground_special().get_bool()
    || fighter.sub_transition_specialflag_hoist().get_bool()
    {
        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
        return L2CValue::I32(1);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4) {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool()
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S4_START.into(), true.into());
            VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4) {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool()
        && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
            if ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
                VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
                return 1.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), true.into());
                VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
                return 1.into();
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START)
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), true.into());
        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
        return 1.into();
    }

    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("pass_flick_y"));
    let fgc_dashback_pass_disable_frame = ParamModule::get_int(fighter.object(), ParamType::Common, "fgc_dashback_pass_disable_frame");
    if GroundModule::is_passable_ground(fighter.module_accessor)
    && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y
    && fighter.global_table[STICK_Y].get_f32() < pass_stick_y
    {
        if fighter.global_table[CURRENT_FRAME].get_i32() >= fgc_dashback_pass_disable_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
            return 1.into();
        }
    }

    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
        return true.into();
    }
    if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0)
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0)
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S)
        && (fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0
        || fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0))
    && {
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be));
        app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
        return 1.into();
    }
    if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME)
    && (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    || FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false)) {
        if fighter.global_table[CMD_CAT1].get_i32() & (
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
        ) != 0 {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY);
            sv_module_access::item(fighter.lua_state_agent);
            if fighter.pop_lua_stack(1).get_bool() {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH) {
                    if ItemModule::is_have_item(fighter.module_accessor, 0) == false {
                        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
                        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
                        return true.into();
                    }
                }
            }
        }
        if ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_LIGHT as u64
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM);
            sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
            VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
            return true.into();
        }
    }

    let is_dash_input: bool = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0;
    let is_backdash_input: bool = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0;

    if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_AFTER_DASH_TO_RUN_FRAME)
    && WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x")) <= fighter.global_table[STICK_X].get_f32() * -1.0 * PostureModule::lr(fighter.module_accessor)
    && !is_backdash_input {
        let kind;
        if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_DOLLY {
            kind = FIGHTER_DOLLY_STATUS_KIND_TURN_RUN_BACK;
        }
        else if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_DEMON {
            kind = FIGHTER_DEMON_STATUS_KIND_TURN_RUN_BACK;
        }
        else  {
            kind = FIGHTER_RYU_STATUS_KIND_TURN_RUN_BACK;
        }
        fighter.change_status(kind.into(), false.into());
        return 1.into();
    }
    if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_DASH_TO_RUN_FRAME) {
        VarModule::off_flag(fighter.battle_object, vars::common::status::IS_DASH_TO_RUN_FRAME);
        VarModule::on_flag(fighter.battle_object, vars::common::status::IS_AFTER_DASH_TO_RUN_FRAME);
    }

    // Disables dashbacks when stick falls below threshold
    // For ease of moonwalking
    let moonwalk_disable_dashback_stick_y = ParamModule::get_float(fighter.battle_object, ParamType::Common, "moonwalk_disable_dashback_stick_y");
    if fighter.global_table[STICK_Y].get_f32() <= moonwalk_disable_dashback_stick_y
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    }
    
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)
    && is_dash_input {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
        return 1.into();
    }

    if is_backdash_input {
        let frame = MotionModule::frame(fighter.module_accessor);
        let re_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("re_dash_frame")) as f32;
        if re_dash_frame <= frame {
            fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
            return 1.into();
        }
    }

    if fighter.sub_transition_group_check_ground_jump().get_bool() == false {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr = PostureModule::lr(fighter.module_accessor);
            let run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x"));
            if run_stick_x <= stick_x * lr * -1.0
            && fighter.global_table[CMD_CAT1].get_i32() & (
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 |
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 |
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 |
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N |
                *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI |
                *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW
            ) == 0 {
                // this part *shouldn't* matter because it's all the same value constant, but just to be safe...
                let kind;
                if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_DOLLY {
                    kind = FIGHTER_DOLLY_STATUS_KIND_TURN_RUN_BACK;
                }
                else if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_DEMON {
                    kind = FIGHTER_DEMON_STATUS_KIND_TURN_RUN_BACK;
                }
                else  {
                    kind = FIGHTER_RYU_STATUS_KIND_TURN_RUN_BACK;
                }
                fighter.change_status(kind.into(), false.into());
                return 1.into();
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            if GroundModule::get_down_friction(fighter.module_accessor) < 1.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_RUN_BRAKE.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            return 1.into();
        }
        else {
            return 0.into();
        }
    }
    else {
        return 1.into();
    }
    // 0.into()
}

#[utils::export(common::shoto_status)]
pub unsafe fn fgc_end_dashback(fighter: &mut L2CFighterCommon) {
	let mut initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);

	if [*FIGHTER_RYU_STATUS_KIND_TURN_RUN_BACK, *FIGHTER_DOLLY_STATUS_KIND_TURN_RUN_BACK, *FIGHTER_DEMON_STATUS_KIND_TURN_RUN_BACK, *FIGHTER_STATUS_KIND_WALK].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
		//println!("run/walk next");
		let applied_speed_clamped = initial_speed.clamp(-run_speed_max, run_speed_max);

		initial_speed = applied_speed_clamped;
	}
    if StatusModule::status_kind_next(fighter.module_accessor) == *FIGHTER_STATUS_KIND_GUARD_ON {
        fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
    }

    if VarModule::is_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL) {
        let applied_speed_clamped = initial_speed.clamp(-run_speed_max, run_speed_max);
        let dash_end_speed_mul = ParamModule::get_float(fighter.battle_object, ParamType::Common, "dash_end_speed_mul");
        fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed_clamped * dash_end_speed_mul);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);
}

#[no_mangle]
pub unsafe extern "Rust" fn ryu_attack_main_uniq_chk(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo_count = ComboModule::count(fighter.module_accessor) as i32;
        let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
        if combo_count < attack_combo_max {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                return 0.into();
            }
            ryu_attack_main_uniq_chk2(fighter, hash40("attack_11_s").into(), hash40("attack_11_w").into());
        }
    }
    else {
        ryu_attack_main_uniq_chk2(fighter, hash40("attack_11_s").into(), hash40("attack_11_w").into());
    }
    0.into()
}


#[no_mangle]
pub unsafe extern "C" fn ryu_attack_main_uniq_chk2(fighter: &mut L2CFighterCommon, mot1: L2CValue, mot2: L2CValue) {
    ryu_attack_main_uniq_chk3(fighter);
    fighter.attack_mtrans_pre_process();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if ComboModule::count(fighter.module_accessor) == 1 {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT) {
                WorkModule::set_int64(fighter.module_accessor, 0x10556e6036, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK_NEAR, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
            }
            else {
                WorkModule::set_int64(fighter.module_accessor, 0xb4f4e6f8f, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK11, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
            }
        }
    }
    fighter.attack_mtrans_post_process();
    notify_event_msc_cmd!(fighter, 0x265a5c1b6bu64, mot1.get_int(), mot2.get_int());
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
    if 1 < ComboModule::count(fighter.module_accessor) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_CHANGE_LOG);
    }
}

#[utils::export(common::shoto_status)]
unsafe extern "C" fn ryu_attack_main_uniq_chk3(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_ATTACK_INT_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_CHANGE_LOG);
}

extern "Rust" {
    fn only_jabs(fighter: &mut L2CFighterCommon) -> bool;
}


#[no_mangle]
pub unsafe extern "Rust" fn ryu_attack_main_uniq_chk4(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() == false {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            if !WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0) {
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            }
        }
        else {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER) {
                return 0.into();
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && only_jabs(fighter) {
                let stick_y = fighter.global_table[STICK_Y].get_f32();
                let attack_hi3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi3_stick_y"));
                let cont;
                if !(stick_y < attack_hi3_stick_y) {
                    cont = false;
                }
                else {
                    let attack_lw3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_y"));
                    cont = attack_lw3_stick_y < stick_y;
                }
                if cont {
                    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
                    return 0.into();
                }
            }
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
        }
    }
    else {
        fighter.attack_uniq_chk();
        if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                if !AttackModule::is_infliction_status(fighter.module_accessor, 0x7f) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                        ComboModule::reset(fighter.module_accessor);
                    }
                }
            }
        }
        else if only_jabs(fighter) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                if !AttackModule::is_infliction_status(fighter.module_accessor, 0x7f) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                        ComboModule::reset(fighter.module_accessor);
                    }
                }
                else {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_COMBO) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                let count = ComboModule::count(fighter.module_accessor) as i32;
                let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
                if count & attack_combo_max == 0 {
                    return 0.into();
                }
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER) {
                    return 0.into();
                }
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
            }
        }
    }
    0.into()
}

#[no_mangle]
pub unsafe extern "Rust" fn ryu_final_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let val;
    let final_cancel = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    if situation.get_i32() != *SITUATION_KIND_GROUND {
        val = fighter.sub_transition_group_check_air_special().get_bool();
    }
    else {
        val = fighter.sub_transition_group_check_ground_special().get_bool();
    }
    // println!("final? {}", val);
    if !final_cancel {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    }
    val.into()
}

#[no_mangle]
pub unsafe extern "Rust" fn ryu_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let val;
    let special_n = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    let special_s = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    let special_hi = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    let special_lw = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    let special_n_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    let special_n2_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    let special_s_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    let special_hi_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    let attack_command1 = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    if situation.get_i32() != *SITUATION_KIND_GROUND {
        val = fighter.sub_transition_group_check_air_cliff().get_bool();
    }
    else {
        val = fighter.sub_transition_group_check_ground_special().get_bool();
    }
    if !special_n {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    }
    if !special_s {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    }
    if !special_hi {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    }
    if !special_lw {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    }
    if !special_n_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    }
    if !special_n2_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    }
    if !special_s_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    }
    if !special_hi_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    }
    if !attack_command1 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    }
    val.into()
}

#[no_mangle]
pub unsafe extern "Rust" fn ryu_kara_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    let val;
    let special_n_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    let special_n2_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    let special_s_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    let special_hi_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    let attack_command1 = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    val = fighter.sub_transition_group_check_special_command().get_bool();
    if !special_n_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    }
    if !special_n2_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    }
    if !special_s_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    }
    if !special_hi_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    }
    if !attack_command1 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    }
    val.into()
}

// pub unsafe extern "C" fn ryu_idkwhatthisis(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
//     let cont = fighter.get_mini_jump_attack_data_cancel_function(param_1.clone()).get_bool();
//     if fighter.get_mini_jump_attack_data_cancel_function(param_1.clone()).get_bool() == false {
//     }
//     else {
//         cont.into()
//     }
// }

#[utils::export(common::shoto_status)]
pub unsafe extern "C" fn ryu_idkwhatthisis2(fighter: &mut L2CFighterCommon) {
    if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && only_jabs(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
        }
    }
}

#[no_mangle]
pub unsafe extern "Rust" fn fgc_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HDR
    if fighter.kind() == *FIGHTER_KIND_DEMON {
        app::FighterSpecializer_Demon::update_opponent_lr_1on1(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING);
    }
    else {
        app::FighterSpecializer_Dolly::update_opponent_lr_1on1(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING);
    }
    // vanilla
    fighter.status_LandingSub();
    fighter.status_LandingStiffness();
    fighter.sub_landing_start_check_damage_face();
    // HDR
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr != 0.0 && PostureModule::lr(fighter.module_accessor) != lr {
        PostureModule::set_lr(fighter.module_accessor, lr);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(fgc_landing_main_loop as *const () as _))
}

unsafe extern "C" fn fgc_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    // HDR
    if MotionModule::is_end(fighter.module_accessor)
    && lr != 0.0
    && PostureModule::lr(fighter.module_accessor) != lr
    {
        fighter.sub_wait_common();
    }
    // vanilla
    if !fighter.status_Landing_MainSub().get_bool() {
        fighter.sub_landing_cancel_check_damage_face();
        return 0.into();
    }
    1.into()
}