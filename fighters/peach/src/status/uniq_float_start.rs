use super::*;
use globals::*;

#[no_mangle]
unsafe fn peach_float_start_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_FALL_CONTROL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_CONTROL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_FALL_TIME);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_TIME);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_TRANS_ID_SPECIAL_LW_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);

    let uniq_float_float_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("uniq_float_float_frame"));
    WorkModule::set_int(fighter.module_accessor, uniq_float_float_frame, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_FLOAT_FRAME);
    WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_ENABLE_UNIQ);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("fuwafuwa_start"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    let shield_stiff_mul_attack_air = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_stiff_mul_attack_air"));
    AttackModule::set_shield_stiff_mul(fighter.module_accessor, shield_stiff_mul_attack_air);
    peach_float_check_aerial(fighter);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT_RAY_CHECK) {
        let pos_x = PostureModule::pos_x(fighter.module_accessor);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);
        let pos_z = PostureModule::pos_z(fighter.module_accessor);
        let result = &mut Vector2f{x: 0.0, y: 0.0};
        let ray_check = GroundModule::ray_check_hit_pos(
            fighter.module_accessor,
            &Vector2f{x: pos_x, y: pos_y},
            &Vector2f{x: 0.0, y: -6.0},
            result,
            true
        );
        if ray_check {
            let ray_check_y = result.y;
            let uniq_float_start_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("uniq_float_start_y"));
            PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos_x, y: ray_check_y + uniq_float_start_y, z: pos_z});
        }
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(peach_float_main_loop_common as *const () as _))
}

#[no_mangle]
pub unsafe fn peach_float_check_aerial(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_MTRANS) != 1 {
            peach_float_set_aerial(fighter);
            return;
        }
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_NONE, FIGHTER_LOG_ACTION_KIND_NONE);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);

    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_FALL_CONTROL);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_CONTROL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_FALL_TIME);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_TIME);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_TRANS_ID_SPECIAL_LW_ITEM_THROW);

    WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_MTRANS);
}

#[no_mangle]
pub unsafe fn peach_float_set_aerial(fighter: &mut L2CFighterCommon) {
    let mot = fighter.sub_attack_air_kind_set_log_info();
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot.get_u64()),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_CANCEL_UNABLE_CANCEL);
    sv_module_access::cancel(fighter.lua_state_agent);
    let _ = fighter.pop_lua_stack(1);
    ControlModule::clear_command(fighter.module_accessor, false);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);

    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_FALL_CONTROL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_CONTROL);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_FALL_TIME);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_TIME);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_TRANS_ID_SPECIAL_LW_ITEM_THROW);

    WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_MTRANS);
}

#[no_mangle]
pub unsafe fn peach_float_main_loop_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if StatusModule::is_changing(fighter.module_accessor)
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_AIR
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
        peach_float_set_aerial(fighter);
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR.into(), false.into());
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_FALL_CONTROL)
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_CONTROL)
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_FALL_TIME)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_FLOAT_FRAME) <= 0 {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_TIME)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_FLOAT_FRAME) <= 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
        return 0.into();
    }

    let mut daikon = *ITEM_KIND_PEACHDAIKON;
    if fighter.global_table[0x2].get_i32() == *FIGHTER_KIND_DAISY {
        daikon = *ITEM_KIND_DAISYDAIKON;
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_TRANS_ID_SPECIAL_LW_ITEM_THROW) {
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
            let stick_y = fighter.left_stick_y();
            let special_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_y"));
            if stick_y <= -special_stick_y
            && ItemModule::get_have_item_kind(fighter.module_accessor, 0) == daikon {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), true.into());
                return 0.into();
            }
        }
    }

    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_ENABLE_UNIQ) == 1 {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_FLOAT_FRAME) > 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_FLOAT_FRAME);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_FLOAT_FRAME) == 0 {
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_ENABLE_UNIQ);
            }
        }
        if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING) {
            fighter.sub_transition_group_check_air_landing();
        }
        if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL) {
            fighter.sub_transition_group_check_air_special();
        }

        let mut check_aerial = false;

        if !StatusModule::is_changing(fighter.module_accessor) {
            let mtrans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_MTRANS);

            if mtrans == 1 {
                if MotionModule::is_end(fighter.module_accessor) {
                    check_aerial = true;
                }
                if CancelModule::is_enable_cancel(fighter.module_accessor) {
                    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                        check_aerial = true;
                    }
                }
            }
            else if mtrans == 2 {
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                    check_aerial = true;
                }
                if MotionModule::is_end(fighter.module_accessor) {
                    MotionModule::change_motion(
                        fighter.module_accessor,
                        Hash40::new("fuwafuwa"),
                        0.0,
                        1.0,
                        false,
                        0.0,
                        false,
                        false
                    );
                }
            }
        }

        if check_aerial {
            peach_float_check_aerial(fighter);
        }
    }
    0.into()
}

unsafe extern "C" fn uniq_float_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    peach_float_start_main_common(fighter)
}

pub fn install() {
    Agent::new("peach")
        .status(Main, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, uniq_float_start)
        .install();
}