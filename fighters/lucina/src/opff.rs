use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn side_special_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, motion_kind: u64) {
    if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3 {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            // Up
            if [hash40("special_s3_hi"), hash40("special_air_s3_hi")].contains(&motion_kind) {
                // Check for tilt attack inputs
                if boma.is_cat_flag(Cat1::AttackHi3) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
                // Check for smash attack inputs
                if boma.is_cat_flag(Cat1::AttackHi4) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
            }
            // Forward
            if [hash40("special_s3_s"), hash40("special_air_s3_s")].contains(&motion_kind) {
                // Check for tilt attack inputs
                if boma.is_cat_flag(Cat1::AttackS3) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
                // Check for smash attack inputs
                if boma.is_cat_flag(Cat1::AttackS4) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
            }
            // Down
            if [hash40("special_s3_lw"), hash40("special_air_s3_lw")].contains(&motion_kind) {
                // Check for tilt attack inputs
                if boma.is_cat_flag(Cat1::AttackLw3) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
                // Check for smash attack inputs
                if boma.is_cat_flag(Cat1::AttackLw4) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
            }
        }
    }
    // Jump cancel 4th hit high
    if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4 {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if [hash40("special_s4_hi"), hash40("special_air_s4_hi")].contains(&motion_kind) && MotionModule::frame(boma) > 13.0 {
                if moveset_utils::jump_checker_buffer(boma, cat1) {
                    if situation_kind == *SITUATION_KIND_AIR {
                        if hdr::get_jump_count(boma) < hdr::get_jump_count_max(boma) {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                        }
                    } else if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                }
            }
        }
    }
}

// Soaring Slash Hit
unsafe fn soaring_slash(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2].contains(&status_kind) {
        if AttackModule::is_infliction(boma, 2) {
            VarModule::on_flag(get_battle_object_from_accessor(boma), vars::common::SOARING_SLASH_HIT);
        }
    }
    if ![*FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind) {
        VarModule::off_flag(get_battle_object_from_accessor(boma), vars::common::SOARING_SLASH_HIT);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    fe::moveset(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
    // Magic Series
    side_special_cancels(boma, status_kind, situation_kind, cat[0], motion_kind);
    soaring_slash(boma, id, status_kind);
}

#[utils::opff(FIGHTER_KIND_LUCINA )]
pub fn lucina_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		lucina_frame(fighter)
    }
}

pub unsafe fn lucina_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
