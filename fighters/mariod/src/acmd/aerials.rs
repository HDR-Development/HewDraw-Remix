use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 10.0, 361, 100, 0, 20, 3.7, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 20, 3.7, 0.9, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 14.0, 361, 100, 0, 25, 3.0, 0.8, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 14.0, 361, 100, 0, 25, 3.0, 0.9, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("legl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_impact"), Hash40::new("legl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_impact"), true, true);
    }
}

unsafe extern "C" fn sound_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_s"));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 16.0, 50, 100, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 17.0, 50, 100, 0, 35, 4.0, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 16.0, 50, 100, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 17.0, 50, 100, 0, 35, 4.6, 3.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 16.0, 50, 100, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 17.0, 50, 100, 0, 35, 4.6, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 1, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("handl"), -1.5, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("handl"), 1.0, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 7, -1, -3, -20, -113, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(lua_state, 20.0);
    if is_excute(agent){
        LAST_EFFECT_SET_RATE(agent, 1.0);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_impact"), true, true);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 8.0, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(agent, 0, 0, Hash40::new("legr"), 10.0, 35, 70, 0, 40, 4.7, 1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(agent, 1, 0, Hash40::new("kneer"), 13.0, 38, 112, 0, 25, 3.7, 5.5, 0.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(agent, 8.0, 13.0, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 9.0, 361, 100, 0, 20, 4.5, 1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(agent, 1, 0, Hash40::new("kneer"), 9.0, 361, 100, 0, 20, 3.5, 3.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }   
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 3, 5.4, 5, 180, 0, 0, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(agent, 1.65, 1.65, 1.65);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -10.0, 5.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_impact"), Hash40::new("hip"), -7.0, 0.0, 0.0, 0, 0, 0, 0.75, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_impact"), true, true);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.5, 3.0);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 4.5);
    FT_MOTION_RATE_RANGE(agent, 4.5, 10.0, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 10.0, 54, 85, 0, 25, 3.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 54, 85, 0, 25, 4.7, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 9.0, 361, 90, 0, 20, 3.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 9.0, 361, 90, 0, 20, 4.7, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }    
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 10, 0, -4, 80, 96, 0.95, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 16.0, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        /* Ground-only */
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 15.0, 270, 82, 0, 14, 5.0, 3.25, 0.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        /* Air-only */
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 15.0, 270, 58, 0, 12, 5.0, 3.25, 0.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("legr"), 15.0, 361, 82, 0, 14, 4.0, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -6, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("hip"), -6.0, 0.0, 0.0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1.8, 11, 0, 90, 0, 0, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(agent, 1.65, 1.65, 1.65);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, -5, 0, 0, 0, 0, 1.35, false);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_impact"), Hash40::new("hip"), -6.0, 0.0, 0.0, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_impact"), true, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn);
    agent.acmd("effect_attackairn", effect_attackairn);
    agent.acmd("sound_attackairn", sound_attackairn);

    agent.acmd("game_attackairf", game_attackairf);
    agent.acmd("effect_attackairf", effect_attackairf);

    agent.acmd("game_attackairb", game_attackairb);
    agent.acmd("effect_attackairb", effect_attackairb);

    agent.acmd("game_attackairhi", game_attackairhi);
    agent.acmd("effect_attackairhi", effect_attackairhi);
    
    agent.acmd("game_attackairlw", game_attackairlw);
    agent.acmd("effect_attackairlw", effect_attackairlw);
}
