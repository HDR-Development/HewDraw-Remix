// status imports
use super::*;
use globals::*;

const ZERO_VEC: Vector3f = Vector3f {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

#[skyline::hook(replace = L2CFighterCommon_sub_GuardDamageUniq)]
unsafe fn sub_GuardDamageUniq(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    misc::check_enable_cstick_buffer_rolls(fighter);
    if !arg.get_bool() {
        fighter.FighterStatusGuard__landing_effect_control();
        return false.into();
    }
    let boma = fighter.module_accessor;

    let mut min_frame = fighter.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
    if min_frame > 0 {
        fighter.dec_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        min_frame -= 1;
    }

    if min_frame <= 0 && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
    }

    let just_frame = fighter.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
    if just_frame > 0 && fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        fighter.dec_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        if just_frame == 1 {
            ShieldModule::set_status(boma, *FIGHTER_SHIELD_KIND_GUARD, app::ShieldStatus(*SHIELD_STATUS_NONE), 0);
            let type_of_guard = app::FighterUtil::get_shield_type_of_guard(fighter.global_table[0x2].get_i32()) as i32;
            ShieldModule::set_shield_type(boma, app::ShieldType(type_of_guard), *FIGHTER_SHIELD_KIND_GUARD, 0);
            ReflectorModule::set_status(boma, 0, app::ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        }
    }

    // this code is responsible for making the static shield effect transparent as shieldlag ends
    let stiff_frame = fighter.get_int(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME) as f32;
    let static_eff_handle = fighter.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE_EFFECT_HANDLE);
    let static_alpha_decay_frame = 10.0 as f32;
    if stiff_frame < static_alpha_decay_frame && static_eff_handle != 0 {
        EffectModule::set_alpha(boma, static_eff_handle as u32, stiff_frame / static_alpha_decay_frame);
    }

    if !WorkModule::count_down_int(boma, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME, 0) {
        return false.into();
    }

    if fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        if fighter.is_flag(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE) {
            if !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) {
                ModelModule::disable_gold_eye(boma);
            }
            fighter.off_flag(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE);
        }

        if fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU) {
            HitModule::set_whole(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
            fighter.off_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU);
        }
        EffectModule::req_on_joint(boma, Hash40::new("sys_windwave"), Hash40::new("top"), &ZERO_VEC, &ZERO_VEC, 1.0, &ZERO_VEC, &ZERO_VEC, false, 0, 0, 0);
    } else {
        let disable_frame = fighter.get_param_int("common", "guard_damage_just_shield_disable_frame");
        fighter.set_int(disable_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_JUST_SHIELD_FRAME);
    }
    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage_common)]
