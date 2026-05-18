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

// TODO: this has got to be expensive?
unsafe fn var_resets(boma: &mut BattleObjectModuleAccessor) {
    let death_statuses = &[*FIGHTER_STATUS_KIND_DEAD,
                                        *FIGHTER_STATUS_KIND_REBIRTH,
                                        *FIGHTER_STATUS_KIND_WIN,
                                        *FIGHTER_STATUS_KIND_LOSE,
                                        *FIGHTER_STATUS_KIND_ENTRY];

    let damage_statuses = &[*FIGHTER_STATUS_KIND_DAMAGE,
                                        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FALL];

    // Up Special Cancel
    if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
        if !boma.is_situation(*SITUATION_KIND_AIR)
        || boma.is_status_one_of(damage_statuses)
        || boma.is_status_one_of(death_statuses) 
        || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP])
            {
            VarModule::off_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
        }
    }

    // Side Special Cancel
    if VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL) {
        if !boma.is_situation(*SITUATION_KIND_AIR)
        || boma.is_status_one_of(damage_statuses)
        || boma.is_status_one_of(death_statuses)
        || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP])
        {
            VarModule::off_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL);
        }
    }

    // Side Special Cancel (doesn't reset on hit)
    if VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT)
    && (!boma.is_situation(*SITUATION_KIND_AIR)
    || boma.is_status_one_of(death_statuses))
    || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP])
    {
        VarModule::off_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }

    // Up Special Wall Jump
    if VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_WALL_JUMP) {
        if !boma.is_situation(*SITUATION_KIND_AIR)
        || boma.is_status_one_of(death_statuses)
        || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP])
        {
            VarModule::off_flag(boma.object(), vars::common::instance::SPECIAL_WALL_JUMP);
        }
    }

    // Up Special Landing Lag
    if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_LAG)
    && !StatusModule::is_situation_changed(boma)
    && !boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL])
    {
        if !boma.is_situation(*SITUATION_KIND_AIR)
        || boma.is_status_one_of(damage_statuses)
        || boma.is_status_one_of(death_statuses)
        {
            VarModule::off_flag(boma.object(), vars::common::instance::UP_SPECIAL_LAG);
        }
    }

    // Special Motion Reset
    if !boma.is_situation(*SITUATION_KIND_AIR)
    || boma.is_status_one_of(death_statuses)
    || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP])
    {
        VarModule::off_flag(boma.object(), vars::common::instance::SPECIAL_STALL);
        VarModule::off_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED);
    }

    // Wall Jump Reset
    if !boma.is_situation(*SITUATION_KIND_AIR)
    || AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT)
    || boma.is_status_one_of(death_statuses) 
    || boma.is_status_one_of(damage_statuses)
    || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP])
    {
        boma.set_int(0, *FIGHTER_INSTANCE_WORK_ID_INT_WALL_JUMP_COUNT);
    }

    // Successive aerial jump timer for multijump characters
    if VarModule::get_int(boma.object(), vars::common::instance::FLY_NEXT_FRAME) > 0 {
        VarModule::dec_int(boma.object(), vars::common::instance::FLY_NEXT_FRAME);
    }

    // Reset total damage dealt this stock
    if boma.is_status_one_of(death_statuses) {
        VarModule::set_float(boma.object(), vars::common::instance::DAMAGE_DEALT_THIS_STOCK, 0.0);
    }

    // Clear last attacker when grounded (used by War mode)
    if !boma.is_situation(*SITUATION_KIND_AIR)
    && !boma.is_status_one_of(damage_statuses)
    && !boma.is_status_one_of(death_statuses)
    {
        VarModule::set_int(boma.object(), vars::common::instance::LAST_ATTACKER_ENTRY_ID, -1);
    }

    // Zair once-per-airtime reset
    if VarModule::is_flag(boma.object(), vars::common::instance::DISABLE_AIR_LASSO)
    && (!boma.is_situation(*SITUATION_KIND_AIR)
    || boma.is_status_one_of(death_statuses))
    || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP])
    {
        VarModule::off_flag(boma.object(), vars::common::instance::DISABLE_AIR_LASSO);
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    var_resets(boma);
}
