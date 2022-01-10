use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn bouncing_fish_return_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind != *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN || frame <= 10.0 {
        return;
    }

    if situation_kind == *SITUATION_KIND_AIR {
        if moveset_utils::jump_checker_buffer(boma, cat1) {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
            }
        } else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        } else if boma.is_cat_flag(Cat1::SpecialHi) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
        }
    }
}

unsafe fn nspecial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
            }
        }
    }
}

// Sheik Grenade Pull Cancel
unsafe fn grenade_pull(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if VarModule::get_int(fighter.battle_object, vars::common::GIMMICK_TIMER) != 0 {
        return;
    }

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && frame < 15.0 {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            if situation_kind == *SITUATION_KIND_AIR {
                if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                    VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 1); // Start counting
                }
            }
        }
    }
}

// Grenade Cancel Timer Count
unsafe fn grenade_cancel_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    let gimmick_timerr = VarModule::get_int(fighter.battle_object, vars::common::GIMMICK_TIMER);
    if gimmick_timerr > 0 && gimmick_timerr < 901 {
        if gimmick_timerr > 899 {
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
            gimmick_flash(boma);
        } else {
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, gimmick_timerr + 1);
        }
    }
}

// Grenade Cancel Timer Death Reset
unsafe fn grenade_cancel_reset(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
        VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
    }
}

// Training Mode Grenade Cancel Timer taunt reset
unsafe fn grenade_cancel_training(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if hdr::is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
        }
    }
}

// Up Special Cancels
unsafe fn up_special_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if moveset_utils::jump_checker_buffer(boma, cat1) {
                if situation_kind == *SITUATION_KIND_AIR {
                    if hdr::get_jump_count(boma) < hdr::get_jump_count_max(boma) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                } else if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    bouncing_fish_return_cancel(fighter, boma, status_kind, situation_kind, cat[0], frame);
    nspecial_cancels(fighter, boma, status_kind, situation_kind);
    //grenade_pull(fighter, boma, id, status_kind, situation_kind, frame);
    grenade_cancel_timer(fighter, boma, id);
    grenade_cancel_reset(fighter, id, status_kind);
    grenade_cancel_training(fighter, id, status_kind);
    teleports::sheik_teleport_cancel(boma, status_kind, id);

    // Magic Series
    //up_special_cancels(fighter, boma, status_kind, situation_kind, cat[0]);
}

#[utils::opff(FIGHTER_KIND_SHEIK )]
pub fn sheik_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		sheik_frame(fighter)
    }
}

pub unsafe fn sheik_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}