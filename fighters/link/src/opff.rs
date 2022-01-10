use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn bow_fastfall(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if hdr::compare_cat(cat2, *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Link Remote Bomb pull and detonation B-Reverse
unsafe fn bomb_n_deton_b_reverse(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST].contains(&status_kind) {
        if frame < 5.0 {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if frame > 1.0 && frame < 5.0 && !b_reversed[id] {
                    let b_reverse = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &b_reverse, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    b_reversed[id] = true;
                }
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bow_fastfall(boma, status_kind, situation_kind, cat[1], stick_y);
    bomb_n_deton_b_reverse(boma, id, status_kind, situation_kind, stick_x, stick_y, facing, frame);
    links::moveset(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

#[utils::opff(FIGHTER_KIND_LINK )]
pub fn link_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		link_frame(fighter)
    }
}

pub unsafe fn link_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}