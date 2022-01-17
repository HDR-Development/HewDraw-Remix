// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Robin Thunder dash and airdodge cancel
unsafe fn thunder_dash_airdodge_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT {
        if situation_kind == *SITUATION_KIND_AIR {
            if frame > 17.0 {
                if boma.is_cat_flag(Cat1::JumpButton) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        } else if situation_kind == *SITUATION_KIND_GROUND {
            if frame > 17.0 {
                if boma.is_cat_flag(Cat1::Walk) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, false);
                }
                if boma.is_cat_flag(Cat1::Turn) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN_DASH, false);
                }
            }
        }
    }
}

// Robin Elwind 1 Cancel
unsafe fn elwind1_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, motion_kind: u64, frame: f32) {
    if [hash40("special_hi"), hash40("special_air_hi")].contains(&motion_kind) {
        if frame > 8.0 && frame <= 12.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                VarModule::on_flag(boma.object(), vars::common::UP_SPECIAL_CANCEL);
                //ControlModule::clear_command(boma, true);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_HI{
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
    }
}

// Lengthen sword
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    let long_sword_scale = Vector3f{x: 1.0, y: 1.175, z: 1.0475};
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    nspecial_cancels(boma, status_kind, situation_kind);
    thunder_dash_airdodge_cancel(boma, status_kind, situation_kind, cat[0], frame);
    elwind1_cancel(boma, id, status_kind, motion_kind, frame);
    sword_length(boma);
}

#[utils::macros::opff(FIGHTER_KIND_REFLET )]
pub fn reflet_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        reflet_frame(fighter)
    }
}

pub unsafe fn reflet_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}