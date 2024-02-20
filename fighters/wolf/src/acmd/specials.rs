use super::*;

#[acmd_script( agent = "wolf", scripts = ["game_specialsend", "game_specialairsend"] , category = ACMD_GAME , low_priority)]
unsafe fn wolf_special_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 34, 85, 0, 30, 4.5, 0.0, 6.0, 6.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn wolf_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 1.0, 40, 20, 0, 60, 4.5, -2.0, 0.0, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 1.0, 367, 20, 0, 35, 4.5, -2.0, 0.0, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 1.0, 40, 20, 0, 60, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 1.0, 367, 20, 0, 40, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 1.0, 40, 20, 0, 70, 4.5, -2.0, 0.0, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 1.0, 367, 20, 0, 35, 4.5, -2.0, 0.0, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 1.0, 40, 20, 0, 70, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 1.0, 367, 20, 0, 40, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_specialhifall" , category = ACMD_GAME , low_priority)]
unsafe fn wolf_special_hi_fall_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 361, 120, 0, 55, 6.0, 4.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 361, 120, 0, 55, 6.0, -3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_WOLF_KICK);
        WorkModule::on_flag(boma, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_WOLF_ENABLE_CONTROL);
    }
}

#[acmd_script(agent = "wolf", scripts = ["effect_specialairsend", "effect_specialsend"], category = ACMD_EFFECT, low_priority)]
unsafe fn wolf_special_s_end_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("wolf_slash_scratch"), Hash40::new("top"), 5, 13, 0, -40, 0, 0, 0.45, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5.5, 5.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.8);
    }
}

#[acmd_script(agent = "wolf", scripts = ["game_specialairs", "game_specials"], category = ACMD_GAME, low_priority)]
unsafe fn wolf_special_s_game(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, 1.5);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 0.66);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_ILLUSION, false, -1);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 60, 60, 0, 68, 3.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
}

#[acmd_script(agent = "wolf_illusion", scripts = ["game_moveair", "game_moveground"], category = ACMD_GAME, low_priority)]
unsafe fn wolf_illusion_move_game(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        // macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 60, 0, 68, 3.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
}

#[acmd_script(agent = "wolf", scripts = ["effect_specialairs", "effect_specials"], category = ACMD_EFFECT, low_priority)]
unsafe fn wolf_special_s_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("wolf_slash"), Hash40::new("top"), -3, 5.5, 0, 65, 0, 0, 0.75, false);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        // macros::EFFECT_FOLLOW(fighter, Hash40::new("wolf_slash_rush"), Hash40::new("top"), -3, 20.7, 35, 65, 0, 0, 0.75, false);
    }
}

#[acmd_script(agent = "wolf", scripts = ["sound_specialairs", "sound_specials"], category = ACMD_SOUND, low_priority)]
unsafe fn wolf_special_s_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.33);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_wolf_special_s01"));
        macros::PLAY_SE(fighter, Hash40::new("se_wolf_special_s02"));
    }
}

