use super::*;

unsafe extern "C" fn game_max(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    //let mut halflife=30.0;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 55, 60, 0, 80, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(boma);
        //halflife = WorkModule::get_int(weapon.module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE) as f32 /2.0;
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 55, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn effect_max(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    for _ in 0..100 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
            EFFECT_FOLLOW(agent, Hash40::new("koopa_breath_m_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        wait(lua_state, 15.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
        }
        wait(lua_state, 15.0);
    }
}

unsafe extern "C" fn game_end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_end(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lr = PostureModule::lr(boma);
    let pos = *PostureModule::pos(boma);
    EffectModule::req(
        boma,
        Hash40::new("sys_damage_fire"),
        &Vector3f{x: pos.x + 4.0*lr, y: pos.y, z:pos.z},
        &Vector3f::zero(),
        2.0,
        0,
        -1,
        false,
        0
    );
}


pub fn install(agent: &mut Agent) {
    agent.acmd("game_max", game_max);
    agent.acmd("effect_max", effect_max);

    agent.acmd("game_end", game_end);
    agent.acmd("effect_end", effect_end);
}
