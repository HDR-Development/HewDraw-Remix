use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AREA_WIND_2ND_RAD_arg9(agent, 0, 2, 0.05, 200, 1, 3, 3, 25, 30);
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 65, 16, 0, 54, 3.3, 0.0, 5.7, 2.6, Some(0.0), Some(2.8), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 80, 35, 0, 51, 3.3, 0.0, 5.7, 2.6, Some(0.0), Some(2.8), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn game_shockspell(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 73, 55, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 17.0, 84, 72, 0, 40, 3.0, 0.0, 1.0, 0.0, Some(0.0), Some(27.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 17.0, 84, 72, 0, 40, 3.5, 0.0, 30.0, -4.0, Some(0.0), Some(30.0), Some(4.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_kick_hit_l"));
        AttackModule::set_optional_hit_sound(boma, 2, Hash40::new("se_common_kick_hit_l"));
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_shockspell(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_smokescreen"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_smokescreen"), Hash40::new("top"), 0, 30, 0, 0, 0, 0, 0.4, false);
        LAST_EFFECT_SET_SCALE_W(agent, 0.4, 0.25, 0.4);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.1);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_thunder_spark"), Hash40::new("top"), 0, 0, 0, 0, 0, 180, 0.2, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_thunder_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 180, 0.2, false);
        LAST_EFFECT_SET_SCALE_W(agent, 0.6, 0.6, 0.2);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_smokescreen"), false, false);
    }
}

unsafe extern "C" fn sound_shockspell(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_s"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly, Priority::Low);

    agent.acmd("game_shockspell", game_shockspell, Priority::Low);
    agent.acmd("effect_shockspell", effect_shockspell, Priority::Low);
    agent.acmd("sound_shockspell", sound_shockspell, Priority::Low);
}
