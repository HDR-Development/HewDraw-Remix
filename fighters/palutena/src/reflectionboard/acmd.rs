use super::*;

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 125, 40, 0, 75, 5.0, 0.0, 8.5, 0.0, Some(0.0), Some(-4.5), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 40, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.1);
    }
    frame(lua_state, 210.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_break(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("palutena_mirror_break"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
        }
        else {
            EFFECT(agent, Hash40::new("palutena_mirror_break"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shoot", game_shoot, Priority::Low);
    agent.acmd("effect_break", effect_break, Priority::Low);
}