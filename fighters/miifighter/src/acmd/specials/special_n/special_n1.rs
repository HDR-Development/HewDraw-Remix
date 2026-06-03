use super::*;

// ================================================================================================
// ======================================== SHOT PUT ==============================================
// ================================================================================================

unsafe extern "C" fn game_specialn1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article_enable(boma, *FIGHTER_MIIFIGHTER_GENERATE_ARTICLE_IRONBALL, false, -1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_N1_START_HOLD);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_N1_END_HOLD);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ArticleModule::shoot_exist(boma, *FIGHTER_MIIFIGHTER_GENERATE_ARTICLE_IRONBALL, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(agent, 40.0, 80.0, 20.0);
    frame(lua_state, 80.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialn1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -5, 8.5, -0.5, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 360, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        let eff_handle = VarModule::get_int(agent.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE) as u32;
        if EffectModule::is_exist_effect(boma, eff_handle) {
            EffectModule::kill(boma, eff_handle, true, true);
            VarModule::set_int(agent.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE, -1);
        }
    }
}

unsafe extern "C" fn game_specialn1bowl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 25.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_N1_END_HOLD);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ArticleModule::shoot_exist(boma, *FIGHTER_MIIFIGHTER_GENERATE_ARTICLE_IRONBALL, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(agent, 40.0, 80.0, 20.0);
    frame(lua_state, 80.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialn1bowl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 25.0);
    if is_excute(agent) {
        let eff_handle = VarModule::get_int(agent.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE) as u32;
        if EffectModule::is_exist_effect(boma, eff_handle) {
            EffectModule::kill(boma, eff_handle, true, true);
            VarModule::set_int(agent.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE, -1);
        }
    }
}

unsafe extern "C" fn sound_specialn1bowl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_special_n02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_special_c1_n01"));
    }
}

unsafe extern "C" fn expression_specialn1bowl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrowitem"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 79.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn1", game_specialn1, Priority::Low);
    agent.acmd("game_specialairn1", game_specialn1, Priority::Low);
    agent.acmd("effect_specialn1", effect_specialn1, Priority::Low);
    agent.acmd("effect_specialairn1", effect_specialn1, Priority::Low);

    agent.acmd("game_specialn1bowl", game_specialn1bowl, Priority::Low);
    agent.acmd("game_specialairn1bowl", game_specialn1bowl, Priority::Low);
    agent.acmd("effect_specialn1bowl", effect_specialn1bowl, Priority::Low);
    agent.acmd("effect_specialairn1bowl", effect_specialn1bowl, Priority::Low);
    agent.acmd("sound_specialn1bowl", sound_specialn1bowl, Priority::Low);
    agent.acmd("sound_specialairn1bowl", sound_specialn1bowl, Priority::Low);
    agent.acmd("expression_specialn1bowl", expression_specialn1bowl, Priority::Low);
    agent.acmd("expression_specialairn1bowl", expression_specialn1bowl, Priority::Low);
}