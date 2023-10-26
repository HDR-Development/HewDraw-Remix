use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        //pre_special_hi_escape,
        special_hi_escape,
        end_special_hi_escape,
        special_s_jump_init,
        special_hi_damage_end_main
    );
}

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE //

#[status_script(agent = "koopajr", status = FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_special_hi_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick = app::sv_math::vec2_length(fighter.global_table[STICK_X].get_f32(), fighter.global_table[STICK_Y].get_f32());
    if stick >= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_stick")) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE);
    }
    original!(fighter)
}

#[status_script(agent = "koopajr", status = FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
        return L2CValue::I32(0);
    }
    sub_escape_air_common(fighter);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_jr_escape"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_jr_escape"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_EscapeAir_Main as *const () as _))
}

unsafe extern "C" fn status_EscapeAir_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !sub_escape_air_common_main(fighter).get_bool() {
        fighter.sub_escape_check_rumble();
    }
    0.into()
}

unsafe fn sub_escape_air_common(fighter: &mut L2CFighterCommon) {
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
    let enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL
    ];
    for x in enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_NO_WATER_INOUT_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_escape_air_uniq(fighter, L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_escape_air_uniq as *const () as _));
}

unsafe extern "C" fn sub_escape_air_uniq(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if arg.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        // if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO) {
        //     let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        //     if WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("attack_air_lasso_enable_frame")) <= frame {
        //         WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
        //     }
        // }
    } else {
        let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        if frame <= WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("item_air_catch_frame_escape")) {
            fighter.sub_GetLightItemImm(L2CValue::Void());
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE)
            && ItemModule::is_have_item(fighter.module_accessor, 0)
            && frame <= WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("escape_throw_item_frame")) {
            fighter.sub_AIRChkDropItemImm();
        }
        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                fighter.exec_escape_air_slide();
            }
            if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_FLAG_HIT_XLU) {
                    let stale_motion_rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
                    MotionModule::set_rate(fighter.module_accessor, stale_motion_rate);
                    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
                }
            }
            if StatusModule::is_changing(fighter.module_accessor) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_STIFF) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_END_STIFF)
                    && CancelModule::is_enable_cancel(fighter.module_accessor) {
                    MotionModule::set_rate(fighter.module_accessor, 1.0);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_END_STIFF);
                }
            } else if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_STIFF) {
                let start_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
                let frame = MotionModule::frame(fighter.module_accessor);
                let end_frame = MotionModule::end_frame(fighter.module_accessor);
                if 0.0 <= start_frame && start_frame <= frame {
                    let mut cancel_frame = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                        WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_cancel_frame"))
                    } else {
                        WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_cancel_frame"))
                    };
                    if cancel_frame < 0.0 {
                        cancel_frame = end_frame;
                    }
                    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME) <= frame {
                        WorkModule::set_float(fighter.module_accessor, end_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
                    }
                    let stiff_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
                    let ratio = (cancel_frame - frame) / (stiff_frame - frame);
                    let new_rate = ratio * MotionModule::rate(fighter.module_accessor);
                    MotionModule::set_rate(fighter.module_accessor, new_rate);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_STIFF);
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_FALL) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL);
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                    fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
                }
            }
            fighter.sub_fall_common_uniq(arg);
        }
    }
    0.into()
}

unsafe extern "C" fn sub_escape_air_common_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let id = VarModule::get_int(fighter.battle_object, vars::common::instance::COSTUME_SLOT_NUMBER) as usize;

    // RoA airdodge stalling [
    if  MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_hi_jr_escape") {
        if MotionModule::frame(fighter.module_accessor) >= 1.0 && MotionModule::frame(fighter.module_accessor) < 28.0 {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
        }
        if MotionModule::frame(fighter.module_accessor) == 28.0 {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
    // ]

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return L2CValue::Bool(true);
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
        || (!fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()){
        if sub_escape_air_common_strans_main(fighter).get_bool() {
            return L2CValue::Bool(true);
        }
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING),
                L2CValue::Bool(false)
            );
            return L2CValue::Bool(true);
        }
        if !MotionModule::is_end(fighter.module_accessor) {
            return L2CValue::Bool(false);
        } else {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_FALL),
                L2CValue::Bool(false)
            );
        }
    }
    L2CValue::Bool(true)
}

unsafe extern "C" fn sub_escape_air_common_strans_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("air_escape_passive_trigger_frame"));
    let trigger_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0);
    let trigger_frame = (trigger_frame as f32) * trigger_frame_mul;
    let curr_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    let pad = fighter.global_table[PAD_FLAG].get_i32();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
        && pad & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        && ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
            smash::app::sv_module_access::item(fighter.lua_state_agent);
            let throwable = !fighter.pop_lua_stack(1).get_bool();
            if throwable {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_ITEM_THROW),
                    L2CValue::Bool(false)
                );
                return 1.into();
        }
    }
    let lasso_type = WorkModule::get_param_int(fighter.module_accessor, hash40("air_lasso_type"), 0);
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO)
        && lasso_type != *FIGHTER_AIR_LASSO_TYPE_NONE
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && pad & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        && !LinkModule::is_link(fighter.module_accessor, *FIGHTER_LINK_NO_CONSTRAINT) {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_AIR_LASSO),
            L2CValue::Bool(false)
        );
        return 1.into();
    }
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let passive_fb_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND)
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
        && situation_kind == *SITUATION_KIND_GROUND
        && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
        && passive_fb_value <= stick_x.abs()
        && (curr_frame as f32) < trigger_frame {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE),
            L2CValue::Bool(false)
        );
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_AIR) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON)
            && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_LEFT) as u32)
            && (ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP as u8) as f32) < trigger_frame
            && (curr_frame as f32) < trigger_frame {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP),
                L2CValue::Bool(false)
            );
            return 1.into();
        }
        let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP)
            && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_LEFT) as u32)
            && jump_stick_y <= stick_y
            && (curr_frame as f32) < trigger_frame {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP),
                L2CValue::Bool(true)
            );
            return 1.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL)
            && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_LEFT) as u32)
            && (curr_frame as f32) < trigger_frame {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE_WALL),
                L2CValue::Bool(false)
            );
            return 1.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL)
            && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32)
            && (curr_frame as f32) < trigger_frame {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE_CEIL),
                L2CValue::Bool(false)
            );
            return 1.into();
        }
    }
    0.into()
}

#[status_script(agent = "koopajr", status = FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn end_special_hi_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_EscapeAir()
}

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP

#[status_script(agent = "koopajr", status = FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn special_s_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Burn double jump when jumping out of Clown Kart Dash
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    && fighter.get_num_used_jumps() < fighter.get_jump_count_max() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    0.into()
}

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END

#[status_script(agent = "koopajr", status = FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_damage_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    original!(fighter);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DAMAGE_FLY) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_DAMAGE_FALL, false);
        }
        else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
    }
    1.into()
}