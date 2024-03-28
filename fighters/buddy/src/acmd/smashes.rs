use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, smash::phx::Hash40::new("attack_s4s"), true, 0.0);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, smash::phx::Hash40::new("attack_s4s"), true, 11.0);
        FT_MOTION_RATE(agent, 1.2);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 16.0, 361, 97, 0, 36, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 16.0, 361, 97, 0, 36, 3.5, 0.0, 4.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 16.0, 361, 97, 0, 36, 4.0, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 16.0, 361, 97, 0, 36, 4.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 4, 0, Hash40::new("haver"), 16.0, 361, 97, 0, 36, 4.5, 0.0, 15.5, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 4, 0, Hash40::new("haver"), 16.0, 361, 97, 0, 36, 4.5, 0.0, 22.0, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("haver"), 16.0, 361, 97, 0, 36, 3.5, 0.0, 4.5, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 16.0, 361, 97, 0, 36, 4.0, 0.0, 9.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 16.0, 361, 97, 0, 36, 4.5, 0.0, 12.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 4, 0, Hash40::new("haver"), 16.0, 361, 97, 0, 36, 4.5, 0.0, 19.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 13, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 14, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 15, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 16, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 17, *HIT_STATUS_NORMAL);
        FighterAreaModuleImpl::enable_fix_jostle_area( boma, 4.0, 3.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    //Launcher hitbox, chains into main hitbox
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 110, 100, 100, 0, 5.0, 0.0, 3.0, 6.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 1, Hash40::new("legr"), 13.0, 86, 116, 0, 40, 4.4, 1.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 1, Hash40::new("footr"), 13.0, 86, 116, 0, 40, 4.6, 2.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 1, Hash40::new("legl"), 13.0, 86, 116, 0, 40, 4.4, 2.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_OFF);
        HIT_NO(agent, 13, *HIT_STATUS_OFF);
        HIT_NO(agent, 14, *HIT_STATUS_OFF);
        HIT_NO(agent, 15, *HIT_STATUS_OFF);
        HIT_NO(agent, 16, *HIT_STATUS_OFF);
        HIT_NO(agent, 17, *HIT_STATUS_OFF);
    }
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 0, 1.5, -90, 0, 0, 1.7, true, *EF_FLIP_YZ);
    LAST_EFFECT_SET_RATE(agent, 1);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 5, 1, -90, 0, 0, 1.2, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("footr"), 0, -2, 0, 0, 0, 0, 1.4, true, 1.0);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_h01"));
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_buddy_attack05"));
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_buddy_attackhard_h02"));
    }
}

unsafe extern "C" fn expression_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 44, 69, 0, 64, 5.0, 0.0, 4.0, 13.0, Some(0.0), Some(4.0), Some(-14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.75);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4);

    agent.acmd("game_attackhi4", game_attackhi4);
    agent.acmd("effect_attackhi4", effect_attackhi4);
    agent.acmd("sound_attackhi4", sound_attackhi4);
    agent.acmd("expression_attackhi4", expression_attackhi4);

    agent.acmd("game_attacklw4", game_attacklw4);
}
