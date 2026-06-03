use super::*;

unsafe extern "C" fn effect_specialhihold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("brave_chant"), Hash40::new("top"), 0, 9, 0, 0, -60, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("brave_tornado1_hold"), Hash40::new("handl"), 2, 2, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn effect_specialhiholdm(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("brave_tornado1_hold"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("brave_tornado2_hold_flash"), Hash40::new("handl"), 2, 2, 0, 0, 0, 0, 0.75, false);
        EFFECT_FOLLOW(agent, Hash40::new("brave_tornado2_hold"), Hash40::new("handl"), 2, 2, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn game_specialhiempty(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
}

unsafe extern "C" fn game_specialhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.0, 7.0);
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, 0);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 41.0, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
    frame(lua_state, 41.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::brave::instance::SPECIAL_HI_ENABLE_FREEFALL) {
            boma.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        }
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 11.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, 0);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
}

unsafe extern "C" fn effect_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("brave_chant_finish"), Hash40::new("top"), 0, 9, 0, 0, -60, 0, 1, false);
        EFFECT_FOLLOW(agent, Hash40::new("brave_tornado3_hold_flash"), Hash40::new("handl"), 2, 2, 0, 0, 0, 0, 0.5, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        EFFECT_FOLLOW(agent, Hash40::new("brave_tornado2_wind"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
}

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 14.0);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, 0);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_specialhihold", effect_specialhihold, Priority::Low);
    agent.acmd("effect_specialairhihold", effect_specialhihold, Priority::Low);

    agent.acmd("effect_specialhiholdm", effect_specialhiholdm, Priority::Low);
    agent.acmd("effect_specialhiholdm", effect_specialhiholdm, Priority::Low);

    agent.acmd("game_specialhi1", game_specialhi1, Priority::Low);
    agent.acmd("game_specialairhi1", game_specialhi1, Priority::Low);
    
    agent.acmd("game_specialhiempty", game_specialhiempty, Priority::Low);
    agent.acmd("game_specialairhiempty", game_specialhiempty, Priority::Low);
    
    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);
    agent.acmd("game_specialairhi2", game_specialhi2, Priority::Low);
    agent.acmd("effect_specialhi2", effect_specialhi2, Priority::Low);
    agent.acmd("effect_specialairhi2", effect_specialhi2, Priority::Low);

    agent.acmd("game_specialhi3", game_specialhi3, Priority::Low);
    agent.acmd("game_specialairhi3", game_specialhi3, Priority::Low);
}