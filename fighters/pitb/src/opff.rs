// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn bow_ff_lc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if [*FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
        if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT {
            if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
            }
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Dark Pit Guardian Orbitar Jump Cancels
unsafe fn guardian_orbitar_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if [*FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if boma.status_frame() > 1 && !boma.is_in_hitlag(){
            boma.check_jump_cancel(false);
        }
    }
}


extern "Rust" {
    fn pits_common(boma: &mut BattleObjectModuleAccessor, status_kind: i32);
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bow_ff_lc(boma, status_kind, situation_kind, cat[1], stick_y);
    guardian_orbitar_jc(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    pits_common(boma, status_kind);
}

#[utils::macros::opff(FIGHTER_KIND_PITB )]
pub fn pitb_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pitb_frame(fighter)
    }
}


pub unsafe fn pitb_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
