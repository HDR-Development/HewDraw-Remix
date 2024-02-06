
use super::*;



unsafe extern "C" fn game_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("bat").hash as i64, Hash40::new("bat_visible").hash as i64);
        FT_MOTION_RATE(fighter, 11.0/(14.0-1.0));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("bust"), 18.0, 361, 70, 0, 65, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 18.0, 361, 70, 0, 65, 2.3, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 20.0, 361, 70, 0, 65, 2.6, 0.0, 3.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 22.0, 361, 70, 0, 65, 3.3, 0.0, 8.2, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BAT);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_END);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("bat").hash as i64, Hash40::new("bat_invisible").hash as i64);
    }
    
}


unsafe extern "C" fn expression_attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 11.0);
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
            ItemModule::set_have_item_visibility(boma, false, 0);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
            RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}


unsafe extern "C" fn game_attackhi4 (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(fighter, 0.74);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		ArticleModule::set_visibility_whole(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::set_visibility_whole(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		ArticleModule::shoot(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
	}
	frame(lua_state, 12.0);
	FT_MOTION_RATE(fighter, 1);
	if is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 56.0);
	if is_excute(fighter) {
		ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
}


unsafe extern "C" fn game_attacklw4 (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ArticleModule::shoot(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 54.0);
	if is_excute(fighter) {
		ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
}



unsafe extern "C" fn expression_attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 11.0);
    app::sv_animcmd::execute(lua_state, 11.0);
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
        }
    }
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    /*frame(lua_state, 23.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }*/
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}


unsafe extern "C" fn game_yoyo_attackhi4 (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(fighter, 0.74);
	frame(lua_state, 11.0);
	FT_MOTION_RATE(fighter, 1);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("attach"), 1.0, 90, 100, 30, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
	}
    frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("attach"), 13.0, 80, 79, 0, 70, 4.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(lua_state, 37.0);
}


unsafe extern "C" fn game_yoyo_attacklw4 (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("attach"), 1.0, 367, 100, 12, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("attach"), 10.0, 28, 80, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	// frame(lua_state, 18.0);
	// if is_excute(fighter) {
	// 	AttackModule::clear_all(fighter.module_accessor);
	// }
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("attach"), 10.0, 33, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	// frame(lua_state, 25.0);
	// if is_excute(fighter) {
	// 	ATTACK(fighter, 0, 0, Hash40::new("attach"), 1.0, 35, 100, 30, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
	// 	AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	// 	AttackModule::clear_all(fighter.module_accessor);
	// }
	// frame(lua_state, 30.0);
	// if is_excute(fighter) {
	// 	ATTACK(fighter, 0, 0, Hash40::new("attach"), 10.0, 33, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
	// 	AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	// }
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(lua_state, 40.0);
}




pub fn install() {
    smashline::Agent::new("ness_yoyohead")
        .acmd("game_attackhi4", game_yoyo_attackhi4)
        .acmd("game_attacklw4", game_yoyo_attacklw4)
        .install();
    smashline::Agent::new("ness")
        .acmd("game_attacks4", game_attacks4)
        .acmd("expression_attackhi4", expression_attackhi4)
        .acmd("game_attackhi4", game_attackhi4)
        .acmd("game_attacklw4", game_attacklw4)
        .acmd("expression_attacklw4", expression_attacklw4)
        .install();
}
