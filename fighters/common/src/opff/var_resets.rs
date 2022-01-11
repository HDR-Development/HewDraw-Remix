use crate::opff_import::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;


unsafe fn special_cancel_flag_reset(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    
    // Up Special Cancel
    if VarModule::is_flag(boma.object(), common::UP_SPECIAL_CANCEL) {
        if situation_kind != *SITUATION_KIND_AIR
            || [*SITUATION_KIND_AIR,
                *FIGHTER_STATUS_KIND_DAMAGE,
                *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY ,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                *FIGHTER_STATUS_KIND_DAMAGE_FALL,
                *FIGHTER_STATUS_KIND_DEAD,
                *FIGHTER_STATUS_KIND_REBIRTH,
                *FIGHTER_STATUS_KIND_WIN,
                *FIGHTER_STATUS_KIND_LOSE,
                *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
            VarModule::off_flag(boma.object(), common::UP_SPECIAL_CANCEL);
        }
    }

    // Side Special Cancel
    if VarModule::is_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL) {
        if situation_kind != *SITUATION_KIND_AIR
            || [*SITUATION_KIND_AIR,
                *FIGHTER_STATUS_KIND_DAMAGE,
                *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY ,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                *FIGHTER_STATUS_KIND_DAMAGE_FALL,
                *FIGHTER_STATUS_KIND_DEAD,
                *FIGHTER_STATUS_KIND_REBIRTH,
                *FIGHTER_STATUS_KIND_WIN,
                *FIGHTER_STATUS_KIND_LOSE,
                *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
            VarModule::off_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL);
        }
    }

    // Aerial Special Used
    if VarModule::is_flag(boma.object(), vars::common::AIR_SPECIAL_USED) {
        if situation_kind != *SITUATION_KIND_AIR
             || [*SITUATION_KIND_AIR,
                *FIGHTER_STATUS_KIND_DAMAGE,
                *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY ,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                *FIGHTER_STATUS_KIND_DAMAGE_FALL,
                *FIGHTER_STATUS_KIND_DEAD,
                *FIGHTER_STATUS_KIND_REBIRTH,
                *FIGHTER_STATUS_KIND_WIN,
                *FIGHTER_STATUS_KIND_LOSE,
                *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
            VarModule::off_flag(boma.object(), vars::common::AIR_SPECIAL_USED);
        }
    }

    // Up Special Wall Jump
    if VarModule::is_flag(boma.object(), vars::common::SPECIAL_WALL_JUMP) {
        if situation_kind != SITUATION_KIND_AIR
            || [*FIGHTER_STATUS_KIND_DEAD,
                *FIGHTER_STATUS_KIND_REBIRTH,
                *FIGHTER_STATUS_KIND_WIN,
                *FIGHTER_STATUS_KIND_LOSE,
                *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
            VarModule::off_flag(boma.object(), vars::common::SPECIAL_WALL_JUMP);
        }
    }

    // Up Special Interrupt
    if VarModule::is_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT) {
        if situation_kind != *SITUATION_KIND_AIR
            || [*FIGHTER_STATUS_KIND_DAMAGE,
                *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                *FIGHTER_STATUS_KIND_DAMAGE_FALL,
                *FIGHTER_STATUS_KIND_DEAD,
                *FIGHTER_STATUS_KIND_REBIRTH,
                *FIGHTER_STATUS_KIND_WIN,
                *FIGHTER_STATUS_KIND_LOSE,
                *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
            VarModule::off_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT);
        }
    }

    // Up Special Intterupt Airtime
    if VarModule::is_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT_AIRTIME) {
        if situation_kind != *SITUATION_KIND_AIR
            || [*FIGHTER_STATUS_KIND_DAMAGE,
                *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                *FIGHTER_STATUS_KIND_DAMAGE_FALL,
                *FIGHTER_STATUS_KIND_DEAD,
                *FIGHTER_STATUS_KIND_REBIRTH,
                *FIGHTER_STATUS_KIND_WIN,
                *FIGHTER_STATUS_KIND_LOSE,
                *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
            VarModule::off_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT_AIRTIME);
        }
    }
}

unsafe fn special_motion_reset(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if situation_kind == *SITUATION_KIND_GROUND
        || [*FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::common::SPECIAL_STALL);
        VarModule::off_flag(boma.object(), vars::common::SPECIAL_STALL_USED);
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    special_cancel_flag_reset(boma, status_kind, situation_kind);
    special_motion_reset(boma, status_kind, situation_kind)
}
