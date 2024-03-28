use super::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
        if VarModule::is_flag(boma.object(), vars::demon::instance::DEVASTATOR){
            MotionModule::change_motion(boma, Hash40::new("flash_punch"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FT_DESIRED_RATE(agent, 6.0-2.0, 2.0);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 3.0, 80, 100, 25, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 3.0, 80, 100, 25, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 3.0, 80, 100, 25, 0, 3.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 80, 100, 25, 0, 4.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        // Jab lock hitbox
        ATTACK(agent, 4, 0, Hash40::new("top"), 3.0, 361, 10, 0, 30, 3.0, 0.0, 4.0, 3.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(boma, 4, false);

        AttackModule::set_add_reaction_frame_revised(boma, 0, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 2.0, false);

        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.3);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.3);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.3);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 1.3);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
    
}

unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
        JostleModule::set_push_speed_x_overlap_rate(boma, 20.0);
        FT_DESIRED_RATE(agent, 7.0-1.0, 4.0);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_ENABLE_COMBO);
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 3.0, 75, 100, 25, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 3.0, 75, 100, 25, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 3.0, 75, 100, 25, 0, 3.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 75, 100, 25, 0, 4.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        // Jab lock hitbox
        ATTACK(agent, 4, 0, Hash40::new("top"), 3.0, 361, 10, 0, 30, 3.0, 0.0, 4.0, 3.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(boma, 4, false);

        AttackModule::set_add_reaction_frame_revised(boma, 0, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 2.0, false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        JostleModule::set_push_speed_x_overlap_rate(boma, 0.0);
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
    }
}

unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 2.0, 361, 10, 0, 35, 2.0, 0.0, 14.5, 10.0, None, None, None, 0.5, 5.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 1, Hash40::new("top"), 2.0, 361, 10, 0, 35, 3.5, 0.0, 13.5, 9.0, None, None, None, 0.5, 5.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 1, Hash40::new("top"), 2.0, 361, 10, 0, 35, 3.5, 0.0, 13.5, 3.0, None, None, None, 0.5, 5.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 1, Hash40::new("top"), 2.0, 361, 10, 0, 35, 3.5, 0.0, 8.75, 9.0, Some(0.0), Some(13.5), Some(9.0), 0.5, 5.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 4, 1, Hash40::new("top"), 2.0, 361, 10, 0, 35, 3.5, 0.0, 9.0, 3.0, Some(0.0), Some(13.5), Some(3.0), 0.5, 5.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 5, 1, Hash40::new("top"), 2.0, 33, 15, 0, 35, 3.6, 0.0, 15.0, 3.0, Some(0.0), Some(15.0), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

