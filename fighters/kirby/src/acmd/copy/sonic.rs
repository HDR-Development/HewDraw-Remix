use super::*;

unsafe extern "C" fn game_sonicspecialnhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let temp = Vector3f { x: -0.3, y: 1.0, z: 0.0 };
		KineticModule::add_speed(boma, &temp);
    }
    FT_MOTION_RATE(agent, 0.5);

}

unsafe extern "C" fn effect_sonicspecialnhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

}

unsafe extern "C" fn sound_sonicspecialnhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("vc_kirby_copy_sonic_01"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_appeal01"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_sonicspecialnhit", game_sonicspecialnhit);
    agent.acmd("effect_sonicspecialnhit", effect_sonicspecialnhit);
    agent.acmd("sound_sonicspecialnhit", sound_sonicspecialnhit);
}