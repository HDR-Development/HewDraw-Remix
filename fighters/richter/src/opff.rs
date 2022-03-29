// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff, check_b_reverse});
use super::*;
use globals::*;

 
unsafe fn cross_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && frame > 23.0)
        || (status_kind == *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2 && frame > 28.0) {
        if frame > 23.0 {
            if situation_kind == *SITUATION_KIND_GROUND {
                if boma.is_cat_flag(Cat1::Dash) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, false);
                }
                if boma.is_cat_flag(Cat1::TurnDash) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN_DASH, false);
                }
            }
        }
    }
}

// Richter Holy Water B-Reverse
unsafe fn holy_water_b_rev(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        common::opff::check_b_reverse(fighter);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    cross_dash_cancel(boma, status_kind, situation_kind, cat[0], frame);
    holy_water_b_rev(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_RICHTER )]
pub fn richter_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		richter_frame(fighter)
    }
}

pub unsafe fn richter_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}