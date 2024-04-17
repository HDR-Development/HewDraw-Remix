use super::*;

unsafe extern "C" fn game_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("attack_lw4_charge"), false, -1.0);
    }
    frame(lua_state, 1.0);
    FT_DESIRED_RATE(agent, 14.0, 16.0);
    if is_excute(agent) {
        MeterModule::drain(boma.object(), 2);
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("attack_lw4"), false, -1.0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 258, 90, 105, 0, 6.0, 0.0, 4.0, 14.0, Some(0.0), Some(4.0), Some(-14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 258, 60, 0, 30, 6.0, 0.0, 4.0, 14.0, Some(0.0), Some(4.0), Some(-14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        AttackModule::set_add_reaction_frame(boma, 0, 20.0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            FT_DESIRED_RATE(agent, 45.0, 25.0);
        }
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 67.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn effect_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_smash_lw_trace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 3, 13.5, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if !agent.is_situation(*SITUATION_KIND_AIR){
            EFFECT_FOLLOW(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, false);
            LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
        }
        EFFECT(agent, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 3, 8, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 3, -8, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

unsafe extern "C" fn sound_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_03"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_smash_l"));
        PLAY_SE(agent, Hash40::new("se_palutena_smash_l01"));
        PLAY_SE(agent, Hash40::new("se_common_down_soil_l"));
    }
}

unsafe extern "C" fn expression_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 4);
    }
    frame(lua_state, 14.0);
    app::sv_animcmd::execute(lua_state, 14.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        }
        else {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        }
        AREA_WIND_2ND_arg10(agent, 0, 0.75, 110, 300, 0.8, 0, 15, 24, 30, 40);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 0.75, 70, 300, 0.8, 0, 12, 24, 24, 40);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnp", game_specialnp);
    agent.acmd("game_specialairnp", game_specialnp);
    agent.acmd("effect_specialnp", effect_specialnp);
    agent.acmd("effect_specialairnp", effect_specialnp);
    agent.acmd("sound_specialnp", sound_specialnp);
    agent.acmd("sound_specialairnp", sound_specialnp);
    agent.acmd("expression_specialnp", expression_specialnp);
    agent.acmd("expression_specialairnp", expression_specialnp);
}