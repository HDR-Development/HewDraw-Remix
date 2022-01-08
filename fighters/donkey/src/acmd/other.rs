
use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::utility::*;
use smash_script::*;
use smashline::*;
use smash_script::macros::*;
use crate::utils::hdr;
use crate::vars::*;

#[acmd_script( agent = "donkey", script = "expression_landingheavy" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn expression_landingheavy(fighter: &mut L2CAgentBase) {
  let lua_state = fighter.lua_state_agent;
  let boma = sv_system::battle_object_module_accessor(lua_state);
  if is_excute(fighter) {
      ControlModule::set_rumble(boma, Hash40::new("rbkind_landl"), 0, false, 0x50000000 /* default value */);
      slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
  } 
}

#[acmd_script( agent = "donkey", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.93, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "donkey", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn dk_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		  WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "donkey", script = "game_itemheavythrowf" , category = ACMD_GAME , low_priority)]
unsafe fn heavy_item_throw_f(fighter: &mut L2CAgentBase) {
  let lua_state = fighter.lua_state_agent;
  let boma = sv_system::battle_object_module_accessor(lua_state);
  if is_excute(fighter) {
    FT_MOTION_RATE(fighter, 1.0);
  }
  frame(lua_state, 16.0);
  if is_excute(fighter) {
    ItemModule::throw_item(boma, 75.0, 4.0, 1.0, 0, true, 0.0);
  }
}

#[acmd_script( agent = "donkey", script = "game_itemheavythrowb" , category = ACMD_GAME , low_priority)]
unsafe fn heavy_item_throw_b(fighter: &mut L2CAgentBase) {
  let lua_state = fighter.lua_state_agent;
  let boma = sv_system::battle_object_module_accessor(lua_state);
  if is_excute(fighter) {
    FT_MOTION_RATE(fighter, 1.0);
  }
  frame(lua_state, 18.0);
  if is_excute(fighter) {
    ItemModule::throw_item(boma, 125.0, 4.0, 1.0, 0, true, 0.0);
  }
}

pub fn install() {
    install_acmd_scripts!(
    dash_effect,
		dk_turn_dash_game,
    heavy_item_throw_f,
    heavy_item_throw_b,
    expression_landingheavy,
	);
}

