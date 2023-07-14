// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn extreme_speed_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END, false);
        }
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32) {
    //PM-like neutral-b canceling
    /***if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD {
        if boma.is_cat_flag(Cat2::CommonGuard) {
            if situation_kind == *SITUATION_KIND_AIR {
                if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                    WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL, true);
                }
            }
            else {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL, true);
            }
        }
    }***/
    if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    extreme_speed_cancel(boma, status_kind);
    nspecial_cancels(boma, status_kind, situation_kind, cat[1]);
    fastfall_specials(fighter);

    // Magic Series
    magic_series(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

unsafe fn magic_series(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    let cat1 = cat[0];
    // Level 1: Jab and Dash Attack Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
            || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
            // Check for tilt attack inputs
            if boma.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
            }
            if boma.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3,false);
            }
            if boma.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3,false);
            }

            // Check for smash attack inputs
            if boma.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
            }

            // Check for special attack inputs
            if boma.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
            }
            if boma.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
            }
            if boma.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
            }
            if boma.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
            }

            // Check for jump inputs during dash attack (on hit)
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
                && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) {
                boma.check_jump_cancel(false);
            }

        }
    }

    // Level 2: Tilt Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
            || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
            // Check for smash attack inputs
            if boma.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
            }

            // Check for special attack inputs
            if boma.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
            }
            if boma.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
            }
            if boma.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
            }
            if boma.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
            }

            // Check for jump inputs during utilt
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
            && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            && !boma.is_in_hitlag()) {
                boma.check_jump_cancel(false);
            }
        }
    }

    // Smash Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
            || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {

            // Check for special attack inputs
            if boma.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
            }
            if boma.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
            }
            if boma.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
            }
            if boma.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
            }

            // Check for jump inputs
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
            && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            && !boma.is_in_hitlag()) {
                boma.check_jump_cancel(false);
            }
        }
    }

    // Aerial Cancels
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
            || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
            // Check for jump inputs
            if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) {
                boma.check_jump_cancel(false);
            }
            // Check for special attack inputs
            if boma.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
            }
            if boma.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
            }
            if boma.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
            }
            if boma.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
            }
        }
    }

    // Extreme Speed Cancels
    if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
            || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag())
        {
            boma.check_jump_cancel(false);
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_LUCARIO )]
pub fn lucario_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		lucario_frame(fighter)
    }
}

pub unsafe fn lucario_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
