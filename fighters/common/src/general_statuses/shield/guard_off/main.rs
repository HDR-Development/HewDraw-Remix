// status imports
use super::*;
use globals::*;
use smash_script::macros::{ EFFECT_FOLLOW, EFFECT_FOLLOW_FLIP };

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_cancel)]
unsafe fn sub_status_guard_off_main_common_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF)
    && fighter.status_frame() <= 30 {
        return (0).into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        return fighter.sub_wait_ground_check_common(false.into());
    }

    // check shorthop aerial
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return true.into();
    }

    // check USpecial/USmash
    if fighter.check_guard_attack_special_hi(false.into()).get_bool() {
        return true.into();
    }
    
    // check parry
    if fighter.is_cat_flag(CatHdr::Parry) {
        VarModule::on_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
        return true.into();
    }

    // check jump
    if fighter.sub_check_button_jump().get_bool() || fighter.sub_check_button_frick().get_bool() {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
        return true.into();
    }

    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        if misc::check_item_oos(fighter).get_bool() {
            return true.into();
        }
    } else if misc::check_grab_oos(fighter).get_bool() {
        // TODO: never reaches this code for some reason
        return true.into();
    }

    true.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_air)]
unsafe fn sub_status_guard_off_main_common_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }
    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_control)]
unsafe fn sub_status_guard_off_main_common_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF)
    && fighter.sub_transition_group_check_ground_jump().get_bool() 
    && misc::check_guard_attack_special_hi(fighter, false.into()).get_bool() {
        return true.into();
    }
    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOff_Main_common)]
unsafe fn status_GuardOff_Main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if sub_status_guard_off_main_common_cancel(fighter).get_bool()
    || sub_status_guard_off_main_common_air(fighter).get_bool()
    || sub_status_guard_off_main_common_control(fighter).get_bool() {
        return true.into();
    }
    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOff_Main)]
unsafe fn status_GuardOff_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !status_GuardOff_Main_common(fighter).get_bool()
    && MotionModule::is_end(fighter.module_accessor)
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_sub_guard_off_uniq)]
unsafe fn sub_guard_off_uniq(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if !arg.get_bool() {
        return false.into();
    }

    let boma = fighter.module_accessor;
    if VarModule::is_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF) {
        let just_frame = fighter.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        if just_frame > 0 {
            fighter.dec_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
            if just_frame == 1 {
                ShieldModule::set_status(boma, *FIGHTER_SHIELD_KIND_GUARD, app::ShieldStatus(*SHIELD_STATUS_NONE), 0);
                let type_of_guard = app::FighterUtil::get_shield_type_of_guard(fighter.global_table[0x2].get_i32()) as i32;
                ShieldModule::set_shield_type(boma, app::ShieldType(type_of_guard), *FIGHTER_SHIELD_KIND_GUARD, 0);
                ReflectorModule::set_status(boma, 0, app::ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);

                // ModelModule::disable_gold_eye(boma);
                // EffectModule::remove_common(boma, Hash40::new("just_shield"));
                let end_frame = MotionModule::end_frame_from_hash(boma, Hash40::new("guard_off"));
                let rate = (end_frame * 0.75) / (30.0 - (fighter.status_frame() as f32));
                MotionModule::change_motion(boma, Hash40::new("guard_off"), end_frame * 0.25, rate, false, 0.0, false, false);
            }
        } else if just_frame == 0 {
            SoundModule::stop_se(boma, Hash40::new("se_common_guardoff"), 0);
        }
    }

    let cancel_frame = fighter.get_int(*FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
    if cancel_frame > 0 {
        fighter.dec_int(*FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
        if cancel_frame == 1 {
            CancelModule::enable_cancel(boma);
        }
    }
    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOff_Common)]
