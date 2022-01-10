use super::*;
use smash::app::utility::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::BattleObjectModuleAccessor;
use smash::hash40;
use smash::phx::{Vector2f, Vector3f, Hash40};
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;

// Dtilt and Utilt repeat increment
unsafe fn dtilt_utilt_repeat_increment(boma: &mut BattleObjectModuleAccessor, id: usize, motion_kind: u64) {
    if [hash40("attack_hi3_w"), hash40("attack_lw3_w")].contains(&motion_kind)
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        && !VarModule::is_flag(get_battle_object_from_accessor(boma), vars::shotos::REPEAT_INCREMENTED) {
        if motion_kind == hash40("attack_hi3_w") {
            repeat_num_hi[id] += 1;
        } else if motion_kind == hash40("attack_lw3_w") {
            repeat_num_lw[id] += 1;
        }
        VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::REPEAT_INCREMENTED);
    }
}

// Shotos Tatsumaki Land Cancel, hover, and EX momentum handling
unsafe fn tatsumaki_ex_land_cancel_hover(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    let jump_rising = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let stop_rise = Vector3f{x: 1.0, y: 0.0, z: 1.0};
	let ex_momentum = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let prev_situation_kind = StatusModule::prev_situation_kind(boma);

    if [*FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
		if VarModule::is_flag(get_battle_object_from_accessor(boma), vars::shotos::EX_SPECIAL){
			KineticModule::mul_speed(boma, &ex_momentum, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
		}
		
        if [*FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
            *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_AIR {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
                    || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    if jump_rising < 0.0 {
                        KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    }
                }
            }
        }
    }
}

// Shotos EX Shoryuken
unsafe fn ex_shoryuken(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64) {
	if [hash40("attack_11_w"), hash40("attack_11_s")].contains(&motion_kind) && ex_special[hdr::get_player_number(boma)] {
		println!("EX Shoryu");
		ControlModule::clear_command(boma, true);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
		MotionModule::change_motion_kind(boma, Hash40::new("attack_11_near_s"));
	}
	if [hash40("attack_11_near_s")].contains(&motion_kind) && ex_special[hdr::get_player_number(boma)] {
		ControlModule::clear_command(boma, true);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
		WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
	}
	if status_kind == *FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR{
		DamageModule::add_damage(boma, 10.0, 0);
	}
}
// Shotos Hadoken FADC and Super (FS) cancels
unsafe fn hadoken_fadc_sfs_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat: [i32; 4], frame: f32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    let cat1 = cat[0];
    let cat4 = cat[3];
    let fighter_kind = get_kind(boma);

    let ryu_enable = (fighter_kind == *FIGHTER_KIND_RYU) &&
        hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND
                                | *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND
                                | *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND);

    let ken_enable = (fighter_kind == *FIGHTER_KIND_KEN) &&
        hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND
                                | *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND);

    if [*FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND].contains(&status_kind) {
        // Final Smash Cancels
        if frame > 5.0 {
            if meter::get_meter_level(boma) >= 10 {
                if ryu_enable || ken_enable {
                    if !meter_used[id] {
                        meter_used[id] = true;
                        meter::use_meter_level(&mut agent_base, boma, 10);
                        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
                    }
                }
            }
        }

        // FADC
        if frame > 15.0 {
            if meter::get_meter_level(boma) >= 1 {
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) {
                    if !meter_used[id] {
                        meter_used[id] = true;
                        meter::use_meter_level(&mut agent_base, boma, 1);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                    }
                }
            }
        }
    }
}

// Shotos Special hit cancels
// Only for Ken?
unsafe fn special_hit_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let fighter_kind = get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_KEN {
        if [*FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1,
            *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2].contains(&status_kind) {
            if !WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL) {
                WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
            }
        }
    }
}

