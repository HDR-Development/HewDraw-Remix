
use super::*;

#[acmd_script( agent = "gekkouga", script = "effect_speciallwhit" , category = ACMD_EFFECT , low_priority)]
unsafe fn gekkouga_special_lw_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("gekkouga_migawari_smoke"), Hash40::new("top"), 0, -2.0, 15.0, 0, 0, 0, 1.35, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.05);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("gekkouga_migawari_smoke"), Hash40::new("top"), 0, -2.0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
    }
}

#[acmd_script( agent = "gekkouga", script = "game_specialsattackf" , category = ACMD_GAME , low_priority)]
unsafe fn gekkouga_special_s_attack_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 1.1);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 0.9);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 30, 55, 0, 65, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "gekkouga", script = "game_specialairsattackf" , category = ACMD_GAME , low_priority)]
unsafe fn gekkouga_special_air_s_attack_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        SET_SPEED_EX(fighter, 0, 2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(fighter, 1.1);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 0.9);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 30, 55, 0, 65, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "gekkouga", script = "game_specialsattackb" , category = ACMD_GAME , low_priority)]
unsafe fn gekkouga_special_s_attack_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 1.1);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 0.9);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 30, 55, 0, 65, 8.0, 0.0, 8.0, -15.0, Some(0.0), Some(8.5), Some(-12.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "gekkouga", script = "game_specialairsattackb" , category = ACMD_GAME , low_priority)]
unsafe fn gekkouga_special_air_s_attack_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
    SET_SPEED_EX(fighter, 0, 2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(fighter, 1.1);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 0.9);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 30, 55, 0, 65, 8.0, 0.0, 8.0, -15.0, Some(0.0), Some(8.5), Some(-12.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "gekkouga", scripts = [ "effect_specialsattackf", "effect_specialairsattackf" ] , category = ACMD_EFFECT , low_priority)]
unsafe fn gekkouga_special_s_attack_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("gekkouga_kageuchi_warp_end"), Hash40::new("top"), 0, 7, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gekkouga_kageuchi_arc"), Hash40::new("gekkouga_kageuchi_arc"), Hash40::new("top"), 0, 14, 4, 0, -70, 120, 1, true, *EF_FLIP_YZ);
    }
}

#[acmd_script( agent = "gekkouga", scripts = [ "effect_specialsattackb", "effect_specialairsattackb" ] , category = ACMD_EFFECT , low_priority)]
unsafe fn gekkouga_special_s_attack_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("gekkouga_kageuchi_warp_end"), Hash40::new("top"), 0, 7, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gekkouga_kageuchi_line"), Hash40::new("gekkouga_kageuchi_line"), Hash40::new("top"), 2, 13, 8, 0, -170, 70, 1.2, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), -20, 7.5, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, false, *EF_FLIP_NONE, 0.8);
    }
}

pub fn install() {
    install_acmd_scripts!(
    gekkouga_special_lw_hit_effect,

    gekkouga_special_s_attack_f_game,
    gekkouga_special_air_s_attack_f_game,
    gekkouga_special_s_attack_b_game,
    gekkouga_special_air_s_attack_b_game,
    gekkouga_special_s_attack_f_effect,
    gekkouga_special_s_attack_b_effect,
);
}

