use crate::opff_import::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;


unsafe fn special_cancel_flag_reset(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    let player_number = hdr::get_player_number(boma);

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
    if side_special_cancel[player_number] {
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
            side_special_cancel[player_number] = false;
        }
    }

    // Aerial Special Used
    if air_special_used[player_number] {
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
            air_special_used[player_number] = false;
        }
    }

    // Up Special Wall Jump
    if special_wall_jump[player_number] {
        if situation_kind != SITUATION_KIND_AIR
            || [*FIGHTER_STATUS_KIND_DEAD,
                *FIGHTER_STATUS_KIND_REBIRTH,
                *FIGHTER_STATUS_KIND_WIN,
                *FIGHTER_STATUS_KIND_LOSE,
                *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
            special_wall_jump[player_number] = false;
        }
    }

    // Up Special Interrupt
    if up_special_interrupt[player_number] {
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
            up_special_interrupt[player_number] = false;
        }
    }

    // Up Special Intterupt Airtime
    if up_special_interrupt_airtime[player_number] {
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
            up_special_interrupt_airtime[player_number] = false;
        }
    }
}

unsafe fn special_motion_reset(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    let player_number = hdr::get_player_number(boma);
    if situation_kind == *SITUATION_KIND_GROUND
        || [*FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
        special_stall[player_number] = false;
        special_stall_used[player_number] = false;
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    special_cancel_flag_reset(boma, status_kind, situation_kind);
    special_motion_reset(boma, status_kind, situation_kind)
}
