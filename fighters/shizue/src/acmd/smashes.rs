use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER, smash::phx::Hash40::new("fire"), false, 0.0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		if WorkModule::is_flag(boma,  *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
			ArticleModule::change_motion(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER, smash::phx::Hash40::new("fire"), true, WorkModule::get_float(boma, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME) );
		}
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
		ArticleModule::change_motion(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER, smash::phx::Hash40::new("fire"), false, 13.0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 361, 108, 0, 25, 5.5, 0.0, 5.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 17.0, 361, 108, 0, 25, 7.0, 0.0, 7.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 113, 0, 40, 5.5, 0.0, 5.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 361, 113, 0, 40, 7.0, 0.0, 7.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        ArticleModule::remove(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.8);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_TRAFFICSIGN, true, 0);
        ArticleModule::change_motion(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_TRAFFICSIGN, Hash40::new("attack"), true, 0.0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 4.5, 0.0, 4.0, 6.5, 0.0, 5.0, 6.5, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 100, 100, 70, 0, 4.5, 0.0, 4.0, 6.5, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 100, 100, 60, 0, 4.5, 0.0, 4.0, 6.5, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 4.5, 0.0, 4.0, 6.5, 0.0, 10.0, 6.5, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 4.5, 0.0, 4.0, 6.5, 0.0, 18.0, 6.5, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.6);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BUCKET, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BUCKET, smash::phx::Hash40::new("attack"), false, 0.0);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		if WorkModule::is_flag(boma,  *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
			ArticleModule::change_motion(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BUCKET, smash::phx::Hash40::new("attack"), true, WorkModule::get_float(boma, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME) );
		}
    }
	frame(lua_state, 5.0);
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BUCKET, smash::phx::Hash40::new("attack"), false, 5.0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 28, 99, 0, 35, 5.0, 0.0, 3.6, 13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 28, 99, 0, 35, 4.5, 0.0, 3.6, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 28, 114, 0, 40, 5.0, 0.0, 3.6, -13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 28, 114, 0, 40, 4.5, 0.0, 3.6, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        ArticleModule::remove(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BUCKET, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);

    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);

    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
}