#[acmd_script(agent = "wolf", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME, low_priority)]
unsafe fn wolf_special_air_n_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER) && macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        // macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, -3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        // macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        // macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 38.0);
    if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_n") {
        macros::FT_MOTION_RATE(fighter, 1.8);
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER) && macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "wolf", script = "effect_specialn" , category = ACMD_EFFECT , low_priority)]
unsafe fn wolf_special_n_effect (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
		AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 3, Hash40::new("haver"), 0.0, -0.3, 3.0, Hash40::new("haver"), 0.0, 0.77, 6.2, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		AFTER_IMAGE_OFF(fighter, 4);
		EFFECT_OFF_KIND(fighter, Hash40::new("wolf_bayonet"), false, false);
		EFFECT(fighter, Hash40::new("wolf_blaster_shot"), Hash40::new("top"), 0, 6.8, 13.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	frame(lua_state, 19.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
		AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 4, Hash40::new("haver"), 0.0, -0.3, 2.5, Hash40::new("haver"), 0.0, 0.77, 6.3, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		AFTER_IMAGE_OFF(fighter, 3);
		EFFECT_OFF_KIND(fighter, Hash40::new("wolf_bayonet"), false, false);
	}
}

#[acmd_script( agent = "wolf", script = "effect_specialairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn wolf_special_air_n_effect (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
		AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 3, Hash40::new("haver"), 0.0, -0.3, 3.0, Hash40::new("haver"), 0.0, 0.77, 6.2, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		AFTER_IMAGE_OFF(fighter, 4);
		EFFECT(fighter, Hash40::new("wolf_blaster_shot"), Hash40::new("top"), 0, 6.8, 13.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	frame(lua_state, 19.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
		AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 4, Hash40::new("haver"), 0.0, -0.3, 2.5, Hash40::new("haver"), 0.0, 0.77, 6.3, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		AFTER_IMAGE_OFF(fighter, 3);
		EFFECT_OFF_KIND(fighter, Hash40::new("wolf_bayonet"), false, false);
	}
}

#[acmd_script( agent = "wolf", script = "game_speciallwstart" , category = ACMD_GAME , low_priority)]
unsafe fn wolf_special_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 69, 100, 0, 70, 8.0, 0.0, 7.7, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        //Reflection begins on same frame shine hitbox is active
        ReflectorModule::set_status(boma, *FIGHTER_FOX_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.72);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_specialairlwstart" , category = ACMD_GAME , low_priority)]
unsafe fn wolf_special_air_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 69, 100, 0, 70, 8.0, 0.0, 8.5, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ReflectorModule::set_status(boma, *FIGHTER_FOX_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
        // Reflection begins on same frame shine hitbox is active
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.75);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "wolf", scripts = ["effect_speciallwstart", "effect_specialairlwstart"] , category = ACMD_EFFECT , low_priority)]
unsafe fn wolf_special_lw_start_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
    frame(lua_state, 0.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("wolf_ref_loop"), Hash40::new("top"), 0, 6.5, 0, 0, 0, 0, 1, true);
        EFFECT_FLW_POS(fighter, Hash40::new("wolf_ref_ref"), Hash40::new("top"), 0, 6.5, 0, 0, 0, 0, 0.6, true);
        EFFECT_FOLLOW(fighter, Hash40::new("wolf_ref_start"), Hash40::new("top"), 0, 6.5, 1, 0, 0, 0, 0.8, true);
    }
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		EFFECT_DETACH_KIND(fighter, Hash40::new("wolf_ref_start"), -1);
	}
	else{;
	frame(lua_state, 0.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("wolf_ref_start"), Hash40::new("top"), 0, 6.5, 0, 0, 0, 0, 1, true);
	}
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
    EFFECT_DETACH_KIND(fighter, Hash40::new("wolf_ref_start"), -1);
    }
}

#[acmd_script( agent = "wolf", scripts = ["sound_speciallwstart", "sound_specialairlwstart"] , category = ACMD_SOUND , low_priority)]
unsafe fn wolf_special_lw_start_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 0.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_wolf_special_l01"));
	}
}

#[acmd_script( agent = "wolf", script = "expression_speciallwstart" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn wolf_special_lw_start_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_shield_on"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "wolf", script = "expression_specialairlwstart" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn wolf_special_air_lw_start_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_shield_on"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        wolf_special_air_n_game,
        wolf_special_n_effect,
        wolf_special_air_n_effect,
        wolf_special_s_game,
        wolf_special_s_effect,
        wolf_special_s_sound,
        wolf_illusion_move_game,
        wolf_special_s_end_game,
        wolf_special_s_end_effect,
        wolf_special_hi_game,
        wolf_special_lw_start_game,
        wolf_special_air_lw_start_game,
        wolf_special_hi_fall_game,
        wolf_special_lw_start_effect,
        wolf_special_lw_start_sound,
        wolf_special_lw_start_expression,
        wolf_special_air_lw_start_expression,
    );
}