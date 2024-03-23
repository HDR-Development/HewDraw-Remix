// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn bow_lc(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT) {
        if boma.is_prev_situation(*SITUATION_KIND_AIR) && boma.is_situation(*SITUATION_KIND_GROUND) {
            MotionModule::set_frame_sync_anim_cmd(boma, 26.0, true, true, false);
        }
    }
}

// Dark Pit Guardian Orbitar Jump Cancels
unsafe fn guardian_orbitar_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if [*FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if boma.status_frame() > 1 && !boma.is_in_hitlag(){
            boma.check_jump_cancel(false, false);
        }
    }
}

extern "Rust" {
    fn pits_common(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32);
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bow_lc(boma);
    guardian_orbitar_jc(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    pits_common(fighter, boma, status_kind);
}

pub extern "C" fn pitb_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pitb_frame(fighter)
    }
}

pub unsafe fn pitb_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
        agent.on_line(Main, pitb_frame_wrapper)
}
