// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

#[no_mangle]
pub unsafe extern "Rust" fn pits_common(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    upperdash_arm_whiff_freefall(fighter);
    up_special_startup_ledgegrab(fighter);
    fastfall_specials(fighter);
}

unsafe fn upperdash_arm_jump_and_aerial_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) || (boma.is_status(*FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END) && boma.status_frame() > 6) {
        if (boma.is_situation(*SITUATION_KIND_GROUND) || WorkModule::is_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF))
        && boma.status_frame() > 28 {
            boma.check_jump_cancel(true, false, true);
        }
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) {
        let start_frame = if boma.is_situation(*SITUATION_KIND_GROUND) { 16 } else { 19 };
        if (start_frame..35).contains(&boma.status_frame()) 
        && boma.is_cat_flag(Cat1::SpecialAny) {
            WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT);
        }
    }
}

unsafe fn upperdash_arm_whiff_freefall(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END])
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && MotionModule::frame(fighter.module_accessor) >= MotionModule::end_frame(fighter.module_accessor) - 1.0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
    }
}

unsafe fn up_special_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        // allows ledgegrab during upB startup
        fighter.sub_transition_group_check_air_cliff();

        if fighter.status_frame() == 12 {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    upperdash_arm_jump_and_aerial_cancel(boma);
    pits_common(fighter, boma, status_kind);
}

pub extern "C" fn pit_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pit_frame(fighter)
    }
}

pub unsafe fn pit_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pit_frame_wrapper);
}
