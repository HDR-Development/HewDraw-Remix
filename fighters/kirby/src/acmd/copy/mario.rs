use super::*;

unsafe extern "C" fn game_mariospecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 14.0, 8.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE_RANGE(agent, 21.0, 49.0, 23.0);
    frame(lua_state, 49.0);
    FT_MOTION_RATE(agent, 1.0);

}

unsafe extern "C" fn effect_mariospecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0, 0.353);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, false);
    }

}

unsafe extern "C" fn sound_mariospecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_special_n01"));
    }
}

unsafe extern "C" fn game_mariospecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 11.0, 7.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f::new(-0.5, 0.0, 0.0));
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 3.0, 0.0, 5.5, 4.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 5.0, 0.0, 6.5, 11.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 40, 100, 0, 50, 3.0, 0.0, 5.5, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 40, 100, 0, 50, 5.0, 0.0, 6.5, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_mariospecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.5);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 6, 11, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 0.7, true);
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0, 0, 0, 0, 45, 0, 0.7, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 0.7, true);
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0, 0, 0, 0, -45, 0, 0.7, true);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0, 0.35);
        EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("handl"), 0.0, 0, 0, 0, 0, 0, 0.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("handr"), 0.0, 0, 0, 0, 0, 0, 0.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 6.5, 11.5, 0, 0, 0, 0.26, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
        EffectModule::enable_sync_init_pos_last(boma);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0, 0.75);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_bomb_a"), false, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0, 0.35);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_flame"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, false);
    }
}

unsafe extern "C" fn sound_mariospecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_special_n01"));
        PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
        PLAY_SE(agent, Hash40::new("vc_kirby_attack02"));
    }
}

unsafe extern "C" fn expression_mariospecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_55_smash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_mariospecialn", game_mariospecialn);
    agent.acmd("game_mariospecialairn", game_mariospecialn);
    agent.acmd("effect_mariospecialn", effect_mariospecialn);
    agent.acmd("effect_mariospecialairn", effect_mariospecialn);
    agent.acmd("sound_mariospecialn", sound_mariospecialn);
    agent.acmd("sound_mariospecialairn", sound_mariospecialn);
    agent.acmd("game_mariospecialnfire", game_mariospecialnfire);
    agent.acmd("game_mariospecialairnfire", game_mariospecialnfire);
    agent.acmd("effect_mariospecialnfire", effect_mariospecialnfire);
    agent.acmd("effect_mariospecialairnfire", effect_mariospecialnfire);
    agent.acmd("sound_mariospecialnfire", sound_mariospecialnfire);
    agent.acmd("sound_mariospecialairnfire", sound_mariospecialnfire);
    agent.acmd(
        "expression_mariospecialnfire",
        expression_mariospecialnfire,
    );
    agent.acmd(
        "expression_mariospecialairnfire",
        expression_mariospecialnfire,
    );
}