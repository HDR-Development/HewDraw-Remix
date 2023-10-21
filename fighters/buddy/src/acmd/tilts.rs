use super::*;

#[acmd_script( agent = "buddy", script = "game_attacks3hi" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER,  Hash40::new("attack_s3_hi"), false, 0.0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.75);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 46, 75, 0, 44, 2.6, 0.0, 16.2, 17.4, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.2, 0.0, 14.2, 12.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.6, 0.0, 12.4, 6.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 46, 75, 0, 44, 2.6, 0.0, 16.2, 17.8, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.2, 0.0, 14.2, 12.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.6, 0.0, 12.4, 8.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
    ArticleModule::remove_exist(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}


#[acmd_script( agent = "buddy", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER,  Hash40::new("attack_s3_s"), false, 0.0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.75);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 46, 75, 0, 44, 2.6, 0.0, 7.2, 18.4, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.2, 0.0, 7.2, 13.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.6, 0.0, 7.2, 7.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 46, 75, 0, 44, 2.6, 0.0, 7.2, 20.2, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.2, 0.0, 7.2, 15.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.6, 0.0, 7.2, 9.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
    ArticleModule::remove_exist(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}


#[acmd_script( agent = "buddy", script = "game_attacks3lw" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER,  Hash40::new("attack_s3_lw"), false, 0.0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.75);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 46, 75, 0, 44, 2.6, 0.0, 1.2, 17.4, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.2, 0.0, 2.8, 12.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.6, 0.0, 4.2, 6.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 46, 75, 0, 45, 2.6, 0.0, 1.2, 18.4, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.2, 0.0, 2.8, 13.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 46, 74, 0, 44, 3.6, 0.0, 4.2, 8.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
    ArticleModule::remove_exist(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}


#[acmd_script( agent = "buddy", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 

    let kbg_b = 95; //90
    let bkb_b = 80; //75
    let kbg_k = 65; //70
    let bkb_k = 70; //80

    frame(lua_state, 3.0);
    {
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        //Banjo//
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 11.0, 70, 81, 0, 74, 3.6, 1.2, 0.0, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 11.0, 70, 81, 0, 74, 3.6, 1.2, 0.5, 1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        //Kazooie//
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 98, 58, 0, 71, 5.3, 0.0, 6.25, -7.5, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BUDDY_WING, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        //Kazooie//
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 98, 58, 0, 71, 5.3, 0.0, 16.0, -4.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BUDDY_WING, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        //Kazooie//
        ATTACK(fighter, 2, 0, Hash40::new("k_wingr4"), 6.5, 98, 50, 0, 73, 4.8, 0.0, 0.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BUDDY_WING, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        //Banjo//
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 8.0, 105, 74, 0, 74, 3.6, 1.2, 0.0, -0.8, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 105, 84, 0, 82, 3.6, 1.2, 0.0, -0.8, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    //Remove kazooie hitbox
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear(boma,2,false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
    }
}
#[acmd_script( agent = "buddy", script = "effect_attackhi3" , category = ACMD_EFFECT , low_priority)]
unsafe fn buddy_attack_hi3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("buddy_attack100"), false, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        //Banjo
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 4, 11.5, 6, 0, 4, 115, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter,1.4);
        //Kazooie
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -0.25, 14, 1.5, 180, 4, 76, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter,1.4);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_all"), 3, 0, 0, 0, 0, 0, 0.9, true);
    }

    
}
#[acmd_script( agent = "buddy", script = "sound_attackhi3" , category = ACMD_SOUND , low_priority)]
unsafe fn buddy_attack_hi3_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    //frame(lua_state, 2.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_buddy_attack100_01"));
        PLAY_SE(fighter, Hash40::new("se_buddy_attack100_02"));
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_attack100_03"));
    }
}

#[acmd_script( agent = "buddy", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let slide_speed = 0.9;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.0 / 1.571); // originally was MotionModule::set_rate, but let's use the lua macros for consistency
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        HIT_NO(fighter, 12, *HIT_STATUS_NORMAL);
        HIT_NO(fighter, 13, *HIT_STATUS_NORMAL);
        HIT_NO(fighter, 14, *HIT_STATUS_NORMAL);
        HIT_NO(fighter, 15, *HIT_STATUS_NORMAL);
        KineticModule::add_speed(boma, &Vector3f::new(slide_speed, 0.0, 0.0));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 38, 68, 0, 59, 3.75, 0.0, 4.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 38, 68, 0, 59, 3.75, 0.0, 4.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 38, 75, 0, 59, 3.9, 0.0, 4.2, 12.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 90, 61, 0, 73, 3.25, 0.0, 3.75, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 90, 61, 0, 73, 3.25, 0.0, 3.75, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 90, 67, 0, 73, 3.5, 0.0, 3.9, 10.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 2.0);
    frame(lua_state, 17.5);
    if is_excute(fighter) {
        HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        HIT_NO(fighter, 14, *HIT_STATUS_OFF);
        HIT_NO(fighter, 15, *HIT_STATUS_OFF);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE(fighter, 1.0);
    wait(lua_state, 1.0);
    for _ in 0..20 {
        if is_excute(fighter) {
            KineticModule::add_speed(boma, &Vector3f::new((-1.0*slide_speed)/20.0, 0.0, 0.0));
        }
        wait(lua_state, 1.0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        buddy_attack_s3_hi_game,
        buddy_attack_s3_s_game,
        buddy_attack_s3_lw_game,
        buddy_attack_hi3_game,
        buddy_attack_hi3_sound,
        buddy_attack_hi3_effect,
        buddy_attack_lw3_game,
    );
}