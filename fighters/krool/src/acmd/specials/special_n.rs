use super::*;

unsafe extern "C" fn game_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 25.0, 13.0);
    ArticleModule::set_rate(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, (25.0 - 1.0)/13.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::krool::instance::SPECIAL_N_GRAB);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE(agent, 1.0);
    ArticleModule::set_rate(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, 1.0);
    if is_excute(agent) {
        if boma.is_button_on(Buttons::SpecialRaw) {
            VarModule::on_flag(agent.battle_object, vars::krool::instance::SPECIAL_N_GRAB);
            agent.on_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_IRONBALL);
        }
        else {
            VarModule::on_flag(agent.battle_object, vars::krool::status::SPECIAL_N_ANGLED);
        }
    }
    frame(lua_state, 26.0);
    if VarModule::is_flag(agent.battle_object, vars::krool::instance::SPECIAL_N_GRAB) {
        FT_MOTION_RATE_RANGE(agent, 26.0, 37.0, 1.0);
        ArticleModule::set_rate(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, (37.0 - 26.0)/1.0);
    }
    else {
        FT_MOTION_RATE_RANGE(agent, 26.0, 30.0, 8.0);
        ArticleModule::set_rate(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, (30.0 - 26.0)/8.0);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
    ArticleModule::set_rate(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_CANCEL);
        if !VarModule::is_flag(agent.battle_object, vars::krool::instance::SPECIAL_N_GRAB) {
            WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL);
        }
    }
    frame(lua_state, 37.0);
    FT_MOTION_RATE_RANGE(agent, 37.0, 70.0, 25.0);
    ArticleModule::set_rate(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, (70.0 - 37.0)/25.0);
    frame(lua_state, 70.0);
    FT_MOTION_RATE(agent, 1.0);
    ArticleModule::set_rate(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, 1.0);
}

unsafe extern "C" fn effect_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if boma.is_button_on(Buttons::SpecialRaw) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 4, 18, 17, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::krool::instance::SPECIAL_N_GRAB) {
            EFFECT(agent, Hash40::new("krool_cannon_shot"), Hash40::new("top"), 16, 10, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_n08"));
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL) {
            PLAY_SE(agent, Hash40::new("se_krool_special_n01"));
        }
        else {
            PLAY_SE(agent, Hash40::new("se_krool_special_n07"));
        }
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_n09"));
    }
}

unsafe extern "C" fn expression_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
        VisibilityModule::set_int64(boma, hash40("crown") as i64, hash40("crown_hide") as i64);
    }
    if IS_EXIST_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT) {
        if is_excute(agent) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::krool::instance::SPECIAL_N_GRAB)
        && IS_GENERATABLE_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("crown") as i64, hash40("crown_normal") as i64);
    }
    if is_excute(agent) {
        if IS_EXIST_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 89.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_specialnfirehi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.4);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.25);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL);
    }
}

unsafe extern "C" fn effect_specialnfirehi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("krool_cannon_shot"), Hash40::new("haver"), 0, 2, 25, 0, 0, 0, 1.3, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("krool_cannon_shot"), -1);
    }
}

unsafe extern "C" fn sound_specialnfirehi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL) {
            PLAY_SE(agent, Hash40::new("se_krool_special_n07"));
        }
        else {
            PLAY_SE(agent, Hash40::new("se_krool_special_n01"));
        }
    }
}

unsafe extern "C" fn expression_specialnfirehi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if IS_GENERATABLE_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("crown") as i64, hash40("crown_normal") as i64);
        if ArticleModule::is_exist(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_specialnfireb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.4);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.25);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(agent, 15.0, 22.0, 4.0);
    ArticleModule::set_rate(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, (22.0 - 15.0)/4.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(agent, 1.0);
    ArticleModule::set_rate(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL);
    }
}

unsafe extern "C" fn effect_specialnfireb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("krool_cannon_shot"), Hash40::new("haver"), 0, 2, 25, 0, 0, 0, 1.3, true);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("krool_cannon_shot"), -1);
    }
}

unsafe extern "C" fn sound_specialnfireb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_n10"));
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL) {
            PLAY_SE(agent, Hash40::new("se_krool_special_n07"));
        }
        else {
            PLAY_SE(agent, Hash40::new("se_krool_special_n01"));
        }
    }
}

unsafe extern "C" fn expression_specialnfireb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
        VisibilityModule::set_int64(boma, hash40("crown") as i64, hash40("crown_hide") as i64);
        if IS_EXIST_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        if IS_GENERATABLE_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("crown") as i64, hash40("crown_normal") as i64);
        if IS_EXIST_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT) {
            if is_excute(agent) {
                ArticleModule::set_visibility_whole(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x23c33f3bdc));
    }
}

