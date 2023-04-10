// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_cancel)]
unsafe fn sub_status_guard_off_main_common_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(
        fighter.object(),
        vars::common::instance::IS_PARRY_FOR_GUARD_OFF,
    ) {
        if fighter.status_frame() <= 30 {
            return 0.into();
        }
    }

    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter
            .sub_transition_group_check_ground_jump_mini_attack()
            .get_bool()
        {
            if WorkModule::is_enable_transition_term(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD,
            ) {
                if ItemModule::is_have_item(fighter.module_accessor, 0) {
                    fighter.clear_lua_stack();
                    lua_args!(
                        fighter,
                        MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT,
                        ITEM_TRAIT_FLAG_THROW
                    );
                    smash::app::sv_module_access::item(fighter.lua_state_agent);
                    let throwable = fighter.pop_lua_stack(1).get_bool();
                    fighter.clear_lua_stack();
                    lua_args!(
                        fighter,
                        MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT,
                        ITEM_TRAIT_FLAG_SHOOT
                    );
                    smash::app::sv_module_access::item(fighter.lua_state_agent);
                    let shootable = fighter.pop_lua_stack(1).get_bool();
                    if throwable
                        || !throwable
                            && shootable
                            && ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0
                    {
                        fighter.clear_lua_stack();
                        lua_args!(
                            fighter,
                            MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT,
                            ITEM_TRAIT_FLAG_NO_THROW
                        );
                        smash::app::sv_module_access::item(fighter.lua_state_agent);
                        if !fighter.pop_lua_stack(1).get_bool()
                            || ControlModule::check_button_on(
                                fighter.module_accessor,
                                *CONTROL_PAD_BUTTON_GUARD,
                            )
                        {
                            if fighter.global_table[PAD_FLAG].get_i32()
                                & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER
                                != 0
                                || (fighter.global_table[CMD_CAT3].get_i32()
                                    & *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI
                                    != 0
                                    || fighter.global_table[CMD_CAT3].get_i32()
                                        & *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4
                                        != 0)
                            {
                                fighter.change_status(
                                    FIGHTER_STATUS_KIND_ITEM_THROW.into(),
                                    false.into(),
                                );
                                return true.into();
                            }
                        }
                    }
                }
            }
            if !fighter.sub_transition_group_check_ground_item().get_bool()
                && !fighter
                    .sub_transition_group_check_ground_special()
                    .get_bool()
                && !fighter
                    .sub_transition_group_check_ground_attack()
                    .get_bool()
            {
                return false.into();
            }
        }
    } else {
        if !fighter
            .sub_wait_ground_check_common(false.into())
            .get_bool()
        {
            return false.into();
        }
    }
    true.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_air)]
unsafe fn sub_status_guard_off_main_common_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        true.into()
    } else {
        false.into()
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_control)]
unsafe fn sub_status_guard_off_main_common_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(
        fighter.object(),
        vars::common::instance::IS_PARRY_FOR_GUARD_OFF,
    ) && fighter.sub_transition_group_check_ground_jump().get_bool()
        && super::super::misc::check_guard_attack_special_hi(fighter, false.into()).get_bool()
    {
        true.into()
    } else {
        false.into()
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOff_Main_common)]
unsafe fn status_GuardOff_Main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if sub_status_guard_off_main_common_cancel(fighter).get_bool()
        || sub_status_guard_off_main_common_air(fighter).get_bool()
        || sub_status_guard_off_main_common_control(fighter).get_bool()
    {
        true.into()
    } else {
        false.into()
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOff_Main)]
unsafe fn status_GuardOff_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !status_GuardOff_Main_common(fighter).get_bool()
        && MotionModule::is_end(fighter.module_accessor)
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    L2CValue::I32(0)
}

