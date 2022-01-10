use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn jab_dash_attack_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    // Level 1: Jab and Dash Attack Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

            // Check for tilt attack inputs during jab
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
                if boma.is_cat_flag(Cat1::AttackS3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                }
                if boma.is_cat_flag(Cat1::AttackHi3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
                }
                if boma.is_cat_flag(Cat1::AttackLw3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
                }

                // Check for special attack inputs during jab (on hit)
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    if boma.is_cat_flag(Cat1::SpecialN) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
                    }
                    if boma.is_cat_flag(Cat1::SpecialS) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
                    }
                    if boma.is_cat_flag(Cat1::SpecialHi) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
                    }
                }
            }

            // Check for side special and up special inputs during dash attack (on hit)
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
                && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if boma.is_cat_flag(Cat1::SpecialS) {
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
                }
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
                }
            }
        }
    }
}

unsafe fn tilt_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    // Level 2: Tilt Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

            // Check for smash attack inputs during ftilt and utilt
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                && [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind) {
                if boma.is_cat_flag(Cat1::AttackS4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
                }
                if boma.is_cat_flag(Cat1::AttackHi4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
                }
                if boma.is_cat_flag(Cat1::AttackLw4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
                }
            }

            // Check for special attack inputs
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if boma.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
                }
                if boma.is_cat_flag(Cat1::SpecialS) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
                }
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
                }
            }

            // Check for jump inputs during utilt
            /*
               if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3){
                   if jump_checker_buffer(boma, cat1) && AttackModule::is_infliction_status(boma, COLLISION_KIND_MASK_HIT)) {
                       StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT,true);
                   }
               }
               */
        }
    }
}

unsafe fn aerial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    // Aerial Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            // Check for jump inputs
            if moveset_utils::jump_checker_buffer(boma, cat1)
                && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if hdr::get_jump_count(boma) < hdr::get_jump_count_max(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            }
            // Check for special attack inputs
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if boma.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
                }
                if boma.is_cat_flag(Cat1::SpecialS) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
                }
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
                }
            }

            // Check for aerial attack inputs during fair
            if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    //if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_N
                    if boma.is_cat_flag(Cat1::AttackN) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                    //if boma.is_cat_flag(Cat1::AttackAirB) ||
                    if (boma.is_cat_flag(Cat1::AttackS3) && hdr::is_stick_backward(boma))
                        || (boma.is_cat_flag(Cat1::AttackS4) && hdr::is_stick_backward(boma)) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                    //if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_HI
                    if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                            | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                    //if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW
                    if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                            | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
            }
        }
    }
}

unsafe fn special_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    // Special Cancels
    if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if MotionModule::frame(boma) > 33.0 {
                CancelModule::enable_cancel(boma);
            }
        }
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::JumpButton) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // Magic Series
    jab_dash_attack_cancels(boma, status_kind, cat[0]);
    tilt_cancels(boma, status_kind, cat[0]);
    aerial_cancels(boma, status_kind, cat[0]);
    special_cancels(boma, status_kind, frame);
    nspecial_cancels(boma, status_kind, situation_kind, cat[0]);
}

#[utils::opff(FIGHTER_KIND_BAYONETTA )]
pub fn bayonetta_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		bayonetta_frame(fighter)
    }
}

pub unsafe fn bayonetta_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
         moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}