use ::common::opff_import::*;
use super::*;
use globals::*;
use ::common::opff::*;
 
// Disable QA jump cancels if not directly QA into the ground
unsafe fn disable_qa_jc(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END {
        if situation_kind == *SITUATION_KIND_AIR && frame > 1.0 {
            VarModule::on_flag(boma.object(), vars::common::DISABLE_SPECIAL_JC);
        }
    }
}

// Reset JC disable flag
unsafe fn reset_jc_disable_flag(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    if situation_kind == *SITUATION_KIND_GROUND && status_kind != *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        VarModule::off_flag(boma.object(), vars::common::DISABLE_SPECIAL_JC);
    }
}

// JC Quick Attack/Agility
unsafe fn jc_qa_agility(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        if frame > 3.0 {
            if situation_kind == *SITUATION_KIND_GROUND {
                if [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP,
                    *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&prev_status_kind) {
                    if  !VarModule::is_flag(boma.object(), vars::common::DISABLE_SPECIAL_JC) {
                        if boma.is_input_jump() {
                            if facing * stick_x < 0.0 {
                                PostureModule::reverse_lr(boma);
                            }
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                        }
                    }
                }
            }
        }
    }
}

pub unsafe fn electric_rats_moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    disable_qa_jc(boma, id, status_kind, situation_kind, frame);
    reset_jc_disable_flag(boma, id, status_kind, situation_kind);
    jc_qa_agility(boma, id, status_kind, situation_kind, cat[0], stick_x, facing, frame);
}


#[no_mangle]
pub unsafe extern "Rust" fn electric_rats_common(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        electric_rats_moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // nothing lol
}

#[utils::opff(FIGHTER_KIND_PIKACHU )]
pub fn pikachu_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		pikachu_frame(fighter);
        electric_rats_common(fighter);
    }
}

pub unsafe fn pikachu_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}