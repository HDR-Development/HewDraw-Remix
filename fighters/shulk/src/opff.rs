// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn air_slash_cancels(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if frame > 22.0 {
                if boma.is_cat_flag(Cat1::AirEscape) {
                    VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // Magic Series
    air_slash_cancels(boma, id, status_kind, cat[0], frame);
}

#[utils::macros::opff(FIGHTER_KIND_SHULK )]
pub fn shulk_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		shulk_frame(fighter)
    }
}

pub unsafe fn shulk_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}