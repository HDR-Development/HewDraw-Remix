
use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, Hash40::new("bat").hash as i64, Hash40::new("bat_visible").hash as i64);
        FT_MOTION_RATE(agent, 11.0/(14.0-1.0));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 18.0, 361, 70, 0, 65, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 18.0, 361, 70, 0, 65, 2.3, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 20.0, 361, 70, 0, 65, 2.6, 0.0, 3.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 22.0, 361, 70, 0, 65, 3.3, 0.0, 8.2, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BAT);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_END);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, Hash40::new("bat").hash as i64, Hash40::new("bat_invisible").hash as i64);
    }
    
}

unsafe extern "C" fn expression_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 11.0);
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
            ItemModule::set_have_item_visibility(boma, false, 0);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

unsafe extern "C" fn game_attackhi4 (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 0.74);
	frame(lua_state, 3.0);
	if is_excute(agent) {
		ArticleModule::set_visibility_whole(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::set_visibility_whole(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	frame(lua_state, 11.0);
	if is_excute(agent) {
		ArticleModule::shoot(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
	}
	frame(lua_state, 12.0);
	FT_MOTION_RATE(agent, 1);
	if is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 56.0);
	if is_excute(agent) {
		ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
}

unsafe extern "C" fn game_attacklw4 (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	frame(lua_state, 3.0);
	if is_excute(agent) {
		ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	frame(lua_state, 6.0);
	if is_excute(agent) {
		ArticleModule::shoot(agent.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 54.0);
	if is_excute(agent) {
		ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
}

unsafe extern "C" fn expression_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 11.0);
    app::sv_animcmd::execute(lua_state, 11.0);
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
            RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        }
    }
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    /*frame(lua_state, 23.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }*/
    frame(lua_state, 23.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}
pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4);
    agent.acmd("expression_attackhi4", expression_attackhi4);
    agent.acmd("game_attackhi4", game_attackhi4);
    agent.acmd("game_attacklw4", game_attacklw4);
    agent.acmd("expression_attacklw4", expression_attacklw4);
}
