
use super::*;

unsafe extern "C" fn richter_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 79, 0, 65, 4.5, 0.0, 10.0, 34.0, Some(0.0), Some(10.0), Some(31.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 110, 45, 0, 75, 3.8, 0.0, 11.0, 34.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 361, 40, 0, 60, 2.3, 0.0, 11.0, 7.0, Some(0.0), Some(11.0), Some(32.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 50, 0, 50, 4.6, 0.0, 11.0, 7.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_HOLD);
    }
}

unsafe extern "C" fn richter_attack_s3_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.141, 0.851, 1.0);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("sys_damage_aura"), -1);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("haver"), 0, 0, 0, 4, 30, 4, 0.93, true);
    }
}

unsafe extern "C" fn richter_attack_s3_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_attackhard_s01"));
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_richter_rnd_attack"));
    }
}

unsafe extern "C" fn richter_attack_s3_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn richter_whip_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

unsafe extern "C" fn richter_whip_attack_s3_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot6"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot26"), 2, 0, 0, 0, 0, 0, 1.5, true);
    }
}

unsafe extern "C" fn richter_attack_squat_s3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 110, 45, 0, 75, 3.8, 0.0, 7.0, 31.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 361, 40, 0, 60, 2.3, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(29.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 50, 0, 50, 4.6, 0.0, 7.0, 7.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn richter_attack_squat_s3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.141, 0.851, 1.0);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_aura"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("haver"), 0, 0, 0, 4, 30, 4, 0.9, true);
    }
}

unsafe extern "C" fn richter_attack_squat_s3_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_richter_attackhard_s01"));
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_richter_rnd_attack"));
    }
}

unsafe extern "C" fn richter_attack_squat_s3_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn richter_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 100, 80, 0, 50, 4.5, 0.0, 9.5, 8.5, Some(0.0), Some(13.0), Some(8.6), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 285, 55, 0, 45, 3.8, 0.0, 46.0, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        /* Air-only */
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 285, 47, 0, 45, 3.8, 0.0, 46.0, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 100, 75, 0, 60, 5.5, 0.0, 18.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 100, 75, 0, 60, 2.5, 0.0, 17.0, 1.0, Some(0.0), Some(44.0), Some(1.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn richter_attack_hi3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.141, 0.851, 1.0);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(fighter, 3.0);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_aura"), false, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_trace"), Hash40::new("top"), 2, 10.5, -2, 20, 0, 90, 1.0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.4);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EffectModule::kill_kind(boma, Hash40::new("richter_whip_trace"), true, true);
    }
}

unsafe extern "C" fn richter_attack_hi3_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn richter_whip_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

unsafe extern "C" fn richter_whip_attack_hi3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("hookshot3"), -6, 0, 0, 180, 50, 90, 1.0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1, false, 0.65);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 2, 0, 0, 0, 0, 0, 1.5, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EffectModule::kill_kind(boma, Hash40::new("richter_whip_straight"), true, true);
    }
}

unsafe extern "C" fn richter_attack_hi3_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_attackair_b01"));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_richter_attackair_b01"));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_richter_rnd_attack"));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_attackair_b02"));
    }
}

unsafe extern "C" fn richter_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 7.0, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.0, 72, 65, 0, 70, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("footl"), 9.0, 72, 65, 0, 70, 3.0, 0.0, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 5.0, 72, 80, 0, 50, 2.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("footl"), 5.0, 72, 80, 0, 50, 2.5, 0.0, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn richter_attack_lw32_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 5.0, 6.0);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_ATTACK_LW32_WORK_ID_FLAG_LANDING_AIR);
    }
    frame(lua_state, 7.0);
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8);
    FT_MOTION_RATE_RANGE(fighter, 7.0, 34.0, 18.0);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 12.0, 55, 90, 0, 60, 3.5, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("footl"), 12.0, 55, 90, 0, 60, 3.0, 0.0, 1.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 12.0, 55, 90, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.0, 72, 24, 0, 63, 3.5, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("footl"), 9.0, 72, 24, 0, 63, 3.0, 0.0, 1.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::clear(boma, 2, false);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 34.0);
    FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    smashline::Agent::new("richter")
        .acmd("game_attacks3", richter_attack_s3_s_game)
        .acmd("effect_attacks3", richter_attack_s3_s_effect)
        .acmd("sound_attacks3", richter_attack_s3_sound)
        .acmd("expression_attacks3", richter_attack_s3_expression)
        .acmd("game_attacksquats3", richter_attack_squat_s3_game)
        .acmd("effect_attacksquats3", richter_attack_squat_s3_effect)
        .acmd("sound_attacksquats3", richter_attack_squat_s3_sound)
        .acmd("expression_attacksquats3", richter_attack_squat_s3_expression)
        .acmd("game_attackhi3", richter_attack_hi3_game)
        .acmd("effect_attackhi3", richter_attack_hi3_effect)
        .acmd("sound_attackhi3", richter_attack_hi3_sound)
        .acmd("expression_attackhi3", richter_attack_hi3_expression)
        .acmd("game_attacklw3", richter_attack_lw3_game)
        .acmd("game_attacklw32", richter_attack_lw32_game)
        .install();
    smashline::Agent::new("richter_whip")
        .acmd("game_attacks3", richter_whip_attack_s3_s_game)
        .acmd("effect_attacks3", richter_whip_attack_s3_s_effect)
        .acmd("game_attackhi3", richter_whip_attack_hi3_game)
        .acmd("effect_attackhi3", richter_whip_attack_hi3_effect)
        .install();
}