unsafe fn status_GuardOff_Common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if !VarModule::is_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF) {
        fighter.enable_transition_term_many(&[
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
        ]);
    }

    let lifetime = fighter.get_command_life(CatHdr::Parry) as i32;
    let buffer = ControlModule::get_command_life_count_max(fighter.module_accessor) as i32;
    let shield_just_frame = fighter.get_param_int("common", "shield_just_frame");
    let just_frame = (shield_just_frame + lifetime + 1 - buffer).clamp(3, shield_just_frame);
    fighter.set_int(just_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);

    let guard_off_cancel_frame = fighter.get_param_int("common", "guard_off_cancel_frame");
    fighter.set_int(guard_off_cancel_frame, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);

    let mut fighter_guard_off_cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma, Hash40::new("guard_off"), true);
    if fighter_guard_off_cancel_frame == 0.0 { fighter_guard_off_cancel_frame = MotionModule::end_frame(boma); }
    let guard_off_motion_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Common, "guard_off_motion_start_frame");
    let ret_val = if fighter_guard_off_cancel_frame > 0.0 && guard_off_cancel_frame > 0 {
        (fighter_guard_off_cancel_frame - guard_off_motion_start_frame) / (guard_off_cancel_frame as f32)
    } else {
        1.0
    };

    let guard_off_work_cancel_frame = fighter.get_int(*FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
    let guard_off_enable_shield_frame = fighter.get_param_int("common", "guard_off_enable_shield_frame");
    fighter.set_int(guard_off_enable_shield_frame + guard_off_work_cancel_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
    fighter.set_int(guard_off_enable_shield_frame + guard_off_work_cancel_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);

    if !StopModule::is_stop(boma) {
        sub_guard_off_uniq(fighter, false.into());
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_guard_off_uniq as *const () as _));
    ret_val.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOff)]
unsafe fn status_GuardOff(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let rate = status_GuardOff_Common(fighter).get_f32();
    if VarModule::is_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF) {
        MotionModule::change_motion(boma, Hash40::new("guard_damage"), 2.0, 0.0, false, 0.0, false, false);
        // app::FighterUtil::flash_eye_info(boma);
        // if !WorkModule::is_flag(
        //     boma,
        //     *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL,
        // ) {
        //     ModelModule::enable_gold_eye(boma);
        //     WorkModule::on_flag(
        //         boma,
        //         *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE,
        //     );
        // }
        let shield_radius = fighter.get_param_float("shield_radius", "");
        let lr = PostureModule::lr(boma);
        EffectModule::req_follow(
            boma,
            Hash40::new("sys_genesis_end"),
            Hash40::new("throw"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            shield_radius * 0.06,
            true,
            *EFFECT_SUB_ATTRIBUTE_NONE as u32,
            0,
            0,
            *EFFECT_FLIP_NONE,
            0,
            false,
            false
        );
        EffectModule::set_rate_last(boma, 1.2);
        // EffectModule::set_alpha_last(boma, 0.4);
        EffectModule::req_common(boma, Hash40::new("just_shield"), 0.0);
        // let shield_se = app::FighterUtil::get_just_shield_se(fighter.global_table[0x2].get_i32());
        let sfx_handle = SoundModule::play_se(boma, Hash40::new("se_item_backshield_guard01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, sfx_handle as i32, 0.9, 0);
        SoundModule::stop_se(boma, Hash40::new("se_common_guardon"), 0);
    } else {
        let guard_off_motion_start_frame = ParamModule::get_float(
            fighter.battle_object,
            ParamType::Common,
            "guard_off_motion_start_frame"
        );
        MotionModule::change_motion(boma, Hash40::new("guard_off"), guard_off_motion_start_frame, rate, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_GuardOff_Main as *const () as _))
}

pub fn install() {
    skyline::install_hooks!(
        sub_status_guard_off_main_common_cancel,
        sub_status_guard_off_main_common_air,
        sub_status_guard_off_main_common_control,
        status_GuardOff_Main_common,
        status_GuardOff_Main,
        sub_guard_off_uniq,
        status_GuardOff_Common,
        status_GuardOff
    );
}
