use super::*;

unsafe extern "C" fn sound_bravespecialn1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);
    let sound = if rng == 0 { "vc_kirby_copy_brave_02" } else { "vc_kirby_copy_brave_05" };
    
        frame(lua_state, 1.0);
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
            if is_excute(agent) {
                STOP_SE(agent, Hash40::new("se_brave_special_n01"));
            }
        }
        frame(lua_state, 8.0);
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_brave_special_n02"));
            PLAY_SE(agent, Hash40::new(sound));
            }
        }
        else {
        if is_excute(agent) {
            STOP_SE(agent, Hash40::new("se_brave_special_n01"));
            PLAY_SE(agent, Hash40::new("se_brave_miss"));
        }
    }
}


unsafe extern "C" fn sound_bravespecialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);
    let sound = if rng == 0 { "vc_kirby_copy_brave_03" } else { "vc_kirby_copy_brave_06" };
    
        frame(lua_state, 1.0);
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
            if is_excute(agent) {
                STOP_SE(agent, Hash40::new("se_brave_special_n01"));
            }
        }
        frame(lua_state, 11.0);
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_brave_special_n03"));
            PLAY_SE(agent, Hash40::new(sound));
            }
        }
        else {
        if is_excute(agent) {
            STOP_SE(agent, Hash40::new("se_brave_special_n01"));
            PLAY_SE(agent, Hash40::new("se_brave_miss"));
        }
    }
}

unsafe extern "C" fn sound_bravespecialn3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);
    let sound = if rng == 0 { "vc_kirby_copy_brave_04" } else { "vc_kirby_copy_brave_07" };
    
        frame(lua_state, 1.0);
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_brave_special_n07"));
                STOP_SE(agent, Hash40::new("se_brave_special_n04"));
            }
        }
        frame(lua_state, 17.0);
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_brave_special_n05"));
            PLAY_SE(agent, Hash40::new(sound));
        }
        }
        else {
        if is_excute(agent) {
            STOP_SE(agent, Hash40::new("se_brave_special_n04"));
            PLAY_SE(agent, Hash40::new("se_brave_miss"));
            }
        }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_bravespecialn1", sound_bravespecialn1, Priority::Low);
    agent.acmd("sound_bravespecialairn1", sound_bravespecialn1, Priority::Low);
    agent.acmd("sound_bravespecialn2", sound_bravespecialn2, Priority::Low);
    agent.acmd("sound_bravespecialairn2", sound_bravespecialn2, Priority::Low);
    agent.acmd("sound_bravespecialn3", sound_bravespecialn3, Priority::Low);
    agent.acmd("sound_bravespecialairn3", sound_bravespecialn3, Priority::Low);
}