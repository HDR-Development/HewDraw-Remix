use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn areadbhar_autocancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT,
        *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            if frame < 26.0 {
                VarModule::off_flag(get_battle_object_from_accessor(boma), vars::common::SPECIAL_AUTOCANCEL);
            }
            if frame >= 26.0 {
                VarModule::on_flag(get_battle_object_from_accessor(boma), vars::common::SPECIAL_AUTOCANCEL);
            }
        }
    }
    if status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_LANDING && special_autocancel[id] {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
    }
}

//Amyr Jump Cancel (Raging Storm)
unsafe fn amyr_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_TURN,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_CANCEL,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_LANDING_1,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_LANDING_2,
        *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if situation_kind == *SITUATION_KIND_GROUND {
                if moveset_utils::jump_checker_buffer(boma, cat1) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

// Areadbhar Dash Cancel (Raging Storm)
unsafe fn areadbhar_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT,
        *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if situation_kind == *SITUATION_KIND_GROUND {
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

// Dsmash Dash Cancel (Raging Storm)
unsafe fn dsmash_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32, frame: f32) {
    if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT,    // ?
        *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if frame > 28.0 {
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

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    areadbhar_autocancel(boma, id, status_kind, situation_kind, frame);
    nspecial_cancels(boma, status_kind, situation_kind);

    // Magic Series
    amyr_jc(boma, status_kind, situation_kind, cat[0]);
    areadbhar_dash_cancel(boma, status_kind, situation_kind, cat[0]);
    dsmash_dash_cancel(boma, status_kind, cat[0], frame);
}

#[utils::opff(FIGHTER_KIND_MASTER )]
pub fn master_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		master_frame(fighter)
    }
}

pub unsafe fn master_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}