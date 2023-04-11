
use super::*;


#[acmd_script( agent = "tantan", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn tantan_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 7.5, 70, 114, 0, 48, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 7.5, 70, 114, 0, 48, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 7.5, 70, 114, 0, 48, 4.0, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "tantan", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn tantan_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.571);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 13.0, 285, 90, 0, 30, 4.75, 5.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        /* Air-only */
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 13.0, 285, 53, 0, 30, 4.75, 5.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        
        ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 80, 79, 0, 70, 4.5, 0.0, 6.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 11.0, 80, 79, 0, 70, 4.75, 5.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::clear(boma, 1, false);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 80, 79, 0, 70, 4.5, 0.0, 6.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(fighter, 10.0/(41.0-18.0));
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 41.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}
#[acmd_script( agent = "tantan", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn tantan_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 14.5, -1.75, 78, 0, 0, 1.0, true, 0.8);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footl"), 1.0, 0, 0, 0, 0, 0, 1.0, true, 0.95);
    }
}

#[acmd_script( agent = "tantan", script = "sound_attackairlw", category = ACMD_SOUND, low_priority )]
unsafe fn tantan_attack_air_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_tantan_attackair_l01"));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_tantan_smash_h01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_tantan_rnd_attack03"));
    }
}

#[acmd_script( agent = "tantan", script = "game_landingairlw" , category = ACMD_GAME , low_priority)]
unsafe fn tantan_landing_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
}

#[acmd_script( agent = "tantan", script = "expression_landingairlw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn tantan_landing_air_lw_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 8, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}


