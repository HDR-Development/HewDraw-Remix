use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn dtilt_repeat_increment(boma: &mut BattleObjectModuleAccessor, id: usize, motion_kind: u64) {
    if motion_kind == hash40("attack_lw3")
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        &&  !VarModule::is_flag(get_battle_object_from_accessor(boma), vars::shotos::REPEAT_INCREMENTED) {
        repeat_num_lw[id] += 1;
        VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::REPEAT_INCREMENTED);
    }
}

// Terry Power Wave Dash Cancel and Super Cancels
unsafe fn power_wave_dash_cancel_super_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat: [i32; 4], motion_kind: u64, frame: f32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    let cat1 = cat[0];
    let cat4 = cat[3];
    let prev_situation_kind = StatusModule::prev_situation_kind(boma);

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        // Super Cancel
        if frame > 21.0 {
            // Check to see if supers are available
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                // Enable transition term
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);

                // Buster Wolf
                if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, false);
                }

                // Power Geyser
                if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, false);
                }
            }
        }

        // Triple Geyser
        if meter::get_meter_level(boma) >= 10 {
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                if !meter_used[id] {
                    meter_used[id] = true;
                    meter::use_meter_level(&mut agent_base, boma, 10);
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
                }
            }
        }

        // Dash Cancel
        if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            if frame > 36.0 {
                if situation_kind == *SITUATION_KIND_GROUND {
                    if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, false);
                    }
                }
            }
        } else {
            if frame > 33.0 {
                if situation_kind == *SITUATION_KIND_GROUND {
                    if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, false);
                    }
                }
            }
        }
    }
}

// Special and Super Cancels into Triple Geyser
unsafe fn special_super_cancels_triple_geyser(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat4: i32, motion_kind: u64) {
    let mut agent_base = fighter.fighter_base.agent_base;
    if [*FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK].contains(&status_kind) {
        // Triple Geyser
        if meter::get_meter_level(boma) >= 10 {
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                if !meter_used[id] {
                    meter_used[id] = true;
                    meter::use_meter_level(&mut agent_base, boma, 10);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
                }
            }
        }
    }

    // Super Cancels into Triple Geyser
    if [*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW].contains(&status_kind)
        && motion_kind == 0x13434c5490 as u64 {
        if meter::get_meter_level(boma) >= 6 {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                if !meter_used[id] {
                    meter_used[id] = true;
                    meter::use_meter_level(&mut agent_base, boma, 6);
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
                }
            }
        }
    }
}

// Terry Burn Knuckle Land Cancel
// Check for aerial startup
unsafe fn burn_knuckle_land_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if motion_kind == hash40("special_air_f_start") {
        if situation_kind == *SITUATION_KIND_AIR {
            air_special_used[id] = true;
        }
    }
    if air_special_used[id] {
        if [*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END,
            *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
            }
        }
    }
}

// Terrry Super Special Meter Activation
unsafe fn super_special_meter_activation(boma: &mut BattleObjectModuleAccessor) {
    if meter::get_meter_level(boma) >= 4 {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
    }
    if meter::get_meter_level(boma) < 4 {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
    }
}

// Cancel supers early
unsafe fn cancel_supers_early(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if [*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2].contains(&status_kind) {
        if frame < 25.0 {
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, false);
                }
            }
        }
    }
}

// Super Cancels
unsafe fn super_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat4: i32, motion_kind: u64) {
    let mut agent_base = fighter.fighter_base.agent_base;
    // Power Geyser
    if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL {
        if meter::get_meter_level(boma) >= 2 {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
            // Buster Wolf
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND
                                    | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND) {
                if !meter_used[id] {
                    meter_used[id] = true;
                    meter::use_meter_level(&mut agent_base, boma, 2);
                    super_cancel[id] = true;
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, false);
                }
            }
        }
    }
    // Buster Wolf
    if [*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW].contains(&status_kind)
        || motion_kind == 0x13434c5490 as u64 {
        if meter::get_meter_level(boma) >= 2 {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
            // Power Geyser
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND
                                    | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND) {
                if !meter_used[id] {
                    meter_used[id] = true;
                    meter::use_meter_level(&mut agent_base, boma, 2);
                    super_cancel[id] = true;
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, false);
                }
            }
        }
    }
}

