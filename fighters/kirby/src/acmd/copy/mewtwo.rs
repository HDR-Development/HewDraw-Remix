use super::*;

unsafe extern "C" fn game_mewtwospecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MEWTWO_GENERATE_ARTICLE_SHADOWBALL, false, -1);
    }
}

unsafe extern "C" fn effect_mewtwospecialnhold(agent: &mut L2CAgentBase) {
    if is_excute(agent) && agent.is_situation(*SITUATION_KIND_GROUND) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.5, 10, 0, 4, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 8.0);
}

unsafe extern "C" fn sound_mewtwospecialnhold(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_mewtwo_special_n01"));
    }
}

unsafe extern "C" fn effect_mewtwospecialnmax(agent: &mut L2CAgentBase) {
    if agent.is_prev_status(*FIGHTER_KIRBY_STATUS_KIND_MEWTWO_SPECIAL_N_HOLD) {
        if is_excute(agent) {
            EFFECT_FLW_POS(agent, Hash40::new("mewtwo_shadowball_max_sign"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.4, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        if is_excute(agent) {
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.8, 10, 0, 4, 0, 0, 0, false);
            }
            FLASH(agent, 0.9, 0.7, 1, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            FLASH_FRM(agent, 5, 0.9, 0.4, 1, 0.1);
        }
        wait(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    } else {
        return effect_mewtwospecialnhold(agent);
    }
}

unsafe extern "C" fn sound_mewtwospecialnmax(agent: &mut L2CAgentBase) {
    if agent.is_prev_status(*FIGHTER_KIRBY_STATUS_KIND_MEWTWO_SPECIAL_N_HOLD) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_mewtwo_special_n02"));
            STOP_SE(agent, Hash40::new("se_mewtwo_special_n01"));
            PLAY_STATUS(agent, Hash40::new("se_mewtwo_special_n07"));
        }
    } else {
        if is_excute(agent) {
            PLAY_STATUS(agent, Hash40::new("se_mewtwo_special_n07"));
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_mewtwospecialnstart", game_mewtwospecialnstart, Priority::Low);
    agent.acmd("game_mewtwospecialairnstart", game_mewtwospecialnstart, Priority::Low);
    agent.acmd("effect_mewtwospecialnhold", effect_mewtwospecialnhold, Priority::Low);
    agent.acmd("sound_mewtwospecialnhold", sound_mewtwospecialnhold, Priority::Low);
    agent.acmd("effect_mewtwospecialnmax", effect_mewtwospecialnmax, Priority::Low);
    agent.acmd("sound_mewtwospecialnmax", sound_mewtwospecialnmax, Priority::Low);
    agent.acmd("effect_mewtwospecialairnhold", effect_mewtwospecialnhold, Priority::Low);
    agent.acmd("sound_mewtwospecialairnhold", sound_mewtwospecialnhold, Priority::Low);
    agent.acmd("effect_mewtwospecialairnmax", effect_mewtwospecialnmax, Priority::Low);
    agent.acmd("sound_mewtwospecialairnmax", sound_mewtwospecialnmax, Priority::Low);
}