unsafe extern "C" fn game_attack14(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 10, 0, 35, 2.0, 0.0, 14.0, 10.0, None, None, None, 0.5, 4.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 10, 0, 35, 3.6, 0.0, 13.0, 9.0, None, None, None, 0.5, 4.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 361, 10, 0, 35, 3.6, 0.0, 13.0, 4.0, None, None, None, 0.5, 4.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 361, 10, 0, 35, 3.6, 0.0, 9.5, 9.0, Some(0.0), Some(13.0), Some(9.0), 0.5, 4.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 4, 0, Hash40::new("top"), 3.0, 361, 10, 0, 35, 3.6, 0.0, 9.5, 4.0, Some(0.0), Some(13.0), Some(4.0), 0.5, 4.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 5, 0, Hash40::new("top"), 3.0, 36, 15, 0, 30, 3.6, 0.0, 14.0, 4.0, Some(0.0), Some(14.0), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

unsafe extern "C" fn game_attack15(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 42, 20, 0, 40, 4.0, 0.0, 13.0, 4.0, Some(0.0), Some(15.0), Some(11.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 0, 5, 0, 40, 3.5, 0.0, 10.0, 5.0, None, None, None, 0.4, 4.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 0, 5, 0, 40, 3.5, 0.0, 6.0, 5.0, None, None, None, 0.4, 4.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 0, 5, 0, 40, 3.6, 0.0, 9.75, 7.5, None, None, None, 0.4, 4.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 4, 0, Hash40::new("top"), 3.0, 0, 5, 0, 40, 3.6, 0.0, 13.5, 10.75, None, None, None, 0.4, 4.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 5, 0, Hash40::new("top"), 3.0, 0, 5, 0, 40, 2.5, 0.0, 14.5, 10.5, None, None, None, 0.4, 4.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 9.0, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

unsafe extern "C" fn game_attack16(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 0, 5, 0, 30, 3.9, 0.0, 12.0, 6.0, None, None, None, 0.4, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 0, 5, 0, 30, 3.9, 0.0, 10.0, 7.25, None, None, None, 0.4, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 0, 5, 0, 30, 3.9, 0.0, 8.0, 4.0, None, None, None, 0.4, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 0, 5, 0, 30, 2.5, 0.0, 15.0, 11.5, None, None, None, 0.4, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 4, 0, Hash40::new("top"), 4.0, 0, 5, 0, 30, 4.0, 0.0, 12.0, 10.5, None, None, None, 0.4, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 4.0, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 0, 5, 0, 30, 3.9, 0.0, 10.0, 6.0, None, None, None, 0.4, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 0, 5, 0, 30, 3.9, 0.0, 8.0, 7.25, None, None, None, 0.4, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 0, 5, 0, 30, 3.9, 0.0, 8.0, 4.0, None, None, None, 0.4, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 0, 5, 0, 30, 2.5, 0.0, 10.0, 10.5, Some(0.0), Some(10.1), Some(10.5), 0.4, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 4, 0, Hash40::new("top"), 4.0, 0, 5, 0, 30, 4.0, 0.0, 8.0, 10.5, None, None, None, 0.4, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 4.0, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

unsafe extern "C" fn game_attack17(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 10, 0, 30, 3.3, 0.0, 6.25, 6.5, None, None, None, 0.5, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 0, 10, 0, 30, 3.3, 0.0, 7.5, 2.0, None, None, None, 0.5, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.5, 0, 10, 0, 30, 2.0, 0.0, 6.0, 11.0, None, None, None, 0.5, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.5, 0, 10, 0, 30, 3.5, 0.0, 5.0, 11.0, None, None, None, 0.5, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 9.0, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

unsafe extern "C" fn game_attack18(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 10, 0, 30, 3.3, 0.0, 6.25, 8.0, None, None, None, 0.5, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 0, 10, 0, 30, 3.3, 0.0, 6.75, 5.0, None, None, None, 0.5, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.5, 0, 10, 0, 30, 3.3, 0.0, 7.25, 2.0, None, None, None, 0.5, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.5, 0, 10, 0, 30, 2.0, 0.0, 6.25, 10.0, None, None, None, 0.5, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 4, 0, Hash40::new("top"), 1.5, 0, 10, 0, 30, 3.5, 0.0, 5.75, 11.0, None, None, None, 0.5, 3.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 6.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 6.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 6.0, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

unsafe extern "C" fn game_attack19(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 2.0, 0.0, 11.5, 7.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 4.0, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(7.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_level(boma, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

unsafe extern "C" fn game_attack110(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 53, 65, 0, 60, 2.0, 0.0, 16.0, 7.0, Some(0.0), Some(14.0), Some(6.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 53, 65, 0, 60, 4.0, 0.0, 10.0, 5.5, Some(0.0), Some(16.0), Some(7.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 53, 65, 0, 60, 4.0, 0.0, 4.0, 4.0, Some(0.0), Some(10.0), Some(5.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_flashpunch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::demon::instance::DEVASTATOR){
            ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 368, 100, 40, 0, 7.0, 0.0, 15.0, 5.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 1, Hash40::new("top"), 0.0, 368, 100, 40, 0, 7.0, 0.0, 15.0, 0.0, Some(0.0), Some(7.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            let x_pos = 13.0;
            let y_pos = 6.0;
            AttackModule::set_vec_target_pos(boma, 0, smash::phx::Hash40::new("top"), &Vector2f::new(x_pos, y_pos), 4, false);
            AttackModule::set_vec_target_pos(boma, 1, smash::phx::Hash40::new("top"), &Vector2f::new(x_pos, y_pos), 4, false);
        }
        else{
            ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 35, 0, 4.0, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::demon::instance::DEVASTATOR){
            VarModule::off_flag(boma.object(), vars::demon::instance::DEVASTATOR);
            ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 40, 40, 0, 76, 2.0, 0.0, 9.5, 9.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 40, 40, 0, 76, 4.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(8.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 40, 40, 0, 76, 4.0, 0.0, 10.5, 4.0, Some(0.0), Some(10.5), Some(8.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 43, 70, 0, 76, 2.0, 0.0, 9.5, 9.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 43, 70, 0, 76, 4.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(8.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 43, 70, 0, 76, 4.0, 0.0, 10.5, 4.0, Some(0.0), Some(10.5), Some(8.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        }
        
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack11", game_attack11);
    agent.acmd("game_attack12", game_attack12);
    agent.acmd("game_attack13", game_attack13);
    agent.acmd("game_attack14", game_attack14);
    agent.acmd("game_attack15", game_attack15);
    agent.acmd("game_attack16", game_attack16);
    agent.acmd("game_attack17", game_attack17);
    agent.acmd("game_attack18", game_attack18);
    agent.acmd("game_attack19", game_attack19);
    agent.acmd("game_attack110", game_attack110);
    agent.acmd("game_flashpunch", game_flashpunch);
}