// Turn off Super Cancel Flag
unsafe fn super_cancel_flag_off(id: usize, status_kind: i32) {
    if ![*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW].contains(&status_kind) {
        super_cancel[id] = false;
    }
}

// Terry Shield Stop and Run Drop
unsafe fn shield_stop_run_drop(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_y: f32, situation_kind: i32) {
    if hdr::compare_cat(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_GUARD_TRIGGER)
        && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH)
        && situation_kind == *SITUATION_KIND_GROUND
        && [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK].contains(&status_kind)
    {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
        ControlModule::clear_command(boma, true);
        if GroundModule::is_passable_ground(boma) && hdr::stick_y_flick_check(boma, -0.66) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
        }
    }
}

// TRAINING MODE
// Full Meter Gain via shield during taunt
unsafe fn full_meter_training_taunt(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    if hdr::is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                meter::add_meter(&mut agent_base, boma, meter_max);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //dtilt_repeat_increment(boma, id, motion_kind); // UNUSED
    power_wave_dash_cancel_super_cancels(fighter, boma, id, status_kind, situation_kind, cat, motion_kind, frame);
    special_super_cancels_triple_geyser(fighter, boma, id, status_kind, cat[3], motion_kind);
    //burn_knuckle_land_cancel(boma, id, status_kind, situation_kind, motion_kind); // UNUSED
    super_special_meter_activation(boma);
    cancel_supers_early(boma, status_kind, situation_kind, frame);
    super_cancels(fighter, boma, id, status_kind, cat[3], motion_kind);
    super_cancel_flag_off(id, status_kind);
    shield_stop_run_drop(boma, status_kind, stick_y, situation_kind);
    full_meter_training_taunt(fighter, boma, status_kind);

    // Magic Series
    magic_series(fighter, boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

unsafe fn magic_series(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    let cat1 = cat[0];
    let cat4 = cat[3];
    // Terry
    // Level 1: Jab Cancels
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

            // Check for tilt attack inputs
            if motion_kind != hash40("attack_13") {
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) {
                    VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) {
                    VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3,false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) {
                    VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3,false);
                }
            }

            // Check for smash attack inputs
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) {
                VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
            }
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
            }
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
            }

            // Check for HCF inputs for Power Charge
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                ControlModule::clear_command(boma, true);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH,false);
            }

        }

    }


    // Level 2: Tilt Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

            // Check for utilt inputs during ftilt
            if motion_kind == hash40("attack_s3_s") {
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::common::TILT_CHECKS);
                    VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3,false);
                }
            }

            // Check for smash attack inputs
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) {
                VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
            }
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
            }
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
            }

            // Check for HCF inputs for Power Charge
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                ControlModule::clear_command(boma, true);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH,false);
            }

            // Check for jump and dash inputs during utilt, and dash inputs during ftilt
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
                && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && frame > 13.0 {
                if moveset_utils::jump_checker_buffer(boma, cat1) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT,true);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH,false);
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
                && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && frame > 13.0 {
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) && !magic_cancel_additional[id] {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH,false);
                }
            }
        }
    }

    // Dash Attack Cancels
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            // Cancel into supers
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
            }

            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                // Buster Wolf
                if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,false);
                }

                // Power Geyser
                if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,false);
                }
            }
        }
    }


    // Smash Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

            // Check for HCF inputs for Power Charge
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                ControlModule::clear_command(boma, true);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH,false);
            }

            // Check for special attack inputs

            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);

            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
            }

            // Power Wave
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
            }
            // Burn Knuckle
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) && hdr::is_stick_forward(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
            }
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,false);
            }
            // Crack Shoot
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) && hdr::is_stick_backward(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,false);
            }
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,false);
            }
            // Rising Tackle
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
            }
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,false);
            }
            // Power Dunk
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
            }
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,false);
            }

            // Check for HCF inputs for Power Charge
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                ControlModule::clear_command(boma, true);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH,false);
            }

            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                // Buster Wolf
                if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,false);
                }

                // Power Geyser
                if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,false);
                }
            }
        }
    }

    // Aerial Cancels
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            // Aerial Magic Series
            // Nair
            if motion_kind == hash40("attack_air_n") {
                /*
                   if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_N) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N))) {
                   StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                   }
                   */
                //if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_F) ||
                if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) && hdr::is_stick_forward(boma))
                    || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) && hdr::is_stick_forward(boma)) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
                //if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_B) ||
                if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) && hdr::is_stick_backward(boma))
                    || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) && hdr::is_stick_backward(boma)) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                    //PostureModule::reverse_lr(boma);
                }
                //if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_HI) ||
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
                //if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW) ||
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
            }
            // Fair
            if motion_kind == hash40("attack_air_f") {
                //if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_B) ||
                if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) && hdr::is_stick_backward(boma))
                    || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) && hdr::is_stick_backward(boma)) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                    //PostureModule::reverse_lr(boma);
                }
                //if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_HI) ||
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
                //if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW) ||
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
            }
            // Bair
            if motion_kind == hash40("attack_air_b") {

            }
            // Uair
            if motion_kind == hash40("attack_air_hi") {
                //if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_B) ||
                if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) && hdr::is_stick_backward(boma))
                    || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) && hdr::is_stick_backward(boma)) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                    //PostureModule::reverse_lr(boma);
                }
                //if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW) ||
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
            }
            // Dair
            if motion_kind == hash40("attack_air_lw") {
                //if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_B) ||
                if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) && hdr::is_stick_backward(boma))
                    || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) && hdr::is_stick_backward(boma)) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
            }
        }
    }

    // Special Cancels
    if [*FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {

            // Super Cancels for all specials
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
            }
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                // Buster Wolf
                if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,false);
                }

                // Power Geyser
                if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,false);
                }
            }

            // Burn Knuckle cancels
            if [*FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
                *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK].contains(&status_kind) {

                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);

                // Each cancel costs 2 meter
                if meter::get_meter_level(boma) >= 1 {
                    // Crack Shoot
                    if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) && hdr::is_stick_backward(boma) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,false);
                        }
                    }
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND) {
                        if !meter_used[id] {
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,false);
                        }
                    }
                    // Rising Tackle
                    if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                        }
                    }
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,false);
                        }
                    }
                    // Power Dunk
                    if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                        }
                    }
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,false);
                        }
                    }
                }
            }

            // Rising Tackle cancels
            if [*FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,
                *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP].contains(&status_kind) {

                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);

                // Each cancel costs 2 meter
                if meter::get_meter_level(boma) >= 1 {
                    // Power Wave
                    if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                        }
                    }
                    // Burn Knuckle
                    if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) && hdr::is_stick_forward(boma) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                        }
                    }
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,false);
                        }
                    }
                    // Crack Shoot
                    if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) && hdr::is_stick_backward(boma) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,false);
                        }
                    }
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,false);
                        }
                    }
                    // Power Dunk
                    if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                        }
                    }
                    if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND) {
                        if !meter_used[id]{
                            meter_used[id] = true;
                            meter::use_meter_level(&mut agent_base, boma, 1);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,false);
                        }
                    }
                }
            }
        }
    }

    // Super Cancels
    /*
       if (status_kind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL || status_kind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2{
           if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){

           }
       }
       */
}
#[utils::opff(FIGHTER_KIND_DOLLY )]
pub fn dolly_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		dolly_frame(fighter)
    }
}

pub unsafe fn dolly_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