// Shotos Shield Stop and Run Drop
unsafe fn shield_stop_run_drop(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_RYU_STATUS_KIND_DASH_BACK {
        if hdr::compare_cat(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_GUARD_TRIGGER) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
            ControlModule::clear_command(boma, true);
        }
        if GroundModule::is_passable_ground(boma) && hdr::stick_y_flick_check(boma, -0.66) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, false);
        }
    }
}

// TRAINING MODE
// Full Meter Gain via shield during taunt
unsafe fn training_mode_full_meter(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
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
    MeterModule::update(fighter.battle_object, true);
    //dtilt_utilt_repeat_increment(boma, id, motion_kind); // UNUSED
    tatsumaki_ex_land_cancel_hover(boma, status_kind, situation_kind);
	//ex_shoryuken(boma, status_kind, situation_kind, motion_kind);
    hadoken_fadc_sfs_cancels(fighter, boma, id, status_kind, cat, frame);
    special_hit_cancels(boma, status_kind);
    shield_stop_run_drop(boma, status_kind);
    training_mode_full_meter(fighter, boma, status_kind);

    // Magic Series
    magic_series(fighter, boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

unsafe fn magic_series(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    let cat1 = cat[0];
    let cat4 = cat[3];
    let fighter_kind = get_kind(boma);
    let ryu_enable = (fighter_kind == *FIGHTER_KIND_RYU) &&
        hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND
                                | *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND
                                | *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND);

    let ken_enable = (fighter_kind == *FIGHTER_KIND_KEN) &&
        hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND
                                | *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND);
    // Level 1: Jab and Dash Attack Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

            // Check for tilt attack inputs
            if MotionModule::motion_kind(boma) != hash40("attack_13") {
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

            // Check for jump inputs during dash attack (on hit)
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if moveset_utils::jump_checker_buffer(boma, cat1) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT,true);
                }
            }

        }

    }


    // Level 2: Tilt Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

            // Tilts -> Roundhouse
            /*
               if MotionModule::motion_kind(boma) != hash40("attack_11_s")){
               if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) {
            //StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_N,true);
            MotionModule::change_motion_kind(boma, hash40("attack_11_s"));
            }
            }
            */
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

            // Check for jump inputs during utilt
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                if moveset_utils::jump_checker_buffer(boma, cat1)
                    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT,true);
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

            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
            if fighter_kind == *FIGHTER_KIND_RYU {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);

                if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND,false);
                }
            }
            if fighter_kind == *FIGHTER_KIND_KEN {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);

                if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_ATTACK_COMMAND1) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1,false);
                }
                if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2,false);
                }
            }

            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
            }
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,false);
            }
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
            }
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,false);
            }
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
            }
            if hdr::compare_cat(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,false);
            }
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,true);
            }

            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                if moveset_utils::jump_checker_buffer(boma, cat1)
                    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT,true);
                }
            }

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
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL,true);
                }
            }

            // Aerial Magic Series
            // Nair
            if motion_kind == hash40("attack_air_n") {
                /*
                   if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_N) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N)) {
                   StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                   }
                   */
                //if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_F) ||
                if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) && hdr::is_stick_forward(boma))
                    || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) && hdr::is_stick_forward(boma)) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
                //if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_B) ||
                if (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) && hdr::is_stick_backward(boma))
                    || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) && hdr::is_stick_backward(boma))  {
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
                //if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_HI) ||
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                    //VarModule::on_flag(get_battle_object_from_accessor(boma), vars::shotos::MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
                //if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW) ||
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
                //if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW) ||
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

    // Shoryu/Tatsu Cancels
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

            // Check for down special inputs
            if meter::get_meter_level(boma) >= 1 {
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) {
                    if !meter_used[id] {
                        meter_used[id] = true;
                        meter::use_meter_level(&mut agent_base, boma, 1);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,true);
                    }
                }
            }

            // Final Smash Cancels
            if meter::get_meter_level(boma) >= 10 {
                if ryu_enable || ken_enable {
                    if !meter_used[id] {
                        meter_used[id] = true;
                        meter::use_meter_level(&mut agent_base, boma, 10);
                        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
                    }
                }
            }
        }
    }
}
