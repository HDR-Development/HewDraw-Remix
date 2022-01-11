use ::common::opff_import::*;
use super::*;
use globals::*;
use ::common::opff::*;
 
unsafe fn actionable_teleport_air(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 && situation_kind == *SITUATION_KIND_AIR && frame > 8.0 {
        if hdr::get_jump_count(boma) < hdr::get_jump_count_max(boma) {
            VarModule::on_flag(boma.object(), common::UP_SPECIAL_CANCEL);
            CancelModule::enable_cancel(boma);
        }
    }
}

// Put Mewtwo into fall after teleport if he has his double jump
unsafe fn fall_after_teleport(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 {
        if hdr::get_jump_count(boma) < hdr::get_jump_count_max(boma) {
            VarModule::on_flag(boma.object(), common::UP_SPECIAL_CANCEL);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            if MotionModule::is_end(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    actionable_teleport_air(boma, id, status_kind, situation_kind, frame);
    fall_after_teleport(boma, id, status_kind);
    nspecial_cancels(boma, status_kind, situation_kind);
    teleports::mewtwo_teleport_cancel(boma, status_kind, id);
}
#[utils::opff(FIGHTER_KIND_MEWTWO )]
pub fn mewtwo_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		mewtwo_frame(fighter)
    }
}

pub unsafe fn mewtwo_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}