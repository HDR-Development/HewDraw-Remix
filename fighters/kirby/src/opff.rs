use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn final_cutter_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat1: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2].contains(&status_kind){
        if(AttackModule::is_infliction(boma, 2)){
            final_cutter_hit[id] = true;
        }
    }
    else{
        final_cutter_hit[id] = false;
    }

    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2 {
        if frame > 10.0 && frame < 19.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if final_cutter_hit[id] {
                    VarModule::on_flag(boma.object(), common::UP_SPECIAL_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                } else {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                }
            }
        }
    }
}

// Kirby Hammer Fast Fall & Land Cancel
unsafe fn hammer_fastfall_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_s"), 43.0, 1.0, 1.0); // 55 FAF - F43 = 12F landing lag
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if hdr::compare_cat(cat2, *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Hammer Flip B-Reverse
unsafe fn hammer_flip_b_reverse(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if situation_kind == *SITUATION_KIND_AIR {
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
}

// Reset Star Rod flag each match
unsafe fn reset_star_rod(id: usize, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_ENTRY && kirby_star_rod[id] {
        kirby_star_rod[id] = false;
    }
}

// Magic Series
unsafe fn magic_series(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    let cat1 = cat[0];
    let cat4 = cat[3];
    if(    (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_RYU)
        || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_KEN)
        || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCARIO)
        || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_DOLLY)
        || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_BAYONETTA)){
        // Level 1: Jab and Dash Attack Cancels
        if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                // Check for tilt attack inputs
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3,false);
                }

                // Check for smash attack inputs
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
                }

                // Check for special attack inputs
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                // Check for jump inputs during dash attack (on hit)
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
                    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    if moveset_utils::jump_checker_buffer(boma, cat1)
                        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT,true);
                    }
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_ATTACK_COMMAND1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------

            }
        }

        // Level 2: Tilt Cancels
        if [*FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                // Check for smash attack inputs
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
                }

                // Check for special attack inputs
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                // Check for jump inputs during utilt
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                    if moveset_utils::jump_checker_buffer(boma, cat1)
                        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT,true);
                    }
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_ATTACK_COMMAND1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------

            }
        }

        // Smash Cancels
        if [*FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

                // Check for special attack inputs
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                // Check for jump inputs
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                    if moveset_utils::jump_checker_buffer(boma, cat1)
                        & AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT,true);
                    }
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_ATTACK_COMMAND1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------
            }
        }

        // Aerial Cancels
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                // Check for jump inputs
                if moveset_utils::jump_checker_buffer(boma, cat1)
                    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    if hdr::get_jump_count(boma) < hdr::get_jump_count_max(boma) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL,false);
                    }
                }
                // Check for special attack inputs
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_ATTACK_COMMAND1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------

            }
        }

    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    final_cutter_cancel(boma, id, status_kind, cat[0], frame);
    hammer_fastfall_landcancel(boma, status_kind, situation_kind, cat[1], stick_y);
    hammer_flip_b_reverse(boma, id, status_kind, situation_kind, stick_x, facing, frame);


    // Frame Data
    frame_data(boma, status_kind, motion_kind, frame);

    // Magic Series
    magic_series(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("special_air_s_start") {
        MotionModule::set_rate(boma, 1.75);
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK {
        if frame <= 10.0 {
            MotionModule::set_rate(boma, 2.5);
        }
        if frame > 10.0 {
            MotionModule::set_rate(boma, 1.0);
        }
    }
}

#[utils::opff(FIGHTER_KIND_KIRBY )]
pub fn kirby_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		kirby_frame(fighter)
    }
}

pub unsafe fn kirby_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}