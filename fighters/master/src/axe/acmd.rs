use super::*;

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 49.0, 17.0);
    frame(lua_state, 49.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 52.0);
    FT_MOTION_RATE(agent, 13.0 / 3.0);
    frame(lua_state, 55.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 63.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 70, 85, 0, 75, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 70, 85, 0, 75, 4.5, 0.0, 14.0, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 15.0, 70, 85, 0, 75, 4.5, 0.0, 10.0, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 15.0, 70, 85, 0, 75, 4.5, 0.0, 6.0, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 67.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.4);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.4);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.4);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.41);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.41);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.41);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.42);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.42);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.42);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.43);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.43);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.43);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.44);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.44);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.44);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.45);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.45);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.45);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.46);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.46);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.46);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.47);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.47);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.47);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.48);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.48);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.48);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.49);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.49);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.49);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.5);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.5);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.5);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.52);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.52);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.52);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.54);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.54);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.54);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.56);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.56);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.56);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.58);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.58);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.58);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.6);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.6);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.6);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.61);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.61);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.61);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.62);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.62);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.62);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.63);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.63);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.63);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), false, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.64);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.64);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.64);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare1"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare2"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare3"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 49.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp1"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp2"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp3"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp2flare"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
        // EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash_edge"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        // EffectModule::set_disable_render_offset_last(boma);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_master_axe"), Hash40::new("tex_master_sword2"), 10, Hash40::new("haver"), 0, 6, 0, Hash40::new("haver"), 0, 19, 1, true, Hash40::new("null"), Hash40::new("invalid"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 67.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axe_slash_edge"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp2flare"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp3"), false, true);
        AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn game_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("haver"), 15.0, 70, 85, 0, 75, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 15.0, 70, 85, 0, 75, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 15.0, 70, 85, 0, 75, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 4, 0, Hash40::new("haver"), 15.0, 70, 85, 0, 75, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        AttackModule::clear(boma, 0, false);
        WorkModule::on_flag(boma, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("master_axe_aura"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("master_axe_slash_edge"), -1);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp1_end"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp2_end"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp3_end"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp3"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axe_aura"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);

    agent.acmd("game_speciallwhit", game_speciallwhit, Priority::Low);
    agent.acmd("effect_speciallwhit", effect_speciallwhit, Priority::Low);
    agent.acmd("game_specialairlwhit", game_speciallwhit, Priority::Low);
    agent.acmd("effect_specialairlwhit", effect_speciallwhit, Priority::Low);
}