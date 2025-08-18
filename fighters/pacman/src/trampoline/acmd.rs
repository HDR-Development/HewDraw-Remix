use super::*;

unsafe extern "C" fn game_shake(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 3.0, 4.0);
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_FLAG_ON_JUMP);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shake", game_shake, Priority::Low);
}