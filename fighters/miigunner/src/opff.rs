// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn special_waza_charge_handle(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion_one_of(&[
        Hash40::new("special_n3_start"),
        Hash40::new("special_air_n3_start")]) {
        let is_hold = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL);
        let charge = VarModule::get_float(boma.object(), vars::miigunner::status::ATTACK_CHARGE);
        let mut charge_start_frame = 0.0;
        let mut charge_end_frame = 0.0;
        let mut max_charge_frames = ParamModule::get_float(boma.object(), ParamType::Agent, "param_charge.max_charge_frames");

        match MotionModule::motion_kind(boma) {
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
            if boma.is_motion_one_of(&[Hash40::new("special_hi1"), Hash40::new("special_air_hi1")]) {
                let motion_vec = if charge <= 10.0 { Vector3f{ x: 1.0, y: 0.55, z: 1.0 } } else { Vector3f{ x: 1.0, y: 0.35, z: 1.0 } };
                KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let handle = VarModule::get_int64(boma.object(), vars::miigunner::instance::SPECIAL_HI1_LAUNCH_EFFECT_HANDLE);
                EffectModule::set_rate(boma, handle as u32, 1.0/boma.motion_frame());
            }
            let motion_rate = (charge_end_frame - charge_start_frame)/max_charge_frames;
            MotionModule::set_rate(boma, motion_rate);
            VarModule::set_float(boma.object(), vars::miigunner::status::ATTACK_CHARGE, charge + 1.0);
        }
        else {
            if boma.is_motion_one_of(&[Hash40::new("special_n3_start"), Hash40::new("special_air_n3_start")]) {
                VarModule::set_float(boma.object(), vars::miigunner::instance::SPECIAL_N3_CHARGE, charge);
                MotionModule::set_rate(boma, 1.0);
            }
            else if boma.is_motion_one_of(&[Hash40::new("special_hi1"), Hash40::new("special_air_hi1")]) {
                let handle = VarModule::get_int64(boma.object(), vars::miigunner::instance::SPECIAL_HI1_LAUNCH_EFFECT_HANDLE);
                EffectModule::set_rate(boma, handle as u32, 1.0);
                MotionModule::set_rate(boma, 1.0);
            }
        }
    }
}

unsafe fn reflector_jc(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion_one_of(&[Hash40::new("special_lw1_start"), Hash40::new("special_air_lw1_start")]) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 {
        GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    if boma.is_status_one_of(&[
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_HIT,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_END,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_LOOP]) {
        if !boma.is_in_hitlag() {
            if (boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_LOOP) && boma.status_frame() > 1) {
                boma.check_jump_cancel(false, false);
            }
        }
    }
}

unsafe fn laser_blaze_ff_land_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion_one_of(&[
        Hash40::new("special_air_n2_start"),
        Hash40::new("special_air_n2_loop"),
        Hash40::new("special_air_n2_end"),
        Hash40::new("special_n2_start"),
        Hash40::new("special_n2_loop"),
        Hash40::new("special_n2_end") ]) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.is_prev_situation(*SITUATION_KIND_AIR) {
            fighter.set_float(6.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
        if StatusModule::is_changing(fighter.module_accessor)
        && fighter.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
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

unsafe fn stealth_burst_land_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S2_END) {
        if boma.is_situation(*SITUATION_KIND_GROUND) && boma.is_prev_situation(*SITUATION_KIND_AIR) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
    }
}

unsafe fn vortex_item_grab_ac(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HIT, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_END])
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !fighter.is_in_hitlag() {
        fighter.check_airdodge_cancel();
    }
    if fighter.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_END) {
        if fighter.status_frame() < 6 {
            fighter.try_pickup_item(15.0, Some(Hash40::new("top")), Some(&Vector2f::new(0.0, 0.0)));
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
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_END,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HIT,
                *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD
            ])
        )
    )
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    special_waza_charge_handle(boma);
    reflector_jc(boma);
    laser_blaze_ff_land_cancel(fighter);
    remove_homing_missiles(boma);
    missile_land_cancel(boma);
    stealth_burst_land_cancel(boma);
    vortex_item_grab_ac(fighter);
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
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, miigunner_frame_wrapper);
}