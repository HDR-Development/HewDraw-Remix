use super::*;

unsafe extern "C" fn effect_specialhilanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("inkling_superjump_drop"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);

            let r = WorkModule::get_float(boma, *WEAPON_INKLING_SQUID_INSTANCE_WORK_ID_FLOAT_R);
            let g = WorkModule::get_float(boma, *WEAPON_INKLING_SQUID_INSTANCE_WORK_ID_FLOAT_G);
            let b = WorkModule::get_float(boma, *WEAPON_INKLING_SQUID_INSTANCE_WORK_ID_FLOAT_B);
            LAST_PARTICLE_SET_COLOR(agent, r, g, b);

        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("inkling_superjump_drop"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        
            let r = WorkModule::get_float(boma, *WEAPON_INKLING_SQUID_INSTANCE_WORK_ID_FLOAT_R);
            let g = WorkModule::get_float(boma, *WEAPON_INKLING_SQUID_INSTANCE_WORK_ID_FLOAT_G);
            let b = WorkModule::get_float(boma, *WEAPON_INKLING_SQUID_INSTANCE_WORK_ID_FLOAT_B);
            LAST_PARTICLE_SET_COLOR(agent, r, g, b);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_specialhilanding", effect_specialhilanding, Priority::Low);
}