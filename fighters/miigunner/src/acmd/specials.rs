use super::*;

unsafe extern "C" fn game_specialn1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 10.0);
	if is_excute(agent) {
		ArticleModule::generate_article_enable(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, false, 0);
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_BULLET_DISP);
	}

}

unsafe extern "C" fn game_specialnifiremax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
		FT_MOTION_RATE_RANGE(agent, 1.0, 3.0, 10.0);
	}
    if is_excute(agent) {
		VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
			WorkModule::off_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_BULLET_DISP);
        }
    }
	frame(lua_state, 3.0);
	if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
		FT_MOTION_RATE_RANGE(agent, 3.0, 5.0, 5.0);
	}
	else {
		FT_MOTION_RATE(agent, 1.0);
	}
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			ArticleModule::remove_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_SHOOT);
			ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 361, 74, 0, 61, 7.0, 0.0, 7.5, 8.0, Some(0.0), Some(7.5), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
			ATTACK(agent, 1, 0, Hash40::new("top"), 22.0, 361, 74, 0, 61, 5.0, 0.0, 7.5, 8.0, Some(0.0), Some(7.5), Some(21.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		}
		else {
			ArticleModule::shoot_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
			WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_SHOOT);
		}
	} 
	frame(lua_state, 5.0);
	if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
		FT_MOTION_RATE_RANGE(agent, 8.0, 40.0, 37.0);
	}
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	} 
}

unsafe extern "C" fn effect_specialn1firemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 0, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(agent, 2.0);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 100.0, 10.0);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 90, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(agent, 2.0);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 100.0, 3.0);
		}
	}
	frame(lua_state, 2.6);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("miigunner_sb_tama"), false, false);
		EFFECT_DETACH_KIND(agent, Hash40::new("miigunner_sb_tama"), -1);
	}
	frame(lua_state, 2.8);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 5.0, 0.55);
			LAST_EFFECT_SET_RATE(agent, 0.85);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, false);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_laser"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_SCALE_W(agent, 1.0, 0.7, 1.0);
			LAST_EFFECT_SET_RATE(agent, 0.8);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot_s"), Hash40::new("armr"), 6.3, 0, 0, 0, 0, -90, 1, false);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 5.0, 5.0);
			LAST_EFFECT_SET_RATE(agent, 0.5);
			if agent.is_situation(*SITUATION_KIND_GROUND) {
				LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
				FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
			}
		}
		else {
			EFFECT(agent, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			if agent.is_situation(*SITUATION_KIND_GROUND) {
				LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			}
		}
	}

}

unsafe extern "C" fn sound_specialn1firemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		PLAY_SEQUENCE(agent, Hash40::new("seq_miigunner_rnd_special_c1_n01"));
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			PLAY_SE(agent, Hash40::new("se_miigunner_final01"));
		}
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			STOP_SE(agent, Hash40::new("se_miigunner_final01"));
			PLAY_SE(agent, Hash40::new("se_miigunner_final04"));
		}
	}

}

unsafe extern "C" fn game_specialn2loop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 1.0);
	frame(lua_state, 2.0);
	if is_excute(agent) {
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
	}
	frame(lua_state, 4.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 45, 100, 0, 30, 4.0, 2.0, 0.0, 0.0, Some(4.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, false, 0);
	}
    frame(lua_state, 6.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}

}

unsafe extern "C" fn game_specialn3start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		VarModule::set_float(agent.battle_object, vars::miigunner::instance::GRENADE_CHARGE, 0.0);
	}
	frame(lua_state, 16.0);
	FT_MOTION_RATE_RANGE(agent, 16.0, 30.0, 6.0);
	frame(lua_state, 25.0);

}

unsafe extern "C" fn effect_specialn3start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_flame_shot"), Hash40::new("armr"), 5.5, 0, 0, 0, 90, 0, 0.49, true);
        LAST_EFFECT_SET_RATE(agent, 0.15);
    }
}

unsafe extern "C" fn game_specialn3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE_RANGE(agent, 1.0, 23.0, 30.0);
	if is_excute(agent) {
		ArticleModule::generate_article_enable(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GRENADELAUNCHER, false, 0);
	}
}

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE_RANGE(agent, 1.0, 21.0, 17.0);
	frame(lua_state, 21.0);
	FT_MOTION_RATE_RANGE(agent, 21.0, 64.0, 36.0);
	if is_excute(agent) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FLAMEPILLAR, false, 0);
	}
}

