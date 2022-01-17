// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn special_n_article_fix(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
        if frame <= 1.0 {
            VarModule::off_flag(boma.object(), vars::common::SPECIAL_PROJECTILE_SPAWNED);
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    special_n_article_fix(boma, id, status_kind, situation_kind, frame);

}

#[utils::macros::opff(FIGHTER_KIND_LUIGI )]
pub fn luigi_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		luigi_frame(fighter)
    }
}

pub unsafe fn luigi_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}