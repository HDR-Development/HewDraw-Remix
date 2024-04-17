use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 50, 48, 0, 40, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 30.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 4.0);
        WorkModule::on_flag(boma, *WEAPON_RIDLEY_BREATH_INSTANCE_WORK_ID_FLAG_CHANGE_SCALE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly);
}