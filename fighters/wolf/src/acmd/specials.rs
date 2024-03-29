use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER) && is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        // ATTACK(agent, 0, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, -3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        // ATTACK(agent, 1, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 1.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        // ATTACK(agent, 2, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 5.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(lua_state, 38.0);
    if MotionModule::motion_kind(agent.module_accessor) == smash::hash40("special_air_n") {
        FT_MOTION_RATE(agent, 1.8);
    }
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER) && is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	frame(lua_state, 8.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
		AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 3, Hash40::new("haver"), 0.0, -0.3, 3.0, Hash40::new("haver"), 0.0, 0.77, 6.2, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
	}
	frame(lua_state, 14.0);
	if is_excute(agent) {
		AFTER_IMAGE_OFF(agent, 4);
		EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
		EFFECT(agent, Hash40::new("wolf_blaster_shot"), Hash40::new("top"), 0, 6.8, 13.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	frame(lua_state, 17.0);
	if is_excute(agent) {
		FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	frame(lua_state, 19.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
		AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 4, Hash40::new("haver"), 0.0, -0.3, 2.5, Hash40::new("haver"), 0.0, 0.77, 6.3, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
	}
	frame(lua_state, 22.0);
	if is_excute(agent) {
		AFTER_IMAGE_OFF(agent, 3);
		EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
	}
}

unsafe extern "C" fn effect_specialairn(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	frame(lua_state, 8.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
		AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 3, Hash40::new("haver"), 0.0, -0.3, 3.0, Hash40::new("haver"), 0.0, 0.77, 6.2, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
	}
	frame(lua_state, 14.0);
	if is_excute(agent) {
		AFTER_IMAGE_OFF(agent, 4);
		EFFECT(agent, Hash40::new("wolf_blaster_shot"), Hash40::new("top"), 0, 6.8, 13.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	frame(lua_state, 19.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
		AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 4, Hash40::new("haver"), 0.0, -0.3, 2.5, Hash40::new("haver"), 0.0, 0.77, 6.3, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
	frame(lua_state, 22.0);
	if is_excute(agent) {
		AFTER_IMAGE_OFF(agent, 3);
		EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
	}
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 1.5);
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(lua_state, 0.66);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_ILLUSION, false, -1);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 60, 60, 0, 68, 3.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wolf_slash"), Hash40::new("top"), -3, 5.5, 0, 65, 0, 0, 0.75, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        // EFFECT_FOLLOW(agent, Hash40::new("wolf_slash_rush"), Hash40::new("top"), -3, 20.7, 35, 65, 0, 0, 0.75, false);
    }
}

unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.33);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_wolf_special_s01"));
        PLAY_SE(agent, Hash40::new("se_wolf_special_s02"));
    }
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 34, 85, 0, 30, 4.5, 0.0, 6.0, 6.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wolf_slash_scratch"), Hash40::new("top"), 5, 13, 0, -40, 0, 0, 0.45, true);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5.5, 5.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.8);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 1.0, 40, 20, 0, 60, 4.5, -2.0, 0.0, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 367, 20, 0, 35, 4.5, -2.0, 0.0, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 1.0, 40, 20, 0, 60, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("kneel"), 1.0, 367, 20, 0, 40, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 1.0, 40, 20, 0, 70, 4.5, -2.0, 0.0, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 367, 20, 0, 35, 4.5, -2.0, 0.0, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 1.0, 40, 20, 0, 70, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("kneel"), 1.0, 367, 20, 0, 40, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialhifall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 6.0, 361, 120, 0, 55, 6.0, 4.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 6.0, 361, 120, 0, 55, 6.0, -3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_WOLF_KICK);
        WorkModule::on_flag(boma, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_WOLF_ENABLE_CONTROL);
    }
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 69, 100, 0, 70, 8.0, 0.0, 7.7, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        //Reflection begins on same frame shine hitbox is active
        ReflectorModule::set_status(boma, *FIGHTER_FOX_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.72);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    frame(lua_state, 0.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wolf_ref_loop"), Hash40::new("top"), 0, 6.5, 0, 0, 0, 0, 1, true);
        EFFECT_FLW_POS(agent, Hash40::new("wolf_ref_ref"), Hash40::new("top"), 0, 6.5, 0, 0, 0, 0, 0.6, true);
        EFFECT_FOLLOW(agent, Hash40::new("wolf_ref_start"), Hash40::new("top"), 0, 6.5, 1, 0, 0, 0, 0.8, true);
    }
	frame(lua_state, 3.0);
	if is_excute(agent) {
		EFFECT_DETACH_KIND(agent, Hash40::new("wolf_ref_start"), -1);
	}
	else{;
	frame(lua_state, 0.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("wolf_ref_start"), Hash40::new("top"), 0, 6.5, 0, 0, 0, 0, 1, true);
	}
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
    EFFECT_DETACH_KIND(agent, Hash40::new("wolf_ref_start"), -1);
    }
}

unsafe extern "C" fn sound_speciallwstart(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	frame(lua_state, 0.0);
	if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_wolf_special_l01"));
	}
}

unsafe extern "C" fn expression_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_shield_on"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialairlwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 69, 100, 0, 70, 8.0, 0.0, 8.5, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ReflectorModule::set_status(boma, *FIGHTER_FOX_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
        // Reflection begins on same frame shine hitbox is active
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.75);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("effect_specialn", effect_specialn);
    agent.acmd("game_specialairn", game_specialn);
    agent.acmd("effect_specialairn", effect_specialairn);
    
    agent.acmd("game_specials", game_specials);
    agent.acmd("effect_specials", effect_specials);
    agent.acmd("sound_specials", sound_specials);
    agent.acmd("game_specialairs", game_specials);
    agent.acmd("effect_specialairs", effect_specials);
    agent.acmd("sound_specialairs", sound_specials);

    agent.acmd("game_specialsend", game_specialsend);
    agent.acmd("effect_specialsend", effect_specialsend);
    agent.acmd("game_specialairsend", game_specialsend);
    agent.acmd("effect_specialairsend", effect_specialsend);
    
    agent.acmd("game_specialhi", game_specialhi);

    agent.acmd("game_specialhifall", game_specialhifall);

    agent.acmd("game_speciallwstart", game_speciallwstart);
    agent.acmd("effect_speciallwstart", effect_speciallwstart);
    agent.acmd("sound_speciallwstart", sound_speciallwstart);
    agent.acmd("expression_speciallwstart", expression_speciallwstart);
    agent.acmd("game_specialairlwstart", game_specialairlwstart);
    agent.acmd("effect_specialairlwstart", effect_speciallwstart);
    agent.acmd("sound_specialairlwstart", sound_speciallwstart);
    agent.acmd("expression_specialairlwstart", expression_speciallwstart); 
}
