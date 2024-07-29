use super::*;

// unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
//     let lua_state = agent.lua_state_agent;
//     frame(lua_state, 1.0);
//     if is_excute(agent) {
//         SEARCH(agent, 0, 0, Hash40::new("top"), 60.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ALL, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
//     }
//     wait(lua_state, 1.0);
//     if is_excute(agent) {
//         search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
//     }
// }

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("gekkouga_migawari_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_speciallwjump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn sound_speciallwjump(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_gekkouga_jump01"));
    }
}

pub fn install(agent: &mut Agent) {
    // agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);

    agent.acmd("game_speciallwjump", game_speciallwjump, Priority::Low);
    agent.acmd("sound_speciallwjump", sound_speciallwjump, Priority::Low);
}
