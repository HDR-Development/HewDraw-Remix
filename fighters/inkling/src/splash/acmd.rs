use super::*;

unsafe extern "C" fn game_normal(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if !WorkModule::is_flag(owner_boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_AFTER_SPECIAL_HI) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 50, 30, 0, 60, 5.0, 0.0, 2.0, -0.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 50, 30, 0, 60, 5.0, 0.0, 2.0, -0.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(boma, 0, 50.0);
            AttackModule::set_ink_value(boma, 1, 50.0);
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 50, 30, 0, 60, 5.0, 0.0, 2.0, -7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 50, 30, 0, 60, 5.0, 0.0, 2.0, 7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(boma, 0, 40.0);
            AttackModule::set_ink_value(boma, 1, 40.0);
        }
        frame(lua_state, 5.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_normal", game_normal);
}