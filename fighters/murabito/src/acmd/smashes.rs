use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ArticleModule::generate_article_enable(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_BOWLING_BALL, false, 0);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 60.0, 29.0);
    if is_excute(agent) {
        ArticleModule::shoot(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_BOWLING_BALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 60.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_FIREWORK, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 75, 100, 120, 0, 4.0, 0.0, 5.0, -3.5, Some(0.0), Some(5.0), Some(-0.5), 0.5, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 110, 100, 120, 0, 6.0, 0.0, 5.0, 6.0, None, None, None, 0.5, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ArticleModule::shoot(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_FIREWORK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL);
        VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X, 0.0);
        VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y, 0.0);
        VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z, 0.0);
        if (ArticleModule::is_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT)) {
            // Find the sapling's BOMA
            let sprout_article = ArticleModule::get_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT);
            let object_id = smash::app::lua_bind::Article::get_battle_object_id(sprout_article) as u32;
            let article_boma = sv_battle_object::module_accessor(object_id);
            let sprout_pos = *PostureModule::pos(article_boma); // stage pos of the sapling
            let char_pos = *PostureModule::pos(boma);           // stage pos of villager
            let offset = Vector3f::new(11.0 * PostureModule::lr(boma), 0.0, 0.0);    // offest, in case we want to move the zone
            // Check if the sapling is in range
            if ((sprout_pos.x - (char_pos.x + offset.x)).abs() < 8.0 && (sprout_pos.y - (char_pos.y + offset.y)).abs() < 4.5) {
                VarModule::on_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL);
                VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X, sprout_pos.x);
                VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y, sprout_pos.y);
                VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z, sprout_pos.z);
            }
        }
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
            ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 90, 82, 0, 55, 5.5, pos_z, pos_y + 3.5, pos_x - 5.0, Some(pos_z), Some(pos_y + 3.5), Some(pos_x + 5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        } else {
            ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 40, 98, 0, 40, 6.0, 0.0, 3.0, 13.0, Some(0.0), Some(3.0), Some(9.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL);
        VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X, 0.0);
        VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y, 0.0);
        VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z, 0.0);
        if (ArticleModule::is_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT)) {
            // Find the sapling's BOMA
            let sprout_article = ArticleModule::get_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT);
            let object_id = smash::app::lua_bind::Article::get_battle_object_id(sprout_article) as u32;
            let article_boma = sv_battle_object::module_accessor(object_id);
            let sprout_pos = *PostureModule::pos(article_boma); // stage pos of the sapling
            let char_pos = *PostureModule::pos(boma);           // stage pos of villager
            let offset = Vector3f::new(-11.0 * PostureModule::lr(boma), 0.0, 0.0);    // offest, in case we want to move the zone
            // Check if the sapling is in range
            if ((sprout_pos.x - (char_pos.x + offset.x)).abs() < 8.0 && (sprout_pos.y - (char_pos.y + offset.y)).abs() < 4.5) {
                VarModule::on_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL);
                VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X, sprout_pos.x);
                VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y, sprout_pos.y);
                VarModule::set_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z, sprout_pos.z);
            }
        }
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        // If the flag is set, use special hitboxes
        if VarModule::is_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL) {
            ArticleModule::remove(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            let char_pos = *PostureModule::pos(boma);   // stage pos of villager
            let pos_x = PostureModule::lr(boma) * (VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X) - char_pos.x);
            let pos_y = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y) - char_pos.y;
            let pos_z = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z) - char_pos.z;
            ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 90, 82, 0, 55, 5.5, pos_z, pos_y + 3.5, pos_x - 5.0, Some(pos_z), Some(pos_y + 3.5), Some(pos_x + 5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        } else {
            ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 40, 98, 0, 40, 6.0, 0.0, 3.0, -13.0, Some(0.0), Some(3.0), Some(-9.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }        
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.0, 0.0, 15.0, 0, 0, 0, 1.0, 4, 4, 4, 0, 0, 0, true);
    }

    frame(lua_state, 6.0);
    if is_excute(agent) {
        // If the flag is set, use different effects
        if VarModule::is_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL) {
            let char_pos = *PostureModule::pos(boma);   // stage pos of villager
            let pos_x = PostureModule::lr(boma) * (VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X) - char_pos.x);
            let pos_y = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y) - char_pos.y;
            let pos_z = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z) - char_pos.z;
            EFFECT(agent, Hash40::new("murabito_soil2"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 1.0, 4, 4, 4, 0, 0, 0, false);
        } else {
            EFFECT(agent, Hash40::new("murabito_soil2"), Hash40::new("top"), 15.0, 0.0, 0.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
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
            EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);      
            LAST_EFFECT_SET_RATE(agent, 2.0);   
            EFFECT(agent, Hash40::new("murabito_putaway_catch"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.65);    
        } else {
            LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 15.0, 0.0, 0.0, 0, 0, 0, 0.55, 2, 2, 2, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("murabito_clay"), Hash40::new("top"), 15.0, 0.0, 0.0, 0, 0, 0, 1.0, 2, 2, 2, 0, 0, 0, false);
        }
    }

    frame(lua_state, 26.0);
    if is_excute(agent) {
        // If the flag is set, use different effects
        if VarModule::is_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL) {
            let char_pos = *PostureModule::pos(boma);   // stage pos of villager
            let pos_x = PostureModule::lr(boma) * (VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_X) - char_pos.x);
            let pos_y = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Y) - char_pos.y;
            let pos_z = VarModule::get_float(boma.object(), vars::murabito::instance::SAPLING_PULL_SAPLING_POS_Z) - char_pos.z;
            EFFECT(agent, Hash40::new("murabito_soil2"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 1.0, 4, 4, 4, 0, 0, 0, false);
        } else {
            EFFECT(agent, Hash40::new("murabito_soil2"), Hash40::new("top"), -15.0, 0.0, 0.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }

    frame(lua_state, 29.0);
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
            EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 2.0);   
            EFFECT(agent, Hash40::new("murabito_putaway_catch"), Hash40::new("top"), pos_x, pos_y, pos_z, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.65);    
        } else {
            LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -15.0, 0.0, 0.0, 0, 0, 0, 0.55, 2, 2, 2, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("murabito_clay"), Hash40::new("top"), -15.0, 0.0, 0.0, 0, 0, 0, 1.0, 2, 2, 2, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 2.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }

    frame(lua_state, 7.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL) {
            PLAY_SE(agent, Hash40::new("se_murabito_special_s04"));
        } else {
            PLAY_SE(agent, Hash40::new("se_murabito_smash_l02"));
        }
    }

    frame(lua_state, 27.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::murabito::instance::IS_TILT_LW_SAPLING_PULL) {
            PLAY_SE(agent, Hash40::new("se_murabito_special_s04"));
        } else {
            PLAY_SE(agent, Hash40::new("se_murabito_smash_l02"));
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);

    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    
    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
    agent.acmd("effect_attacklw4", effect_attacklw4, Priority::Low);
    agent.acmd("sound_attacklw4", sound_attacklw4, Priority::Low);
}
