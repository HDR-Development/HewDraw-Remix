// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn gale_stab_jc_attack(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    // Rush
    if [*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH].contains(&status_kind) {
        // Jump and Attack cancels
        let pad_flag = ControlModule::get_pad_flag(boma);
        if boma.is_input_jump() {
            if situation_kind == *SITUATION_KIND_AIR && frame > 8.0 {
                if boma.get_num_used_jumps() < boma.get_jump_count_max() {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            } else if situation_kind == *SITUATION_KIND_GROUND {
                if facing * stick_x < 0.0 {
                    PostureModule::reverse_lr(boma);
                }
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
        }
        else if compare_mask(pad_flag, *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) || compare_mask(pad_flag, *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK, true);
        }
        // Wall Jump
        if situation_kind == *SITUATION_KIND_AIR {
            if !VarModule::is_flag(fighter.battle_object, vars::common::SPECIAL_WALL_JUMP) {
                let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
                let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
                if touch_left || touch_right {
                    if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH | *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) {
                        VarModule::on_flag(fighter.battle_object, vars::common::SPECIAL_WALL_JUMP);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                    }
                }
            }
        }
    }
    // Attack
    if [*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK].contains(&status_kind) {
        // Jump cancels
        let pad_flag = ControlModule::get_pad_flag(boma);
        if boma.is_input_jump() && frame > 6.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if situation_kind == *SITUATION_KIND_AIR {
                if boma.get_num_used_jumps() < boma.get_jump_count_max() {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            } else if situation_kind == *SITUATION_KIND_GROUND {
                if facing * stick_x < 0.0 {
                    PostureModule::reverse_lr(boma);
                }
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
        }
    }
}

// Mii Swordfighter Aerial Power Thrust Jump Reset
unsafe fn aerial_power_thrust_jump_reset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if motion_kind == hash40("special_lw3") || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END {
        if boma.get_num_used_jumps() == boma.get_jump_count_max() {
            WorkModule::set_int(boma, boma.get_jump_count_max() - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
}

// Mii Swordfighter Hero's Spin Movement
unsafe fn heros_spin_movement(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, frame: f32) {
    let valueMii = 0.7;
    let valueWalk = 0.28;
    let motion_value = valueMii;
    if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_HOLD {
        if situation_kind == *SITUATION_KIND_GROUND {
            if stick_x != 0.0 {
                let motion_vec = x_motion_vec(valueWalk, stick_x);
                KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
            }
        }
    }
    /*if motion_kind == hash40("special_hi3") || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_END {
        if situation_kind == *SITUATION_KIND_GROUND {
            if frame < 46.0 {
                if stick_x != 0.0 {
                    let motion_vec = x_motion_vec(motion_value, stick_x);
                    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
                }
            }
        }
    }*/
}

// Land cancel stuff
unsafe fn land_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, motion_kind: u64) {
    // Activate Land cancel flag
    if motion_kind == hash40("special_hi3") {
        VarModule::on_flag(fighter.battle_object, vars::common::SPIN_ATTACK_LAND_CANCEL);
    }
    // Reset Land cancel flag
    if !(motion_kind == hash40("special_hi3") || motion_kind == hash40("special_air_hi3")) {
        VarModule::off_flag(fighter.battle_object, vars::common::SPIN_ATTACK_LAND_CANCEL);
    }
    // Land cancel
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL && VarModule::is_flag(fighter.battle_object, vars::common::SPIN_ATTACK_LAND_CANCEL) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
    }
}

// Attacks out of Aerial Assault
unsafe fn aerial_acrobatics(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, motion_kind: u64, frame: f32) {

    if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK {
        if motion_kind == hash40("special_s1") || (motion_kind == hash40("special_air_s1") && frame >= 15.0){
            VarModule::on_flag(fighter.battle_object, vars::common::SIDE_SPECIAL_CANCEL);
            if boma.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
            }
            if (boma.is_cat_flag(Cat1::AttackS3) && boma.is_stick_forward())
                || (boma.is_cat_flag(Cat1::AttackS4) && boma.is_stick_forward()) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
            }
            if (boma.is_cat_flag(Cat1::AttackS3) && boma.is_stick_backward())
                || (boma.is_cat_flag(Cat1::AttackS4) && boma.is_stick_backward()) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
            }
            if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
            }
            if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
            }
        }
    }
}

// Re-enable Gale Strike once tornado is gone
unsafe fn gale_strike_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    let gimmick_timer = VarModule::get_int(fighter.battle_object, vars::common::GIMMICK_TIMER);
	if gimmick_timer > 0 {
        VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, gimmick_timer - 1);
    }
}

// Skyward Slash Dash actionability
unsafe fn skyward_slash_dash_act(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
	if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, vars::miiswordsman::SKYWARD_SLASH_DASH_HIT);
            //println!("SSD Hit");
        }
    }
    if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH_END {
        if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::SKYWARD_SLASH_DASH_HIT) && !VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK) && situation_kind == *SITUATION_KIND_AIR {
            //println!("SSD Success");
            if frame >= 30.0 {
                //println!("SSD Fall Act");
                VarModule::off_flag(fighter.battle_object, vars::miiswordsman::SKYWARD_SLASH_DASH_HIT);
                VarModule::on_flag(fighter.battle_object, vars::common::UP_SPECIAL_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

// Kinesis Blade OPFF stuff
unsafe fn kinesis_blade(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64) {
	if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && (motion_kind == hash40("special_lw1") || motion_kind == hash40("special_air_lw1")){
        if VarModule::get_int(boma.object(), vars::miiswordsman::SPECIAL_LW1_CHARGE_LEVEL) > 0 {
            //println!("Kinesis ready");
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                //println!("Kinesis activation");
                VarModule::on_flag(boma.object(), vars::miiswordsman::SPECIAL_LW1_ATTACK_TRIGGER);
                StatusModule::change_status_request(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT, false);
            }
        }
        /*
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) {
            println!("Kinesis anim test");
            MotionModule::change_motion(boma, Hash40::new("special_lw1_hit_lv2"), 0.0, 1.0, false, 0.0, false, false);
        }
        */
    }
}

// Transition into hitgrab on hit
unsafe fn hitgrab_transition(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64) {
	if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && ((motion_kind == hash40("special_lw3") && MotionModule::frame(boma) > 16.0) || (motion_kind == hash40("special_air_lw3") && MotionModule::frame(boma) > 10.0)){
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !fighter.is_in_hitlag() {
            //println!("Swordfighter gon' give it to ya");
            StatusModule::change_status_request(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END, false);
        }
    }
}

// Lengthen sword
unsafe fn sword_length(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.015, y: 1.3, z: 1.045};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}



pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //gale_stab_jc_attack(fighter, boma, id, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    //aerial_power_thrust_jump_reset(fighter, boma, status_kind, situation_kind, motion_kind);
    //heros_spin_movement(fighter, boma, status_kind, situation_kind, motion_kind, stick_x, frame);
    //land_cancel(boma, id, status_kind, motion_kind);
	sword_length(fighter, boma);
    //aerial_acrobatics(fighter, boma, id, status_kind, situation_kind, cat[0], motion_kind, frame);
    gale_strike_timer(fighter, boma, id);
    skyward_slash_dash_act(fighter, boma, id, status_kind, situation_kind, frame);
    //kinesis_blade(fighter, boma, status_kind, motion_kind);
    //hitgrab_transition(fighter, boma, status_kind, motion_kind);

}

#[utils::macros::opff(FIGHTER_KIND_MIISWORDSMAN )]
pub fn miiswordsman_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		miiswordsman_frame(fighter)
    }
}

pub unsafe fn miiswordsman_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
