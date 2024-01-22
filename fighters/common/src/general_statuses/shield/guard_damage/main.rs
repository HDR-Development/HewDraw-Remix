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
    if !arg.get_bool() {
        fighter.FighterStatusGuard__landing_effect_control();
        return L2CValue::I32(0);
    }

    let mut min_frame = WorkModule::get_int(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME
    );
    if 0 < min_frame {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        min_frame -= 1;
    }

    if
        ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) &&
        min_frame <= 0
    {
        WorkModule::unable_transition_term(
            fighter.module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD
        );
    }

    let just_frame = WorkModule::get_int(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME
    );
    if
        WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD
        ) &&
        0 < just_frame
    {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        if just_frame - 1 == 0 {
            ShieldModule::set_status(
                fighter.module_accessor,
                *FIGHTER_SHIELD_KIND_GUARD,
                app::ShieldStatus(*SHIELD_STATUS_NONE),
                0
            );
            let type_of_guard = app::FighterUtil::get_shield_type_of_guard(
                fighter.global_table[0x2].get_i32()
            ) as i32;
            ShieldModule::set_shield_type(
                fighter.module_accessor,
                app::ShieldType(type_of_guard),
                *FIGHTER_SHIELD_KIND_GUARD,
                0
            );
            ReflectorModule::set_status(
                fighter.module_accessor,
                0,
                app::ShieldStatus(*SHIELD_STATUS_NONE),
                *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD
            );
        }
    }
    if
        !WorkModule::count_down_int(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME,
            0
        )
    {
        return L2CValue::I32(0);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        if
            WorkModule::is_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE
            )
        {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) {
                ModelModule::disable_gold_eye(fighter.module_accessor);
            }
            WorkModule::off_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE
            );
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU) {
            HitModule::set_whole(fighter.module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
            WorkModule::off_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU
            );
        }
        EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new_raw(0xc004c15ac),
            Hash40::new("top"),
            &ZERO_VEC,
            &ZERO_VEC,
            1.0,
            &ZERO_VEC,
            &ZERO_VEC,
            false,
            0,
            0,
            0
        );
    } else {
        let disable_frame = WorkModule::get_param_int(
            fighter.module_accessor,
            hash40("common"),
            hash40("guard_damage_just_shield_disable_frame")
        );
        WorkModule::set_int(
            fighter.module_accessor,
            disable_frame,
            *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_JUST_SHIELD_FRAME
        );
    }
    L2CValue::I32(0)
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage_common)]
unsafe fn status_GuardDamage_common(fighter: &mut L2CFighterCommon, arg: L2CValue) {
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_sub_x(fighter.module_accessor);
    fighter.global_table[FLICK_X].assign(&L2CValue::I32(0xfe));
    if
        !WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD
        )
    {
        WorkModule::enable_transition_term(
            fighter.module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD
        );
        if
            !WorkModule::is_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM
            )
        {
            notify_event_msc_cmd!(
                fighter,
                Hash40::new_raw(0x20cbc92683),
                1,
                FIGHTER_LOG_DATA_INT_GUARD_DAMAGE_NUM
            );
            WorkModule::on_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM
            );
        }
        if arg.get_bool() {
            let prev_shield = WorkModule::get_float(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD
            );
            let prev_shield_scale = fighter
                .FighterStatusGuard__calc_shield_scale(prev_shield.into())
                .get_f32();
            let curr_shield = WorkModule::get_float(
                fighter.module_accessor,
                *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD
            );
            let curr_shield_scale = fighter
                .FighterStatusGuard__calc_shield_scale(curr_shield.into())
                .get_f32();
            let team_color = app::FighterUtil::get_team_color(fighter.module_accessor);
            let color = app::FighterUtil::get_effect_team_color(
                app::EColorKind(team_color as i32),
                Hash40::new("shield_effect_color")
            );
            let color = smash::phx::Vector3f {
                x: color.x,
                y: color.y,
                z: color.z,
            };
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new_raw(0x12c9377e3d),
                Hash40::new("throw"),
                &ZERO_VEC,
                &ZERO_VEC,
                0.1,
                false,
                *EFFECT_SUB_ATTRIBUTE_NONE as u32,
                0,
                -1,
                *EFFECT_FLIP_NONE,
                1,
                false,
                false
            );
            EffectModule::set_rgb_partial_last(fighter.module_accessor, color.x, color.y, color.z);
            let effect2_handle = EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new_raw(0x12be304eab),
                Hash40::new("throw"),
                &ZERO_VEC,
                &ZERO_VEC,
                0.1,
                false,
                *EFFECT_SUB_ATTRIBUTE_NONE as u32,
                0,
                -1,
                *EFFECT_FLIP_NONE,
                1,
                false,
                false
            ) as u32;
            EffectModule::set_rgb_partial_last(fighter.module_accessor, color.x, color.y, color.z);
            WorkModule::set_int(
                fighter.module_accessor,
                effect2_handle as i32,
                *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE2_EFFECT_HANDLE
            );
            let effect_handle = EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new_raw(0x113434cb66),
                Hash40::new("throw"),
                &ZERO_VEC,
                &ZERO_VEC,
                1.0,
                false,
                *EFFECT_SUB_ATTRIBUTE_NONE as u32,
                0,
                -1,
                *EFFECT_FLIP_NONE,
                1,
                false,
                false
            ) as u32;
            EffectModule::set_rgb_partial_last(fighter.module_accessor, color.x, color.y, color.z);
            WorkModule::set_int(
                fighter.module_accessor,
                effect_handle as i32,
                *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE_EFFECT_HANDLE
            );
            if effect_handle != 0 {
                let effect_scale = 0.1 * (curr_shield_scale / prev_shield_scale);
                EffectModule::set_scale(
                    fighter.module_accessor,
                    effect_handle,
                    &(Vector3f {
                        x: effect_scale,
                        y: effect_scale,
                        z: effect_scale,
                    })
                );
            }
        }
    } else {
        WorkModule::set_int(
            fighter.module_accessor,
            0,
            *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME
        );
        WorkModule::unable_transition_term(
            fighter.module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD
        );
        WorkModule::set_int(
            fighter.module_accessor,
            0,
            *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME
        );
        WorkModule::set_int(
            fighter.module_accessor,
            0,
            *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME
        );
        HitModule::set_whole(fighter.module_accessor, app::HitStatus(*HIT_STATUS_XLU), 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU);
        let just_shield_precede_extension = WorkModule::get_param_int(
            fighter.module_accessor,
            hash40("common"),
            hash40("just_shield_precede_extension")
        );
        notify_event_msc_cmd!(
            fighter,
            Hash40::new_raw(0x20cbc92683),
            1,
            *FIGHTER_LOG_DATA_INT_JUST_SHIELD
        );
        app::FighterUtil::flash_eye_info(fighter.module_accessor);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) {
            ModelModule::enable_gold_eye(fighter.module_accessor);
            WorkModule::on_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE
            );
        }
        EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new_raw(0xff4f9200f),
            Hash40::new("throw"),
            &ZERO_VEC,
            &ZERO_VEC,
            1.0,
            &ZERO_VEC,
            &ZERO_VEC,
            false,
            *EFFECT_SUB_ATTRIBUTE_NONE as u32,
            *EFFECT_FLIP_NONE,
            1
        );
        let shield_lr = WorkModule::get_float(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR
        );
        ColorBlendModule::set_last_attack_direction(
            fighter.module_accessor,
            &(Vector3f {
                x: -shield_lr,
                y: 0.0,
                z: 0.0,
            })
        );
        EffectModule::req_common(fighter.module_accessor, Hash40::new("just_shield"), 0.0);
        if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_GUARD_OFF {
            EffectModule::req_screen(
                fighter.module_accessor,
                Hash40::new_raw(0x12698ccf2b),
                false,
                false,
                false
            );
        }
        let shield_se = app::FighterUtil::get_just_shield_se(fighter.global_table[0x2].get_i32());
        SoundModule::play_se(
            fighter.module_accessor,
            shield_se,
            true,
            false,
            false,
            false,
            app::enSEType(0)
        );
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
        true.into()
    } else {
        false.into()
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_guard_damage_main_common)]
unsafe fn status_guard_damage_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_stick_y = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("common"),
        hash40("special_stick_y")
    );
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if
        special_stick_y <= fighter.global_table[STICK_Y].get_f32() &&
        (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0
    {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_SPECIAL_HI);
    }

    if fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        if
            fighter.is_button_release(Buttons::Guard) ||
            WorkModule::get_int(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME
            ) == 0
        {
            WorkModule::enable_transition_term_group(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD
            );
        }
    } else if
        WorkModule::is_enable_transition_term(
            fighter.module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD
        ) &&
        WorkModule::get_int(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME
        ) == 0
    {
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD.into(), false.into());
        return true.into();
    }

    if
        WorkModule::get_int(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME
        ) == 0 ||
        fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD)
    {
        if
            WorkModule::is_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD
            )
        {
            if
                CancelModule::is_enable_cancel(fighter.module_accessor) &&
                fighter.sub_wait_ground_check_common(false.into()).get_bool()
            {
                return L2CValue::I32(0);
            }
            if
                MotionModule::is_end(fighter.module_accessor) &&
                fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
            {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return L2CValue::I32(0);
            }
        } else if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
            VarModule::off_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
            return true.into();
        }
    }

    let damage = WorkModule::get_int(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_DAMAGE
    );
    let setoff_escape = WorkModule::get_param_int(
        fighter.module_accessor,
        hash40("common"),
        hash40("shield_setoff_escape")
    );
    let cat2 = fighter.global_table[CMD_CAT2].get_i32();

    if
        setoff_escape <= damage &&
        fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND &&
        ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    {
        let escapes = [
            (*FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE),
            (*FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_F),
            (*FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_B),
        ];

        for (flag, status) in escapes.iter() {
            if (cat2 & *flag) != 0 {
                WorkModule::on_flag(
                    fighter.module_accessor,
                    *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_TRANSITION_STATUS_STOP
                );
                WorkModule::on_flag(
                    fighter.module_accessor,
                    *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F
                );
                fighter.change_status(L2CValue::I32(*status), true.into());
                return true.into();
            }
        }
    }

    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage_Main)]
