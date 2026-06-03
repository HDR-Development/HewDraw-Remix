use super::*;

unsafe fn manage_sword_motion(agent: &mut L2CAgentBase, motion: Hash40) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    let exists = {
        agent.clear_lua_stack();
        lua_args!(agent, FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD);
        app::sv_animcmd::IS_EXIST_ARTICLE(lua_state)
    };

    if !exists {
        return;
    }

    if is_excute(agent) {
        ArticleModule::add_motion_partial(
            boma,
            *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD,
            *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE,
            motion,
            5.0,
            5.0,
            false,
            false,
            0.0,
            false,
            true,
            false
        );
    }

    if MotionModule::is_changing(boma) && is_excute(agent) {
        agent.on_flag(*FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
    }
}

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if ArticleModule::is_exist(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
		if is_excute(agent) {
			ArticleModule::add_motion_partial(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
		}
	}
    if MotionModule::is_changing(boma){
        WorkModule::on_flag(boma, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
    }
	frame(lua_state, 5.0);
	let sword_length = 16 +  (WorkModule::get_int(boma, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_HOLD_FRAME))*14/60;
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 40, 0, 6.0, 0.0, 9.0, 8.0, Some(0.0), Some(9.0), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 10, 0, 5.0, 0.0, 10.0, 8.0, Some(0.0), Some(10.0), Some(16.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 90, 100, 15, 0, 5.0, 0.0, 4.0, 8.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 7.0, 8.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 110, 100, 20, 10, 5.0, 0.0, 4.0, 16.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 7.0, 16.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
	}
	frame(lua_state, 7.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK1);
	}
	frame(lua_state, 16.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 40, 0, 6.0, 0.0, 9.0, 8.0, Some(0.0), Some(9.0), Some(sword_length as f32), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 90, 100, 20, 10, 4.0, 0.0, 1.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 90, 100, 20, 10, 3.5, 0.0, 6.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 95, 100, 20, 10, 2.8, 0.0, 11.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 95, 100, 20, 10, 2.8, 0.0, 14.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 100, 100, 20, 10, 6.0, 0.0, 6.0, 15.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 90, 100, 20, 10, 6.0, 0.0, 6.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
	}
	frame(lua_state, 18.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 26.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(sword_length as f32), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{;
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 4.0, 0.0, 2.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 3.5, 0.0, 7.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 366, 100, 10, 0, 2.8, 0.0, 14.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(16.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
	}
	frame(lua_state, 28.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 36.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 42, 84, 0, 75, 7.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(sword_length as f32), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{;
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 42, 84, 0, 75, 4.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 6.0, 42, 84, 0, 75, 3.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 6.0, 42, 84, 0, 75, 2.8, 0.0, 15.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 6.0, 42, 84, 0, 75, 2.8, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 6.0, 42, 84, 0, 75, 7.0, 0.0, 11.0, 10.0, Some(0.0), Some(11.0), Some(17.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
	}
	frame(lua_state, 38.0);
	if is_excute(agent) {
		AttackModule::clear(boma, /*ID*/ 0, false);
		AttackModule::clear(boma, /*ID*/ 1, false);
		AttackModule::clear(boma, /*ID*/ 2, false);
		AttackModule::clear(boma, /*ID*/ 3, false);
	}
	frame(lua_state, 38.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK2);
	}
	frame(lua_state, 42.0);
	FT_MOTION_RATE(agent, 0.5);
	frame(lua_state, 46.0);
	if ArticleModule::is_exist(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD){
		if is_excute(agent) {
			ArticleModule::add_motion_partial(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false);
		}
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
	}
	frame(lua_state, 52.0);
	FT_MOTION_RATE(agent, 1);
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::set_float(boma, 17.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT1_POS_X_MIN);
        WorkModule::set_float(boma, 30.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT1_POS_X_MAX);
        WorkModule::set_float(boma, 1.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT2_POS_X_MIN);
        WorkModule::set_float(boma, 12.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT2_POS_X_MAX);
        EFFECT_DETACH_KIND(agent, Hash40::new("elight_buster_flash"), -1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword2"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1.08, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_linear"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        
        agent.clear_lua_stack();
        lua_args!(agent, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_EFFECT_ID);
        sv_animcmd::LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);

        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_wind"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_wind"), false, true);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_linear"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword2"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

unsafe extern "C" fn effect_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::set_float(boma, 17.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT1_POS_X_MIN);
        WorkModule::set_float(boma, 30.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT1_POS_X_MAX);
        WorkModule::set_float(boma, 1.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT2_POS_X_MIN);
        WorkModule::set_float(boma, 12.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT2_POS_X_MAX);
        EFFECT_DETACH_KIND(agent, Hash40::new("elight_buster_flash"), -1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword2"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1.08, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_linear"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        
        agent.clear_lua_stack();
        lua_args!(agent, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_EFFECT_ID);
        sv_animcmd::LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
        
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_wind"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_wind"), false, true);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_linear"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword2"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

unsafe extern "C" fn game_specialn2(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if ArticleModule::is_exist(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD){
		if is_excute(agent) {
			ArticleModule::add_motion_partial(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
		}
	}
    if MotionModule::is_changing(boma){
        WorkModule::on_flag(boma, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
    }
	frame(lua_state, 5.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 40, 0, 7.0, 0.0, 10.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 7.0, 0.0, 10.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 110, 100, 40, 10, 7.0, 0.0, 10.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 7.0, 0.0, 10.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 40, 0, 7.0, 0.0, 8.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 7.0, 0.0, 8.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 110, 100, 40, 10, 7.0, 0.0, 8.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 7.0, 0.0, 8.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
	}
	frame(lua_state, 6.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 9.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK1);
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
		WHOLE_HIT(agent, *HIT_STATUS_XLU);
	}
	frame(lua_state, 16.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 15, 100, 40, 15, 4.0, 1.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 15, 100, 40, 15, 3.5, 8.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 100, 100, 40, 15, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 50, 10, 2.8, 14.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 11.0, 12.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 11.0, 17.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 6, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 11.0, 22.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 7, 0, Hash40::new("top"), 2.0, 15, 100, 40, 15, 4.0, 0.0, 11.0, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 15, 100, 40, 15, 4.0, 0.0, 1.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 15, 100, 40, 15, 3.5, 0.0, 8.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 100, 100, 40, 15, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 366, 100, 40, 10, 2.8, 0.0, 14.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 7.0, 12.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 7.0, 17.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 6, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 7.0, 22.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 7, 0, Hash40::new("top"), 2.0, 15, 100, 40, 15, 6.0, 0.0, 7.0, 22.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 5, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 6, 2.0);
	}
	frame(lua_state, 17.0);
	if is_excute(agent) {
		WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
	}
	frame(lua_state, 18.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 26.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 40, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 40, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 40, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 40, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 13.0, 14.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 13.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 90, 100, 40, 0, 4.0, 0.0, 2.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 90, 100, 40, 0, 3.5, 0.0, 7.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 90, 100, 40, 0, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 366, 100, 40, 0, 2.8, 0.0, 15.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 8.0, 14.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 8.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 5, 2.0);
	}
	frame(lua_state, 28.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 35.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 30, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 30, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 30, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 90, 100, 30, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 5.0, 0.0, 13.0, 14.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 5.0, 0.0, 13.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 90, 100, 30, 0, 4.0, 0.0, 2.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 90, 100, 30, 0, 3.5, 0.0, 7.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 90, 100, 30, 0, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 90, 100, 30, 0, 2.8, 0.0, 15.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 5.0, 0.0, 6.0, 14.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 5.0, 0.0, 6.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 5, 2.0);
	}
	frame(lua_state, 37.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 45.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 4.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 3.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 2.8, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 2.8, 15.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 8.5, 42, 78, 0, 55, 7.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(26.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 8.5, 42, 78, 0, 55, 4.0, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 8.5, 42, 78, 0, 55, 3.5, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 8.5, 42, 78, 0, 55, 2.8, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 8.5, 42, 78, 0, 55, 2.8, 0.0, 15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 8.5, 42, 78, 0, 55, 7.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(26.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
	}
	frame(lua_state, 46.0);
	if is_excute(agent) {
		AttackModule::clear(boma, /*ID*/ 0, false);
		AttackModule::clear(boma, /*ID*/ 1, false);
		AttackModule::clear(boma, /*ID*/ 2, false);
		AttackModule::clear(boma, /*ID*/ 3, false);
	}
	frame(lua_state, 47.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK2);
	}
	frame(lua_state, 52.0);
	if ArticleModule::is_exist(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD){
		if is_excute(agent) {
			ArticleModule::add_motion_partial(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false);
		}
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
	}
}

