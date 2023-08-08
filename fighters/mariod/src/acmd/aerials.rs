use super::*;

#[acmd_script( agent = "mariod", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.0, 50, 95, 0, 25, 3.7, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 10.0, 50, 95, 0, 25, 3.7, 0.9, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 14.0, 361, 100, 0, 20, 3.0, 0.8, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 14.0, 361, 100, 0, 20, 3.0, 0.9, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "mariod", script = "effect_attackairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("legl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("legl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

#[acmd_script( agent = "mariod", script = "sound_attackairn" , category = ACMD_SOUND , low_priority)]
unsafe fn mariod_attack_air_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_s"));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
    
}

#[acmd_script( agent = "mariod", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 15.0, 50, 95, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 16.0, 50, 95, 0, 30, 3.0, 3.2, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 16.0, 50, 95, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 17.0, 50, 95, 0, 30, 4.6, 3.2, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 14.0, 50, 95, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 15.0, 50, 95, 0, 30, 4.6, 3.2, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "mariod", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("havel"), -1.5, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("havel"), 1.0, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 7, 0, -3, -20, -113, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 0.25, 0.5, 1.0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), false, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

#[acmd_script( agent = "mariod", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(fighter, 6.0, 8.0, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 8.0, 361, 70, 0, 43, 4.7, 1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 13.0, 361, 105, 0, 20, 3.7, 5.5, 0.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(fighter, 8.0, 13.0, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 7.0/(13.0-8.0));
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 7.0, 361, 100, 0, 20, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 361, 100, 0, 20, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }   
}

#[acmd_script( agent = "mariod", script = "effect_attackairb" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -11.0, 5.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("hip"), -7.0, 0.0, 0.0, 0, 0, 0, 0.75, true);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

#[acmd_script( agent = "mariod", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 4.5, 3.0);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 4.5);
    FT_MOTION_RATE_RANGE(fighter, 4.5, 10.0, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 10.0, 54, 85, 0, 25, 3.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 54, 85, 0, 25, 4.7, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 9.0, 361, 90, 0, 20, 3.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 9.0, 361, 90, 0, 20, 4.7, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }    
}

#[acmd_script( agent = "mariod", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 15.0, 270, 82, 0, 14, 5.0, 3.25, 0.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        /* Air-only */
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 15.0, 270, 58, 0, 12, 5.0, 3.25, 0.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("legr"), 15.0, 361, 82, 0, 14, 4.0, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "mariod", script = "effect_attackairlw" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -6, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("hip"), -6.0, 0.0, 0.0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1.8, 11, 0, 90, 0, 0, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 1.65, 1.65, 1.65);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, -5, 0, 0, 0, 0, 1.35, false);
        EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("hip"), -6.0, 0.0, 0.0, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        mariod_attack_air_n_game,
        mariod_attack_air_n_effect,
        mariod_attack_air_n_sound,
        mariod_attack_air_f_game,
        mariod_attack_air_f_effect,
        mariod_attack_air_b_game,
        mariod_attack_air_b_effect,
        mariod_attack_air_hi_game,
        mariod_attack_air_lw_game,
        mariod_attack_air_lw_effect,
    );
}