use super::*;
use globals::*;

// FIGHTER_STATUS_KIND_GUARD_OFF //

pub unsafe extern "C" fn guard_off_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rate = fighter.status_GuardOff_Common().get_f32();
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
        // app::FighterUtil::flash_eye_info(fighter.module_accessor);
        // if !WorkModule::is_flag(
        //     fighter.module_accessor,
        //     *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL,
        // ) {
        //     ModelModule::enable_gold_eye(fighter.module_accessor);
        //     WorkModule::on_flag(
        //         fighter.module_accessor,
        //         *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE,
        //     );
        // }
        let shield_radius = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);
        let lr = PostureModule::lr(fighter.module_accessor);
        EffectModule::req_follow(
            fighter.module_accessor,
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
            false,
        );
        EffectModule::set_rate_last(fighter.module_accessor, 1.2);
        // EffectModule::set_alpha_last(fighter.module_accessor, 0.4);
        EffectModule::req_common(fighter.module_accessor, Hash40::new("just_shield"), 0.0);
        // let shield_se = app::FighterUtil::get_just_shield_se(fighter.global_table[0x2].get_i32());
        let sfx_handle = SoundModule::play_se(
            fighter.module_accessor,
            smash::phx::Hash40::new("se_item_backshield_guard01"),
            true,
            false,
            false,
            false,
            app::enSEType(0),
        );
        SoundModule::set_se_vol(fighter.module_accessor, sfx_handle as i32, 0.9, 0);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_yoshi_guardon"), 0);
    } else {
        let guard_off_motion_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Common, "guard_off_motion_start_frame");
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("guard_off"),
            guard_off_motion_start_frame,
            rate,
            false,
            0.0,
            false,
            false,
        );
    };
    fighter.sub_shift_status_main(L2CValue::Ptr(guard_off_main_loop as *const () as _))
}

unsafe extern "C" fn guard_off_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_GuardOff_Main_common().get_bool() {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT),
                    L2CValue::Bool(false),
                );
                return 1.into();
            }
            if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_JUMP_TRIGGER != 0 {
                if WorkModule::is_enable_transition_term(
                    fighter.module_accessor,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
                ) {
                    fighter.change_status(
                        L2CValue::I32(*FIGHTER_STATUS_KIND_ESCAPE),
                        L2CValue::Bool(false),
                    );
                    return 1.into();
                }
            }
        }
    }
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD_OFF, guard_off_main);
}
