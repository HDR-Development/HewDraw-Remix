use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.3, 0.0, 8.5, 0.0, Some(0.0), Some(8.5), Some(9.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.3, 0.0, 8.5, 4.0, Some(0.0), Some(8.5), Some(10.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.3, 0.0, 8.5, -4.0, Some(0.0), Some(8.5), Some(-13.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 16.0, 11.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 40, 118, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        if IS_EXIST_ARTICLE(agent, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO) {
            ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, Hash40::new("throw_f"), false, -1.0);
            ArticleModule::set_rate(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, (16.0 - 1.0)/11.0);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 50, 0, 55, 4.5, 0.0, 7.5, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(agent, 11, 5);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(agent, 16.0, 36.0, 16.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(boma);
        if IS_EXIST_ARTICLE(agent, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO) {
            ArticleModule::set_rate(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, (16.0 - 1.0)/11.0);
        }
    }
    frame(lua_state, 36.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 127, 84, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    if IS_EXIST_ARTICLE(agent, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO) {
        if is_excute(agent) {
            ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, Hash40::new("throw_b"), false, -1.0);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 70, 200, 0, 10, 6.2, 0.0, 7.2, -6.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(agent, -10, 5);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE_RANGE(agent, 35.0, 50.0, 8.0);   
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 88, 65, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    if IS_EXIST_ARTICLE(agent, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO) {
        if is_excute(agent) {
            ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, Hash40::new("throw_hi"), false, -1.0);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 2.0, 80, 100, 20, 0, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(agent, 0, 24);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch);
    agent.acmd("game_catchdash", game_catchdash);
    agent.acmd("game_catchturn", game_catchturn);

    agent.acmd("game_throwf", game_throwf);

    agent.acmd("game_throwb", game_throwb);
    
    agent.acmd("game_throwhi", game_throwhi);
}
