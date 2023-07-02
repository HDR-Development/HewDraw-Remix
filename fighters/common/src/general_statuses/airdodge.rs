// status imports
use interpolation::Lerp;
use super::*;
use globals::*;
// This file contains code for aerial glide tosses, wavelanding

pub fn install() {
    install_status_scripts!(
        status_pre_EscapeAir,
        status_EscapeAir,
        status_end_EscapeAir
    );
    install_hooks!(
        sub_escape_air_common,
        sub_escape_air_uniq,
        sub_escape_air_common_main,
        sub_escape_air_common_strans_main
    );
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            setup_escape_air_slide_common,
            exec_escape_air_slide
        );
    }
}

// pre status
#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon20status_pre_EscapeAirEv")]
pub unsafe fn status_pre_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    // handles instant wavedashes/wavelands
    // we make sure to include this before change_motion so we check for proximity to the ground using our jumpsquat animation's ECB, rather than airdodge anim's ECB
    // a character's ECB x position can shift on the first frame of their airdodge anim, which sometimes makes them unable to wavedash on an edge even if standing on solid ground the previous frame
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_DAMAGE_FALL
    && (VarModule::is_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH) || fighter.handle_waveland(false))
    {
        VarModule::on_flag(fighter.battle_object, vars::common::status::SHOULD_WAVELAND);
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon16status_EscapeAirEv")]
unsafe fn status_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_escape_air_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
        VarModule::on_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(status_EscapeAir_Main as *const () as _))
}

unsafe extern "C" fn status_EscapeAir_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::common::status::SHOULD_WAVELAND) {
        return 1.into();
    }
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    0.into()
}

// end status
#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon20status_end_EscapeAirEv")]
pub unsafe fn status_end_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].clone();
    if status_kind == FIGHTER_STATUS_KIND_FALL || status_kind == FIGHTER_STATUS_KIND_LANDING {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide_max"));
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            let global_speed_mul = ParamModule::get_float(fighter.object(), ParamType::Common, "wavedash_speed_mul");
            let speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_speed_mul_escape_air_slide"));
            let escape_air_slide_speed_clamp = WorkModule::get_param_float(fighter.module_accessor, hash40("escape_air_slide_speed"), 0) * global_speed_mul;

            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = (app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent) * speed_mul).clamp(-escape_air_slide_speed_clamp, escape_air_slide_speed_clamp);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = (app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent) * speed_mul).clamp(-escape_air_slide_speed_clamp, escape_air_slide_speed_clamp);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        } else {
            let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("landing_frame_escape_air")) as f32;
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
        if status_kind == FIGHTER_STATUS_KIND_LANDING {
            // prevents knockback speed from applying into wavelands (boosted wavelands out of hitstun)
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);

            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
        }
        VarModule::off_flag(fighter.battle_object, vars::common::status::SHOULD_WAVELAND);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
    }
    VarModule::on_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET);
    0.into()
}

// common air dodge init code
#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon21sub_escape_air_commonEv")]
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
        fighter.sub_escape_air_uniq(L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_escape_air_uniq as *const () as _));
}

// custom substatus for airdodges
#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon19sub_escape_air_uniqEN3lib8L2CValueE")]
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
            else {
                // Allows fastfall during nairdodge
                if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_FALL
                && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE)
                && KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0
                && fighter.global_table[STICK_Y].get_f32() <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dive_cont_value"))
                && fighter.global_table[FLICK_Y].get_i32() < WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dive_flick_frame_value")) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT);
                    fighter.check_mach_stamp();
                }
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

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon26sub_escape_air_common_mainEv")]
unsafe extern "C" fn sub_escape_air_common_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let id = VarModule::get_int(fighter.battle_object, vars::common::instance::COSTUME_SLOT_NUMBER) as usize;
    let curr_frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let cancel_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_cancel_frame")) - 1.0;  // subtract 1 because curr_frame is 0 indexed

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return L2CValue::Bool(true);
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
        || (!fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()){
        if fighter.sub_escape_air_common_strans_main().get_bool() {
            return L2CValue::Bool(true);
        }
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING),
                L2CValue::Bool(false)
            );
            return L2CValue::Bool(true);
        }
        if MotionModule::end_frame(fighter.module_accessor) < cancel_frame {
            // If our airdodge animation is shorter than its FAF
            if curr_frame < (cancel_frame as i32) {
                // Stop the animation on its second-to-last frame
                // Continue looping the status until our FAF is reached
                if MotionModule::prev_frame(fighter.module_accessor) < (MotionModule::end_frame(fighter.module_accessor) - 1.0)
                && MotionModule::frame(fighter.module_accessor) >= (MotionModule::end_frame(fighter.module_accessor) - 1.0)
                {
                    MotionModule::set_rate(fighter.module_accessor, 0.0);
                }
                return L2CValue::Bool(false);
            } else {
                // Transition to fall on FAF
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                    L2CValue::Bool(false)
                );
            }
        }
        else {
            // Vanilla logic
            if !MotionModule::is_end(fighter.module_accessor) {
                return L2CValue::Bool(false);
            } else {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                    L2CValue::Bool(false)
                );
            }
        }
    }
    L2CValue::Bool(true)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon33sub_escape_air_common_strans_mainEv")]
