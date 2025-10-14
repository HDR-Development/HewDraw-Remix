use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    loop {
        if is_excute(agent) {
            let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 368, 100, 30, 0, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            agent.clear_lua_stack();
            lua_args!(agent, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
            let speed_x = sv_kinetic_energy::get_speed_x(agent.lua_state_agent).abs();
            agent.clear_lua_stack();
            lua_args!(agent, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
            let speed_y = sv_kinetic_energy::get_speed_y(agent.lua_state_agent);
            AttackModule::set_vec_target_pos(
                boma,
                0,
                Hash40::new("top"),
                &Vector2f{x: speed_x * life as f32, y: speed_y * life as f32 + 1.0},
                (life as u32).max(4),
                false
            );
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 80, 134, 0, 71, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false); // equal kb to finisher
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn game_burst(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 90, 80, 0, 80, 11.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
}


pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly, Priority::Low);

    agent.acmd("game_burst", game_burst, Priority::Low);
}