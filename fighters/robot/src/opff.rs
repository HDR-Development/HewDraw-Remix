// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn gyro_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_END {
        if frame > 10.0 {
            if situation_kind == *SITUATION_KIND_GROUND {
                if boma.is_cat_flag(Cat1::Dash) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, false);
                }
                if boma.is_cat_flag(Cat1::Turn) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN_DASH, false);
                }
            }
        }
    }
}

// Neutral Special Cancels
unsafe fn neutral_special_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if boma.is_input_jump() {
                if situation_kind == *SITUATION_KIND_AIR {
                    if boma.get_jump_count() < boma.get_jump_count_max() {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                } else if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

unsafe fn dspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like down-b canceling
    if status_kind == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_HOLD {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::AirEscape) {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    gyro_dash_cancel(boma, status_kind, situation_kind, cat[0], frame);

    // Magic Series
    neutral_special_cancels(boma, status_kind, situation_kind, cat[0]);
    dspecial_cancels(boma, status_kind, situation_kind, cat[0]);
}

#[utils::macros::opff(FIGHTER_KIND_ROBOT )]
pub fn robot_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		robot_frame(fighter)
    }
}

pub unsafe fn robot_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}