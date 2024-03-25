
use super::*;

unsafe fn manage_sword_motion(agent: &mut L2CAgentBase, motion: Hash40) {
    let exists = {
        agent.clear_lua_stack();
        lua_args!(agent, FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD);
        app::sv_animcmd::IS_EXIST_ARTICLE(agent.lua_state_agent)
    };

    if !exists {
        return;
    }

    if is_excute(agent) {
        ArticleModule::add_motion_partial(
            agent.module_accessor,
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

    if MotionModule::is_changing(agent.module_accessor) && is_excute(agent) {
        agent.on_flag(*FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
    }
}

unsafe extern "C" fn game_specialn (agent: &mut L2CAgentBase) {
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
		ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 10, 0, 5.0, 0.0, 12.0, 8.0, Some(0.0), Some(12.0), Some(16.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 90, 100, 15, 0, 5.0, 0.0, 6.0, 8.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 9.0, 8.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 110, 100, 20, 10, 5.0, 0.0, 6.0, 16.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 9.0, 16.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
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
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
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
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 10, 4.0, 1.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 10, 3.5, 6.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 95, 100, 20, 10, 2.8, 11.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 95, 100, 20, 10, 2.8, 14.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 100, 100, 20, 10, 6.0, 0.0, 10.0, 15.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 90, 100, 20, 10, 6.0, 0.0, 10.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
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
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
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
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 10, 0, 2.8, 14.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(16.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
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
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
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
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 3.5, 10.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 2.8, 15.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 2.8, 18.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 6.0, 42, 84, 0, 75, 7.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
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
	frame(lua_state, 37.0);
	if is_excute(agent) {
		AttackModule::clear(boma, /*ID*/ 0, false);
		AttackModule::clear(boma, /*ID*/ 1, false);
		AttackModule::clear(boma, /*ID*/ 2, false);
		AttackModule::clear(boma, /*ID*/ 3, false);
	}
	frame(lua_state, 38.0);
	if is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
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

unsafe extern "C" fn game_specialn2 (agent: &mut L2CAgentBase) {
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
		ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 15, 0, 7.0, 0.0, 10.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 10.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 110, 100, 20, 10, 7.0, 0.0, 10.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 10.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 15, 0, 7.0, 0.0, 8.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 8.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 110, 100, 20, 10, 7.0, 0.0, 8.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 8.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
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
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 15, 100, 20, 15, 4.0, 1.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 15, 100, 20, 15, 3.5, 8.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 100, 100, 20, 15, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 50, 10, 2.8, 14.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 11.0, 12.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 11.0, 17.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 6, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 11.0, 22.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 7, 0, Hash40::new("top"), 2.0, 15, 100, 20, 15, 4.0, 0.0, 11.0, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 15, 100, 20, 15, 4.0, 0.0, 1.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 15, 100, 20, 15, 3.5, 0.0, 8.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 100, 100, 20, 15, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 366, 100, 50, 10, 2.8, 0.0, 14.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 7.0, 12.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 7.0, 17.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 6, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 7.0, 22.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 7, 0, Hash40::new("top"), 2.0, 15, 100, 20, 15, 6.0, 0.0, 7.0, 22.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
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
		AttackModule::clear_all(agent.module_accessor);
	}
	frame(lua_state, 26.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 10, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 13.0, 14.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 13.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 4.0, 0.0, 2.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 3.5, 0.0, 7.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 366, 100, 10, 0, 2.8, 0.0, 15.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 8.0, 14.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 8.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 5, 2.0);
	}
	frame(lua_state, 28.0);
	if is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
	frame(lua_state, 35.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 5.0, 0.0, 13.0, 14.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 5.0, 0.0, 13.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 4.0, 0.0, 2.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 3.5, 0.0, 7.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 2.8, 0.0, 15.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 5.0, 0.0, 6.0, 14.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 5.0, 0.0, 6.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 5, 2.0);
	}
	frame(lua_state, 37.0);
	if is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
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
		AttackModule::clear_all(agent.module_accessor);
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

// for SOME reason, full charge aerial neutral b would crash if i did the above script as a multiscript install
// so we need to reimplement the aerial version too /shrug

unsafe extern "C" fn game_specialairn2 (agent: &mut L2CAgentBase) {
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
		ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 90, 100, 15, 0, 2.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(20.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 50, 10, 7.0, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(20.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 90, 100, 15, 0, 2.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(20.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
	}
	frame(lua_state, 6.0);
	if is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
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
		WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
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
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 80, 0, 5.0, 0.0, 11.0, 10.0, Some(0.0), Some(11.0), Some(22.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 4.0, 0.0, 2.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 3.5, 0.0, 7.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 2.8, 0.0, 15.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 80, 0, 5.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(22.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 2.0);
		ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 2.0);
	}
	frame(lua_state, 28.0);
	if is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
	frame(lua_state, 35.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 3, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 80, 0, 5.0, 0.0, 11.0, 10.0, Some(0.0), Some(11.0), Some(22.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY{
		if is_excute(agent) {
			ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 4.0, 0.0, 2.5, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 3.5, 0.0, 7.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 2.8, 0.0, 12.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 3, 0, Hash40::new("haver"), 2.0, 90, 100, 10, 0, 2.8, 0.0, 15.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 366, 100, 80, 0, 5.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(22.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	if is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
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

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
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
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword1"), Hash40::new("tex_elight_sword2"), 5, Hash40::new("sword1"), 0.0, 0.0, -0.08, Hash40::new("sword1"), 15.5, 0.0, -0.08, false, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_specialairhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    manage_sword_motion(agent, Hash40::new("to_open"));

    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 0.75);

    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);

    frame(lua_state, 24.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_EXPROSIVESHOT, false, -1);
    }

    frame(lua_state, 25.0);
    manage_sword_motion(agent, Hash40::new("to_close"));
}

unsafe extern "C" fn game_specialairhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);

    if is_excute(agent) {
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_NONE);
    }

    manage_sword_motion(agent, Hash40::new("to_open"));

    frame(lua_state, 23.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }

    frame(lua_state, 24.0);

    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }

    frame(lua_state, 25.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }

    frame(lua_state, 26.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }

    frame(lua_state, 27.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }

    frame(lua_state, 29.0);
    manage_sword_motion(agent, Hash40::new("to_close"));
}

unsafe extern "C" fn effect_specialairhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 14, 0, 3.3, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
        EFFECT(agent, Hash40::new("elight_lay_shot"), Hash40::new("sword1"), 6, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_m"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

unsafe extern "C" fn effect_specialairhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 12, 0, -1.7, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
        EFFECT(agent, Hash40::new("elight_lay_dust_shot"), Hash40::new("sword1"), 6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_m"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

unsafe extern "C" fn sound_specialairhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_elight_special_h01_rand"));
    }
    
    frame(lua_state, 23.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_elight_special_h02_01_shot"));
    }
}

unsafe extern "C" fn sound_specialairhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_elight_special_h02_rand"));
    }
    
    frame(lua_state, 23.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_elight_special_h03_shot"));
    }
}

unsafe extern "C" fn expression_specialairhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 0, false, 0x50000000);
    }
}

