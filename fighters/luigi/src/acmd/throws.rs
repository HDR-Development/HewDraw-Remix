use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 1.0, 13.0, 5.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
            ArticleModule::remove_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        if ArticleModule::is_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
            ArticleModule::remove_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, smash::phx::Hash40::new("catch"), false, 0.0);
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, -1);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::set_rate(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, (13.0-1.0)/5.0);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::set_rate(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, 1.0);
        GrabModule::set_rebound(boma, true);
        SEARCH(agent, 0, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        //CATCH(fighter, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 0, Hash40::new("top"), 3.5, 0.0, 6.6, 1.0, Some(0.0), Some(6.6), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        //ArticleModule::shoot(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    wait(lua_state, 20.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn sound_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_plunger_shoot"));
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_f"), false, -1.0);
        }
    }
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 45, 65, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("throw"), 10.0, 361, 100, 0, 30, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        CHECK_FINISH_CAMERA(agent, 18, 4);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 12.0, y: -1.0, z: 0.0});
        FT_CATCH_STOP(agent, 5, 1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_luigi_rnd_attack"));
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_DOWN_SE(agent, Hash40::new("se_common_down_soil_s"));
        PLAY_SE(agent, Hash40::new("se_common_kick_hit_m"));
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if is_excute(agent) {
            ArticleModule::change_motion(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_b"), false, -1.0);
        }
    }
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 45, 73, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("throw"), 10.0, 361, 100, 0, 30, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(agent, 21, 3);
        FT_CATCH_STOP(agent, 5, 1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 0.0, 16.0, 18.0);
    if IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_hi"), false, -1.0);
            ArticleModule::set_rate(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, 16.0/18.0);
        }
    }
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, 72, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, -2, 24);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 7.0, z: 0.0});
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(agent, 16.0, 40.0, 22.0);
    if is_excute(agent) {
        ArticleModule::set_rate(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, (40.0-16.0)/22.0);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE(agent, 1.000);
    if is_excute(agent) {
        ArticleModule::set_rate(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, 1.0);
    }
}

unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_luigi_rnd_attack"));
    }
}

unsafe extern "C" fn expression_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
            ArticleModule::change_motion(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, smash::phx::Hash40::new("throw_lw"), false, 0.0);
        }
        FT_LEAVE_NEAR_OTTOTTO(agent, -1, 1);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 80, 30, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 0, 0);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn expression_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch);
    agent.acmd("sound_catch", sound_catch);

    agent.acmd("game_throwf", game_throwf);
    agent.acmd("sound_throwf", sound_throwf);

    agent.acmd("game_throwb", game_throwb);

    agent.acmd("game_throwhi", game_throwhi);
    agent.acmd("effect_throwhi", effect_throwhi);
    agent.acmd("sound_throwhi", sound_throwhi);
    agent.acmd("expression_throwhi", expression_throwhi);

    agent.acmd("game_throwlw", game_throwlw);
    agent.acmd("expression_throwlw", expression_throwlw);
}
