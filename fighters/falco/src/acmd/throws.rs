use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("trans"), 4.1, 0.0, 7.2, 4.1, Some(0.0), Some(7.2), Some(9.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("trans"), 4.1, 0.0, 6.0, 4.1, Some(0.0), Some(6.0), Some(9.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("trans"), 4.1, 0.0, 7.2, -4.1, Some(0.0), Some(7.2), Some(-14.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 50, 135, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 50, 112, 0, 44, 5.76, 0.0, 10.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ELBOW);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(agent, 9, 3);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ModelModule::set_joint_translate(boma, Hash40::new("throw"), &Vector3f{ x: 0.0, y: 7.0, z: 18.0 }, false, false);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(agent, 15.0, 35.0, 24.0);
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 8, 4.5, -10, 20, 210, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 1, 9, 7, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, 0.75);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, 8, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_falco_rnd_attack"));
    }
}

unsafe extern "C" fn expression_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        lua_args!(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        smash::app::sv_animcmd::FT_ATTACK_ABS_CAMERA_QUAKE(lua_state);
        agent.clear_lua_stack();
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let scale = PostureModule::scale(boma);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 35, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        // CHECK_FINISH_CAMERA(agent, 7, 8);
        // lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
        // lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 2.0, y: 2.0, z: 0.0});
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 14, -13, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("falco_blaster_bullet_b"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("falco_blaster_shot"), Hash40::new("haver"), 0, 1.2, 4.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.7);
    }
}

unsafe extern "C" fn sound_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_special_n01"));
    }
}

unsafe extern "C" fn expression_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("blaster") as i64, hash40("blaster_hide") as i64);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("blaster") as i64, hash40("blaster_normal") as i64);
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 90, 74, 0, 81, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 6, 13);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ModelModule::set_joint_translate(boma, Hash40::new("throw"), &Vector3f{ x: 0.0, y: 15.0, z: 2.0 }, false, false);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(agent, 8.0, 13.0, 3.0);
    if is_excute(agent) {
        ModelModule::set_joint_translate(boma, Hash40::new("throw"), &Vector3f{ x: 0.0, y: 20.0, z: 0.0 }, false, false);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("falco_blaster_bullet_hi"), Hash40::new("haver"), 0, 1.2, 4.3, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("falco_blaster_shot"), Hash40::new("haver"), 0, 1.2, 4.3, 0, 0, 120, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
}

unsafe extern "C" fn sound_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_special_n01"));
    }
}

unsafe extern "C" fn expression_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("blaster") as i64, hash40("blaster_hide") as i64);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("blaster") as i64, hash40("blaster_normal") as i64);
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 50, 110, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        // CHECK_FINISH_CAMERA(agent, 2, 0);
        // lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        // lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), -6.5, 0, 4, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("falco_blaster_bullet_lw"), Hash40::new("haver"), 0, 1.2, 4.3, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("falco_blaster_shot"), Hash40::new("haver"), 0, 1.2, 4.3, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("falco_blaster_bullet_lw"), false, false);
    }
}

unsafe extern "C" fn sound_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_DOWN_SE(agent, Hash40::new("se_common_down_soil_s"));
        PLAY_SE(agent, Hash40::new("se_common_kick_hit_m"));
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_special_n01"));
    }
}

unsafe extern "C" fn expression_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("blaster") as i64, hash40("blaster_hide") as i64);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("blaster") as i64, hash40("blaster_normal") as i64);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch, Priority::Low);
    agent.acmd("game_catchdash", game_catchdash, Priority::Low);
    agent.acmd("game_catchturn", game_catchturn, Priority::Low);
    
    agent.acmd("game_throwf", game_throwf, Priority::Low);
    agent.acmd("effect_throwf", effect_throwf, Priority::Low);
    agent.acmd("sound_throwf", sound_throwf, Priority::Low);
    agent.acmd("expression_throwf", expression_throwf, Priority::Low);
    
    agent.acmd("game_throwb", game_throwb, Priority::Low);
    agent.acmd("effect_throwb", effect_throwb, Priority::Low);
    agent.acmd("sound_throwb", sound_throwb, Priority::Low);
    agent.acmd("expression_throwb", expression_throwb, Priority::Low);
    
    agent.acmd("game_throwhi", game_throwhi, Priority::Low);
    agent.acmd("effect_throwhi", effect_throwhi, Priority::Low);
    agent.acmd("sound_throwhi", sound_throwhi, Priority::Low);
    agent.acmd("expression_throwhi", expression_throwhi, Priority::Low);
    
    agent.acmd("game_throwlw", game_throwlw, Priority::Low);
    agent.acmd("effect_throwlw", effect_throwlw, Priority::Low);
    agent.acmd("sound_throwlw", sound_throwlw, Priority::Low);
    agent.acmd("expression_throwlw", expression_throwlw, Priority::Low);
}