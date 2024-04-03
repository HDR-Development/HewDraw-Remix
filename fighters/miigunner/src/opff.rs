// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn charge_handle(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion_one_of(&[
        Hash40::new("attack_air_lw"),
        Hash40::new("special_n3_start"),
        Hash40::new("special_air_n3_start"),
        Hash40::new("special_hi1"),
        Hash40::new("special_air_hi1")]) {
        let is_hold =
            if boma.is_motion(Hash40::new("attack_air_lw")) {
                ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)
            } else {
                ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
            };
        let charge = VarModule::get_float(boma.object(), vars::miigunner::status::CURRENT_CHARGE);
        let mut charge_start_frame = 0.0;
        let mut charge_end_frame = 0.0;
        let mut max_charge_frames = ParamModule::get_float(boma.object(), ParamType::Agent, "param_charge.max_charge_frames");

        match MotionModule::motion_kind(boma) {
            _ if boma.is_motion(Hash40::new("attack_air_lw")) => {
                charge_start_frame = ParamModule::get_float(boma.object(), ParamType::Agent, "param_charge.attack_air_lw_charge_start");
                charge_end_frame = ParamModule::get_float(boma.object(), ParamType::Agent, "param_charge.attack_air_lw_charge_end");
            },
            _ if boma.is_motion_one_of(&[Hash40::new("special_n3_start"), Hash40::new("special_air_n3_start")]) => {
                charge_start_frame = ParamModule::get_float(boma.object(), ParamType::Agent, "param_charge.special_n3_charge_start");
                charge_end_frame = ParamModule::get_float(boma.object(), ParamType::Agent, "param_charge.special_n3_charge_end");
            },
            _ if boma.is_motion_one_of(&[Hash40::new("special_hi1"), Hash40::new("special_air_hi1")]) => {
                charge_start_frame = ParamModule::get_float(boma.object(), ParamType::Agent, "param_charge.special_hi1_charge_start");
                charge_end_frame = ParamModule::get_float(boma.object(), ParamType::Agent, "param_charge.special_hi1_charge_end");
            },
            _ => {}
        }

        if (charge_start_frame..charge_end_frame).contains(&boma.motion_frame()) && charge < max_charge_frames && is_hold {
            if boma.is_motion(Hash40::new("attack_air_lw")) {
                let handle = VarModule::get_int64(boma.object(), vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
                EffectModule::set_scale(boma, handle as u32, &Vector3f::new(0.75 + 0.018 * charge, 0.75 + 0.018 * charge, 0.75 + 0.018 * charge));
            }
            else if boma.is_motion_one_of(&[Hash40::new("special_hi1"), Hash40::new("special_air_hi1")]) {
                let motion_vec = if charge <= 10.0 { Vector3f{ x: 1.0, y: 0.55, z: 1.0 } } else { Vector3f{ x: 1.0, y: 0.35, z: 1.0 } };
                KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let handle = VarModule::get_int64(boma.object(), vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
                EffectModule::set_rate(boma, handle as u32, 1.0/boma.motion_frame());
            }
            let motion_rate = (charge_end_frame - charge_start_frame)/max_charge_frames;
            MotionModule::set_rate(boma, motion_rate);
            VarModule::set_float(boma.object(), vars::miigunner::status::CURRENT_CHARGE, charge + 1.0);
        }
        else {
            if boma.is_motion(Hash40::new("attack_air_lw")) {
                let handle = VarModule::get_int64(boma.object(), vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
                EffectModule::set_rate(boma, handle as u32, 1.0);
                MotionModule::set_rate(boma, 1.0);
            }
            else if boma.is_motion_one_of(&[Hash40::new("special_n3_start"), Hash40::new("special_air_n3_start")]) {
                VarModule::set_float(boma.object(), vars::miigunner::instance::GRENADE_CHARGE, charge);
                MotionModule::set_rate(boma, 1.0);
            }
            else if boma.is_motion_one_of(&[Hash40::new("special_hi1"), Hash40::new("special_air_hi1")]) {
                let handle = VarModule::get_int64(boma.object(), vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
                EffectModule::set_rate(boma, handle as u32, 1.0);
                MotionModule::set_rate(boma, 1.0);
            }
        }
    }
}
 
unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL) {
        if boma.is_situation(*SITUATION_KIND_AIR) {
            if WorkModule::get_int(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            }
        }
    }
}

