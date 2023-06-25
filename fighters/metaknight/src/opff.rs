// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn dim_cape_early_attack_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if frame > 11.0 {
            if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, false);
            }
        }
    }
}

// Meta Knight Special Fall Hit Reset
// Set flags for each special move
unsafe fn flag_resets(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, motion_kind: u64, frame: f32) {
    
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN {
            VarModule::on_flag(boma.object(), vars::metaknight::instance::NEUTRAL_SPECIAL_HIT);
        } else if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP].contains(&status_kind) {
            VarModule::on_flag(boma.object(), vars::metaknight::instance::UP_SPECIAL_HIT);
        } else if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK {
            VarModule::on_flag(boma.object(), vars::metaknight::instance::DOWN_SPECIAL_HIT);
        } else if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH || status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END {
            VarModule::on_flag(boma.object(), vars::metaknight::instance::SIDE_SPECIAL_HIT);
        }
    }
    // if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
    //     if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH || status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END {
    //         VarModule::on_flag(boma.object(), vars::metaknight::instance::SIDE_SPECIAL_HIT);
    //     }
    // }
}

// Transition to fall after side special rebound on hit/shield
unsafe fn transition_fall(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        if (boma.is_prev_status(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END) && VarModule::is_flag(boma.object(), vars::metaknight::instance::SIDE_SPECIAL_HIT))
        || (boma.is_prev_status_one_of(&[*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK]) && VarModule::is_flag(boma.object(), vars::metaknight::instance::DOWN_SPECIAL_HIT)) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

// Reset flags
unsafe fn reset_flags(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    if situation_kind != SITUATION_KIND_AIR
        || [*FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
            VarModule::off_flag(boma.object(), vars::metaknight::instance::NEUTRAL_SPECIAL_HIT);
            VarModule::off_flag(boma.object(), vars::metaknight::instance::SIDE_SPECIAL_HIT);
            VarModule::off_flag(boma.object(), vars::metaknight::instance::UP_SPECIAL_HIT);
            VarModule::off_flag(boma.object(), vars::metaknight::instance::DOWN_SPECIAL_HIT);
    }
}

/// this cancels side special early if you hit the opponent
unsafe fn drill_rush_on_hit_cancel(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH)
        && AttackModule::is_infliction_status(fighter.boma(), *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        // Allows MK to rebound off of shield
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT);
        // Transition to rebound
        fighter.change_status_req(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END, false);
    }
}

// Lengthen sword
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.0, y: 0.85, z: 1.0};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}				 

unsafe fn fspecial_once_per_airtime(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S)
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.object(), vars::common::instance::SIDE_SPECIAL_CANCEL);
    }
}

unsafe fn up_special_proper_landing(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP])
    && fighter.is_prev_situation(*SITUATION_KIND_AIR)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.status_frame() > 20 {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
    }
}

unsafe fn attack_100_end_early(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
    if motion_kind != hash40("attack_100") {
        return;
    }
    if frame >= 19.0 
    || (frame >= 10.0 && !fighter.is_button_on(Buttons::AttackRaw)) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_100_end"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe fn special_lw_landing_lag(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if [
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END,
    ].contains(&status_kind)
    && situation_kind == *SITUATION_KIND_GROUND
    && StatusModule::prev_situation_kind(boma) != *SITUATION_KIND_GROUND {
        StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    dim_cape_early_attack_cancel(boma, status_kind, frame);
    flag_resets(boma, id, status_kind, motion_kind, frame);
    transition_fall(boma, id, status_kind);
    reset_flags(boma, id, status_kind, situation_kind);
    sword_length(boma);
    up_special_proper_landing(fighter);
    attack_100_end_early(fighter, boma, motion_kind, frame);
    special_lw_landing_lag(fighter, boma, status_kind, situation_kind);
}

#[utils::macros::opff(FIGHTER_KIND_METAKNIGHT )]
pub fn metaknight_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		metaknight_frame(fighter);
        // println!("motion: {:#x}", MotionModule::motion_kind(fighter.module_accessor));
    }
}

pub unsafe fn metaknight_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
        drill_rush_on_hit_cancel(fighter);
        fspecial_once_per_airtime(fighter);
    }
}