use super::*;

unsafe extern "C" fn effect_chargemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::lucario::instance::IS_POWERED_UP) {
            EFFECT_FLW_POS(agent, Hash40::new("lucario_hadoudan_max_sign"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, true);
            EFFECT_FOLLOW(agent, Hash40::new("lucario_hadoudan_max_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("lucario_hadoudan_hold"), false, false);
        if VarModule::is_flag(agent.battle_object, vars::lucario::instance::IS_POWERED_UP) {
            EFFECT_FLW_POS(agent, Hash40::new("lucario_hadoudan_max_sign"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, true);
            EFFECT_FOLLOW(agent, Hash40::new("lucario_hadoudan_max_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, true);
            EFFECT_FOLLOW(agent, Hash40::new("lucario_hadoudan_hold"), Hash40::new("virtualeffect"), 0, 0, 0, 0, 0, 0, 0.5, true);
        }
    }
}

unsafe extern "C" fn effect_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("lucario_hadoudan_max_hold"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_hadoudan_tail"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_chargemax", effect_chargemax, Priority::Low);
    agent.acmd("effect_shoot", effect_shoot, Priority::Low);
}