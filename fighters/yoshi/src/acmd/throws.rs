
use super::*;

unsafe extern "C" fn game_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.857);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        CATCH(fighter, 0, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, Some(1.2), Some(-0.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 5.5, 0.0, 6.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
    
}

pub fn install() {
    smashline::Agent::new("yoshi")
        .acmd("game_catch", game_catch)
        .install();
}
