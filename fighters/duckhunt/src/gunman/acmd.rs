use super::*;

unsafe extern "C" fn sound_ready(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_duckhunt_special_l02"));
    }
    frame(lua_state, 275.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_duckhunt_special_l09"));
    } 
}

unsafe extern "C" fn effect_readyl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 275.0);
    let gunman_kind = WorkModule::get_int(agent.boma(), *WEAPON_DUCKHUNT_GUNMAN_INSTANCE_WORK_ID_KIND);
    if is_excute(agent) {
        match gunman_kind {
            0 => {
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 13.3, 0.74, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 13.3, -0.78, 0, 0, 0, 1, true);
            }
            1 => {
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 15.66, 0.42, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 15.66, -0.5, 0, 0, 0, 1, true);
            }
            2 => {
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 16.92, 0.26, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 16.92, -1.29, 0, 0, 0, 1, true);
            }
            3 => {
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 10.9, 0.85, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 10.9, -0.64, 0, 0, 0, 1, true);
            }
            4 => {
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 14.17, 0.4, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 14.16, -1.36, 0, 0, 0, 1, true);
            }
            _ => {
                return
            }
        }
    }
}

unsafe extern "C" fn effect_readyr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 275.0);
    let gunman_kind = WorkModule::get_int(agent.boma(), *WEAPON_DUCKHUNT_GUNMAN_INSTANCE_WORK_ID_KIND);
    if is_excute(agent) {
        match gunman_kind {    
            0 => {
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 13.3, 0.74, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 13.3, -0.78, 0, 0, 0, 1, true);
            }
            1 => {
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 15.66, 0.42, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 15.66, -0.5, 0, 0, 0, 1, true);
            }
            2 => {
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 16.92, 0.26, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 16.92, -1.29, 0, 0, 0, 1, true);
            }
            3 => {
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 10.9, 0.85, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 10.9, -0.64, 0, 0, 0, 1, true);
            }
            4 => {
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 14.17, 0.4, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 14.16, -1.36, 0, 0, 0, 1, true);
            }
            _ => {
                return
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_readyl", sound_ready);
    agent.acmd("sound_readyr", sound_ready);

    agent.acmd("effect_readyl", effect_readyl);
    agent.acmd("effect_readyr", effect_readyr);
}