#[skyline::hook(replace = L2CFighterCommon_sub_guard_off_uniq)]
unsafe fn sub_guard_off_uniq(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if arg.get_bool() {
        let just_frame = WorkModule::get_int(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME,
        );
        if 0 < just_frame {
            WorkModule::dec_int(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME,
            );
            if (just_frame - 1) == 0 {
                ShieldModule::set_status(
                    fighter.module_accessor,
                    *FIGHTER_SHIELD_KIND_GUARD,
                    app::ShieldStatus(*SHIELD_STATUS_NONE),
                    0,
                );
                let type_of_guard =
                    app::FighterUtil::get_shield_type_of_guard(fighter.global_table[0x2].get_i32())
                        as i32;
                ShieldModule::set_shield_type(
                    fighter.module_accessor,
                    app::ShieldType(type_of_guard),
                    *FIGHTER_SHIELD_KIND_GUARD,
                    0,
                );
                ReflectorModule::set_status(
                    fighter.module_accessor,
                    0,
                    app::ShieldStatus(*SHIELD_STATUS_NONE),
                    *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD,
                );

                ModelModule::disable_gold_eye(fighter.module_accessor);
                MotionModule::set_rate(
                    fighter.module_accessor,
                    MotionModule::end_frame(fighter.module_accessor)
                        / (30.0 - fighter.status_frame() as f32),
                );
            }
        }
        let cancel_frame = WorkModule::get_int(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME,
        );
        if 0 < cancel_frame {
            WorkModule::dec_int(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME,
            );
            if (cancel_frame - 1) == 0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }
    L2CValue::I32(0)
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOff_Common)]
unsafe fn status_GuardOff_Common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(
        fighter.object(),
        vars::common::instance::IS_PARRY_FOR_GUARD_OFF,
    ) {
        let enables = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
        ];
        for x in enables.iter() {
            WorkModule::enable_transition_term(fighter.module_accessor, *x);
        }
    }
    let shield_just_frame = WorkModule::get_param_int(
        fighter.module_accessor,
        hash40("common"),
        hash40("shield_just_frame"),
    ) as f32;
    let just_shield_check_frame = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("just_shield_check_frame"),
        0,
    );
    let just_frame = ((shield_just_frame * just_shield_check_frame) + 0.5).round() as i32;
    WorkModule::set_int(
        fighter.module_accessor,
        just_frame,
        *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME,
    );
    let guard_off_cancel_frame = WorkModule::get_param_int(
        fighter.module_accessor,
        hash40("common"),
        hash40("guard_off_cancel_frame"),
    );
    WorkModule::set_int(
        fighter.module_accessor,
        guard_off_cancel_frame,
        *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME,
    );
    let fighter_guard_off_cancel_frame = FighterMotionModuleImpl::get_cancel_frame(
        fighter.module_accessor,
        Hash40::new("guard_off"),
        true,
    );
    let ret_val = if 0.0 < fighter_guard_off_cancel_frame && 0 < guard_off_cancel_frame {
        fighter_guard_off_cancel_frame / (guard_off_cancel_frame as f32)
    } else {
        1.0
    };
    let guard_off_work_cancel_frame = WorkModule::get_int(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME,
    );
    let guard_off_enable_shield_frame = WorkModule::get_param_int(
        fighter.module_accessor,
        hash40("common"),
        hash40("guard_off_enable_shield_frame"),
    );
    WorkModule::set_int(
        fighter.module_accessor,
        guard_off_enable_shield_frame + guard_off_work_cancel_frame,
        *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME,
    );
    WorkModule::set_int(
        fighter.module_accessor,
        guard_off_enable_shield_frame + guard_off_work_cancel_frame,
        *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME,
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_guard_off_uniq(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_guard_off_uniq as *const () as _));
    ret_val.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOff)]
unsafe fn status_GuardOff(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rate = status_GuardOff_Common(fighter).get_f32();
    if VarModule::is_flag(
        fighter.object(),
        vars::common::instance::IS_PARRY_FOR_GUARD_OFF,
    ) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("just_shield_off"),
            0.0,
            0.0,
            false,
            0.0,
            false,
            false,
        );
        app::FighterUtil::flash_eye_info(fighter.module_accessor);
        if !WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL,
        ) {
            ModelModule::enable_gold_eye(fighter.module_accessor);
            WorkModule::on_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE,
            );
        }
        EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new_raw(0xff4f9200f),
            Hash40::new("throw"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            0.5,
            &Vector3f::zero(),
            &Vector3f::zero(),
            false,
            *EFFECT_SUB_ATTRIBUTE_NONE as u32,
            *EFFECT_FLIP_NONE,
            1,
        );
        EffectModule::req_common(fighter.module_accessor, Hash40::new("just_shield"), 0.0);
        // let shield_se = app::FighterUtil::get_just_shield_se(fighter.global_table[0x2].get_i32());
        SoundModule::play_se(
            fighter.module_accessor,
            smash::phx::Hash40::new("se_item_deathscythe_swing_m"),
            true,
            false,
            false,
            false,
            app::enSEType(0),
        );
    } else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("guard_off"),
            0.0,
            rate,
            false,
            0.0,
            false,
            false,
        );
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
