
use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::utility::*;
use smash_script::*;
use smashline::*;
use smashline::*;
use smash_script::macros::*;
use crate::utils::hdr;
use crate::vars::*;


#[acmd_script( agent = "richter", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 50, 0, 70, 4.0, 0.0, 8.5, -7.0, Some(0.0), Some(8.5), Some(10.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 50, 0, 70, 4.0, 0.0, 13.0, 6.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 361, 50, 0, 70, 4.0, 0.0, 4.0, 6.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 75, 40, 0, 90, 6.0, 0.0, 8.5, -10.0, Some(0.0), Some(8.5), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 75, 40, 0, 90, 4.0, 0.0, 13.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 75, 40, 0, 90, 4.0, 0.0, 4.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_dash_game,
    );
}