unsafe extern "C" fn game_specialnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::krool::instance::SPECIAL_N_GRAB) {
            CATCH(agent, 0, Hash40::new("top"), 4.5, 0.0, 9.5, 10.0, Some(0.0), Some(9.5), Some(19.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            hitbox!(agent, { extends: BASE_WINDBOX, id: 1, bone: "top", dmg: 0.0, angle: 160, kbg: 100, fkb: 25, bkb: 0, size: 6.0, x: 0.0, y: 9.0, z: 22.0, x2: 0.0, y2: 9.0, z2: 28.0, rehit: 10, situation: CollisionSituation::G_d, });
            // hitbox!(agent, { extends: BASE_WINDBOX, id: 2, bone: "top", dmg: 0.0, angle: 160, kbg: 100, fkb: 20, bkb: 0, size: 6.0, x: 0.0, y: 9.0, z: 22.0, x2: 0.0, y2: 9.0, z2: 28.0, rehit: 10, situation: CollisionSituation::GA_d, });
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            SEARCH(agent, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(30.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
        }
    }
}

unsafe extern "C" fn effect_specialnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::krool::instance::SPECIAL_N_GRAB) {
            EFFECT_FOLLOW(agent, Hash40::new("krool_cannon_vacuum"), Hash40::new("top"), 0, 10, 17, 0, 0, 0, 0.8, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("krool_cannon_vacuum"), Hash40::new("top"), 0, 10, 17, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_SCALE_W(agent, 0.6, 2.0, 1.0);
            LAST_EFFECT_SET_ALPHA(agent, 0.75);
        }
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.6, 10, 0, 4, 0, 0, 0, false);
        }
    }
    wait(lua_state, 10.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnfire", game_specialnfire, Priority::Low);
    agent.acmd("game_specialairnfire", game_specialnfire, Priority::Low);
    agent.acmd("effect_specialnfire", effect_specialnfire, Priority::Low);
    agent.acmd("effect_specialairnfire", effect_specialnfire, Priority::Low);
    agent.acmd("sound_specialnfire", sound_specialnfire, Priority::Low);
    agent.acmd("sound_specialairnfire", sound_specialnfire, Priority::Low);
    agent.acmd("expression_specialnfire", expression_specialnfire, Priority::Low);
    agent.acmd("expression_specialairnfire", expression_specialnfire, Priority::Low);

    agent.acmd("game_specialnfirehi", game_specialnfirehi, Priority::Low);
    agent.acmd("game_specialairnfirehi", game_specialnfirehi, Priority::Low);
    agent.acmd("effect_specialnfirehi", effect_specialnfirehi, Priority::Low);
    agent.acmd("effect_specialairnfirehi", effect_specialnfirehi, Priority::Low);
    agent.acmd("sound_specialnfirehi", sound_specialnfirehi, Priority::Low);
    agent.acmd("sound_specialairnfirehi", sound_specialnfirehi, Priority::Low);
    agent.acmd("expression_specialnfirehi", expression_specialnfirehi, Priority::Low);
    agent.acmd("expression_specialairnfirehi", expression_specialnfirehi, Priority::Low);

    agent.acmd("game_specialnfireb", game_specialnfireb, Priority::Low);
    agent.acmd("game_specialairnfireb", game_specialnfireb, Priority::Low);
    agent.acmd("effect_specialnfireb", effect_specialnfireb, Priority::Low);
    agent.acmd("effect_specialairnfireb", effect_specialnfireb, Priority::Low);
    agent.acmd("sound_specialnfireb", sound_specialnfireb, Priority::Low);
    agent.acmd("sound_specialairnfireb", sound_specialnfireb, Priority::Low);
    agent.acmd("expression_specialnfireb", expression_specialnfireb, Priority::Low);
    agent.acmd("expression_specialairnfireb", expression_specialnfireb, Priority::Low);

    agent.acmd("game_specialnloop", game_specialnloop, Priority::Low);
    agent.acmd("game_specialairnloop", game_specialnloop, Priority::Low);
    agent.acmd("effect_specialnloop", effect_specialnloop, Priority::Low);
    agent.acmd("effect_specialairnloop", effect_specialnloop, Priority::Low);

    agent.acmd("effect_specialnspitb", effect_specialnfireb, Priority::Low);
    agent.acmd("effect_specialairnspitb", effect_specialnfireb, Priority::Low);
}