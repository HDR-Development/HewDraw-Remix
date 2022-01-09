use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn special_s_article_fix(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
        if frame <= 1.0 {
            special_projectile_spawned[id] = false;
        }
    }
}

// Ivysaur Razor Leaf Airdodge Cancel
unsafe fn razorleaf_adc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_S || situation_kind != *SITUATION_KIND_AIR {
        return;
    }

    if frame > 23.0 {
        if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    special_s_article_fix(boma, id, status_kind, situation_kind, frame);
    razorleaf_adc(boma, status_kind, situation_kind, cat[0], frame);
}

#[utils::opff(FIGHTER_KIND_PFUSHIGISOU )]
pub fn pfushigisou_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		pfushigisou_frame(fighter)
    }
}

pub unsafe fn pfushigisou_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}