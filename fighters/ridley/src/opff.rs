// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn space_pirate_rush_flight(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, stick_x: f32) {
    if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL {
        let motion_value1 = 0.9;
        let motion_value2 = 4.0;
        let mut motion_vec = x_motion_vec(motion_value1, stick_x);

        if situation_kind == *SITUATION_KIND_AIR {
            if stick_x != 0.0 {
                KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                motion_vec.y = motion_value2;
                KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
            }
        }
    }
}

// Ridley Wing Blitz Drift
unsafe fn wing_blitz_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, stick_x: f32, stick_y: f32) {
    let motion_value1 = 0.7;
    let motion_value2 = 0.7;
    if situation_kind == *SITUATION_KIND_AIR {
        if [*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI,
            *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW].contains(&status_kind) {
            if stick_x != 0.0 {
                let motion_vec = x_motion_vec(motion_value1, stick_x);
                KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
            }
        }

        if [*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F,
            *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B].contains(&status_kind) {
            if stick_y != 0.0 {
                let motion_vec = Vector3f{x: 0.0, y: motion_value2 * stick_y.signum(), z: 0.0};
                KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
            }
        }
    }
}

// Puts stabbed opponents into footstool state
// unsafe fn stab_footstool(fighter: &mut L2CFighterCommon) {
//     if VarModule::is_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_IS_THROW) {
//         VarModule::off_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_IS_THROW);
//         let capture_id = VarModule::get_int(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_CATCH_ID);
//         let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
//         if StatusModule::situation_kind(capture_boma) != *SITUATION_KIND_GROUND {
//             StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, false);
//             let hit_stop = StopModule::get_hit_stop_real_frame(capture_boma) as i32;
//             ShakeModule::req(capture_boma, Hash40::new("damage_ground"), hit_stop, false, &Vector2f{x:0.0, y:0.0}, 1.0, 0.0, false, false);
//             SoundModule::stop_se(capture_boma, Hash40::new("se_common_step_jump"), 0);
//         }
//     }
// }

// Angle tail based on stick y position and frame
unsafe fn rotate_bone(boma: &mut BattleObjectModuleAccessor, max_angle: f32, min_angle: f32, strength: f32) {
    let mut angle = min_angle.abs();
    if strength > 0.0 {
        angle = max_angle
    }
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: -((angle * -1.0 * strength) - 2.5)};
    let fighter = utils::util::get_fighter_common_from_accessor(boma);
    fighter.set_joint_rotate("tail1", rotation);
}

// boma: its a boma 
// lean_frame: frame to interpolate to the intended angle amount until
// return_frame: frame to start interpolating back to regular angle
// max_angle: maximum angle you can lean upwards, in degrees
// min_angle: minimum angle that we should be able to rotate downwards, in degrees
unsafe fn tail_lean(boma: &mut BattleObjectModuleAccessor, lean_frame: f32, return_frame: f32, max_angle: f32, min_angle: f32) {
    let stick_y = ControlModule::get_stick_y(boma);
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let tail_y = VarModule::get_float(boma.object(), vars::ridley::status::SKEWER_STICK_Y);
    if frame >= 0.0 && frame < lean_frame {
        // linear interpolate to stick position,
        // while getting stick position still
        VarModule::set_float(boma.object(), vars::ridley::status::SKEWER_STICK_Y, stick_y);
        rotate_bone(boma, max_angle, min_angle, stick_y * ((frame as f32) / 30.0));
    } else if frame >= lean_frame && frame < return_frame {
        // rotate at selected angle for each frame
        rotate_bone(boma, max_angle, min_angle, tail_y);
    } else {
        // linear interpolate back to normal
        rotate_bone(boma, max_angle, min_angle, tail_y * (1.0 - ((frame - return_frame) / (end_frame - return_frame))));
    }
}

// Handles angling of tail
unsafe fn angled_skewer(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.is_situation(*SITUATION_KIND_GROUND) {
        tail_lean(fighter.boma(), 31.0, 41.0, 25.0, -15.0);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_FAILURE,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FAILURE,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_WALL,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL_JUMP,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_CEIL,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL
        ]) 
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
    //space_pirate_rush_flight(boma, status_kind, situation_kind, stick_x);
    wing_blitz_drift(boma, status_kind, situation_kind, stick_x, stick_y);
    //stab_footstool(fighter);
    angled_skewer(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn ridley_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ridley_frame(fighter)
    }
}

pub unsafe fn ridley_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
pub fn install() {
    smashline::Agent::new("ridley")
        .on_line(Main, ridley_frame_wrapper)
        .install();
}
