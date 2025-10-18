use super::*;

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 16.0, 10.0);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE);
        ATTACK(agent, 0, 0, Hash40::new("top"),   1.0, 55, 464, 0, 32, 10.0, 0.0, 20.0, 9.0, Some(0.0), Some(11.0), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("armr"),  1.0, 55, 464, 0, 32, 3.0, 0.0,  0.0,  0.0, None, None, None,                  1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 32, 4.0, 0.0,  4.5, -1.5, None, None, None,                  1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 32, 4.0, 0.0,  9.0, -1.5, None, None, None,                  1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 32, 4.0, 0.0, 13.5, -1.5, None, None, None,                  1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"),   1.0, 55, 464, 0, 24, 10.0, 0.0, 22.0, -2.0, Some(0.0), Some(22.0), Some(-8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("armr"),  1.0, 55, 464, 0, 24, 3.0, 0.0,  0.0,  0.0, None, None, None,                   1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 24, 4.0, 0.0,  4.5, -1.5, None, None, None,                   1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 24, 4.0, 0.0,  9.0, -1.5, None, None, None,                   1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 24, 4.0, 0.0, 13.5, -1.5, None, None, None,                   1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 50, 464, 0, 24, 10.0, 0.0, 16.0, -11.5, Some(0.0), Some(10.0), Some(-11.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x25813802b6), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::off_flag(boma, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_garyotensei_flash"), Hash40::new("top"), 6, 24, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("cloud_garyotensei_tornado"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_limit"), Hash40::new("tex_cloud_sword2"), 7, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("cloud_garyotensei_wind"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 1, true);
        EFFECT_OFF_KIND(agent, Hash40::new("cloud_limitbreak_aura"), false, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_garyotensei_flash2"), Hash40::new("haver"), 0, 12, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_garyotensei_flash2"), Hash40::new("haver"), 0, 8, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 6);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, false, true);
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 16.0, 10.0);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE);
        ATTACK(agent, 0, 0, Hash40::new("top"),   1.0, 55, 464, 0, 24, 10.0, 0.0, 20.0, 9.0, Some(0.0), Some(11.0), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("armr"),  1.0, 55, 464, 0, 24, 3.0, 0.0,  0.0,  0.0, None, None, None,                  1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 24, 4.0, 0.0,  4.5, -1.5, None, None, None,                  1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 24, 4.0, 0.0,  9.0, -1.5, None, None, None,                  1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 24, 4.0, 0.0, 13.5, -1.5, None, None, None,                  1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"),   1.0, 55, 464, 0, 16, 10.0, 0.0, 22.0, -2.0, Some(0.0), Some(22.0), Some(-8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("armr"),  1.0, 55, 464, 0, 16, 3.0, 0.0,  0.0,  0.0, None, None, None,                   1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 16, 4.0, 0.0,  4.5, -1.5, None, None, None,                   1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 16, 4.0, 0.0,  9.0, -1.5, None, None, None,                   1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("haver"), 1.0, 55, 464, 0, 16, 4.0, 0.0, 13.5, -1.5, None, None, None,                   1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 50, 464, 0, 16, 10.0, 0.0, 16.0, -11.5, Some(0.0), Some(10.0), Some(-11.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x25813802b6), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::off_flag(boma, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE);
    }
}

unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_garyotensei_flash"), Hash40::new("top"), 6, 24, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("cloud_garyotensei_tornado"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_limit"), Hash40::new("tex_cloud_sword2"), 7, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("cloud_garyotensei_wind"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 1, true);
        EFFECT_OFF_KIND(agent, Hash40::new("cloud_limitbreak_aura"), false, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_garyotensei_flash2"), Hash40::new("haver"), 0, 12, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_garyotensei_flash2"), Hash40::new("haver"), 0, 8, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 6);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_specialairlw, Priority::Low);
}