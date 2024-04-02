use super::*;

unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        // ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 60, 60, 0, 68, 3.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_moveair", game_move);
    agent.acmd("game_moveground", game_move);
}
