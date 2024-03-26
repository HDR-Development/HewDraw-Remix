use super::*;
unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 361, 50, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.object(), vars::lucas::status::THUNDER_LOOSE) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 361, 50, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }
    }
}
unsafe extern "C" fn game_movechild(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 1, 1, Hash40::new("top"), 0.75, 80, 40, 0, 4, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        AttackModule::set_attack_composition_speed(boma, 1, true);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.object(), vars::lucas::status::THUNDER_LOOSE) {
            ATTACK(fighter, 1, 1, Hash40::new("top"), 0.75, 80, 40, 0, 4, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }
    }
}


pub fn install(agent: &mut Agent) {
    agent.acmd("game_move", game_move);
    agent.acmd("game_movechild", game_movechild);
}
