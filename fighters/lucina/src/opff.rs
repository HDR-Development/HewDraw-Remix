// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn side_special_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, motion_kind: u64) {
    if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3 {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) {
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
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) {
            if [hash40("special_s4_hi"), hash40("special_air_s4_hi")].contains(&motion_kind) && MotionModule::frame(boma) > 13.0 {
                boma.check_jump_cancel(false);
            }
        }
    }
}

// symbol-based call for the fe characters' common opff
extern "Rust" {
    fn fe_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // Magic Series
    side_special_cancels(boma, status_kind, situation_kind, cat[0], motion_kind);
}

#[utils::macros::opff(FIGHTER_KIND_LUCINA )]
pub fn lucina_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		lucina_frame(fighter);
        fe_common(fighter);
    }
}

pub unsafe fn lucina_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
