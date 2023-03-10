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
    // Allow charge jump cancel even when left stick is down
    if boma.is_status(*FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD) {
        if boma.is_input_jump() && boma.get_num_used_jumps() < boma.get_jump_count_max() {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_JUMP_CANCEL, false);
            WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
        }
    }
}

unsafe fn elwind1_burn(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        if fighter.global_table[CURRENT_FRAME].get_i32() == 1 {
            // burn an extra bar of Elwind on upB1 (totals 2 bars)
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
        }
    }
}

// Lengthen sword
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    let long_sword_scale = Vector3f{x: 1.0, y: 1.175, z: 1.0475};
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    nspecial_cancels(boma, status_kind, situation_kind);
    elwind1_burn(fighter);
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
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}