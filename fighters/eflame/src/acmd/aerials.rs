use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 4.0);
    frame(lua_state, 2.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 10.0, 58, 106, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 58, 106, 0, 20, 5.0, -4.0, 3.0, -1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 73, 97, 0, 50, 5.0, -4.0, 12.5, -1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 10.0, 73, 103, 0, 48, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 73, 103, 0, 48, 5.0, -4.0, 3.0, -1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 10.0, 73, 103, 0, 48, 5.0, -4.0, 12.5, -1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m2"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword3"), 4, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 17, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 6.0);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 3, 0, 0, 0, 0, 0, 0.35, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.6, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_airn_sword_blur"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 2, 0, 2, 0, 10, 0, 0.7, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 3, 0, 1, 0, 15, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_airn_sword_blur"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail_end"), false, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_m2"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 11.0, 50, 99, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 11.0, 50, 99, 0, 30, 4.0, 0.0, 3.0, -0.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 14.0, 43, 99, 0, 30, 3.6, 0.0, 8.0, -0.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 14.0, 43, 99, 0, 30, 3.2, 0.0, 13.5, -1.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma); 
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_core_start"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 5, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 17, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m2"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_airf_slash"), Hash40::new("top"), 0, 0.5, -2, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire3"), Hash40::new("top"), 0, 0.5, -2, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 1.5, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 3, 0, 0, 0, 0, 0, 0.45, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("eflame_airf_slash"), -1);
        AFTER_IMAGE_OFF(agent, 2);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), false, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_m2"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_core_start"), false, true);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 13.0, 55, 93, 0, 41, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 13.0, 55, 93, 0, 41, 4.0, 0.0, 3.0, -0.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 16.0, 60, 93, 0, 41, 3.6, 0.0, 8.0, -0.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 16.0, 60, 93, 0, 41, 3.2, 0.0, 13.5, -1.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_core_start"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 4, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 17, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m2"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire3"), Hash40::new("top"), 5.5, -1, 2, 0, 180, 30, 1, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, -15, 0, 0, 0.7, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 2.5, 0, 0, -15, 0, 0, 0.3, true);
        LAST_EFFECT_SET_RATE(agent, 4);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_m2"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_core_start"), false, true);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 12.0, 76, 84, 0, 52, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 76, 84, 0, 52, 4.0, 0.0, 3.0, -0.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 14.0, 81, 84, 0, 52, 3.6, 0.0, 9.0, -0.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 14.0, 81, 84, 0, 52, 3.2, 0.0, 15.0, -1.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        FT_MOTION_RATE(agent, 0.7);
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_core_start"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 5, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 18.5, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 2);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 1, 0, 0, 2, -10, 0, 0.3, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 4, 0, 0, 2, -10, 0, 0.5, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0.5, 0, 0, 2, 0, 0, 0.8, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire"), false, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(agent, 3.5);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_m"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_core_start"), false, true);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 13.0, 57, 90, 0, 35, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 13.0, 57, 90, 0, 35, 4.0, 3.0, 0.0, -0.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword2"), 13.0, 57, 90, 0, 35, 3.6, 8.0, 0.0, -0.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword2"), 16.0, 270, 48, 0, 31, 3.2, 13.5, 0.0, -1.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);   
        }
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 33.0);
    FT_MOTION_RATE(agent, 0.7);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_core_start"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 7, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 5, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 18.5, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), -0.5, 0, 0, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -3);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 1.5, 0, 0, -5, 0, 0, 0.5, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 1, 0, 0, -10, 0, 0, 0.85, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 0.4, true);
        LAST_EFFECT_SET_RATE(agent, 4);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_m"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_core_start"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn, Priority::Low);
    agent.acmd("effect_attackairn", effect_attackairn, Priority::Low);

    agent.acmd("game_attackairf", game_attackairf, Priority::Low);
    agent.acmd("effect_attackairf", effect_attackairf, Priority::Low);

    agent.acmd("game_attackairb", game_attackairb, Priority::Low);
    agent.acmd("effect_attackairb", effect_attackairb, Priority::Low);

    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi", effect_attackairhi, Priority::Low);

    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw", effect_attackairlw, Priority::Low);
}