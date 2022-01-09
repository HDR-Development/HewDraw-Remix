use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn space_pirate_rush_flight(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, stick_x: f32) {
    if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL {
        let motion_value1 = 0.9;
        let motion_value2 = 4.0;
        let mut motion_vec = moveset_utils::x_motion_vec(motion_value1, stick_x);

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
    let motion_value1 = 1.1;
    let motion_value2 = 1.1;
    if situation_kind == *SITUATION_KIND_AIR {
        if [*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI,
            *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW].contains(&status_kind) {
            if stick_x != 0.0 {
                let motion_vec = moveset_utils::x_motion_vec(motion_value1, stick_x);
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

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    space_pirate_rush_flight(boma, status_kind, situation_kind, stick_x);
    wing_blitz_drift(boma, status_kind, situation_kind, stick_x, stick_y);
}

#[utils::opff(FIGHTER_KIND_RIDLEY )]
pub fn ridley_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		ridley_frame(fighter)
    }
}

pub unsafe fn ridley_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}