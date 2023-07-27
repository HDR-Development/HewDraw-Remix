// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Super Sheet Stall
unsafe fn super_sheet_stall(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) {
        let motion_vec = Vector3f{x: 0.0, y: 2.5, z: 0.0};
        let motion_vec_2 = Vector3f{x: 0.75, y: 0.0, z: 0.0};
        if boma.is_situation(*SITUATION_KIND_AIR) {
            if boma.status_frame() >= 2 && boma.status_frame() < 3 {
                KineticModule::mul_speed(boma, &motion_vec_2, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            if boma.status_frame() >= 10 && boma.status_frame() <= 13 {
                KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //super_sheet_stall(boma, status_kind, situation_kind, frame);
}

#[utils::macros::opff(FIGHTER_KIND_MARIOD )]
pub fn mariod_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		mariod_frame(fighter)
    }
}

pub unsafe fn mariod_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}