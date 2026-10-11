use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 1.2);
	frame(lua_state, 5.0);
	if is_excute(agent) {
	}
	frame(lua_state, 6.0);
	FT_MOTION_RATE(agent, 1.0);
	if is_excute(agent) {
		CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 6.6, 6.0, Some(0.0), Some(6.6), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(agent);
	wait(lua_state, 2.0);
	if is_excute(agent) {
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
	}
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 8.0);
	if is_excute(agent) {
	}
	frame(lua_state, 9.0);
	if is_excute(agent) {
		CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 6.7, 5.0, Some(0.0), Some(6.7), Some(10.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(agent);
	wait(lua_state, 2.0);
	if is_excute(agent) {
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
	}
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 9.0);
	if is_excute(agent) {
	}
	frame(lua_state, 10.0);
	if is_excute(agent) {
		CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 6.4, -4.0, Some(0.0), Some(6.4), Some(-14.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(agent);
	wait(lua_state, 2.0);
	if is_excute(agent) {
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
	}
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 48, 90, 0, 48, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 10.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 48, 100, 140, 10, 5.5, 0.0, 6.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_catch_only_all(boma, true, false);
	}
	frame(lua_state, 11.0);
	if is_excute(agent) {
		CHECK_FINISH_CAMERA(agent, 13.0, 0.0);
	}
	frame(lua_state, 12.0);
	FT_MOTION_RATE_RANGE(agent, 12.0, 34.0, 18.0);
	if is_excute(agent) {
		ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 34.0);
	FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 60, 40, 0, 45, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 3.0);
	FT_MOTION_RATE(agent, 1.0);
	frame(lua_state, 10.0);
	FT_MOTION_RATE_RANGE(agent, 10.0, 20.0, 5.0);
	if is_excute(agent) {
		REVERSE_LR(agent);
		ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	frame(lua_state, 20.0);
	FT_MOTION_RATE_RANGE(agent, 20.0, 50.0, 18.0);
	if is_excute(agent) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, false, -1);
		ArticleModule::change_motion(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, Hash40::new("fly_throw_b"), false, -1.0);
	}
	frame(lua_state, 50.0);
	FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 13.0);
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	frame(lua_state, 20.0);
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("miigunner_shot_add"), Hash40::new("armr"), 5.3, 0, 0, 70, 0, 90, 0.7, 0, 0, 0, 0, 0, 0, true);
		EFFECT(agent, Hash40::new("miigunner_blaster_shot"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.7, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 1.6);
	}
}

unsafe extern "C" fn sound_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 2.0);
	if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_common_throw_01"));
	}
	wait(lua_state, 8.0);
	if is_excute(agent) {
		PLAY_SEQUENCE(agent, Hash40::new("seq_miigunner_rnd_attack01"));
	}
	wait(lua_state, 2.0);
	if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_common_throw_02"));
	}
	frame(lua_state, 20.0);
	if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_miigunner_special_c2_n01"));
	}
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 90, 100, 40, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 7.0);
	FT_MOTION_RATE(agent, 1.0);
	if is_excute(agent) {
		ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	frame(lua_state, 18.0);
	if is_excute(agent) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, false, 0);
		ArticleModule::change_motion(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, Hash40::new("fly_throw_hi"), false, 0.0);
	}
	frame(lua_state, 20.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	FT_MOTION_RATE_RANGE(agent, 28.0, 52.0, 17.0);
	if is_excute(agent) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, false, 0);
		ArticleModule::change_motion(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, Hash40::new("fly_throw_hi_2"), false, 0.0);
	}
	frame(lua_state, 52.0);
	FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
		EFFECT(agent, Hash40::new("miigunner_blaster_shot"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.7, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("miigunner_shot_add"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.75);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
		EFFECT(agent, Hash40::new("miigunner_blaster_shot"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.7, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 1.6);
		EFFECT(agent, Hash40::new("miigunner_shot_add"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_miigunner_rnd_attack01"));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_miigunner_special_c2_n01"));
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_miigunner_special_c2_n01"));
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 102, 55, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
	frame(lua_state, 10.0);
	FT_MOTION_RATE_RANGE(agent, 10.0, 15.0, 8.0);
    frame(lua_state, 15.0);
	FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 0, 0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

pub fn install(agent: &mut Agent) {
	agent.acmd("game_catch", game_catch, Priority::Low);
	agent.acmd("game_catchdash", game_catchdash, Priority::Low);
	agent.acmd("game_catchturn", game_catchturn, Priority::Low);

    agent.acmd("game_throwf", game_throwf, Priority::Low);

    agent.acmd("game_throwb", game_throwb, Priority::Low);
    agent.acmd("effect_throwb", effect_throwb, Priority::Low);
    agent.acmd("sound_throwb", sound_throwb, Priority::Low);

    agent.acmd("game_throwhi", game_throwhi, Priority::Low);
    agent.acmd("effect_throwhi", effect_throwhi, Priority::Low);
    agent.acmd("sound_throwhi", sound_throwhi, Priority::Low);

    agent.acmd("game_throwlw", game_throwlw, Priority::Low);
}