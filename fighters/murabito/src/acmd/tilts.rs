
use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 8.0, 361, 82, 0, 31, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 361, 82, 0, 31, 2.5, 0.0, 2.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 36, 82, 0, 31, 4.5, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 12.0, 36, 82, 0, 31, 2.5, -5.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 4, 0, Hash40::new("haver"), 12.0, 36, 82, 0, 31, 2.5, 5.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(agent, 0.877);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.000);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        ArticleModule::remove(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        FT_MOTION_RATE(agent, 0.850);
        ATTACK(agent, 0, 0, Hash40::new("havel"), 5.0, 108, 100, 75, 0, 3.8, 0.0, -1.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("havel"), 5.0, 108, 100, 75, 0, 4.0, 0.0, 5.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("havel"), 5.0, 135, 10, 0, 35, 4.0, 0.0, 5.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 2.0, false);
    }
    wait(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.000);
        ATTACK(agent, 0, 0, Hash40::new("havel"), 7.0, 80, 155, 0, 50, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("havel"), 7.0, 80, 155, 0, 50, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
    }
    
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        // START DTilt Sapling Removal Logic...
        VarModule::off_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL);
        VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X, 0.0);
        VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y, 0.0);
        VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z, 0.0);
        // If a sapling exists
        if (ArticleModule::is_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT)) {
            // Find the sapling's BOMA
            let sprout_article = ArticleModule::get_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT);
            let object_id = smash::app::lua_bind::Article::get_battle_object_id(sprout_article) as u32;
            let article_boma = sv_battle_object::module_accessor(object_id);
            let sprout_pos = *PostureModule::pos(article_boma); // stage pos of the sapling
            let char_pos = *PostureModule::pos(boma);           // stage pos of villager
            let offset = Vector3f::new(7.0 * PostureModule::lr(boma), 0.0, 0.0);    // offest, in case we want to move the zone
            // Check if the sapling is in range
            if ((sprout_pos.x - (char_pos.x + offset.x)).abs() < 8.0 && (sprout_pos.y - (char_pos.y + offset.y)).abs() < 4.5) {
                VarModule::on_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL);
                VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X, sprout_pos.x);
                VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y, sprout_pos.y);
                VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z, sprout_pos.z);
            }
        }
        // ... END Dtilt Sapling Removal Logic

        FT_MOTION_RATE(agent, 0.746);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 4.0);
        // If the flag is not set, generate weeds
        if !VarModule::is_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL) {
            ArticleModule::generate_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_WEEDS, true, 0);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.000);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        // If the flag is set, use special hitboxes
        if VarModule::is_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL) {
            ArticleModule::remove(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            let char_pos = *PostureModule::pos(boma);   // stage pos of villager
            let pos_x = PostureModule::lr(boma) * (VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X) - char_pos.x);
            let pos_y = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y) - char_pos.y;
            let pos_z = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z) - char_pos.z;
            ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 90, 92, 0, 68, 5.5, pos_z, pos_y + 3.5, pos_x - 5.0, Some(pos_z), Some(pos_y + 3.5), Some(pos_x + 5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        } else {
            ArticleModule::remove(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_WEEDS, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 78, 70, 0, 60, 5.0, 0.0, 3.0, 2.0, Some(0.0), Some(3.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 89, 70, 0, 60, 5.0, 0.0, 3.0, 10.0, Some(0.0), Some(3.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.8, 3.0);
    }
    
}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 2.0);
    if is_excute(agent) {
        // If the flag is set, use different effects
        if VarModule::is_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL) {
            let char_pos = *PostureModule::pos(boma);   // stage pos of villager
            let pos_x = PostureModule::lr(boma) * (VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X) - char_pos.x);
            let pos_y = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y) - char_pos.y;
            let pos_z = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z) - char_pos.z;
            EFFECT(agent, Hash40::new("murabito_soil"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 1.0, 4, 4, 4, 0, 0, 0, false);
        } else {
            EFFECT(agent, Hash40::new("murabito_soil"), Hash40::new("top"), 7.0, 0.0, 0.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }

    frame(lua_state, 9.0);
    if is_excute(agent) {
        // If the flag is set, use different effects
        if VarModule::is_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL) {
            // generate pluck effects as pos of sapling
            let char_pos = *PostureModule::pos(boma);   // stage pos of villager
            let pos_x = PostureModule::lr(boma) * (VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X) - char_pos.x);
            let pos_y = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y) - char_pos.y;
            let pos_z = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z) - char_pos.z;
            LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("murabito_grass"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);    
            LAST_EFFECT_SET_RATE(agent, 2.0);   
            EFFECT(agent, Hash40::new("murabito_putaway_catch"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.65);    
        } else {
            LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 1.0, 0.5, 0.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("murabito_grass"), Hash40::new("top"), 1.0, 0, 0.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL) {
            PLAY_SE(agent, Hash40::new("se_murabito_special_s04"));
        } else {
            PLAY_SE(agent, Hash40::new("se_murabito_attackhard_l01"));
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3);
    agent.acmd("game_attackhi3", game_attackhi3);
    agent.acmd("game_attacklw3", game_attacklw3);
    agent.acmd("effect_attacklw3", effect_attacklw3);
    agent.acmd("sound_attacklw3", sound_attacklw3);
}
