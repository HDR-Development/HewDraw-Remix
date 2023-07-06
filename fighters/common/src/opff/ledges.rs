use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f, Hash40};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;
use smash::app::utility::*;

//=================================================================
//== LEDGE ACTIONABILITY
//=================================================================
unsafe fn ledge_act(boma: &mut BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    if [*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&status_kind) {
        if fighter_kind != *FIGHTER_KIND_NANA {
            if boma.status_frame() > 6 {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
            }
        }
    }
}

//=================================================================
//== LEDGE OCCUPANCY
//=================================================================
unsafe fn occupy_ledge(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
        *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3])
    && MotionModule::frame(boma) > (FighterMotionModuleImpl::get_cancel_frame(boma, Hash40::new_raw(MotionModule::motion_kind(boma)), true) * 0.75) {
        VarModule::set_vec3(boma.object(), vars::common::instance::LEDGE_POS, Vector3f {x: std::f32::NAN, y: std::f32::NAN, z: std::f32::NAN});
    }
}

//=================================================================
//== TETHER TRUMP LANDING LAG
//=================================================================
unsafe fn tether_trump_landing(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    let player_number = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);

    if status_kind == *FIGHTER_STATUS_KIND_CLIFF_ROBBED {
        VarModule::on_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
    }

    // Go into special fall after one action after trump
    /*
    if situation_kind == *SITUATION_KIND_AIR && prev_status_kind == *FIGHTER_STATUS_KIND_CLIFF_ROBBED {
        VarModule::off_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
    }
    */

    // Increased landing lag (special fall landing) if landing right after being tether hogged
    if /*prev_status_kind == *FIGHTER_STATUS_KIND_CLIFF_ROBBED &&*/ VarModule::is_flag(boma.object(), vars::common::instance::TETHER_HOGGED) && situation_kind == *SITUATION_KIND_GROUND {
        VarModule::off_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
        let landing_lag = ParamModule::get_int(boma.object(), ParamType::Common, "tether_trump_landing_lag");
        WorkModule::set_float(boma, landing_lag as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
    }

    if situation_kind == *SITUATION_KIND_CLIFF {
        VarModule::off_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
    }

    if [*FIGHTER_STATUS_KIND_DAMAGE,
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
        *FIGHTER_STATUS_KIND_ENTRY,
        /* *FIGHTER_STATUS_KIND_STANDBY*/].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
    }

    /*
    if prev_status_kind != *FIGHTER_STATUS_KIND_CLIFF_ROBBED {
        VarModule::off_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
    }
    */
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    ledge_act(boma, status_kind, fighter_kind);
    occupy_ledge(boma, status_kind, situation_kind, fighter_kind);
    tether_trump_landing(boma, status_kind, situation_kind);
}