unsafe fn status_GuardDamage_common(fighter: &mut L2CFighterCommon, arg: L2CValue) {
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_sub_x(fighter.module_accessor);
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::clear_command(fighter.module_accessor, true);
    fighter.global_table[FLICK_X].assign(&L2CValue::I32(0xfe));

    if !fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {

        fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        if !fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_GUARD_DAMAGE_NUM);
            fighter.on_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM);
        }

        if arg.get_bool() {
            let prev_shield = fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD);
            let prev_shield_scale = fighter.FighterStatusGuard__calc_shield_scale(prev_shield.into()).get_f32();
            let curr_shield = fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
            let curr_shield_scale = fighter.FighterStatusGuard__calc_shield_scale(curr_shield.into()).get_f32();
            let team_color = app::FighterUtil::get_team_color(fighter.module_accessor);
            let color = app::FighterUtil::get_effect_team_color(
                app::EColorKind(team_color as i32),
                Hash40::new("shield_effect_color")
            );
            let color = Vector3f {x: color.x, y: color.y, z: color.z};
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_shield_damage3"), Hash40::new("throw"), &ZERO_VEC, &ZERO_VEC, 0.1, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, false);
            EffectModule::set_rgb_partial_last(fighter.module_accessor, color.x, color.y, color.z);
            
            let effect2_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_shield_damage2"), Hash40::new("throw"), &ZERO_VEC, &ZERO_VEC, 0.1, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, false) as u32;
            EffectModule::set_rgb_partial_last(fighter.module_accessor, color.x, color.y, color.z);
            fighter.set_int(effect2_handle as i32, *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE2_EFFECT_HANDLE);
            
            let effect_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_shield_damage"), Hash40::new("throw"), &ZERO_VEC, &ZERO_VEC, 1.0, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, false) as u32;
            EffectModule::set_rgb_partial_last(fighter.module_accessor, color.x, color.y, color.z);
            fighter.set_int(effect_handle as i32, *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE_EFFECT_HANDLE);
            if effect_handle != 0 {
                let effect_scale = 0.1 * (curr_shield_scale / prev_shield_scale);
                EffectModule::set_scale(fighter.module_accessor, effect_handle, &(Vector3f {x: effect_scale, y: effect_scale, z: effect_scale}));
            }
        }
    } else {
        fighter.enable_transition_term_many(&[
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
        ]);
        fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        fighter.set_int(0, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        fighter.set_int(0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
        fighter.set_int(0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
        HitModule::set_whole(fighter.module_accessor, app::HitStatus(*HIT_STATUS_XLU), 0);
        fighter.on_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_JUST_SHIELD);
        app::FighterUtil::flash_eye_info(fighter.module_accessor);

        if !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) {
            ModelModule::enable_gold_eye(fighter.module_accessor);
            fighter.on_flag(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE);
        }

        EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_just_shield"), Hash40::new("throw"), &ZERO_VEC, &ZERO_VEC, 1.0, &ZERO_VEC, &ZERO_VEC, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, *EFFECT_FLIP_NONE, 1);
        let shield_lr = fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
        ColorBlendModule::set_last_attack_direction(fighter.module_accessor, &(Vector3f {x: -shield_lr, y: 0.0, z: 0.0}));
        EffectModule::req_common(fighter.module_accessor, Hash40::new("just_shield"), 0.0);

        if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_GUARD_OFF {
            EffectModule::req_screen(fighter.module_accessor, Hash40::new("just_shield_screen"), false, false, false);
        }

        let shield_se = app::FighterUtil::get_just_shield_se(fighter.global_table[0x2].get_i32());
        SoundModule::play_se(fighter.module_accessor, shield_se, true, false, false, false, app::enSEType(0));
    }

    if !StopModule::is_stop(fighter.module_accessor) {
        sub_GuardDamageUniq(fighter, false.into());
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_GuardDamageUniq as *const () as _));
}

#[skyline::hook(replace = L2CFighterCommon_status_guard_damage_main_common_air)]
unsafe fn status_guard_damage_main_common_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
        return true.into();
    }
    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_status_guard_damage_main_common)]
unsafe fn status_guard_damage_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    misc::check_enable_cstick_buffer_rolls(fighter);
    if status_guard_damage_main_common_air(fighter).get_bool() {
        return false.into();
    }
    let special_stick_y = fighter.get_param_float("common", "special_stick_y");
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if special_stick_y <= fighter.global_table[STICK_Y].get_f32() 
    && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_SPECIAL_HI);
    }

    if fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
    
        // check OOP options (subset of OOS options)

        // shorthop aerial
        if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
            return true.into();
        }
    
        // USpecial/USmash
        if fighter.check_guard_attack_special_hi(false.into()).get_bool() {
            return true.into();
        }
        
        // parry, force instant transition even during hitstop for chained parries
        if (fighter.is_button_trigger(Buttons::Parry) || fighter.is_button_trigger(Buttons::ParryManual))
        && fighter.is_cat_flag(CatHdr::Parry) {
            VarModule::on_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
            StopModule::cancel_hit_stop(fighter.module_accessor);
            StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
            return true.into();
        }
    
        // jump
        if fighter.sub_check_button_jump().get_bool() || fighter.sub_check_button_frick().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
            return true.into();
        }
    
        // item toss/grab
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            if misc::check_item_oos(fighter).get_bool() {
                return true.into();
            }
        } else if misc::check_grab_oos(fighter).get_bool() {
            return true.into();
        }

        // grounded transitions
        if CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into();
        }
        // end animation, wait transition
        if MotionModule::is_end(fighter.module_accessor) 
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return false.into();
        }
    } else if fighter.get_int(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME) == 0 {
        // transition to guard after shieldstun ends
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD) {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD.into(), false.into());
            return true.into();
        }
        // if guard is disabled, transition to guard off
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            VarModule::off_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
            return true.into();
        }
    }

    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage_Main)]
unsafe fn status_GuardDamage_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_guard_damage_main_common(fighter);
    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage)]
unsafe fn status_GuardDamage(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_GuardDamage_common(fighter, true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(status_GuardDamage_Main as *const () as _))
}

pub fn install() {
    skyline::install_hooks!(
        sub_GuardDamageUniq,
        status_GuardDamage_common,
        status_guard_damage_main_common_air,
        status_guard_damage_main_common,
        status_GuardDamage_Main,
        status_GuardDamage
    );
}
