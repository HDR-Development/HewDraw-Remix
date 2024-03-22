
use super::*;

unsafe extern "C" fn yoshi_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.857);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("mouth2"), 3.5, -2.5, -0.5, 0.0, Some(0.7), Some(-0.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
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

unsafe extern "C" fn yoshi_catch_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("mouth2"), 3.5, -2.5, -0.5, 0.0, Some(0.7), Some(-0.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 5.5, 0.0, 6.0, 9.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

unsafe extern "C" fn yoshi_catch_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, Some(0.7), Some(-0.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 5.5, 0.0, 6.0, -8.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

pub fn install() {
    smashline::Agent::new("yoshi")
        .acmd("game_catch", yoshi_catch_game)
        .acmd("game_catchdash", yoshi_catch_dash_game)
        .acmd("game_catchturn", yoshi_catch_turn_game)
        .install();
}
