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

// Wild Throw
unsafe fn wild_throw(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if boma.is_motion_one_of(&[Hash40::new("special_lw3"),Hash40::new("special_air_lw3")]) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH, false);
    }
    if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH {
        if frame < 15.0 {
            StatusModule::set_keep_situation_air(boma, true);
        } else {
            StatusModule::set_keep_situation_air(boma, false);
        }
    }
}

//Onslaught Shield Activation + No Freefall on hit
unsafe fn onslaught(boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if boma.is_motion_one_of(&[Hash40::new("special_s1_start"),Hash40::new("special_air_s1_start")]) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END, true);
        }
    }
    if boma.is_motion_one_of(&[Hash40::new("special_air_s1_end")]) {
        if frame > 60.0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
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
        if fighter.motion_frame() < 3.0 {
            VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE, 0);
            VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE_DISTANCE, 0.0);
        }
        if MotionModule::end_frame(boma) - fighter.motion_frame() < 2.0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
        //println!("is_hold: {}, charge: {}, charge_distance: {}, is_ground: {}", is_hold, charge, charge_distance, is_ground);
        if (charge_start_frame..charge_end_frame).contains(&fighter.motion_frame()) && charge < (max_charge_frames as i32) && is_hold {
            MotionModule::set_rate(fighter.module_accessor, (charge_end_frame - charge_start_frame)/max_charge_frames);
            let eff_handle = VarModule::get_int64(fighter.battle_object, vars::miifighter::instance::QUAKE_EFFECT_HANDLER);
            let pos_offset = charge_distance + (max_charge_distance/max_charge_frames);
            let mut eff_pos_offset = (charge as f32/max_charge_frames) + charge_distance + (max_charge_distance/max_charge_frames);
            if is_ground {
                VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE_DISTANCE, pos_offset);
                eff_pos_offset = (10.0 - 10.0 * (charge as f32/max_charge_frames)) + charge_distance + (max_charge_distance/max_charge_frames);
            }
            EffectModule::set_pos(boma, eff_handle as u32, &Vector3f::new(0.0, 0.0, eff_pos_offset));
            VarModule::set_int64(fighter.battle_object, vars::miifighter::instance::QUAKE_EFFECT_HANDLER, eff_handle as u64);
            VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE, (charge + 1) as i32);
        } else {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    }
    //Allows Divekick to be cancelled into freefall with second B press
    if boma.is_motion_one_of(&[Hash40::new("special_lw1_loop")]) {
        let is_press = ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
        if is_press || fighter.motion_frame() >= 30.0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
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
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END
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
    feint_jump_jc(boma);
    wild_throw(boma, status_kind, frame);
    earthquake_punch(fighter, boma);
    onslaught(boma, frame);
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
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install() {
    smashline::Agent::new("miifighter")
        .on_line(Main, miifighter_frame_wrapper)
        .install();
}