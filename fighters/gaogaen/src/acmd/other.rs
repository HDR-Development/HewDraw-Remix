
use super::*;

use smash::app::BattleObjectModuleAccessor;


#[acmd_script( agent = "gaogaen", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
        ControlModule::clear_command(boma, true);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        let grab_y = VarModule::get_float(fighter.battle_object, common::ANGLE_GRAB_STICK_Y);
        let mut z_mod = -1.0 * grab_y;
        if grab_y > 0.0 {
            z_mod = 3.0 * grab_y;
        }
        CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, (grab_y * 8.0) + 11.0, 13.0 - z_mod, Some(0.0), Some(10.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}


#[acmd_script( agent = "gaogaen", script = "game_catchturn" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_catchturn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
        ControlModule::clear_command(boma, true);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        let grab_y = VarModule::get_float(fighter.battle_object, common::ANGLE_GRAB_STICK_Y);
        let mut z_mod = 0.0;
        if grab_y > 0.0 {
            z_mod = 3.0 * grab_y;
        }
        CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, (grab_y * 8.0) + 11.0, -17.0 + z_mod, Some(0.0), Some(10.0), Some(-2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}


#[acmd_script( agent = "gaogaen", script = "game_catchdash" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_catchdash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
        ControlModule::clear_command(boma, true);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        let grab_y = VarModule::get_float(fighter.battle_object, common::ANGLE_GRAB_STICK_Y);
        let mut z_mod = 0.0;
        if grab_y > 0.0 {
            z_mod = 4.0 * grab_y;
        }
        CATCH(fighter, 0, Hash40::new("top"), 3.5, 0.0, (grab_y * 8.0) + 9.0, 12.0 - z_mod, Some(0.0), Some(9.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

#[acmd_script( agent = "gaogaen", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.2);
    }
	frame(lua_state, 11.0); // Effectively F13
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "gaogaen", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "gaogaen", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        gaogaen_catch_game,
        dash_game,
        dash_effect,
        turn_dash_game,
        gaogaen_catchturn_game,
        gaogaen_catchdash_game,
    );
}

