utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Feint Jump Jump Cancel
unsafe fn feint_jump_jc(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion_one_of(&[Hash40::new("special_lw2_start"),Hash40::new("special_air_lw2_start")]) {
        if MotionModule::frame(boma) > 31.0 {
            if !boma.is_in_hitlag() {
                boma.check_jump_cancel(false, false);
            }
        }
    }
}

//Earthquake Punch
unsafe fn earthquake_punch(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND) {
        let is_hold = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
        let charge = VarModule::get_int(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE);
        let charge_distance = VarModule::get_float(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE_DISTANCE) as f32;
        let charge_start_frame = ParamModule::get_float(boma.object(), ParamType::Agent, "earthquake_fist_ground.charge_start_frame");
        let charge_end_frame = ParamModule::get_float(boma.object(), ParamType::Agent, "earthquake_fist_ground.charge_end_frame");
        let max_charge_frames = ParamModule::get_float(boma.object(), ParamType::Agent, "earthquake_fist_ground.max_charge_frames");
        let max_charge_distance = ParamModule::get_float(boma.object(), ParamType::Agent, "earthquake_fist_ground.max_charge_distance");
        let lr = PostureModule::lr(fighter.module_accessor);
        let is_ground = GroundModule::ray_check(
            fighter.module_accessor, 
            &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor) + ((charge_distance + 12.0) * lr), y: PostureModule::pos_y(fighter.module_accessor)}, 
            &Vector2f{ x: 0.0, y: -6.0}, true
        ) == 1;
        if MotionModule::end_frame(boma) - fighter.motion_frame() < 2.0 {
            // reimpl status
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
        //println!("is_hold: {}, charge: {}, charge_distance: {}, is_ground: {}", is_hold, charge, charge_distance, is_ground);
        if (charge_start_frame..charge_end_frame).contains(&fighter.motion_frame()) && charge < (max_charge_frames as i32) && is_hold {
            MotionModule::set_rate(fighter.module_accessor, (charge_end_frame - charge_start_frame)/max_charge_frames);
            let eff_handle = VarModule::get_int64(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW1_QUAKE_EFFECT_HANDLE);
            let pos_offset = charge_distance + (max_charge_distance/max_charge_frames);
            let mut eff_pos_offset = (charge as f32/max_charge_frames) + charge_distance + (max_charge_distance/max_charge_frames);
            if is_ground {
                VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE_DISTANCE, pos_offset);
                eff_pos_offset = (10.0 - 10.0 * (charge as f32/max_charge_frames)) + charge_distance + (max_charge_distance/max_charge_frames);
            }
            EffectModule::set_pos(boma, eff_handle as u32, &Vector3f::new(0.0, 0.0, eff_pos_offset));
            VarModule::set_int64(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW1_QUAKE_EFFECT_HANDLE, eff_handle as u64);
            VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE, (charge + 1) as i32);
        } else {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    }
}

// TODO: create cancel animation for aerial EQF cancel
// Prevents aerial EQF cancel from grabbing ledge for first 7f
unsafe fn eqf_cancel_ledgegrab_lockout(fighter: &mut L2CFighterCommon) {
    if fighter.is_prev_status(*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR)
    && fighter.is_status(*FIGHTER_STATUS_KIND_FALL_SPECIAL) {
        if StatusModule::is_changing(fighter.module_accessor) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        }
        if fighter.status_frame() == 6 {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && (
        fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N])
        || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_1,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_1,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1
            ].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO))
            && fighter.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END,
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_2
            ])
        )
        || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_2,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_2,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2
            ].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO))
            && fighter.is_status_one_of(&[
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_MISS,
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH,
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH_MISS,
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI2_END
            ])
        )
        || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_3,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_3,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3
            ].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO))
            && fighter.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_TURN,
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S3_THROW,
            ])
        )
    )
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    feint_jump_jc(boma);
    earthquake_punch(fighter, boma);
    eqf_cancel_ledgegrab_lockout(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn miifighter_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        miifighter_frame(fighter)
    }
}

pub unsafe fn miifighter_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, miifighter_frame_wrapper);
}