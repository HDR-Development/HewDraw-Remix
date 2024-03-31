use super::*;

unsafe extern "C" fn effect_shizuespecialnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("shizue_cracker"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn expression_shizuespecialnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd(
        "effect_shizuespecialnfailure",
        effect_shizuespecialnfailure,
    );
    agent.acmd(
        "effect_shizuespecialairnfailure",
        effect_shizuespecialnfailure,
    );
    agent.acmd(
        "expression_shizuespecialnfailure",
        expression_shizuespecialnfailure,
    );
    agent.acmd(
        "expression_shizuespecialairnfailure",
        expression_shizuespecialnfailure,
    );
}