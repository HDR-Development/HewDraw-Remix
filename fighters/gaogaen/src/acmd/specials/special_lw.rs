use super::*;

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.25);
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_START);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_END);
    }
    frame(lua_state, 28.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 32.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 46.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallwstart", game_speciallwstart, Priority::Low);
    agent.acmd("game_specialairlwstart", game_speciallwstart, Priority::Low);
}