use super::*;

unsafe extern "C" fn game_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.4, 160, 9, 0, 29, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.7, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AREA_WIND_2ND_RAD(agent, 0, 1, 0.02, 1000, 1, 0, 0, 18);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
        ATTACK(agent, 0, 1, Hash40::new("top"), 6.6, 50, 151, 0, 20, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_explode", game_explode);
}