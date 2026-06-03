use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, false, -1);
        ArticleModule::generate_article(boma,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, -1);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT,*WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(boma,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, -1.0);
        CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED,*COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("handr"), 1.5, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED,*COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 2, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED,*COLLISION_SITUATION_MASK_G);
    }
    game_CaptureCutCommon(agent);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        grab!(agent, MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(agent,MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        grab!(agent, MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        ArticleModule::change_status_exist(boma,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(boma,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, -1.0);
        WorkModule::on_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(boma,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, false, -1);
        ArticleModule::generate_article(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, -1);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, -1.0);
        CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None,None, None,*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("handr"), 1.5, 0.0, 0.0, 0.0, None,None, None,*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 2, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None,None, None,*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        ArticleModule::change_status_exist(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, -1.0);
        WorkModule::on_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    frame(lua_state, 63.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, false, -1);
        ArticleModule::generate_article(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, -1);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, -1.0);
        CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None,None, None,*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("handr"), 1.5, 0.0, 0.0, 0.0, None,None, None,*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 2, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None,None, None,*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        ArticleModule::change_status_exist(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, -1.0);
        WorkModule::on_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    frame(lua_state, 72.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 38, 133, 0, 33, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(agent, 9.0, 10.5, 1.0);
	frame(lua_state, 10.5);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("kneer"), 3.0, 361, 100, 90, 0, 3.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 361, 100, 90, 0, 3.1, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_catch_only_all(boma, true, false);
		CHECK_FINISH_CAMERA(agent, 21, 11);
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));	
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn sound_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_younglink_rnd_attack"));
    }
}

unsafe extern "C" fn expression_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_ATTACK_ABS_CAMERA_QUAKE(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(boma, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 130, 105, 0, 35, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 3.0, 361, 0, 120, 0, 2.7, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 3.0, 361, 0, 120, 0, 2.7, 3.1, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(agent, -14, 11);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 88, 160, 0, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 20.0, 5.0);
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), 4.0, 20, 100, 0, 50, 3.1, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), 4.0, 20, 100, 0, 50, 3.1, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 4, 21);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 21.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -1.5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_younglink_sword_hdr"), Hash40::new("younglink_sword2"), 6, Hash40::new("sword"), 0.5, 0, 0, Hash40::new("sword"), 9.7, 0, -0.25, true, Hash40::new("younglink_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 30, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 5);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_LEAVE_NEAR_OTTOTTO(agent, -2, 2);
        FT_LEAVE_NEAR_OTTOTTO(agent, 2.5, 2.5);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 108, 70, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 150, 0, 60, 5.5, 0.0, 3.0, -3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 1, 0);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch, Priority::Low);
    agent.acmd("game_catchdash", game_catchdash, Priority::Low);
    agent.acmd("game_catchturn", game_catchturn, Priority::Low);

    agent.acmd("game_throwf", game_throwf, Priority::Low);
    agent.acmd("sound_throwf", sound_throwf, Priority::Low);
    agent.acmd("expression_throwf", expression_throwf, Priority::Low);

    agent.acmd("game_throwb", game_throwb, Priority::Low);

    agent.acmd("game_throwhi", game_throwhi, Priority::Low);
    agent.acmd("effect_throwhi", effect_throwhi, Priority::Low);

    agent.acmd("game_throwlw", game_throwlw, Priority::Low);
}