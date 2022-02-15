use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;

//=================================================================
//== SHIELD DROPS
//=================================================================
/// Transitions caller's status kind to plat drop if they are inputting one or if they are pressing a taunt button
unsafe fn shield_drop(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON])
    && boma.is_cat_flag(Cat2::GuardToPass | Cat2::AppealAll)
    && GroundModule::is_passable_ground(boma)
    {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    shield_drop(boma);
}