unsafe extern "C" fn sub_escape_air_common_strans_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("air_escape_passive_trigger_frame")) as f32;
    let curr_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    let pad = fighter.global_table[PAD_FLAG].get_i32();
    let agt_window = ParamModule::get_int(fighter.battle_object, ParamType::Common, "glide_toss_cancel_frame");
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
        && pad & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        && ItemModule::is_have_item(fighter.module_accessor, 0)
        && curr_frame <= agt_window {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
            smash::app::sv_module_access::item(fighter.lua_state_agent);
            let throwable = !fighter.pop_lua_stack(1).get_bool();
            if throwable {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_ITEM_THROW),
                    L2CValue::Bool(false)
                );
                let staling_mul = (1.0 - 0.1 * (VarModule::get_int(fighter.object(), vars::common::instance::AGT_USED_COUNTER) as f32)).max(0.0);
                KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: staling_mul, y: staling_mul, z: staling_mul}, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
                VarModule::inc_int(fighter.object(), vars::common::instance::AGT_USED_COUNTER);
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
            L2CValue::Bool(true)
        );
        return 1.into();
    }
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let passive_fb_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
    // lazy eval gaurantees that we don't call handle_waveland if we are on the ground
    // or if you have begun falling (after frame 30)
    if situation_kind == *SITUATION_KIND_GROUND
    || (!WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL) && fighter.handle_waveland(false)) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND) 
        && WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y) <= 0.0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
                && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
                && passive_fb_value <= stick_x.abs()
                && (curr_frame as f32) < trigger_frame {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE_FB),
                    L2CValue::Bool(true)
                );
                return 1.into();
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE)
                && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
                && (curr_frame as f32) < trigger_frame {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE),
                    L2CValue::Bool(false)
                );
                return 1.into();
            }
        }
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
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

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_setup_escape_air_slide_common)]
unsafe fn setup_escape_air_slide_common(fighter: &mut L2CFighterCommon, stick_x: L2CValue, stick_y: L2CValue) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        return;
    }
    //StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), true);
    let stick_vec = sv_math::vec2_normalize(stick_x.get_f32(), stick_y.get_f32());
    WorkModule::set_float(fighter.module_accessor, stick_vec.x, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
    WorkModule::set_float(fighter.module_accessor, stick_vec.y, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
    let escape_air_slide_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_speed"));
    let escape_air_slide_speed_vec = Vector2f{x: escape_air_slide_speed * stick_vec.x, y: escape_air_slide_speed * stick_vec.y};
    
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_FREE, escape_air_slide_speed_vec.x, escape_air_slide_speed_vec.y, 0.0, 0.0, 0.0);
    app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
    app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    let mut escape_air_slide_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_frame"));
    let escape_air_slide_u_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_u_stiff_frame"));
    let escape_air_slide_d_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_d_stiff_frame"));
    let escape_air_angle = (stick_vec.y/stick_vec.x.abs()).atan().to_degrees();
    let mut fuck = 0.0;
    if escape_air_angle < 0.0 {
        fuck = (escape_air_angle * -1.0) / 90.0;
        escape_air_slide_stiff_frame = Lerp::lerp(&fuck, &escape_air_slide_d_stiff_frame, &escape_air_slide_stiff_frame);
    }
    else {
        fuck = escape_air_angle / 90.0;
        escape_air_slide_stiff_frame = Lerp::lerp(&fuck, &escape_air_slide_u_stiff_frame, &escape_air_slide_stiff_frame);
    }
    let escape_air_slide_stiff_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_start_frame"));
    WorkModule::set_float(fighter.module_accessor, escape_air_slide_stiff_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
    let escape_air_slide_back_end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_back_end_frame"));
    let escape_air_add_xlu_start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
    WorkModule::set_int(fighter.module_accessor, escape_air_slide_back_end_frame + escape_air_add_xlu_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
    WorkModule::set_float(fighter.module_accessor, escape_air_slide_stiff_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
    
    // EffectModule::req_on_joint(
    //     fighter.module_accessor,
    //     Hash40::new("sys_smash_flash_s"),
    //     Hash40::new("hip"),
    //     &Vector3f{x: 0.0, y: 4.0, z: 8.0},
    //     &Vector3f::zero(),
    //     1.1,
    //     &Vector3f{x: 18.0, y: 12.0, z: 0.0},
    //     &Vector3f::zero(),
    //     false,
    //     0,
    //     0,
    //     0
    // );
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_exec_escape_air_slide)]
unsafe fn exec_escape_air_slide(fighter: &mut L2CFighterCommon) {
    let mut step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
    let back_end_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
    if step == 0 {
        let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        if frame >= back_end_frame {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL) {
        // Allows fastfall during directional airdodge
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE)
        && KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0
        && fighter.global_table[STICK_Y].get_f32() <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dive_cont_value"))
        && fighter.global_table[FLICK_Y].get_i32() < WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dive_flick_frame_value")) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT);
            fighter.check_mach_stamp();
        }
        return;
    }
    step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
    if step != 0 {
        if step != 1 {
            return;
        }
        if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
        }
        let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        if frame >= 1 {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_KEEP_AIR_TURNED_OFF) {
                //StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_KEEP_AIR_TURNED_OFF);
            }
            let speed_mul = ParamModule::get_float(fighter.battle_object, ParamType::Common, "escape_air_slide_speed_mul");
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_mul, speed_mul);
            app::sv_kinetic_energy::mul_speed(fighter.lua_state_agent);
        }
        let fall_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "escape_air_slide_fall_frame");
        if frame >= fall_frame {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);

            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST_NO_CAP, speed_x, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            app::sv_kinetic_energy::enable(fighter.lua_state_agent);

            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            app::sv_kinetic_energy::enable(fighter.lua_state_agent);

            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
        }
    }
}