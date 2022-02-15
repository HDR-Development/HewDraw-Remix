// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn flame_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    let prev_situation = StatusModule::prev_situation_kind(boma);
    if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_N || situation_kind != *SITUATION_KIND_GROUND || prev_situation != *SITUATION_KIND_AIR {
        return;
    }

    if frame < 18.0 {
        MotionModule::set_frame(boma, 18.0, true);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    flame_cancel(boma, status_kind, situation_kind, frame);

    // Frame Data
    //frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if motion_kind == hash40("attack_air_n") {
            if frame < 7.0 {
                MotionModule::set_rate(boma, 1.75);
            }
            if frame >= 7.0 {
                MotionModule::set_rate(boma, 1.0);
            }
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_PLIZARDON )]
pub fn plizardon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		plizardon_frame(fighter)
    }
}

pub unsafe fn plizardon_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}