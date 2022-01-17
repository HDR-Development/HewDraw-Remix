// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    

    // Frame Data
    frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if motion_kind == hash40("attack_air_b") {
            if frame >= 11.0 && frame < 15.0 {
                MotionModule::set_rate(boma, 0.4);
            }
            if frame >= 15.0 {
                MotionModule::set_rate(boma, 1.0);
            }
        }
    }
}

// symbol-based call for the samus' common opff
extern "Rust" {
    fn samus_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}


#[utils::macros::opff(FIGHTER_KIND_SAMUSD )]
pub fn samusd_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		samusd_frame(fighter);
        samus_common(fighter);
    }
}

pub unsafe fn samusd_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}