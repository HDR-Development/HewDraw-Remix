use super::*;

unsafe extern "C" fn game_kroolspecialnfire(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn effect_kroolspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if boma.is_button_on(Buttons::SpecialRaw) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 4, 13, 12, 0, 0, 0, 0.5, true);
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

unsafe extern "C" fn sound_kroolspecialnfire(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn expression_kroolspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 30.0);
    if IS_GENERATABLE_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL)
    && !VarModule::is_flag(agent.battle_object, vars::krool::instance::SPECIAL_N_GRAB) {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 89.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_kroolspecialnfirehi(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn effect_kroolspecialnfirehi(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn sound_kroolspecialnfirehi(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn expression_kroolspecialnfirehi(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn game_kroolspecialnfireb(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn effect_kroolspecialnfireb(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn sound_kroolspecialnfireb(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn expression_kroolspecialnfireb(agent: &mut L2CAgentBase) {
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
        if agent.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
        if VarModule::is_flag(agent.battle_object, vars::krool::instance::SPECIAL_N_GRAB) {
            CATCH(agent, 0, Hash40::new("top"), 4.5, 0.0, 9.5, 10.7, Some(0.0), Some(9.5), Some(17.5), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 160, 100, 20, 0, 6.0, 0.0, 9.0, 22.0, Some(0.0), Some(9.0), Some(28.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            SEARCH(agent, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(28.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_kroolspecialnfire", game_kroolspecialnfire, Priority::Low);
    agent.acmd("game_kroolspecialairnfire", game_kroolspecialnfire, Priority::Low);
    agent.acmd("effect_kroolspecialnfire", effect_kroolspecialnfire, Priority::Low);
    agent.acmd("effect_kroolspecialairnfire", effect_kroolspecialnfire, Priority::Low);
    agent.acmd("sound_kroolspecialnfire", sound_kroolspecialnfire, Priority::Low);
    agent.acmd("sound_kroolspecialairnfire", sound_kroolspecialnfire, Priority::Low);
    agent.acmd("expression_kroolspecialnfire", expression_kroolspecialnfire, Priority::Low);
    agent.acmd("expression_kroolspecialairnfire", expression_kroolspecialnfire, Priority::Low);

    agent.acmd("game_kroolspecialnfirehi", game_kroolspecialnfirehi, Priority::Low);
    agent.acmd("game_kroolspecialairnfirehi", game_kroolspecialnfirehi, Priority::Low);
    agent.acmd("effect_kroolspecialnfirehi", effect_kroolspecialnfirehi, Priority::Low);
    agent.acmd("effect_kroolspecialairnfirehi", effect_kroolspecialnfirehi, Priority::Low);
    agent.acmd("sound_kroolspecialnfirehi", sound_kroolspecialnfirehi, Priority::Low);
    agent.acmd("sound_kroolspecialairnfirehi", sound_kroolspecialnfirehi, Priority::Low);
    agent.acmd("expression_kroolspecialnfirehi", expression_kroolspecialnfirehi, Priority::Low);
    agent.acmd("expression_kroolspecialairnfirehi", expression_kroolspecialnfirehi, Priority::Low);

    agent.acmd("game_kroolspecialnfireb", game_kroolspecialnfireb, Priority::Low);
    agent.acmd("game_kroolspecialairnfireb", game_kroolspecialnfireb, Priority::Low);
    agent.acmd("effect_kroolspecialnfireb", effect_kroolspecialnfireb, Priority::Low);
    agent.acmd("effect_kroolspecialairnfireb", effect_kroolspecialnfireb, Priority::Low);
    agent.acmd("sound_kroolspecialnfireb", sound_kroolspecialnfireb, Priority::Low);
    agent.acmd("sound_kroolspecialairnfireb", sound_kroolspecialnfireb, Priority::Low);
    agent.acmd("expression_kroolspecialnfireb", expression_kroolspecialnfireb, Priority::Low);
    agent.acmd("expression_kroolspecialairnfireb", expression_kroolspecialnfireb, Priority::Low);
}