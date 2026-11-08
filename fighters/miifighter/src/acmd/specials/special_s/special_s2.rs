use super::*;

// ================================================================================================
// ======================================== BURNING DROP KICK =====================================
// ================================================================================================

unsafe extern "C" fn game_specialairs2start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }
}

unsafe extern "C" fn game_specials2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
        boma.select_cliff_hangdata_from_name("special_s2");
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 13.0, 44, 67, 0, 60, 6.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_kick_hit_l"));
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 13.0, 44, 55, 0, 60, 4.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn game_specialairs2end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_s2");
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        SET_SPEED_EX(agent, 1.4, -0.019, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        SET_SPEED_EX(agent, 1.1, -0.519, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

unsafe extern "C" fn game_specials2end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 0.7);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialairs2start", game_specialairs2start, Priority::Low);
    agent.acmd("game_specials2", game_specials2, Priority::Low);
    agent.acmd("game_specialairs2end", game_specialairs2end, Priority::Low);
}