#[acmd_script( agent = "tantan", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn tantan_attack_air_n_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 3.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.5, 361, 67, 0, 32, 3.7, 5.0, -0.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.5, 361, 67, 0, 32, 3.25, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 9.5, 361, 67, 0, 32, 3.25, -1.5, -1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 8.25, 361, 67, 0, 32, 3.4, 5.0, -0.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 8.25, 361, 67, 0, 32, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 8.25, 361, 67, 0, 37, 2.75, -1.5, -1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "tantan", script = "effect_attackairn", category = ACMD_EFFECT, low_priority)]
unsafe fn tantan_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8.5, -5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        //macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 5.7, -8, -10, 0, 0, 1.1, false);
        //macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        
        let lr = PostureModule::lr(fighter.module_accessor);
        if macros::is_excute(fighter) {
            if(lr>=0.0){
                macros::EFFECT_FLW_POS(fighter, Hash40::new("tantan_attack_dash"), Hash40::new("kneel"), 1, -1.75, 0, 13, 0, 0, 0.9, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 3, 1.2, 0.5);
            }
            else{
                macros::EFFECT_FLW_POS(fighter, Hash40::new("tantan_attack_dash"), Hash40::new("kneel"), -1, 1.75, 0, 13, 0, 0, 0.9, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 3, 1.2, 0.5);
            }
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 7.5, 8.0, 0, 0, 0, 0.9, true, 0.6);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
        //macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), true, true);
    }
}
#[acmd_script( agent = "tantan", script = "sound_attackairn", category = ACMD_SOUND, low_priority)]
unsafe fn tantan_attack_air_n_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_tantan_swing_l01"));
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //macros::PLAY_SE(fighter, Hash40::new("vc_tantan_attack02"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_tantan_rnd_attack03"));
    }
}
#[acmd_script( agent = "tantan", script = "expression_attackairn", category = ACMD_EXPRESSION, low_priority)]
unsafe fn tantan_attack_air_n_expression(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "tantan", script = "effect_landingairn", category = ACMD_EFFECT, low_priority)]
unsafe fn tantan_landing_air_n_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "tantan", script = "sound_landingairn", category = ACMD_SOUND, low_priority)]
unsafe fn tantan_landing_air_n_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_tantan_landing02"));
    }
}
#[acmd_script( agent = "tantan", script = "expression_landingairn", category = ACMD_EXPRESSION, low_priority)]
unsafe fn tantan_landing_air_n_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}
#[acmd_script( agent = "tantan", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn tantan_attack_air_f_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 10.0);
    FT_MOTION_RATE(fighter, 1.0/(12.0-10.0));
    frame(fighter.lua_state_agent, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("footl"), 13.75, 361, 100, 0, 25, 5.0, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 13.75, 361, 100, 0, 25, 4.375, 0.0, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        //Sweetspot ground spike//
        ATTACK(fighter, 0, 0, Hash40::new("footl"), 15.0, 274, 63, 0, 25, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        //Sweetspot semispike//
        ATTACK(fighter, 1, 0, Hash40::new("footl"), 15.0, 1, 63, 0, 25, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 13.75, 361, 100, 0, 25, 4.375, -1.0, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "tantan", script = "effect_attackairf", category = ACMD_EFFECT, low_priority)]
unsafe fn tantan_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10e97de698), Hash40::new("top"), 0, 10.5, 2.0, 180, -150 , 90, 0.83, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.3, true, 0.9);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
}
#[acmd_script( agent = "tantan", script = "sound_attackairf", category = ACMD_SOUND, low_priority)]
unsafe fn tantan_attack_air_f_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        //macros::PLAY_SE(fighter, Hash40::new("se_tantan_swing_s01"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_tantan_swing_l01"));
        if (app::sv_math::rand(hash40("fighter"), 3) == 0)
        {
            PLAY_SE(fighter, Hash40::new("vc_tantan_punch_03"));
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_tantan_swing_l01"));
    }
}
#[acmd_script( agent = "tantan", script = "expression_attackairf", category = ACMD_EXPRESSION, low_priority)]
unsafe fn tantan_attack_air_f_expression(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "tantan", script = "effect_landingairf", category = ACMD_EFFECT, low_priority)]
unsafe fn tantan_landing_air_f_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        //macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "tantan", script = "sound_landingairf", category = ACMD_SOUND, low_priority)]
unsafe fn tantan_landing_air_f_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_tantan_landing02"));
    }
}
#[acmd_script( agent = "tantan", script = "expression_landingairf", category = ACMD_EXPRESSION, low_priority)]
unsafe fn tantan_landing_air_f_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn tantan_attack_air_b_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.5, 42, 98, 0, 42, 3.75, -1.25, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 12.5, 42, 98, 0, 42, 4.0, 5.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "tantan", script = "effect_attackairb", category = ACMD_EFFECT, low_priority)]
unsafe fn tantan_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -1.5, 8.1, -6, -151, 47, 2.9, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        //EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 11.5, -14, 0, 0, 0, 0.9, false, 0.5);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
}
#[acmd_script( agent = "tantan", script = "sound_attackairb", category = ACMD_SOUND, low_priority)]
unsafe fn tantan_attack_air_b_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_tantan_attackair_h01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        if (app::sv_math::rand(hash40("fighter"), 3) == 0)
        {
            PLAY_SE(fighter, Hash40::new("vc_tantan_punch_04"));
        }
    }
}
#[acmd_script( agent = "tantan", script = "expression_attackairb", category = ACMD_EXPRESSION, low_priority)]
unsafe fn tantan_attack_air_b_expression(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "tantan", script = "effect_landingairb", category = ACMD_EFFECT, low_priority)]
unsafe fn tantan_landing_air_b_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "tantan", script = "sound_landingairb", category = ACMD_SOUND, low_priority)]
unsafe fn tantan_landing_air_b_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_tantan_landing02"));
    }
}
#[acmd_script( agent = "tantan", script = "expression_landingairb", category = ACMD_EXPRESSION, low_priority)]
unsafe fn tantan_landing_air_b_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        tantan_attack_air_hi_game,
        tantan_attack_air_lw_game,
        tantan_attack_air_lw_effect,
        tantan_attack_air_lw_sound,
        tantan_landing_air_lw_game,
        tantan_landing_air_lw_expression,

        tantan_attack_air_n_game,
        tantan_attack_air_n_effect,
        tantan_attack_air_n_sound,
        tantan_attack_air_n_expression,

        tantan_attack_air_n_effect,
        tantan_attack_air_n_sound,
        tantan_attack_air_n_expression,

        tantan_attack_air_f_game,
        tantan_attack_air_f_effect,
        tantan_attack_air_f_sound,
        tantan_attack_air_f_expression,
        
        tantan_landing_air_f_effect,
        tantan_landing_air_f_sound,
        tantan_landing_air_f_expression,
        
        tantan_attack_air_b_game,
        tantan_attack_air_b_effect,
        tantan_attack_air_b_sound,
        tantan_attack_air_b_expression,

        tantan_landing_air_b_effect,
        tantan_landing_air_b_sound,
        tantan_landing_air_b_expression,
    );
}

