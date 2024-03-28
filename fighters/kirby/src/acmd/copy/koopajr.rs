use super::*;

unsafe extern "C" fn effect_koopajrspecialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_KOOPAJR_STATUS_SPECIAL_N_FLAG_FAIL) {
            EFFECT(agent, Hash40::new("koopajr_cannon_miss"), Hash40::new("clowntongue2"), 3, 0, 0, 0, 0, -90, 0.5, 0, 0, 0, 0, 0, 0, true);
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        else {
            let offset = if agent.is_situation(*SITUATION_KIND_GROUND) { 0 } else { 2 };
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd(
        "effect_koopajrspecialnshoot",
        effect_koopajrspecialnshoot,
    );
}