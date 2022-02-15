use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);
 
unsafe fn float_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        let prev_status_0 = StatusModule::prev_status_kind(boma, 0);
        let prev_status_1 = StatusModule::prev_status_kind(boma, 1);
        if [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START].contains(&prev_status_0)
            || [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START].contains(&prev_status_1) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    float_cancel(boma, status_kind);
}

#[::utils::macros::opff(FIGHTER_KIND_PEACH )]
pub fn peach_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		peach_frame(fighter)
    }
}

pub unsafe fn peach_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}