unsafe fn reflector_jc(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 {
        GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    if boma.is_status_one_of(&[
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_HIT,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_END,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_LOOP]) {
        if !boma.is_in_hitlag() {
            if (boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD) && boma.status_frame() > 3)
                || !boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD)
            {
                boma.check_jump_cancel(false, false);
            }
        }
        if boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD) {
            if PostureModule::lr(boma) * ControlModule::get_stick_x(boma) < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

unsafe fn laser_blaze_ff_land_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion_one_of(&[
        Hash40::new("special_air_n2_start"),
        Hash40::new("special_air_n2_loop"),
        Hash40::new("special_air_n2_end"),
        Hash40::new("special_n2_start"),
        Hash40::new("special_n2_loop"),
        Hash40::new("special_n2_end") ]) {
        if boma.is_situation(*SITUATION_KIND_GROUND) && boma.is_prev_situation(*SITUATION_KIND_AIR) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if boma.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
}

unsafe fn remove_homing_missiles(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_GROUND) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND, false);
    }
    else if boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR, false);
    }
}

unsafe fn missile_land_cancel(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status_one_of(&[
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR ]) {
        if boma.is_situation(*SITUATION_KIND_GROUND) && boma.is_prev_situation(*SITUATION_KIND_AIR) {
            if boma.status_frame() > 23 {
                MotionModule::set_frame(boma, 40.0, false);
            }
        }
    }
}

unsafe fn arm_rocket_airdash(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if fighter.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH) && fighter.status_frame() > 16 {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END, false);
    }
    if fighter.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END) && fighter.status_frame() > 11 {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
    }
}

/// Allow uncharged or slightly charged Lunar Launch to be actionable
unsafe fn lunar_launch_actionability(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        if fighter.status_frame() >= 35 && VarModule::get_float(fighter.battle_object, vars::miigunner::status::CURRENT_CHARGE) <= 10.0 {
            // if already used once this airtime
            if VarModule::is_flag(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED) {
                VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            }
            else {
                VarModule::on_flag(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

/// Resets Lunar Launch use count
unsafe fn lunar_launch_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_status(*FIGHTER_STATUS_KIND_CLIFF_CATCH) {
        VarModule::off_flag(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED);
    }
}

/// Turn off funny slowed down smoke when hit
unsafe fn lunar_launch_effect_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_prev_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL ]) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miigunner_bottom_shot"), false, false);
    }
}

unsafe fn stealth_burst_land_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S2_END) {
        if boma.is_situation(*SITUATION_KIND_GROUND) && boma.is_prev_situation(*SITUATION_KIND_AIR) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && (
        fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
        ])
        || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_1,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_1,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1
            ].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO))
            && fighter.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_HOLD,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_START,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_END,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_HIT,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_LOOP
            ])
        )
        || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_2,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_2,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2
            ].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO))
            && fighter.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S2_END,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI2_JUMP,
            ])
        )
        || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_3,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_3,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3
            ].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO))
            && fighter.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_LOOP,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_END,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_END,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HIT,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD
            ])
        )
    )
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    charge_handle(boma);
    nspecial_cancels(boma);
    reflector_jc(boma);
    laser_blaze_ff_land_cancel(boma);
    remove_homing_missiles(boma);
    missile_land_cancel(boma);
	arm_rocket_airdash(fighter);
    lunar_launch_actionability(fighter);
    lunar_launch_reset(fighter);
    lunar_launch_effect_reset(fighter);
    stealth_burst_land_cancel(boma);
    fastfall_specials(fighter);
}

pub extern "C" fn miigunner_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		    miigunner_frame(fighter)
    }
}

pub unsafe fn miigunner_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, miigunner_frame_wrapper);
}
