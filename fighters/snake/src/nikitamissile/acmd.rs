use super::*;


unsafe extern "C" fn game_fly(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep_ex"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn effect_fly(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.3, true);
        // LAST_PARTICLE_SET_COLOR(agent, 0.6, 0.6, 2.8);
        LAST_PARTICLE_SET_COLOR(agent, 2.5, 2.5, 0.0);
        LAST_EFFECT_SET_RATE(agent, 0.001);

        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 3, Hash40::new("top"), 0.0, 0.35, -1.5, Hash40::new("haver"), 0.0, -0.25, 1.45, true, Hash40::new("null"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("snake_missile_smoke"), true, true);
            EFFECT_OFF_KIND(agent, Hash40::new("snake_missile_smoke2"), true, true);
            EFFECT_OFF_KIND(agent, Hash40::new("snake_missile_smoke3"), true, true);
        }
        wait(lua_state, 5.0);
    }
}

unsafe extern "C" fn effect_stopfall(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 3, Hash40::new("top"), 0.0, 0.35, -1.5, Hash40::new("haver"), 0.0, -0.25, 1.45, true, Hash40::new("null"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
}

unsafe extern "C" fn game_explosion(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 45, 0, 0, 30, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BOMB);
    }
}

unsafe extern "C" fn effect_explosion(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_piyo"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_explosion(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let sfx = SoundModule::play_se(agent.module_accessor, Hash40::new("se_snake_special_l02"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, sfx as i32, 2.0, 0);
    }
}

unsafe extern "C" fn effect_fallexplosion(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn stub(agent : &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_flyattackcommon", game_fly);

    agent.acmd("game_fly", game_fly);
    agent.acmd("effect_fly", effect_fly);
    agent.acmd("sound_fly", stub);

    agent.acmd("game_stopfall", game_fly);
    agent.acmd("effect_stopfall", effect_stopfall);
    agent.acmd("sound_stopfall", stub);

    agent.acmd("game_explosion", game_explosion);
    agent.acmd("effect_explosion", effect_explosion);
    agent.acmd("sound_explosion", sound_explosion);

    agent.acmd("game_hiexplosion", stub);
    agent.acmd("effect_hiexplosion", effect_explosion);
    agent.acmd("sound_hiexplosion", sound_explosion);

    agent.acmd("game_fallexplosion", stub);
    agent.acmd("effect_fallexplosion", effect_fallexplosion);
    agent.acmd("sound_fallexplosion", sound_explosion);
}