unsafe extern "C" fn expression_specialairhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_79_beams"), 0, false, 0x50000000);
    }
}

unsafe extern "C" fn game_specialairhijump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_NONE);
        let mut angle = app::lua_bind::FighterKineticEnergyMotion::get_angle(KineticModule::get_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as _);
        if PostureModule::lr(agent.module_accessor) > 0.0 {
            angle *= -1.0;
        }
        angle = 80.0 - angle.to_degrees() / 2.0;
        if angle < 0.0 {
            angle += 360.0;
        }
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 7.0, angle as u64, 100, 110, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 7.0, angle as u64, 100, 110, 0, 4.0, 10.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, angle as u64, 100, 110, 0, 4.0, 0.0, 17.0, 6.0, Some(0.0), Some(4.0), Some(6.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 7.0, angle as u64, 100, 110, 0, 4.0, 0.0, 17.0, 13.0, Some(0.0), Some(4.0), Some(13.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
    }

    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 57, 10, 0, 60, 4.0, 0.0, 22.0, 5.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 80, 10, 0, 50, 4.0, 0.0, 22.0, 13.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 68, 10, 0, 75, 4.0, 0.0, 10.0, 7.0,Some( 0.0), Some(15.0), Some(7.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 7.0, 75, 10, 0, 70, 4.0, 0.0, 10.0, 15.0,Some( 0.0), Some(15.0), Some(15.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
    }

    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 7.0, 72, 10, 0, 55, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 7.0, 68, 10, 0, 55, 4.0, 9.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::clear(agent.module_accessor, 2, false);
        AttackModule::clear(agent.module_accessor, 3, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
    }

    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }

    frame(lua_state, 10.0);

    manage_sword_motion(agent, Hash40::new("to_close"));

    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 0.5);

    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);

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

unsafe extern "C" fn game_specialairsstart (agent: &mut L2CAgentBase) { 
    let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 2.0);
    }
}

