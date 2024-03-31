use super::*;

unsafe extern "C" fn game_movechargebullet(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 45, 200, 0, 15, 1.6, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -0.6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_02, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let bayo_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(agent.boma(), *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let max_repeat = bayo_boma.get_param_int("param_special_n", "add_fire_max");
    if is_excute(agent) {
        if WorkModule::get_int(bayo_boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_ADD_FIRE_COUNT) == max_repeat {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 361, 40, 0, 5, 1.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -0.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_OBJECT);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 361, 40, 0, 15, 1.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -0.3, 0.0, 0, true, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_OBJECT);
        }
        AttackModule::enable_safe_pos(boma);
   }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_movechargebullet", game_movechargebullet);
    agent.acmd("game_move", game_move);
}