unsafe extern "C" fn game_specials32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 23.0);
	if is_excute(agent) {
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON);
	}
}

unsafe extern "C" fn game_specialhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 11.0);
	if is_excute(agent) {
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_BOTTOM_SHOOT_FLAG_JUMP);
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_BOTTOMSHOOT, false, 0);
	}
	frame(lua_state, 14.0);
	if is_excute(agent) {
		notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
}

unsafe extern "C" fn effect_specialhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 2.0);
	if is_excute(agent) {
		let handle = EffectModule::req_follow(boma, Hash40::new("miigunner_bottom_shot"), Hash40::new("armr"), &Vector3f::new(6.5, 0.0, 0.0), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
		VarModule::set_int64(agent.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER, handle);
	}
	frame(lua_state, 9.0);
	if is_excute(agent) {
		let handle = VarModule::get_int64(agent.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
		EffectModule::set_rate(boma, handle as u32, 1.0);
		if VarModule::get_float(agent.battle_object, vars::miigunner::status::CURRENT_CHARGE) <= 10.0 && !VarModule::is_flag(agent.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED) {
			EffectModule::set_rgb(boma, handle as u32, 0.15, 0.55, 10.0);
		}
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
		LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
	}
}

unsafe extern "C" fn effect_landingfallspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		EFFECT_OFF_KIND(agent, Hash40::new("miigunner_bottom_shot"), false, false);
	}
}

unsafe extern "C" fn game_specialhi2squat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 3.0);
	FT_MOTION_RATE(agent, 0.5);
	frame(lua_state, 7.0);
	FT_MOTION_RATE(agent, 0.25);
	frame(lua_state, 11.0);
	FT_MOTION_RATE(agent, 1.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 110, 85, 40, 90, 7.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 13.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn effect_specialhi2squat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot_after"), Hash40::new("armr"), 5.5, 0, 0, 0, 0, -90, 1, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
		if agent.is_situation(*SITUATION_KIND_GROUND) {
			LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
			EFFECT(agent, Hash40::new("miigunner_gimmck_bomb1"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
			EFFECT(agent, Hash40::new("miigunner_gimmck_bomb2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
		}
		else {
			EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
		}
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 2.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 80, 100, 0, 83, 5.0, 0.0, 17.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 80, 100, 0, 83, 4.0, 0.0, 12.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 5.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 75, 90, 0, 83, 4.0, 0.0, 17.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 75, 90, 0, 83, 3.5, 0.0, 12.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 8.0);
	if is_excute(agent) {
		notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
	frame(lua_state, 23.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 27.0);
	if is_excute(agent) {
		notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

unsafe extern "C" fn game_specialhi3start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 1.5);
}

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 0.85);
}

unsafe extern "C" fn game_speciallw1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		if agent.is_situation(*SITUATION_KIND_GROUND) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 4.5, 3.0, 6.5, 4.0);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		let offset = if agent.is_situation(*SITUATION_KIND_GROUND) { 0.0 } else { 2.0 };
		ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 69, 100, 0, 70, 8.0, 0.0, 6.5 + offset, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
	}
}

unsafe extern "C" fn game_speciallw3start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE_RANGE(agent, 1.0, 7.0, 5.0);
}

