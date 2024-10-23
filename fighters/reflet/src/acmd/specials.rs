use super::*;

unsafe extern "C" fn game_specialntronstart(agent: &mut L2CAgentBase) {
    //let lua_state = agent.lua_state_agent;
    //let boma = agent.boma();
    //17 -> 20
}

unsafe extern "C" fn game_specialairntronstart(agent: &mut L2CAgentBase) {
    //let lua_state = agent.lua_state_agent;
    //let boma = agent.boma();
    //17 -> 20
}

unsafe extern "C" fn game_specialntronend(agent: &mut L2CAgentBase) {
    //let lua_state = agent.lua_state_agent;
    //let boma = agent.boma();
    //FAF is frame 80/84
}

unsafe extern "C" fn game_specialairntronend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 1.0, 65.0, 47.0);  //FAF is frame 80/84
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 8.0);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 12.0);
    MotionModule::set_rate(boma, 2.0);
    wait(lua_state, 1.0);
    for _ in 0..30 {
        if is_excute(agent) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
            }
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, -1);
        WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialntronstart", game_specialntronstart, Priority::Low);
    agent.acmd( "game_specialairntronstart",game_specialairntronstart, Priority::Low);
    agent.acmd("game_specialntronend", game_specialntronend, Priority::Low);
    agent.acmd("game_specialairntronend", game_specialairntronend, Priority::Low);
    
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);

    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);
    agent.acmd("game_specialairhi2", game_specialhi2, Priority::Low);
}