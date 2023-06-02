// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


unsafe fn boosted_aerials(boma: &mut BattleObjectModuleAccessor) {
    if boma.get_aerial() == Some(AerialKind::Fair) ||
    boma.get_aerial() == Some(AerialKind::Bair) {
        if boma.is_cat_flag(Cat1::AttackS4) {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
    else if boma.get_aerial() == Some(AerialKind::Uair) {
        if boma.is_cat_flag(Cat1::AttackHi4) {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
    else if boma.get_aerial() == Some(AerialKind::Dair) {
        if boma.is_cat_flag(Cat1::AttackLw4) {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
}
 
unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, cat2: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            }
        }
    }
}

unsafe fn absorb_vortex_jc_turnaround_shinejump_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if [*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HIT,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_END,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_HIT,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_END,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_LOOP].contains(&status_kind) {
        if !boma.is_in_hitlag() {
            if (status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD && boma.status_frame() > 3)
                || (status_kind != *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD)
            {
                boma.check_jump_cancel(false);
            }
        }
        if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD {
            if facing * stick_x < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

unsafe fn laser_blaze_ff_land_cancel(boma: &mut BattleObjectModuleAccessor, situation_kind: i32, motion_kind: u64, cat2: i32, stick_y: f32) {
    if [hash40("special_air_n2_start"),
        hash40("special_air_n2_loop"),
        hash40("special_air_n2_end"),
        hash40("special_n2_start"),
        hash40("special_n2_loop"),
        hash40("special_n2_end")].contains(&motion_kind) {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

unsafe fn remove_homing_missiles(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_GROUND {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND, false);
    }
    else if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR, false);
    }
}

unsafe fn missile_land_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if [*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            // Set additional landing lag if the missile has been fired
            if frame >= 23.0 {
                if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR {
                    //let landing_frame = (41.0 - frame).max(30.0);
                    MotionModule::set_frame(boma, 30.0, false);
                }
                else {
                    //let landing_frame = (50.0 - frame).max(40.0);
                    MotionModule::set_frame(boma, 40.0, false);
                }
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
            }
        }
    }
}

unsafe fn arm_rocket_airdash(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
	let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
	if [*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH].contains(&status_kind) {
		// Transition into rush_end early
		if frame > 16.0 {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END, false);
		}
	}
	if [*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END].contains(&status_kind) {
		if frame > 11.0 {
			VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
		}
    }
}

/// Allow uncharged or slightly charged Lunar Launch to be actionable
unsafe fn lunar_launch_actionability(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, motion_kind: u64) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        if fighter.status_frame() >= 35 && VarModule::get_float(boma.object(), vars::miigunner::status::CHARGE_ATTACK_LEVEL) <= 10.0 {
            // if already used once this airtime
            if VarModule::is_flag(boma.object(), vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED) {
                VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
            }
            else {
                VarModule::on_flag(boma.object(), vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

/// Resets Lunar Launch use count
unsafe fn lunar_launch_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_status(*FIGHTER_STATUS_KIND_CLIFF_CATCH) {
        VarModule::off_flag(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED);
    }
}

/// Turn off funny slowed down smoke when hit
unsafe fn lunar_launch_effect_reset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        EFFECT_OFF_KIND(fighter, Hash40::new("miigunner_bottom_shot"), false, false);
    }
}

unsafe fn stealth_burst_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S2_END {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    boosted_aerials(boma);
    nspecial_cancels(boma, status_kind, situation_kind, cat[0], cat[1]);
    absorb_vortex_jc_turnaround_shinejump_cancel(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    laser_blaze_ff_land_cancel(boma, situation_kind, motion_kind, cat[1], stick_y);
    remove_homing_missiles(boma, status_kind);
    missile_land_cancel(fighter, boma, id, status_kind, situation_kind, frame);
	arm_rocket_airdash(boma, id, status_kind, frame);
    lunar_launch_actionability(fighter, boma, motion_kind);
    lunar_launch_reset(fighter);
    lunar_launch_effect_reset(fighter, boma, status_kind);
    stealth_burst_land_cancel(boma, status_kind, situation_kind);
}

#[utils::macros::opff(FIGHTER_KIND_MIIGUNNER )]
pub fn miigunner_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		miigunner_frame(fighter)
    }
}

pub unsafe fn miigunner_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame(agent = WEAPON_KIND_MIIGUNNER_SUPERMISSILE)]
pub fn miigunner_missile_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        // Ensure the boma's owner is Mii Gunner
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
            let gunner = utils::util::get_battle_object_from_id(owner_id);
            let gunner_boma = &mut *(*gunner).module_accessor;
            if StatusModule::status_kind(boma) == *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_STRAIGHT
            && gunner_boma.is_cat_flag(Cat1::SpecialS)
            && VarModule::is_flag(gunner, vars::miigunner::instance::DETONATE_READY) {
                if WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP) {
                    StatusModule::change_status_request_from_script(boma, *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_S_BURST, false);
                    VarModule::on_flag(gunner, vars::miigunner::status::MISSILE_DETONATE);
                    VarModule::off_flag(gunner, vars::miigunner::instance::DETONATE_READY);
                    gunner_boma.clear_commands(Cat1::SpecialS); // Clear command so Gunner doesn't immediately fire another missile
                }
            }
        }
    }
}