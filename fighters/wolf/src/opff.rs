// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn airdodge_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if frame > 16.0 {
            boma.check_airdodge_cancel();
        }
    }
}

// Wolf Shine Jump Cancels
unsafe fn shine_jump_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    if ((fighter.is_status (*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.motion_frame() > 6.0)  // Allows for jump cancel on frame 5 in game
        || fighter.is_status_one_of(&[
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT,
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP,
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END]))
        && !fighter.is_in_hitlag()
        {
            fighter.check_jump_cancel(false);
        }
}

// Side Special Cancels
unsafe fn side_special_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            if frame > 20.0 {
                CancelModule::enable_cancel(boma);
            }
            if frame >= MotionModule::end_frame(boma) - 3.0{
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    airdodge_cancel(boma, status_kind, situation_kind, cat[0], frame);
    shine_jump_cancel(fighter);

    // Magic Series
    side_special_cancels(boma, status_kind, frame);

    // Frame Data
    //frame_data(boma, status_kind, motion_kind, frame);
}

#[utils::macros::opff(FIGHTER_KIND_WOLF )]
pub fn wolf_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		wolf_frame(fighter)
    }
}

pub unsafe fn wolf_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}