use super::*;

unsafe extern "C" fn effect_kroolspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
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
        if !WorkModule::is_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL) {
            PLAY_SE(agent, Hash40::new("se_krool_special_n07"));
        }
        else if !VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
            PLAY_SE(agent, Hash40::new("se_krool_special_n01"));
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
        if !VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) && IS_GENERATABLE_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL) {
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

unsafe extern "C" fn effect_kroolspecialnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
            EFFECT_FOLLOW(agent, Hash40::new("krool_cannon_vacuum"), Hash40::new("top"), 0, 10, 17, 0, 0, 0, 0.8, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("krool_cannon_vacuum"), Hash40::new("top"), 0, 10, 17, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_SCALE_W(agent, 0.6, 2.0, 1.0);
            LAST_EFFECT_SET_ALPHA(agent, 0.9);
        }
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.6, 10, 0, 4, 0, 0, 0, false);
        }
    }
    wait(lua_state, 10.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_kroolspecialnfire", effect_kroolspecialnfire);
    agent.acmd("effect_kroolspecialairnfire", effect_kroolspecialnfire);
    agent.acmd("sound_kroolspecialnfire", sound_kroolspecialnfire);
    agent.acmd("sound_kroolspecialairnfire", sound_kroolspecialnfire);
    agent.acmd(
        "expression_kroolspecialnfire",
        expression_kroolspecialnfire,
    );
    agent.acmd(
        "expression_kroolspecialairnfire",
        expression_kroolspecialnfire,
    );
    agent.acmd("effect_kroolspecialnloop", effect_kroolspecialnloop);
    agent.acmd("effect_kroolspecialairnloop", effect_kroolspecialnloop);
}