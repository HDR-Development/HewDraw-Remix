use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 12.0, 7.0);
    frame(lua_state, 10.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if boma.is_button_on(Buttons::Guard | Buttons::GuardHold)
        && !VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL)
        && !ArticleModule::is_exist(boma, *FIGHTER_RYU_GENERATE_ARTICLE_HADOKEN) {
            agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED);
        } else {
            agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
            if agent.kind() != *FIGHTER_KIND_KIRBY 
            && !VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                MeterModule::add(agent.battle_object, 2.0 * MeterModule::damage_gain_mul(agent.battle_object));
            }
        }
    }
    frame(lua_state, 14.0);
    if agent.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        FT_MOTION_RATE_RANGE(agent, 14.0, 58.0, 18.0);
    } else if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        FT_MOTION_RATE_RANGE(agent, 14.0, 58.0, 26.0);
    } else {
        FT_MOTION_RATE_RANGE(agent, 14.0, 58.0, 31.0);
    }
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 58.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if agent.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 4.0);
    if !agent.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        if is_excute(agent) {
            if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE) == 0 {
                EFFECT_FOLLOW(agent, Hash40::new("ryu_hadoken_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
            } else {
                EFFECT_FOLLOW(agent, Hash40::new("ryu_syakunetsu_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(agent, 0.95, 0.522, 0.051, 1.7);
            }
        }
        frame(lua_state, 6.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(agent, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 8.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(agent, 0.95, 0.522, 0.051, 1.7);
            } else {
                FLASH(agent, 0.392, 1, 1, 0.353);
            }
        }
        frame(lua_state, 10.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(agent, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 11.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
            
            if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE) == 0 {
                EFFECT_FOLLOW(agent, Hash40::new("ryu_hadoken_shot"), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);
            } else {
                EFFECT_FOLLOW(agent, Hash40::new("ryu_syakunetsu_shot"), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);
            }
        }
        frame(lua_state, 12.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("ryu_hadoken_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(agent, 0.95, 0.522, 0.051, 1.7);
            }
        }
        for _ in 0..6 {
            wait(lua_state, 3.0);
            if is_excute(agent)
            && VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(agent, 0.95, 0.522, 0.051, 0.7);
            }
            wait(lua_state, 3.0);
            if is_excute(agent)
            && VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(agent, 0.95, 0.522, 0.051, 1.7);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(agent)
        && VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(agent);
        }
    }
    else{
        frame(lua_state, 10.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 12, 0, -4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 12.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("throw"), 0, 0, 0, 90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("game_specialairn", game_specialn);
    agent.acmd("effect_specialn", effect_specialn);
    agent.acmd("effect_specialairn", effect_specialn);
}
