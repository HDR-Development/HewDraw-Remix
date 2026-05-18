use super::*;

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_hide") as i64);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_show") as i64);
        agent.on_flag(*WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
        LinkModule::unlink_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        // Ground-only
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 268, 100, 115, 0, 9.5, 0.0, 27.5, 13.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // Air-only 
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 268, 55, 0, 15, 9.5, 0.0, 27.5, 13.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    } // weaker @35
    wait(lua_state, 1.0);
    if is_excute(agent) {
        // Ground-only
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 272, 100, 115, 0, 10.5, 0.0, 7.5, 16.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // Air-only
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 272, 55, 0, 15, 10.5, 0.0, 7.5, 16.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        agent.on_flag(*WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT); // weaker @37
        ModelModule::set_scale(boma, 0.975);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_id = agent.get_int(*WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let bayo = utils::util::get_battle_object_from_id(owner_id);
    let bayo_boma = &mut *(*bayo).module_accessor;
    let lr = boma.lr();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("bayonetta_wickedweave_circle"), Hash40::new("rot"), 0, 0, 0, 0, 0, -90, 1, true);
        if bayo_boma.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != 1 {LAST_PARTICLE_SET_COLOR(agent, 0.037, 0.04, 0.039); }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_wickedweave_appearance"), Hash40::new("rot"), 0, 0, 0, 0, -45.0+90.0*lr, -90, 1, true);
        if bayo_boma.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != 1 {LAST_PARTICLE_SET_COLOR(agent, 0.037, 0.04, 0.039); }
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_wickedweave_line"), Hash40::new("rot"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 18, -1, 0, 0, 0, 0, 0.82, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 18, -1, 0, 0, 0, 0, 0.82, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("bayonetta_wickedweave_hiar1"), Hash40::new("rot"), 0, 0, 0, 0, 0, -90, 1, true);
        if bayo_boma.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != 1 {LAST_PARTICLE_SET_COLOR(agent, 0.037, 0.04, 0.039); }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
    agent.acmd("effect_attacklw4", effect_attacklw4, Priority::Low);
}