unsafe fn status_GuardDamage_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    misc::check_enable_cstick_buffer_rolls(fighter);
    if !status_guard_damage_main_common_air(fighter).get_bool() {
        if
            fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND &&
            WorkModule::is_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD
            ) &&
            !fighter.FighterStatusGuard__is_continue_just_shield_count().get_bool()
        {
            let is_hit = StopModule::is_hit(fighter.module_accessor);
            if is_hit {
                WorkModule::on_flag(
                    fighter.module_accessor,
                    *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_TRANSITION_STATUS_STOP
                );
            }
            if
                (CancelModule::is_enable_cancel(fighter.module_accessor) || true) &&
                fighter.sub_wait_ground_check_common(false.into()).get_bool() &&
                is_hit
            {
                StopModule::cancel_hit_stop(fighter.module_accessor);
                return L2CValue::I32(0);
            }
            if is_hit {
                WorkModule::off_flag(
                    fighter.module_accessor,
                    *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_TRANSITION_STATUS_STOP
                );
            }
        }
        status_guard_damage_main_common(fighter);
    }
    L2CValue::I32(0)
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage)]
unsafe fn status_GuardDamage(fighter: &mut L2CFighterCommon) -> L2CValue {
    misc::check_enable_cstick_buffer_rolls(fighter);
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
