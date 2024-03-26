use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    FT_DESIRED_RATE(agent, 25.0-7.0, 10.0);
    frame(lua_state, 25.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 29.0);
    FT_DESIRED_RATE(agent, 43.0-29.0, 19.0);

}

unsafe extern "C" fn game_landingairn(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    frame(lua_state, 16.0);
    FT_DESIRED_RATE(agent, 18.0-16.0, 3.0);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn);
    agent.acmd("game_landingairn", game_landingairn);

    agent.acmd("game_attackairfhi", game_attackairf);
    agent.acmd("game_attackairf", game_attackairf);
    agent.acmd("game_attackairflw", game_attackairf);

    agent.acmd("game_attackairbhi", game_attackairb);
    agent.acmd("game_attackairb", game_attackairb);
    agent.acmd("game_attackairblw", game_attackairb);

    agent.acmd("game_attackairhi", game_attackairhi);
}