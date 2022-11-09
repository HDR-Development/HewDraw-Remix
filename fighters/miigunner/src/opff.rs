// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, cat2: i32) {
    //PM-like neutral-b canceling
    /***if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_HOLD {
        if boma.is_cat_flag(Cat2::CommonGuard) {
            for trans_group in [*FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE].iter() {
                WorkModule::unable_transition_term_group(boma, *trans_group);
            }
            StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL, true);
        }
    }***/
    if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            }
            /*if CancelModule::is_enable_cancel(boma) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }*/
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
            if (status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD && frame > 3.0)
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
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

unsafe fn missile_land_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    if [*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_GROUND,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
    }
}

unsafe fn cannon_jump_kick_actionability(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if [*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI2_JUMP].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
			if frame > 35.0 {
				VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
				CancelModule::enable_cancel(boma);
			}
		}
    }
}

unsafe fn arm_rocket_airdash(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
	let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
	if [*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH].contains(&status_kind) {
		// Release B to end the up special early
		if frame > 15.0 {
			if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END, false);
			}
		}
	}
	if [*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END].contains(&status_kind) {
		if frame > 10.0 {
			VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
		}
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    nspecial_cancels(boma, status_kind, situation_kind, cat[0], cat[1]);
    absorb_vortex_jc_turnaround_shinejump_cancel(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    laser_blaze_ff_land_cancel(boma, situation_kind, motion_kind, cat[1], stick_y);
    missile_land_cancel(fighter, boma, id, status_kind, situation_kind);
	cannon_jump_kick_actionability(boma, id, status_kind, frame);
	arm_rocket_airdash(boma, id, status_kind, frame);

    // Frame Data
    //frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        /*
        if motion_kind == hash40("attack_air_n") {
            if frame < 7.0 {
                MotionModule::set_rate(boma, 1.75);
            }
            if frame >= 7.0 {
                MotionModule::set_rate(boma, 1.0);
            }
        }
        */
        if motion_kind == hash40("attack_air_f") {
            if frame < 9.0 {
                MotionModule::set_rate(boma, 1.0);
            }
            if frame >= 9.0 {
                MotionModule::set_rate(boma, 1.0);
            }
        }
        /*
        if motion_kind == hash40("attack_air_hi") {
            if frame < 16.0 {
                MotionModule::set_rate(boma, 1.455);
            }
            if frame >= 16.0 {
                MotionModule::set_rate(boma, 1.0);
            }
        }
        */
    }
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