unsafe extern "C" fn game_speciallw3hold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 0.8);
	if is_excute(agent) {
		let offset_y = if agent.is_situation(*SITUATION_KIND_GROUND) { 6.5 } else { 9.5 };
		ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 100, 25, 0, 12.5, 0.0, offset_y, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		let offset_y = if agent.is_situation(*SITUATION_KIND_GROUND) { 6.5 } else { 9.5 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 366, 100, 25, 0, 22.0, 0.0, offset_y, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn effect_speciallw3hold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
		EffectModule::kill_kind(boma, Hash40::new("miigunner_absorber"), false, false);
		let offset = if agent.is_situation(*SITUATION_KIND_GROUND) { 3.0 } else { 2.0 };
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_absorber"), Hash40::new("hip"), offset, 0, 0, 0, 0, 0, 0.75, true);
		EFFECT_FOLLOW(agent, Hash40::new("miigunner_absorberlight"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    COL_NORMAL(agent);
    if is_excute(agent) {
        FLASH(agent, 0.5, 0.7, 1, 0.6);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 3, 0, 0.2, 0.5, 0);
    }
    wait(lua_state, 3.0);
}

unsafe extern "C" fn game_speciallw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE_RANGE(agent, 1.0, 7.0, 9.0);
	if is_excute(agent) {
		let offset_y = if agent.is_situation(*SITUATION_KIND_GROUND) { 6.5 } else { 9.5 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 55, 70, 0, 30, 14.0, 0.0, offset_y, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
	frame(lua_state, 3.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn effect_speciallw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("sys_dead_flash"), Hash40::new("armr"), 1, 5.5, 0, 0, 0, 0, 0.075, 0, 0, 0, 0, 0, 0, false);
		EFFECT(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
	}
}

unsafe extern "C" fn sound_speciallw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_miigunner_attackhard_s01"));
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn1start", game_specialn1start);
    agent.acmd("game_specialairn1start", game_specialn1start);

    agent.acmd("game_specialn1firemax", game_specialnifiremax);
    agent.acmd("game_specialairn1firemax", game_specialnifiremax);
    agent.acmd("effect_specialn1firemax", effect_specialn1firemax);
    agent.acmd("effect_specialairn1firemax", effect_specialn1firemax);
    agent.acmd("sound_specialn1firemax", sound_specialn1firemax);
    agent.acmd("sound_specialairn1firemax", sound_specialn1firemax);

    agent.acmd("game_specialn2loop", game_specialn2loop);
    agent.acmd("game_specialairn2loop", game_specialn2loop);

    agent.acmd("game_specialn3start", game_specialn3start);
    agent.acmd("game_specialairn3start", game_specialn3start);
    agent.acmd("effect_specialn3start", effect_specialn3start);

    agent.acmd("game_specialn3end", game_specialn3end);
    agent.acmd("game_specialairn3end", game_specialn3end);

    agent.acmd("game_specials1", game_specials1);
    agent.acmd("game_specialairs1", game_specials1);

    agent.acmd("game_specials32", game_specials32);
    agent.acmd("game_specialairs32", game_specials32);

    agent.acmd("game_specialhi1", game_specialhi1);
    agent.acmd("game_specialairhi1", game_specialhi1);
    agent.acmd("effect_specialhi1", effect_specialhi1);
    agent.acmd("effect_specialairhi1", effect_specialhi1);

    agent.acmd("game_specialhi2squat", game_specialhi2squat);
    agent.acmd("game_specialairhi2squat", game_specialhi2squat);
    agent.acmd("effect_specialhi2squat", effect_specialhi2squat);
    agent.acmd("effect_specialairhi2squat", effect_specialhi2squat);

    agent.acmd("game_specialhi2", game_specialhi2);

    agent.acmd("game_specialhi3start", game_specialhi3start);
    agent.acmd("game_specialairhi3start", game_specialhi3start);

    agent.acmd("game_specialhi3", game_specialhi3);

	agent.acmd("effect_landingfallspecial", effect_landingfallspecial);

    agent.acmd("game_speciallw1start", game_speciallw1start);
    agent.acmd("game_specialairlw1start", game_speciallw1start);

    agent.acmd("game_speciallw3start", game_speciallw3start);
    agent.acmd("game_specialairlw3start", game_speciallw3start);

    agent.acmd("game_speciallw3hold", game_speciallw3hold);
    agent.acmd("game_specialairlw3hold", game_speciallw3hold);
    agent.acmd("effect_speciallw3hold", effect_speciallw3hold);
    agent.acmd("effect_specialairlw3hold", effect_speciallw3hold);
	
    agent.acmd("game_speciallw3end", game_speciallw3end);
    agent.acmd("game_specialairlw3end", game_speciallw3end);
    agent.acmd("effect_speciallw3end", effect_speciallw3end);
    agent.acmd("effect_specialairlw3end", effect_speciallw3end);
    agent.acmd("sound_speciallw3end", sound_speciallw3end);
    agent.acmd("sound_specialairlw3end", sound_speciallw3end);
}