// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn airdodge_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if frame > 15.0 {
            FighterStatusModuleImpl::set_fighter_status_data(
                boma,
                false,
                *FIGHTER_TREADED_KIND_NO_REAC,
                false,
                false,
                false,
                (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
                0,
                *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
                0
            );
            boma.check_airdodge_cancel();
        }
    }
}

// Wolf Shine Jump Cancels
unsafe fn shine_jump_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    if ((fighter.is_status (*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.status_frame() > 2)  // Allows for jump cancel on frame 4 in game
        || fighter.is_status_one_of(&[
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT,
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP,
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END]))
        && !fighter.is_in_hitlag()
        {
            fighter.check_jump_cancel(false);
        }
}

unsafe fn firefox_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        // allows ledgegrab during Firefox startup
        fighter.sub_transition_group_check_air_cliff();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    airdodge_cancel(boma, status_kind, situation_kind, cat[0], frame);
    shine_jump_cancel(fighter);
    firefox_startup_ledgegrab(fighter);
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