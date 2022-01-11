use ::common::opff_import::*;
use super::*;
use globals::*;
use ::common::opff::*;
 
unsafe fn sonic_lightspeed_dash(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, situation_kind: i32, cat1: i32, id: usize){
    if SONIC_LIGHTSPEED_NO_JUMP[id] && status_kind != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END && motion_kind != hash40("special_s_start") {
        SONIC_LIGHTSPEED_NO_JUMP[id] = false;
    }
    //JC the ending anim
    if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END && !SONIC_LIGHTSPEED_NO_JUMP[id] {
        moveset_utils::enable_jump_cancel(boma, situation_kind, cat1, 0, 7);
    }

    if status_kind == *FIGHTER_STATUS_KIND_DEAD {
        VarModule::set_float(boma.object(), vars::common::SONIC_LIGHTSPEED_DASH_FRAME_COUNTER, 0.0);
        WorkModule::set_flag(boma, false, *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_DASH_FLAG);
    }

}

unsafe fn sonic_moveset(boma: &mut BattleObjectModuleAccessor, situation_kind: i32, status_kind: i32, motion_kind: u64, curr_frame: f32, cat1: i32, id: usize){
    /* set hitbox flag for pulsing hitbox on down-b charge when special is inputted */
    if motion_kind == hash40("special_lw_hold") {
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) && !SONIC_PULSE_HITBOX[id] {
            SONIC_PULSE_HITBOX[id] = true;
            MotionModule::set_frame(boma, 1.0, true);
        }
    }

    /* enable b-turnarounds for speciallw */
    moveset_utils::enable_b_turnaround(boma, status_kind, id, *FIGHTER_STATUS_KIND_SPECIAL_LW);

    /* enable jump out of down-b turn... (down-b and side-b share a lot of statuses) */
    if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN && moveset_utils::jump_checker_buffer(boma, cat1) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP, true);
        //ControlModule::clear_command(boma, true);
    }

    /* disable jump during speciallw charge */
    if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD && moveset_utils::jump_checker_buffer(boma, cat1) {
        ControlModule::clear_command(boma, true);
    }

    /* Allow sonic to change facing direction during first few frames of spin jump */
    if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP && curr_frame <= 8.0
    && ControlModule::get_stick_x(boma) * KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 && ControlModule::get_stick_x(boma).abs() != 0.0
    {
        PostureModule::reverse_lr(boma);
        PostureModule::update_rot_y_lr(boma);
    }

    /* transitioning to specialnfail if player presses the standard attack button */
    if motion_kind == hash40("special_n_spin") && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, true);
    }

    /* giving Blast Attack a FAF of 15 if it connects */
    if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL {
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
            SONIC_BLAST_ATTACK[id] = true;
        }
    }
    if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT && hdr::in_range(curr_frame, 14.0, 40.0) && SONIC_BLAST_ATTACK[id] {
        CancelModule::enable_cancel(boma);
        SONIC_BLAST_ATTACK[id] = false;
    }

    /* set flag for disabling airdodge out of up-b */
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP {
        SONIC_NO_AIRDODGE[id] = true;
    }
    if StatusModule::prev_status_kind(boma, 1) == *FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP {
        SONIC_NO_AIRDODGE[id] = false;
    }
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&situation_kind) {
        if SONIC_NO_AIRDODGE[id] {
            SONIC_NO_AIRDODGE[id] = false;
        }
        if SONIC_BLAST_ATTACK[id] {
            SONIC_BLAST_ATTACK[id] = false;
        }
    }

}

// Sonic Spindash Jump Waveland
unsafe fn sonic_spindash_jump_waveland(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP {
        if boma.is_cat_flag(Cat1::JumpButton) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
            if situation_kind == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    sonic_spindash_jump_waveland(boma, status_kind, situation_kind, cat[0]);
    //sonic_moveset(boma, situation_kind, status_kind, motion_kind, frame, cat[0], id);
    //sonic_lightspeed_dash(boma, status_kind, motion_kind, situation_kind, cat[0], id);
}

#[utils::opff(FIGHTER_KIND_SONIC )]
pub fn sonic_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		sonic_frame(fighter)
    }
}

pub unsafe fn sonic_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}