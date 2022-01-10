use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn blue_eggs_land_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
    }
}

// Banjo Grenade Airdodge Cancel
unsafe fn grenade_ac(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if [*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_LW_SHOOT, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) && situation_kind == *SITUATION_KIND_AIR {
        if frame > 15.0 {
            if boma.is_cat_flag(Cat1::JumpButton) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    blue_eggs_land_cancels(boma, status_kind, situation_kind);
    grenade_ac(boma, status_kind, situation_kind, cat[0], frame);
}
#[utils::opff(FIGHTER_KIND_BUDDY )]
pub fn buddy_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		buddy_frame(fighter)
    }
}

pub unsafe fn buddy_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}