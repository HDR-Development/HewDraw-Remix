use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Hero dash cancel Frizz
unsafe fn dash_cancel_frizz(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_SHOOT {
        if situation_kind == *SITUATION_KIND_GROUND {
            if [hash40("special_n1"), hash40("special_n2")].contains(&motion_kind) {
                if frame > 17.0 {
                    if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, false);
                    }
                    if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN_DASH, false);
                    }
                }
            }
        }
    }
}

// Lengthen swords
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    let long_sword_scale = Vector3f{x: 1.125, y: 1.05, z: 1.045};
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword1"), &long_sword_scale);
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    nspecial_cancels(boma, status_kind, situation_kind);
    dash_cancel_frizz(boma, status_kind, situation_kind, cat[0], motion_kind, frame);
    sword_length(boma);

    // Frame Data
    frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
        if motion_kind == hash40("attack_11") {
            if frame < 7.0 {
                MotionModule::set_rate(boma, 1.4);
            }
            if frame >= 7.0 {
                MotionModule::set_rate(boma, 1.0);
            }
        }
    }
}
#[utils::opff(FIGHTER_KIND_BRAVE )]
pub fn brave_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		brave_frame(fighter)
    }
}

pub unsafe fn brave_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}