unsafe extern "C" fn effect_specialsstart (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		EFFECT_FOLLOW(agent, Hash40::new("elight_photon_start"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
	}
	frame(lua_state, 5.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open_l"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
		LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
	}
	frame(lua_state, 10.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("elight_photon_flash"), Hash40::new("rot"), 0, -1.5, 4.8, 0, 0, 0, 1, true);
		EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
		EFFECT_FOLLOW(agent, Hash40::new("elight_photon_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
		LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
	}
}

unsafe extern "C" fn effect_specialairsstart (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("elight_photon_start"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
	}
	frame(lua_state, 5.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open_l"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
		LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
	}
	frame(lua_state, 10.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("elight_photon_flash"), Hash40::new("rot"), 0, -1.5, 4.8, 0, 0, 0, 1, true);
		EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
		EFFECT_FOLLOW(agent, Hash40::new("elight_photon_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
		LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
	}
}

unsafe extern "C" fn game_specials (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
        MotionModule::set_rate(boma, 1.7);
        JostleModule::set_status(agent.module_accessor, false);
        VarModule::off_flag(agent.battle_object, vars::elight::instance::ENABLE_SPECIAL_S_ACTIONABILITY);
	}
	if is_excute(agent) {
		HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_XLU);
		HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
		HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
		HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
		HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
		HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
		HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
		HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
		HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
		HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
		//ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 30, 100, 40, 35, 5.0, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(12.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		//ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 35, 100, 40, 25, 5.0, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(12.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(lua_state, 2.0);
	if is_excute(agent) {
		WorkModule::set_int(agent.module_accessor, 5, *FIGHTER_ELIGHT_STATUS_SPECIAL_S_WORK_INT_BUNSHIN_NUM);
        if ArticleModule::get_num(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_BUNSHIN) == 0 {
			ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_BUNSHIN, false, -1);
            ArticleModule::set_rate(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_BUNSHIN, 1.0);
		}
        VisibilityModule::set_whole(agent.module_accessor, false);
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_NORMAL);
		HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
		HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
		HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
		HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
		HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
		HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
		HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
		HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
		HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
	}

    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("rot"), 15.0, 90, 105, 0, 45, 3.25, 0.0, -1.5, 4.8, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("rot"), 7.0, 120, 100, 65, 30, 10.0, 0.0, -1.5, 4.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
        ATTACK(agent, 7, 0, Hash40::new("rot"), 4.0, 361, 75, 0, 20, 3.0, 0.0, 0.0, 5.0, Some(0.0), Some(0.0), Some(21.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 7, 0, Hash40::new("rot"), 4.0, 361, 75, 0, 20, 3.0, 0.0, 0.0, 21.0, Some(0.0), Some(0.0), Some(35.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }

    frame(lua_state, 18.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, (22.0-18.0)/1.0);
    }
	frame(lua_state, 23.0);
	if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
	}
    frame(lua_state, 30.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 3.0);
    }
	if is_excute(agent) {
		WorkModule::off_flag(agent.module_accessor, /*Flag*/ *FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_CHECK_CLIFF);
    }
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
		EFFECT(agent, Hash40::new("elight_photon_vanish"), Hash40::new("rot"), 0.0, -3.7, 3.8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("elight_photon_speedline"), Hash40::new("top"), 0, 7, 0, 0, 180, 0, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_photon_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
        EFFECT_FOLLOW(agent, Hash40::new("elight_photon_body_lihgt"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) && VarModule::get_int(agent.object(), vars::common::instance::LAST_ATTACK_HITBOX_ID) == 0{
            EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("elight_attack100_finish"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -12, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
		EFFECT(agent, Hash40::new("elight_photon_vanish"), Hash40::new("rot"), 0, -4.2, 3.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("elight_photon_speedline"), Hash40::new("top"), 0, 7, 0, 0, 180, 0, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_photon_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
        EFFECT_FOLLOW(agent, Hash40::new("elight_photon_body_lihgt"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) && VarModule::get_int(agent.object(), vars::common::instance::LAST_ATTACK_HITBOX_ID) == 0{
            EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("elight_attack100_finish"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
    }
}

unsafe extern "C" fn expression_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
	frame(lua_state, 6.0);
	if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
    frame(lua_state, 8.0);
    if is_excute(agent) {
		RUMBLE_HIT(agent, Hash40::new("rbkind_79_slashlarge"), 0);
    }
	frame(lua_state, 10.0);
	if is_excute(agent) {
		RUMBLE_HIT(agent, Hash40::new("rbkind_79_slashsmall"), 0);
	}
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_attacks"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let rand = sv_math::rand(hash40("fighter"), 6) as i32;
        let rand1 = sv_math::rand(hash40("fighter"), 2) as i32;
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            if rand1 == 0 { //if you hold taunt, do foresight voicelines
                PLAY_SE_REMAIN(agent, Hash40::new("vc_elight_escapeforesight01"));
            }
            else if rand1 == 1 {
                PLAY_SE_REMAIN(agent, Hash40::new("vc_elight_escapeforesight02"));
            }
        }
        else { //else just run the normal voice pool
            if rand == 0 {
                PLAY_SE_REMAIN(agent, Hash40::new("vc_elight_special_s01"));
            } 
            else if rand == 1 {
                PLAY_SE_REMAIN(agent, Hash40::new("vc_elight_attack04"));
            } 
            else if rand == 2 {
                PLAY_SE_REMAIN(agent, Hash40::new("vc_elight_attack05"));
            } 
            else if rand == 3 {
                PLAY_SE_REMAIN(agent, Hash40::new("vc_elight_attack06"));
            }
            else if rand == 4 {
                PLAY_SE_REMAIN(agent, Hash40::new("vc_elight_attack09"));
            }
            else if rand == 5 {
                PLAY_SE_REMAIN(agent, Hash40::new("vc_elight_special_l05"));
            }
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) { //slash noise
        PLAY_SE(agent, Hash40::new("se_elight_special_s02_end"));
    }
}

unsafe extern "C" fn effect_specialsend (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_photon_vanish"), Hash40::new("rot"), 0, -3.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("elight_photon_body_lihgt"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
		EFFECT_FOLLOW(agent, Hash40::new("elight_photon_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
		LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_l"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
		LAST_EFFECT_SET_RATE(agent, 1.0);
		LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
		EFFECT_FOLLOW(agent, Hash40::new("elight_photon_sword_close"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
	frame(lua_state, 2.0);
	if is_excute(agent) {
		FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.5, 0, 0, 3, 0, 0, 0, false);
	}
	frame(lua_state, 4.0);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("elight_photon_slash5_light"), false, true);
		EFFECT_OFF_KIND(agent, Hash40::new("elight_photon_body_lihgt"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_photon_sword"), true, true);
	}
	frame(lua_state, 5.0);
	if is_excute(agent) {
		FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.2, 0, 0, 3, 0, 0, 0, false);
	}
	frame(lua_state, 8.0);
	if is_excute(agent) {
		FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.9, 0, 0, 3, 0, 0, 0, false);
		LAST_EFFECT_SET_RATE(agent, 0.85);
	}
}

unsafe extern "C" fn effect_specialairsend (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_photon_vanish"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("elight_photon_body_lihgt"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
		EFFECT_FOLLOW(agent, Hash40::new("elight_photon_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
		LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_l"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
		LAST_EFFECT_SET_RATE(agent, 1.0);
		LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
		EFFECT_FOLLOW(agent, Hash40::new("elight_photon_sword_close"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
	}
	frame(lua_state, 4.0);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("elight_photon_slash5_light"), false, true);
		EFFECT_OFF_KIND(agent, Hash40::new("elight_photon_body_lihgt"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_photon_sword"), true, true);
	}
}
unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 5.0/(12.0-1.0));

    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("game_specialairn", game_specialn);
    agent.acmd("game_specialn2", game_specialn2);
    agent.acmd("game_specialairn2", game_specialairn2);
    agent.acmd("effect_specialhistart", effect_specialhistart);
    agent.acmd("game_specialairhi1", game_specialairhi1);
    agent.acmd("game_specialairhi2", game_specialairhi2);
    agent.acmd("effect_specialairhi1", effect_specialairhi1);
    agent.acmd("effect_specialairhi2", effect_specialairhi2);
    agent.acmd("sound_specialairhi1", sound_specialairhi1);
    agent.acmd("sound_specialairhi2", sound_specialairhi2);
    agent.acmd("expression_specialairhi1", expression_specialairhi1);
    agent.acmd("expression_specialairhi2", expression_specialairhi2);
    agent.acmd("game_specialairhijump", game_specialairhijump);
    agent.acmd("effect_specialairhijump", effect_specialairhijump);
    agent.acmd("game_specialairsstart", game_specialairsstart);
    agent.acmd("game_specialsstart", game_specialairsstart);
    agent.acmd("effect_specialsstart", effect_specialsstart);
    agent.acmd("effect_specialairsstart", effect_specialairsstart);
    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialairs", game_specials);
    agent.acmd("effect_specials", effect_specials);
    agent.acmd("effect_specialairs", effect_specialairs);
    agent.acmd("expression_specials", expression_specials);
    agent.acmd("expression_specialairs", expression_specials);
    agent.acmd("sound_specials", sound_specials);
    agent.acmd("sound_specialairs", sound_specials);
    agent.acmd("effect_specialsend", effect_specialsend);
    agent.acmd("effect_specialairsend", effect_specialairsend);
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("game_specialairlw", game_specialairlw);
}
