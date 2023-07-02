// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn dash_attack_jump_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
    && situation_kind == *SITUATION_KIND_AIR
    && MotionModule::frame(boma) >= 25.0 {
        fighter.check_jump_cancel(false);
    }
}

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

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// Barrel Timer Count
unsafe fn barrel_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    let gimmick_timerr = VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER);
    if gimmick_timerr > 0 && gimmick_timerr < 901 {
        if gimmick_timerr > 899 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
            gimmick_flash(boma);
        } else {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, gimmick_timerr + 1);
        }
    }
}

// Barrel Timer Death Reset
unsafe fn barrel_reset(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
    }
}

// Training Mode Barrel Timer taunt reset
unsafe fn barrel_training(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
        VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
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

// DK Headbutt aerial stall
unsafe fn headbutt_aerial_stall(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        let motion_value = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        let motion_vec = Vector3f{x: 1.0, y: 0.0, z: 1.0};
        if situation_kind == *SITUATION_KIND_AIR {
            if  !VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED) {
                if frame < 25.0 {
                    if motion_value < 0.0 {
                        VarModule::on_flag(boma.object(), vars::common::instance::SPECIAL_STALL);
                        KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    }
                }
            }
        }
    }
    if VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_STALL) && (status_kind != *FIGHTER_STATUS_KIND_SPECIAL_S || (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && frame >= 25.0)) {
            VarModule::on_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED);
            VarModule::off_flag(boma.object(), vars::common::instance::SPECIAL_STALL);
    }
    if situation_kind == *SITUATION_KIND_GROUND && VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED) {
        VarModule::off_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED);
    }
}

// Down Special Cancels
unsafe fn down_special_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if AttackModule::is_infliction(boma, 2) {
            VarModule::on_flag(boma.object(), vars::donkey::status::SPECIAL_CHECKS);
        }
        if VarModule::is_flag(boma.object(), vars::donkey::status::SPECIAL_CHECKS) && frame > 5.0 {
            boma.check_jump_cancel(false);
        }
    } else {
        VarModule::off_flag(boma.object(), vars::donkey::status::SPECIAL_CHECKS);
    }
}

pub unsafe fn dk_bair_rotation(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("attack_air_b")) {
        // angle bair down slightly
        fighter.set_joint_rotate("legl", Vector3f::new(0.0, 0.0, -20.0))
    }
}

/// this sets the ledgegrab box for the backside of up special, which 
/// enables DK to more consistently grab ledge with slipoff uspecial
pub unsafe fn special_hi_slipoff_grab(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR) && fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        fighter.set_back_cliff_hangdata(20.0, 10.0);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    dash_attack_jump_cancels(fighter, boma, status_kind, situation_kind);
    barrel_timer(fighter, boma, id);
    barrel_reset(fighter, id, status_kind);
    barrel_training(fighter, id, status_kind);
    nspecial_cancels(fighter, boma, status_kind, situation_kind);
    barrel_pull(fighter, boma, status_kind, situation_kind);
    headbutt_aerial_stall(fighter, boma, id, status_kind, situation_kind, frame);
    if VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) == 0 {
        utils::ui::UiManager::set_shoto_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_shoto_bar_percentage(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, 100.0);
    } else {
        utils::ui::UiManager::set_dk_barrel_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, false);
        utils::ui::UiManager::set_shoto_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
    }

    // Magic Series
    //down_special_cancels(fighter, boma, id, status_kind, situation_kind, cat[0], frame);
}
#[utils::macros::opff(FIGHTER_KIND_DONKEY )]
pub fn donkey_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		donkey_frame(fighter);
        dk_bair_rotation(fighter);
        special_hi_slipoff_grab(fighter);
    }
}

pub unsafe fn donkey_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}