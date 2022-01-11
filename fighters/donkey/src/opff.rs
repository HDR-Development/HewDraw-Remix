use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn nspecial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}

// Barrel Timer Count
unsafe fn barrel_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
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

// Barrel Timer Death Reset
unsafe fn barrel_reset(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
        VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
    }
}

// Training Mode Barrel Timer taunt reset
unsafe fn barrel_training(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
        VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
    }
}

unsafe fn barrel_pull(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    // barrel pull
    if situation_kind == *SITUATION_KIND_GROUND {
        if status_kind == *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP {
            MotionModule::set_rate(boma, 1.4);
        } else if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF].contains(&status_kind) {
            if ItemModule::is_have_item(boma, 0)
                && ItemModule::get_have_item_kind(fighter.module_accessor, 0) == *ITEM_KIND_BARREL {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(),true.into());
            }
        }
    } 
}

// DK Giant Punch charge B-Reverse
unsafe fn giant_punch_b_reverse(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if frame < 5.0 {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if frame > 1.0 && frame < 5.0 &&  !VarModule::is_flag(boma.object(), vars::common::B_REVERSED) {
                    let b_reverse = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &b_reverse, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    VarModule::on_flag(boma.object(), vars::common::B_REVERSED);
                }
            }
        }
    }
}

// DK Headbutt aerial stall
unsafe fn headbutt_aerial_stall(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        let motion_value = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        let motion_vec = Vector3f{x: 1.0, y: 0.0, z: 1.0};
        if situation_kind == *SITUATION_KIND_AIR {
            if  !VarModule::is_flag(boma.object(), vars::common::SPECIAL_STALL_USED) {
                if frame < 25.0 {
                    if motion_value < 0.0 {
                        VarModule::on_flag(boma.object(), vars::common::SPECIAL_STALL);
                        KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    }
                }
            }
        }
    }
    if VarModule::is_flag(boma.object(), vars::common::SPECIAL_STALL) && (status_kind != *FIGHTER_STATUS_KIND_SPECIAL_S || (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && frame >= 25.0)) {
            VarModule::on_flag(boma.object(), vars::common::SPECIAL_STALL_USED);
            VarModule::off_flag(boma.object(), vars::common::SPECIAL_STALL);
    }
    if situation_kind == *SITUATION_KIND_GROUND && special_stall_used[id] {
        VarModule::off_flag(boma.object(), vars::common::SPECIAL_STALL_USED);
    }
}

// Down Special Cancels
unsafe fn down_special_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if AttackModule::is_infliction(boma, 2) {
            VarModule::on_flag(boma.object(), vars::common::SPECIAL_CHECKS);
        }
        if VarModule::is_flag(boma.object(), vars::common::SPECIAL_CHECKS) && frame > 5.0 {
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
    } else {
        VarModule::off_flag(boma.object(), vars::common::SPECIAL_CHECKS);
    }
}



pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    barrel_timer(fighter, boma, id);
    barrel_reset(fighter, id, status_kind);
    barrel_training(fighter, id, status_kind);
    giant_punch_b_reverse(fighter, boma, id, status_kind, stick_x, facing, frame);
    nspecial_cancels(fighter, boma, status_kind, situation_kind);
    barrel_pull(fighter, boma, status_kind, situation_kind);
    headbutt_aerial_stall(fighter, boma, id, status_kind, situation_kind, frame);

    // Magic Series
    //down_special_cancels(fighter, boma, id, status_kind, situation_kind, cat[0], frame);
}
#[utils::opff(FIGHTER_KIND_DONKEY )]
pub fn donkey_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		donkey_frame(fighter)
    }
}

pub unsafe fn donkey_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}