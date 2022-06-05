use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;

const DAMAGE_STATUSES : &'static [i32] = &[*FIGHTER_STATUS_KIND_DEAD,
                                            *FIGHTER_STATUS_KIND_REBIRTH,
                                            *FIGHTER_STATUS_KIND_WIN,
                                            *FIGHTER_STATUS_KIND_LOSE,
                                            *FIGHTER_STATUS_KIND_ENTRY];

const DEATH_STATUSES : &'static [i32] = &[*FIGHTER_STATUS_KIND_DAMAGE,
                                            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FALL];

unsafe fn special_cancel_flag_reset(boma: &mut BattleObjectModuleAccessor) {
    
    // Up Special Cancel
    if VarModule::is_flag(boma.object(), vars::common::UP_SPECIAL_CANCEL) {
        if !boma.is_situation(*SITUATION_KIND_AIR)
        || boma.is_status_one_of(DAMAGE_STATUSES)
        || boma.is_status_one_of(DEATH_STATUSES) {
            VarModule::off_flag(boma.object(), vars::common::UP_SPECIAL_CANCEL);
        }
    }

    // Side Special Cancel
    if VarModule::is_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL) {
        if !boma.is_situation(*SITUATION_KIND_AIR)
        || boma.is_status_one_of(DAMAGE_STATUSES)
        || boma.is_status_one_of(DEATH_STATUSES) {
            VarModule::off_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL);
        }
    }

    // Side Special Cancel (doesn't reset on hit)
    if VarModule::is_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL_NO_HIT)
    && (!boma.is_situation(*SITUATION_KIND_AIR)
    || boma.is_status_one_of(DEATH_STATUSES)) {
        VarModule::off_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL_NO_HIT);
    }

    // Aerial Special Used
    if VarModule::is_flag(boma.object(), vars::common::AIR_SPECIAL_USED) {
        if !boma.is_situation(*SITUATION_KIND_AIR)
        || boma.is_status_one_of(DAMAGE_STATUSES)
        || boma.is_status_one_of(DEATH_STATUSES) {
            VarModule::off_flag(boma.object(), vars::common::AIR_SPECIAL_USED);
        }
    }

    // Up Special Wall Jump
    if VarModule::is_flag(boma.object(), vars::common::SPECIAL_WALL_JUMP) {
        if !boma.is_situation(*SITUATION_KIND_AIR)
        || boma.is_status_one_of(DEATH_STATUSES) {
            VarModule::off_flag(boma.object(), vars::common::SPECIAL_WALL_JUMP);
        }
    }

    // Up Special Interrupt
    if VarModule::is_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT) {
        if !boma.is_situation(*SITUATION_KIND_AIR)
        || boma.is_status_one_of(DAMAGE_STATUSES)
        || boma.is_status_one_of(DEATH_STATUSES) {
            VarModule::off_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT);
        }
    }

    // Up Special Intterupt Airtime
    if VarModule::is_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT_AIRTIME) {
        if !boma.is_situation(*SITUATION_KIND_AIR)
        || boma.is_status_one_of(DAMAGE_STATUSES)
        || boma.is_status_one_of(DEATH_STATUSES) {
            VarModule::off_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT_AIRTIME);
        }
    }
}

unsafe fn special_motion_reset(boma: &mut BattleObjectModuleAccessor) {
    if !boma.is_situation(*SITUATION_KIND_AIR)
    || boma.is_status_one_of(DEATH_STATUSES) {
        VarModule::off_flag(boma.object(), vars::common::SPECIAL_STALL);
        VarModule::off_flag(boma.object(), vars::common::SPECIAL_STALL_USED);
    }
}

unsafe fn aerial_glide_toss_reset(boma: &mut BattleObjectModuleAccessor) {
    if !boma.is_situation(*SITUATION_KIND_AIR)
    || boma.is_status_one_of(DEATH_STATUSES) {
        VarModule::set_int(boma.object(), vars::common::AGT_USED_COUNTER, 0);
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    special_cancel_flag_reset(boma);
    special_motion_reset(boma);
    aerial_glide_toss_reset(boma);
}
