use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 55, 60, 0, 38, 9.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 55, 60, 0, 38, 9.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let mut lead_wave : u32 = std::u32::MAX;
    let mut wave_2 : u32 = std::u32::MAX;
    let mut wave_3 : u32 = std::u32::MAX;
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_hikari_syuriken"), false, true);
        lead_wave = EffectModule::req_follow(boma, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), &Vector3f{x: 0.0, y: 3.0, z: 0.0}, &Vector3f{x: 80.6, y: -69.5, z: 0.0}, 0.9, true, 0, 0, 0, 0, 0, false, false) as u32;
        EffectModule::set_rate(boma, lead_wave, 0.7);

        wave_2 = EffectModule::req_follow(boma, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), &Vector3f{x: 0.0, y: 3.0, z: -4.0}, &Vector3f{x: 80.6, y: -69.5, z: 0.0}, 0.7, true, 0, 0, 0, 0, 0, false, false) as u32;
        EffectModule::set_rate(boma, wave_2, 0.7);
        EffectModule::set_alpha(boma, wave_2, 0.4);
        //Ray check here is used for checking if you're on the ground. Unfortunately is_touch and is_wall_touch_line didnt work for this. Sorry!
        if GroundModule::ray_check(
            agent.module_accessor, 
            &Vector2f{ x: PostureModule::pos_x(agent.module_accessor), y: PostureModule::pos_y(agent.module_accessor)}, 
            &Vector2f{ x: 0.0, y: -7.0}, true
        ) == 1 {
            FOOT_EFFECT(agent, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.3);
            FOOT_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        EffectModule::set_rate(boma, lead_wave, 0.000001);
        EffectModule::set_rate(boma, wave_2, 0.000001);
        EffectModule::set_alpha(boma, wave_2, 0.4);
        wave_3 = EffectModule::req_follow(boma, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), &Vector3f{x: 0.0, y: 3.0, z: -8.0}, &Vector3f{x: 80.6, y: -69.5, z: 0.0}, 0.5, true, 0, 0, 0, 0, 0, false, false) as u32;
        EffectModule::set_rate(boma, wave_3, 0.7);
        EffectModule::set_alpha(boma, wave_3, 0.4);
        if GroundModule::ray_check(
            agent.module_accessor, 
            &Vector2f{ x: PostureModule::pos_x(agent.module_accessor), y: PostureModule::pos_y(agent.module_accessor)}, 
            &Vector2f{ x: 0.0, y: -7.0}, true
        ) == 1 {
            FOOT_EFFECT(agent, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.3);
            FOOT_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        EffectModule::set_rate(boma, wave_3, 0.000001);
        if GroundModule::ray_check(
            agent.module_accessor, 
            &Vector2f{ x: PostureModule::pos_x(agent.module_accessor), y: PostureModule::pos_y(agent.module_accessor)}, 
            &Vector2f{ x: 0.0, y: -7.0}, true
        ) == 1 {
            FOOT_EFFECT(agent, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.3);
            FOOT_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        }
    }
    for _ in 0..i32::MAX {
        wait(lua_state, 4.0);
        if is_excute(agent) {
                if GroundModule::ray_check(
                    agent.module_accessor, 
                    &Vector2f{ x: PostureModule::pos_x(agent.module_accessor), y: PostureModule::pos_y(agent.module_accessor)}, 
                    &Vector2f{ x: 0.0, y: -7.0}, true
                ) == 1 {
                    FOOT_EFFECT(agent, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
                    LAST_EFFECT_SET_RATE(agent, 0.3);
                    FOOT_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
                }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly);
    agent.acmd("effect_fly", effect_fly);
}
