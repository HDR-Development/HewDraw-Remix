use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state: u64 = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(agent, 16.0, 42.0, 20.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CAN);
    }
    frame(lua_state, 42.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 14.0, 18.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE_RANGE(agent, 14.0, 50.0, 41.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CLAY);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
    frame(lua_state, 50.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_REQUEST_SPECIAL_HI_CANCEL);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state: u64 = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 5.0/(4.0-1.0));
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_STATUS_SPECIAL_LW_FLAG_CALL_TRIGGER);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.15);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("game_specialairn", game_specialn);
    
    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialairs", game_specials);

    agent.acmd("game_specialhi", game_specialhi);

    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("game_specialairlw", game_speciallw);
}
