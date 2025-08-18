use super::*;

unsafe extern "C" fn game_dhspecialnend1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 5.0);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
}

unsafe extern "C" fn game_dhspecialnend2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.4);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_dhspecialnend1", game_dhspecialnend1, Priority::Low);
    agent.acmd("game_dhspecialairnend1", game_dhspecialnend1, Priority::Low);
    agent.acmd("game_dhspecialnend2", game_dhspecialnend2, Priority::Low);
    agent.acmd("game_dhspecialairnend2", game_dhspecialnend2, Priority::Low);
    agent.acmd("game_dhspecialnendmax", game_dhspecialnend2, Priority::Low);
    agent.acmd("game_dhspecialairnendmax", game_dhspecialnend2, Priority::Low);
}