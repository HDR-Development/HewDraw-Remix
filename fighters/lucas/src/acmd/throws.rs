use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch"), false, 0.0);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 0.5 / (9.0 - 8.0));
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.5, 0.0, 6.3, 7.4, Some(0.0), Some(6.3), Some(12.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.5, 0.0, 6.3, 7.4, Some(0.0), Some(6.3), Some(20.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 2.0 / (18.0 - 14.0));
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.5, 0.0, 6.3, 17.0, Some(0.0), Some(6.3), Some(20.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);    
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 69.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch"), false, 0.0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.5, 0.0, 6.3, 7.5, Some(0.0), Some(6.3), Some(13.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.5, 0.0, 6.3, 7.5, Some(0.0), Some(6.3), Some(24.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.5, 0.0, 6.3, 20.0, Some(0.0), Some(6.3), Some(24.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 79.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch_turn"), false, 0.0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.5, 0.0, 6.3, -7.5, Some(0.0), Some(6.3), Some(-13.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(lua_state, 16.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.5, 0.0, 6.3, -7.5, Some(0.0), Some(6.3), Some(-20.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.5, 0.0, 6.3, -17.0, Some(0.0), Some(6.3), Some(-20.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 75.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 1.0, 23.0, 19.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 45, 65, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 6, 11);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 2);
        //FighterCutInManager::set_throw_finish_offset(boma, 5, 8, 0);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 45, 75, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(agent, 13, 3);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        //FighterCutInManager::set_throw_finish_offset(boma, 8, 3, 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, 103, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 7, 16);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, 8, 0);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.500);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 100, 55, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.000);
        CHECK_FINISH_CAMERA(agent, 9.0, 0.0);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn expression_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 1.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 10, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }

    frame(lua_state, 4.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch);
    agent.acmd("game_catchdash", game_catchdash);
    agent.acmd("game_catchturn", game_catchturn);

    agent.acmd("game_throwf", game_throwf);

    agent.acmd("game_throwb", game_throwb);

    agent.acmd("game_throwhi", game_throwhi);
    
    agent.acmd("game_throwlw", game_throwlw);
    agent.acmd("expression_throwlw", expression_throwlw);
}
