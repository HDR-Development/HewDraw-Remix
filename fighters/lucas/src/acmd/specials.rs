
use super::*;


#[acmd_script( agent = "lucas", script = "game_speciallwstart" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
}

#[acmd_script( agent = "lucas", script = "effect_speciallwstart" , category = ACMD_EFFECT , low_priority)]
unsafe fn lucas_special_lw_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -1.5, 0, -1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("havel"), -0.3, -2.0, 0.1, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psimagnet_start"), Hash40::new("trans"), 0, 6.5, 10, 0, 0, 0, 0.65, false);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_specialairlwstart" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_air_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
}

#[acmd_script( agent = "lucas", script = "effect_specialairlwstart" , category = ACMD_EFFECT , low_priority)]
unsafe fn lucas_special_air_lw_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("havel"), -0.3, -2.0, 0.1, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psimagnet_start"), Hash40::new("trans"), 0, 6.5, 10, 0, 0, 0, 0.65, false);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_speciallwhold" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 80, 80, 30, 0, 4.5, 0.0, 6.3, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 80, 80, 30, 0, 4.5, 0.0, 6.3, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 9.0);
    for _ in 0..999 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 80, 80, 10, 0, 4.5, 0.0, 6.3, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 80, 80, 10, 0, 4.5, 0.0, 6.3, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 9.0);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_specialairlwhold" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_air_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 80, 80, 30, 0, 4.25, 0.0, 6.3, 9.25, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 80, 80, 30, 0, 4.25, 0.0, 6.3, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 9.0);
    for _ in 0..999 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 80, 80, 10, 0, 4.25, 0.0, 6.3, 9.25, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 80, 80, 10, 0, 4.25, 0.0, 6.3, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 9.0);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_speciallwend" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_lw_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 127, 80, 0, 77, 4.5, 0.0, 6.3, 9.25, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 127, 80, 0, 77, 4.5, 0.0, 6.3, 2.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.800);
    }
    
}

#[acmd_script( agent = "lucas", script = "effect_speciallwend" , category = ACMD_EFFECT , low_priority)]
unsafe fn lucas_special_lw_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psimagnet_end"), Hash40::new("trans"), 0, 6.5, 12, 0, 0, 0, 0.4, true);
        FLASH(fighter, 0.5, 1, 1, 0.4);
        EFFECT_DETACH_KIND(fighter, Hash40::new("lucas_psimagnet_end"), -1);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 10, 0, 1, 1, 0.1);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_specialairlwend" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_air_lw_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 127, 80, 0, 77, 4.5, 0.0, 6.3, 9.25, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 127, 80, 0, 77, 4.5, 0.0, 6.3, 2.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.800);
    }
    
}

#[acmd_script( agent = "lucas", script = "effect_specialairlwend" , category = ACMD_EFFECT , low_priority)]
unsafe fn lucas_special_air_lw_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psimagnet_end"), Hash40::new("trans"), 0, 6.5, 12, 0, 0, 0, 0.4, true);
        FLASH(fighter, 0.5, 1, 1, 0.4);
        EFFECT_DETACH_KIND(fighter, Hash40::new("lucas_psimagnet_end"), -1);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 10, 0, 1, 1, 0.1);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        lucas_special_lw_start_game,
        lucas_special_lw_start_effect,
        lucas_special_air_lw_start_game,
        lucas_special_air_lw_start_effect,
        lucas_special_lw_hold_game,
        lucas_special_air_lw_hold_game,
        lucas_special_lw_end_game,
        lucas_special_lw_end_effect,
        lucas_special_air_lw_end_game,
        lucas_special_air_lw_end_effect,
    );
}

