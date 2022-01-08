
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

#[acmd_script( agent = "shizue", script = "game_specialnfailure" , category = ACMD_GAME , low_priority)]
unsafe fn shizue_special_n_failure_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 69, 90, 0, 70, 5.0, 0.0, 6.0, 12.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 69, 90, 0, 70, 3.0, 0.0, 6.0, 6.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            FT_MOTION_RATE(fighter, 0.8);
        }
    }
    
}

#[acmd_script( agent = "shizue", script = "effect_specialnfailure" , category = ACMD_EFFECT , low_priority)]
unsafe fn shizue_special_n_failure_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("shizue_cracker"), Hash40::new("top"), 0.0, 6.0, 8.0, 50, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "shizue", script = "game_specialairnfailure" , category = ACMD_GAME , low_priority)]
unsafe fn shizue_special_air_n_failure_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 69, 90, 0, 70, 5.0, 0.0, 6.0, 12.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 69, 90, 0, 70, 3.0, 0.0, 6.0, 6.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            FT_MOTION_RATE(fighter, 0.8);
        }
    }
    
}

#[acmd_script( agent = "shizue", script = "effect_specialairnfailure" , category = ACMD_EFFECT , low_priority)]
unsafe fn shizue_special_air_n_failure_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("shizue_cracker"), Hash40::new("top"), 0.0, 6.0, 8.0, 50, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    
}


pub fn install() {
    install_acmd_scripts!(
        shizue_special_n_failure_game,
        shizue_special_n_failure_effect,
        shizue_special_air_n_failure_game,
        shizue_special_air_n_failure_effect,
    );
}

