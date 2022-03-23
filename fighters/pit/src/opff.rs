// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


#[no_mangle]
pub unsafe extern "Rust" fn pits_common(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    power_of_flight_cancel(boma, status_kind)
}


// Pits Power of Flight cancel
unsafe fn power_of_flight_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, false);
        }
    }
}
 
unsafe fn upperdash_arm_jump_and_aerial_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32, id: usize) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S || (status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END && frame > 5.0) {
        if frame > 27.0 {
            if boma.is_input_jump() {
                if situation_kind == *SITUATION_KIND_AIR {
                    if boma.get_num_used_jumps() < boma.get_jump_count_max() - 1 &&  !VarModule::is_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL) {
                        VarModule::on_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                } else if situation_kind == *SITUATION_KIND_GROUND {
                    if facing * stick_x < 0.0 {
                        PostureModule::reverse_lr(boma);
                        // Does this need PostureModule::update_rot_y_lr(boma)?
                    }
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}



pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    upperdash_arm_jump_and_aerial_cancel(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame, id);
    pits_common(boma, status_kind);
}

#[utils::macros::opff(FIGHTER_KIND_PIT )]
pub fn pit_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pit_frame(fighter)
    }
}

pub unsafe fn pit_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}