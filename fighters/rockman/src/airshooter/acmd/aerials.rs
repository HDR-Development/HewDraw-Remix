use super::*;

unsafe extern "C" fn rockman_airshooter_regular_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 366, 100, 50, 0, 5.0, 0.0, 3.0, 0.0, None, None, None, 0.2, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::rockman_airshooter::status::MOVE);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 90, 50, 0, 90, 7.0, 0.0, 3.0, 0.0, None, None, None, 0.2, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
        WorkModule::on_flag(boma, *WEAPON_ROCKMAN_AIRSHOOTER_INSTANCE_WORK_ID_FLAG_ATTACK_VECTOR_REVERSE_UD_CHECK);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", rockman_airshooter_regular_game);
}