unsafe extern "C" fn effect_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::set_float(boma, 15.5, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT1_POS_X_MIN);
        WorkModule::set_float(boma, 30.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT1_POS_X_MAX);
        WorkModule::set_float(boma, 1.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT2_POS_X_MIN);
        WorkModule::set_float(boma, 12.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT2_POS_X_MAX);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword2"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_max"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 7, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 23.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 4, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 10, 20, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_max"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_linear"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        
        agent.clear_lua_stack();
        lua_args!(agent, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_EFFECT_ID);
        sv_animcmd::LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);

        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 8, 21, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_wind"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 11, 22, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 11, 22, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 10, 23, 0, 0, 0, 0.9, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 10, 23, 0, 0, 0, 1.3, true);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 5);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_linear"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword2"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

// for SOME reason, full charge aerial neutral b would crash if i did the above script as a multiscript install
// so we need to reimplement the aerial version too /shrug

unsafe extern "C" fn game_specialairn2(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if ArticleModule::is_exist(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD){
		if is_excute(agent) {
			ArticleModule::add_motion_partial(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
		}
	}
    if MotionModule::is_changing(boma){
        WorkModule::on_flag(boma, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
    }
	frame(lua_state, 5.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 50, 10, 7.0, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(20.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 90, 100, 20, 0, 2.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(20.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 50, 10, 7.0, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(20.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 90, 100, 20, 0, 2.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(20.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
	}
	frame(lua_state, 6.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 9.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK1);
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
		WHOLE_HIT(agent, *HIT_STATUS_XLU);
	}
	frame(lua_state, 16.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 0, 4.0, 1.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 0, 3.5, 8.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 0, 2.8, 14.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 90, 100, 30, 10, 5.0, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(24.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 80, 10, 5.0, 0.0, 13.0, 8.0, Some(0.0), Some(13.0), Some(24.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 366, 100, 80, 0, 4.0, 0.0, 1.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 366, 100, 80, 0, 3.5, 0.0, 8.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 366, 100, 80, 0, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 366, 100, 80, 0, 2.8, 0.0, 14.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 90, 100, 30, 10, 5.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(24.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 80, 10, 5.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(24.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
	}
	frame(lua_state, 17.0);
	if is_excute(agent) {
		WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
	}
	frame(lua_state, 18.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 26.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 80, 0, 5.0, 0.0, 11.0, 10.0, Some(0.0), Some(11.0), Some(22.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 90, 100, 20, 0, 4.0, 0.0, 2.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 90, 100, 20, 0, 3.5, 0.0, 7.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 90, 100, 20, 0, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 90, 100, 20, 0, 2.8, 0.0, 15.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 80, 0, 5.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(22.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
	}
	frame(lua_state, 28.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 35.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 80, 0, 5.0, 0.0, 11.0, 10.0, Some(0.0), Some(11.0), Some(22.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 90, 100, 20, 0, 4.0, 0.0, 2.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 90, 100, 20, 0, 3.5, 0.0, 7.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 90, 100, 20, 0, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 90, 100, 20, 0, 2.8, 0.0, 15.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 80, 0, 5.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(22.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(boma, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
	}
	frame(lua_state, 37.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 45.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 4.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 3.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 2.8, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 2.8, 15.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 8.5, 42, 78, 0, 55, 7.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(26.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 8.5, 42, 78, 0, 55, 4.0, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 8.5, 42, 78, 0, 55, 3.5, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 8.5, 42, 78, 0, 55, 2.8, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 8.5, 42, 78, 0, 55, 2.8, 0.0, 15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 8.5, 42, 78, 0, 55, 7.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(26.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
	}
	frame(lua_state, 46.0);
	if is_excute(agent) {
		AttackModule::clear(boma, /*ID*/ 0, false);
		AttackModule::clear(boma, /*ID*/ 1, false);
		AttackModule::clear(boma, /*ID*/ 2, false);
		AttackModule::clear(boma, /*ID*/ 3, false);
	}
	frame(lua_state, 47.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK2);
	}
	frame(lua_state, 52.0);
	if ArticleModule::is_exist(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD){
		if is_excute(agent) {
			ArticleModule::add_motion_partial(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false);
		}
        if MotionModule::is_changing(boma){
            WorkModule::on_flag(boma, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
	}
	frame(lua_state, 54.0);
	if is_excute(agent) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL);
	}
}

unsafe extern "C" fn effect_specialairn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::set_float(boma, 15.5, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT1_POS_X_MIN);
        WorkModule::set_float(boma, 30.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT1_POS_X_MAX);
        WorkModule::set_float(boma, 1.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT2_POS_X_MIN);
        WorkModule::set_float(boma, 12.0, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT2_POS_X_MAX);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword2"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_max"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 7, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 23.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 4, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 10, 20, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_max"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_linear"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        
        agent.clear_lua_stack();
        lua_args!(agent, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_EFFECT_ID);
        sv_animcmd::LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);

        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 8, 21, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_wind"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 11, 22, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 11, 22, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword5"), 4, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 18.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_light"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 10, 23, 0, 0, 0, 0.9, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_buster_sword_lensflare"), Hash40::new("top"), 0, 10, 23, 0, 0, 0, 1.3, true);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_light"), false, true);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 5);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword_linear"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_buster_sword2"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
	let boma = agent.boma();
	let lua_state = agent.lua_state_agent;
	FT_MOTION_RATE(agent, 1.0 / 10.0);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
		ArticleModule::generate_article(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_BUNSHIN, false, -1);
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("rot"), 6.0, 80, 100, 70, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 10.0);
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
        // ArticleModule::remove_exist(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_BUNSHIN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
    if IS_EXIST_ARTICLE(agent, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if is_excute(agent) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(boma) {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 3.0);
	if is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
    frame(lua_state, 20.0);
    if IS_EXIST_ARTICLE(agent, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if is_excute(agent) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(boma) {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
	if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
	}
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_buster"), Hash40::new("tex_elight_sword2"), 5, Hash40::new("sword1"), 0.0, 0.0, -0.08, Hash40::new("sword1"), 15.5, 0.0, -0.08, false, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn game_specialairhijump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_NONE);
        let mut angle = app::lua_bind::FighterKineticEnergyMotion::get_angle(KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as _);
        if PostureModule::lr(boma) > 0.0 {
            angle *= -1.0;
        }
        angle = 80.0 - angle.to_degrees() / 2.0;
        if angle < 0.0 {
            angle += 360.0;
        }
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 7.0, angle as u64, 100, 100, 0, 4.0, 2.0, 0.0, -1.2, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 7.0, angle as u64, 100, 100, 0, 4.0, 10.0, 0.0, -1.2, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"),    7.0, angle as u64, 100, 100, 0, 4.0, 0.0, 17.0, 6.0, Some(0.0), Some(4.0), Some(6.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"),    7.0, angle as u64, 100, 100, 0, 4.0, 0.0, 17.0, 13.0, Some(0.0), Some(4.0), Some(13.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 3, 4.0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 10, 0, 60, 4.0, 0.0, 22.0, 5.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 80, 10, 0, 60, 4.0, 0.0, 22.0, 13.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 80, 10, 0, 60, 4.0, 0.0, 10.0, 7.0,Some( 0.0), Some(15.0), Some(7.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 7.0, 80, 10, 0, 60, 4.0, 0.0, 10.0, 15.0,Some( 0.0), Some(15.0), Some(15.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 7.0, 72, 10, 0, 50, 4.0, 2.0, 0.0, -1.2, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 7.0, 68, 10, 0, 50, 4.0, 9.0, 0.0, -1.2, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
    }
    frame(lua_state, 6.0);
    // FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        // notify_event_msc_cmd!(agent, 0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 10.0);
    manage_sword_motion(agent, Hash40::new("to_close"));
    frame(lua_state, 12.0);
	if is_excute(agent) {
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
    // FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialairhijump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_lay_lump"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("elight_sword_light"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 11, 15, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_m"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light"), false, false);
    }
}

unsafe extern "C" fn effect_specialairhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -0.5, 10.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_m"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
		EFFECT_FOLLOW(agent, Hash40::new("elight_lay_shot"), Hash40::new("sword1"), 4.5, 0, 0, 0, 0, 0, 1.25, true);
		LAST_EFFECT_SET_RATE(agent, 1.1);
    }            
	frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("elight_lay_shot"), -1);
    }
}

unsafe extern "C" fn game_specialairhiend2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_CHECK_DIVE);
    }
}

unsafe extern "C" fn effect_specialairhiend2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("elight_sword_light"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.6, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_dash_body_light"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 0.8, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light"), false, false);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_dash_body_light"), false, false);
    }
}

unsafe extern "C" fn expression_specialairhiend2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_aerial"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 12.0, 5.5);
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if VarModule::is_flag(agent.battle_object, vars::elight::instance::HIT_CANCEL) {
        VarModule::off_flag(agent.battle_object, vars::elight::instance::HIT_CANCEL);
    }
    else {
        FT_MOTION_RATE_RANGE(agent, 1.0, 14.0, 18.0);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Low);
	agent.acmd("effect_specialn", effect_specialn, Priority::Low);
    agent.acmd("game_specialairn", game_specialn, Priority::Low);
	agent.acmd("effect_specialairn", effect_specialairn, Priority::Low);
    agent.acmd("game_specialn2", game_specialn2, Priority::Low);
	agent.acmd("effect_specialn2", effect_specialn2, Priority::Low);
    agent.acmd("game_specialairn2", game_specialairn2, Priority::Low);
	agent.acmd("effect_specialairn2", effect_specialairn2, Priority::Low);

	agent.acmd("game_specials", game_specials, Priority::Low);
	agent.acmd("game_specialairs", game_specials, Priority::Low);

	agent.acmd("game_specialsend", game_specialsend, Priority::Low);
	agent.acmd("game_specialairsend", game_specialsend, Priority::Low);

    agent.acmd("effect_specialhistart", effect_specialhistart, Priority::Low);
	agent.acmd("effect_specialairhistart", effect_specialhistart, Priority::Low);
    agent.acmd("game_specialairhijump", game_specialairhijump, Priority::Low);
    agent.acmd("effect_specialairhijump", effect_specialairhijump, Priority::Low);

	agent.acmd("effect_specialairhi1", effect_specialairhi1, Priority::Low);

	agent.acmd("game_specialairhiend2", game_specialairhiend2, Priority::Low);
	agent.acmd("effect_specialairhiend2", effect_specialairhiend2, Priority::Low);
	agent.acmd("expression_specialairhiend2", expression_specialairhiend2, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("game_speciallwend", game_speciallwend, Priority::Low);
    agent.acmd("game_specialairlwend", game_speciallwend, Priority::Low);
}