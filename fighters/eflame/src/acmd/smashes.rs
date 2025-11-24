use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
        WorkModule::set_int64(boma, hash40("attack_s4_hold") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 17.0, 361, 76, 0, 70, 4.5, 0.0, 3.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 17.0, 361, 76, 0, 70, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 20.0, 361, 76, 0, 70, 4.1, 0.0, 9.0, -1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 20.0, 361, 76, 0, 70, 3.7, 0.0, 11.0, -1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("haver"), 20.0, 361, 76, 0, 70, 3.3, 0.0, 12.0, -1.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("eflame_attack_s4s_explosion"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("eflame_attack_s4s_explosion"));
        AttackModule::set_optional_hit_effect(boma, 2, Hash40::new("eflame_attack_s4s_explosion"));
        AttackModule::set_optional_hit_effect(boma, 3, Hash40::new("eflame_attack_s4s_explosion"));
        AttackModule::set_optional_hit_effect(boma, 4, Hash40::new("eflame_attack_s4s_explosion"));
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 17.0, 361, 76, 0, 70, 4.5, 0.0, 5.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 17.0, 361, 76, 0, 70, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 20.0, 361, 76, 0, 70, 4.1, 0.0, 10.0, -1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 20.0, 361, 76, 0, 70, 3.7, 0.0, 14.0, -1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("haver"), 20.0, 361, 76, 0, 70, 3.3, 0.0, 18.5, -1.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("eflame_attack_s4s_explosion"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("eflame_attack_s4s_explosion"));
        AttackModule::set_optional_hit_effect(boma, 2, Hash40::new("eflame_attack_s4s_explosion"));
        AttackModule::set_optional_hit_effect(boma, 3, Hash40::new("eflame_attack_s4s_explosion"));
        AttackModule::set_optional_hit_effect(boma, 4, Hash40::new("eflame_attack_s4s_explosion"));
        WorkModule::on_flag(boma, *FIGHTER_EFLAME_STATUS_ATTACK_FLAG_S4_GROUND_CHECK);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_core_start"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1.14, false);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.7, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_attack_s4s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_s"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_l"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 0.97, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_attack_s4s_ground"), Hash40::new("top"), 0, 0, 20, 0, 90, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), false, true);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire"), false, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_l"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_core_start"), false, true);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if IS_EXIST_ARTICLE(agent, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if is_excute(agent) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(boma) {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    if is_excute(agent) {
        WorkModule::set_int64(boma, hash40("attack_hi4_hold") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 115, 100, 85, 0, 8.0, 0.0, 8.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 125, 100, 85, 0, 8.0, 0.0, 12.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.5, 83, 80, 0, 85, 5.0, 0.0, 22.0, 3.0, Some(0.0), Some(22.0), Some(11.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.5, 83, 80, 0, 85, 5.0, 0.0, 24.0, 0.0, Some(0.0), Some(24.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.5, 83, 80, 0, 85, 5.0, 0.0, 24.0, -3.0, Some(0.0), Some(24.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.5, 83, 80, 0, 85, 5.0, 0.0, 24.0, -8.0, Some(0.0), Some(24.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.5, 83, 80, 0, 85, 5.0, 0.0, 24.0, -11.0, Some(0.0), Some(24.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 83, 75, 0, 80, 5.0, 0.0, 24.0, -11.0, Some(0.0), Some(24.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0);
    if IS_EXIST_ARTICLE(agent, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if is_excute(agent) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(boma) {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 50.0);
    FT_MOTION_RATE(agent, 1.1);
    frame(lua_state, 60.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_core_start"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 11, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 18.5, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 3, 0, 1.4, 2, 10, 0, 0.4, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 6, 0, 1.4, 2, 10, 0, 0.6, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_RATE(agent, 0.3);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 3, 0, 1.4, 0, 10, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(agent, 3);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_m"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if IS_EXIST_ARTICLE(agent, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if is_excute(agent) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(boma) {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    if is_excute(agent) {
        WorkModule::set_int64(boma, hash40("attack_lw4_hold") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
    FT_MOTION_RATE(agent, 3.0);
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 11.0, 35, 74, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 13.5, 35, 71, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword2"), 13.5, 35, 71, 0, 75, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword2"), 13.5, 35, 71, 0, 75, 3.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("legl"), 8.0, 35, 66, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 5, 0, Hash40::new("kneel"), 10.0, 35, 66, 0, 75, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 6, 0, Hash40::new("kneel"), 10.0, 35, 66, 0, 75, 3.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 11.0, 35, 74, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 13.5, 35, 71, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword2"), 13.5, 35, 71, 0, 75, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword2"), 13.5, 35, 71, 0, 75, 3.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("legl"), 8.0, 35, 66, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 5, 0, Hash40::new("kneel"), 10.0, 35, 66, 0, 75, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 6, 0, Hash40::new("kneel"), 10.0, 35, 66, 0, 75, 3.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 45.0);
    if IS_EXIST_ARTICLE(agent, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if is_excute(agent) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(boma) {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_core_start"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m2"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, false);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 7, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 17, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.75);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.55, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, 18, 0, 0, 0.8, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_smash_wind"), Hash40::new("top"), 0, 7.5, -1, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_m2"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_core_start"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);
    agent.acmd("effect_attacks4", effect_attacks4, Priority::Low);

    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    agent.acmd("effect_attackhi4", effect_attackhi4, Priority::Low);
    
    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
    agent.acmd("effect_attacklw4", effect_attacklw4, Priority::Low);
}