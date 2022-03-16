// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn jab_2_ftilt_cancel(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if [*FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) && motion_kind == hash40("attack_12") {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if boma.is_cat_flag(Cat1::AttackS3)
               && (WorkModule::is_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) || WorkModule::is_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)) {
                if !boma.is_in_hitlag() {
                    VarModule::on_flag(boma.object(), vars::trail::ATTACK_12_INTO_S3);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
                }
            }
        }
    }
    if ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::trail::ATTACK_12_INTO_S3);
    }
}

// Fair 2 -> aerial cancel
unsafe fn fair_cancels(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    // Check for aerial attack inputs during fair 2
    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F && motion_kind == hash40("attack_air_f2") {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4)
                && ControlModule::get_attack_air_kind(boma) != *FIGHTER_COMMAND_ATTACK_AIR_KIND_F
                && ControlModule::get_attack_air_kind(boma) != *FIGHTER_COMMAND_ATTACK_AIR_KIND_B {
                if !boma.is_in_hitlag() {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
            }
            //if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_HI
            if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                if !boma.is_in_hitlag() {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
            }
            //if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW
            if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                if !boma.is_in_hitlag() {                                        
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
            }
        }
    }
}

unsafe fn side_special_hit_check(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, id: usize) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            VarModule::on_flag(boma.object(), vars::trail::UP_SPECIAL_TO_SIDE_SPECIAL);
        }
        else {
            VarModule::off_flag(boma.object(), vars::trail::UP_SPECIAL_TO_SIDE_SPECIAL);
        }
    }
    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK {
        if !VarModule::is_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL_NO_HIT) {
            VarModule::on_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL_NO_HIT);
        }
        if fighter.global_table[CURRENT_FRAME].get_i32() == 0 {
            VarModule::off_flag(boma.object(), vars::trail::SIDE_SPECIAL_HIT);
            VarModule::off_flag(boma.object(), vars::trail::STOP_SIDE_SPECIAL);
        }
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        && !fighter.is_in_hitlag()
        && (WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("attack_num")) - 1) > WorkModule::get_int(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT) {
            VarModule::on_flag(boma.object(), vars::trail::SIDE_SPECIAL_HIT);
            if !VarModule::is_flag(boma.object(), vars::trail::UP_SPECIAL_TO_SIDE_SPECIAL)
            && fighter.is_input_jump() {
                if situation_kind == *SITUATION_KIND_GROUND {
                    fighter.change_status_req(*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    return;
                }
                else if fighter.get_jump_count() < fighter.get_jump_count_max() {
                    fighter.change_status_req(*FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    return;
                }
            }
        }
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            VarModule::on_flag(boma.object(), vars::trail::STOP_SIDE_SPECIAL);
        }
    }
    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH {
        if fighter.global_table[CURRENT_FRAME].get_i32() == 0 {
            VarModule::off_flag(boma.object(), vars::trail::IS_SIDE_SPECIAL_INPUT);
        }
        if compare_mask(ControlModule::get_command_flag_cat(boma, 0), *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) {
            VarModule::on_flag(boma.object(), vars::trail::IS_SIDE_SPECIAL_INPUT);
        }
        if VarModule::is_flag(boma.object(), vars::trail::SIDE_SPECIAL_HIT)
        && WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("attack_num")) > WorkModule::get_int(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT) {
            if !VarModule::is_flag(boma.object(), vars::trail::UP_SPECIAL_TO_SIDE_SPECIAL)
            && fighter.is_input_jump() {
                if situation_kind == *SITUATION_KIND_GROUND {
                    fighter.change_status_req(*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    return;
                }
                else if fighter.get_jump_count() < fighter.get_jump_count_max() {
                    fighter.change_status_req(*FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    return;
                }
            }
        }
    }
    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END {
        if !VarModule::is_flag(boma.object(), vars::trail::STOP_SIDE_SPECIAL)
        && WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("attack_num")) > WorkModule::get_int(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT)
        && fighter.global_table[CURRENT_FRAME].get_i32() == 15 {
            VarModule::off_flag(boma.object(), vars::trail::STOP_SIDE_SPECIAL);
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
            }
            else {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
            }
            return;
        }
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    jab_2_ftilt_cancel(boma, cat[0], status_kind, situation_kind, motion_kind);
    fair_cancels(boma, cat[0], status_kind, situation_kind, motion_kind);
    side_special_hit_check(fighter, boma, status_kind, situation_kind, id);
}

#[utils::macros::opff(FIGHTER_KIND_TRAIL)]
pub fn trail_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		trail_frame(fighter)
    }
}

pub unsafe fn trail_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        let status_kind = if info.status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N || info.status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F { // status kind checks for nair/fair
            *FIGHTER_STATUS_KIND_ATTACK_AIR
        } else {
            info.status_kind
        };
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}