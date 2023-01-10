// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
/*unsafe fn egg_roll_jc_waveland(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_TURN].contains(&status_kind)
    {
        boma.check_airdodge_cancel();
    }

    if status_kind == *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END {
        boma.check_jump_cancel(true);
    }
}
*/


pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //egg_roll_jc_waveland(boma, status_kind, situation_kind, cat[0], stick_x, facing);
}


#[utils::macros::opff(FIGHTER_KIND_YOSHI )]
pub fn yoshi_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		yoshi_frame(fighter)
    }
}

pub unsafe fn yoshi_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}