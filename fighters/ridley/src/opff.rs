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

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //space_pirate_rush_flight(boma, status_kind, situation_kind, stick_x);
    wing_blitz_drift(boma, status_kind, situation_kind, stick_x, stick_y);
    //stab_footstool(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_RIDLEY )]
pub fn ridley_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
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