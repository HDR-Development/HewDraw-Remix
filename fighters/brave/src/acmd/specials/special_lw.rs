use super::*;

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 20.0, 17.0);
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 1.0);
}

// Magic Burst
unsafe extern "C" fn game_speciallw8(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_CRASH, false, -1);
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_FULLBURST_INTERRUPT);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 8.0, 0.0, 16.0, 0.0, -3.0, 8.0, 0.0, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 116.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_FULLBURST_INTERRUPT);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 136.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_ENABLE_GRAVITY);
    }
}

// Kaclang
unsafe extern "C" fn game_specialairlw10(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 270, 30, 0, 80, 7.5, 0.0, 2.5, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 270, 40, 0, 38, 7.5, 0.0, 2.5, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 15.0);
}

// Snooze
unsafe extern "C" fn game_speciallw14(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 8.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_SLEEP, false, -1);
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_ENERGY);
    }
}

// Flame Slash
unsafe extern "C" fn game_speciallw17(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
        ATTACK(agent, 0, 0, Hash40::new("sword1"), 22.0, 46, 67, 0, 68, 4.5, 9.0, 0.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 22.0, 46, 67, 0, 68, 4.5, 4.0, 0.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 22.0, 46, 67, 0, 68, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 17.0, 50, 62, 0, 68, 7.5, 0.0, 15.0, 13.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 22.0, 46, 67, 0, 68, 8.5, 0.0, 7.5, 19.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 17.0, 50, 62, 0, 68, 7.5, 0.0, 6.0, 31.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 40.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallw17(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_LIGHTNING_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("brave_fire_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1.2, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_firesword1"), Hash40::new("tex_brave_sword2"), 7, Hash40::new("sword1"), 1.5, 0.0, 0.0, Hash40::new("sword1"), 14.4, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, -90.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("brave_fire_sword"), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_LIGHTNING_SWORD_FLARE, false, true);
    }
}

// Kacrackle Slash
unsafe extern "C" fn game_speciallw18(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
        ATTACK(agent, 0, 0, Hash40::new("sword1"), 17.0, 50, 7, 0, 70, 4.5, 9.0, 0.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 17.0, 50, 7, 0, 70, 4.5, 4.0, 0.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 17.0, 50, 7, 0, 70, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 13.0, 60, 7, 0, 70, 7.5, 0.0, 15.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_ice_frame_mul(boma, 0, 2.0, false);
        AttackModule::set_ice_frame_mul(boma, 1, 2.0, false);
        AttackModule::set_ice_frame_mul(boma, 2, 2.0, false);
        AttackModule::set_ice_frame_mul(boma, 4, 2.0, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 17.0, 50, 7, 0, 70, 8.5, 0.0, 7.5, 19.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 13.0, 60, 7, 0, 70, 7.5, 0.0, 6.0, 31.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_ice_frame_mul(boma, 3, 2.0, false);
        AttackModule::set_ice_frame_mul(boma, 4, 2.0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    FT_MOTION_RATE(agent, 0.7);
    frame(lua_state, 40.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallw18(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 8, Hash40::new("sword1"), 1.5, 0.0, 0.0, Hash40::new("sword1"), 14.4, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, -90.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("brave_ice_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1.3, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 6);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("brave_ice_sword"), false, true);
    }
}

// Metal Slash
unsafe extern "C" fn game_speciallw19(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 1.0, 361, 220, 10, 10, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 29, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup_metal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 1.0, 361, 220, 10, 10, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 29, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup_metal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 1.0, 361, 220, 10, 10, 4.0, 3.0, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 29, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup_metal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 1.0, 361, 220, 10, 10, 4.0, 9.0, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 29, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup_metal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 6.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 6.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 6.0, false);
        AttackModule::set_add_reaction_frame(boma, 3, 6.0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn expression_speciallw19(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

// Hatchet Man
unsafe extern "C" fn game_speciallw20(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 35.0, 29.0);
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 35.0, 55, 89, 0, 36, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 53, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 35.0, 55, 89, 0, 36, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 53, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 35.0, 55, 89, 0, 36, 3.5, 4.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 53, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 35.0, 55, 89, 0, 36, 3.5, 9.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 53, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallwstart", game_speciallwstart, Priority::Low);
    agent.acmd("game_specialairlwstart", game_speciallwstart, Priority::Low);

    agent.acmd("game_speciallw8", game_speciallw8, Priority::Low);
    agent.acmd("game_specialairlw8", game_speciallw8, Priority::Low);

    agent.acmd("game_specialairlw10", game_specialairlw10, Priority::Low);

    agent.acmd("game_speciallw14", game_speciallw14, Priority::Low);
    agent.acmd("game_specialairlw14", game_speciallw14, Priority::Low);

    agent.acmd("game_speciallw17", game_speciallw17, Priority::Low);
    agent.acmd("game_specialairlw17", game_speciallw17, Priority::Low);
    agent.acmd("effect_speciallw17", effect_speciallw17, Priority::Low);
    agent.acmd("game_specialairlw17", effect_speciallw17, Priority::Low);

    agent.acmd("game_speciallw18", game_speciallw18, Priority::Low);
    agent.acmd("game_specialairlw18", game_speciallw18, Priority::Low);
    agent.acmd("effect_speciallw18", effect_speciallw18, Priority::Low);
    agent.acmd("effect_specialairlw18", effect_speciallw18, Priority::Low);

    agent.acmd("game_speciallw19", game_speciallw19, Priority::Low);
    agent.acmd("game_specialairlw19", game_speciallw19, Priority::Low);
    agent.acmd("expression_speciallw19", expression_speciallw19, Priority::Low);
    agent.acmd("expression_specialairlw19", expression_speciallw19, Priority::Low);

    agent.acmd("game_speciallw20", game_speciallw20, Priority::Low);
    agent.acmd("game_specialairlw20", game_speciallw20, Priority::Low);
}