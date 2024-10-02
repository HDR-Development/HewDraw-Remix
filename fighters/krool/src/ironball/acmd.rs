use super::*;

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((agent.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let lr = PostureModule::lr(owner_module_accessor);
        if owner_module_accessor.kind() == *FIGHTER_KIND_KROOL && owner_module_accessor.is_motion(Hash40::new("special_n_fire_hi")) {
            KineticModule::reflect_speed(boma, &Vector3f{x: -0.2164, y: lr * 0.9762, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 60, 92, 0, 18, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn game_spitshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 60, 87, 0, 55, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shoot", game_shoot, Priority::Low);
    agent.acmd("game_spitshoot", game_spitshoot, Priority::Low);
}