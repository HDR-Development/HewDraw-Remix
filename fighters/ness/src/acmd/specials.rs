use super::*;

unsafe extern "C" fn special_n_fire_game(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn special_air_n_fire_game(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);

    if rng == 0 {
        frame(lua_state, 5.0);
        if is_excute(agent) {
            PLAY_SE_REMAIN(agent, Hash40::new("vc_ness_002"));
            PLAY_SE(agent, Hash40::new("se_ness_special_s03"));
        }
    }
    else {
        frame(lua_state, 5.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_ness_special_s03"));
        }
        frame(lua_state, 20.0);
        if is_excute(agent) {
            PLAY_SE_REMAIN(agent, Hash40::new("vc_ness_attack04"));
        }
    }
}

unsafe extern "C" fn game_specials (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 0.85);
	frame(lua_state, 20.0);
	FT_MOTION_RATE(agent, 1);
	frame(lua_state, 21.0);
	if is_excute(agent) {
		ArticleModule::generate_article(agent.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, 0);
		//WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
	}
	FT_MOTION_RATE(agent, 1);
}

unsafe extern "C" fn game_specialairs (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 0.85);
	frame(lua_state, 20.0);
	FT_MOTION_RATE(agent, 1);
	frame(lua_state, 21.0);
	if is_excute(agent) {
		ArticleModule::generate_article(agent.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, 0);
		//WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
	}
	FT_MOTION_RATE(agent, 1);
}

unsafe extern "C" fn sound_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);

    if rng == 0 {
        frame(lua_state, 5.0);
        if is_excute(agent) {
            PLAY_SE_REMAIN(agent, Hash40::new("vc_ness_002"));
        }
    }
    else {
        frame(lua_state, 20.0);
        if is_excute(agent) {
            PLAY_SE_REMAIN(agent, Hash40::new("vc_ness_attack04"));
        }
    }
}

unsafe extern "C" fn ness_special_air_hi_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_NESS_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("rot"), 25.0, 361, 80, 0, 55, 7.0, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        JostleModule::set_status(boma, false);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU); //10
        //WorkModule::on_flag(boma, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_CRITICAL);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("rot"), 18.0, 361, 70, 0, 45, 4.8, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        WorkModule::on_flag(boma, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_ATTACK_FALL_START);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);//33
        //WorkModule::off_flag(boma, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_CRITICAL);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE);
        JostleModule::set_status(boma, true);
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_NESS_CLIFF_HANG_DATA_DEFAULT as u32);
    }
}

unsafe extern "C" fn special_lw_hold_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    for _ in 0..999 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 55, 90, 0, 27, 2.5, 0.0, 6.5, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.5, 55, 90, 0, 27, 8.9, 0.0, 6.7, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
        }
        wait(lua_state, 6.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 14.0);
    }
}

unsafe extern "C" fn special_air_lw_hold_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    for _ in 0..999 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 55, 90, 0, 27, 2.5, 0.0, 6.5, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.5, 55, 90, 0, 27, 8.9, 0.0, 6.7, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
        }   
        wait(lua_state, 6.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 14.0);
    }

}

//Implemented to remove release windbox
unsafe extern "C" fn game_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn effect_speciallwstart (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("ness_psimagnet_start"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 0.4, false);
	}
}

unsafe extern "C" fn effect_speciallwend (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		LAST_EFFECT_SET_RATE(agent, 1.3);
		EFFECT_FOLLOW(agent, Hash40::new("ness_psimagnet_end"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 0.5, false);
		FLASH(agent, 0.5, 1, 1, 0.4);
	}
	wait(lua_state, 5.0);
	if is_excute(agent) {
		FLASH_FRM(agent, 10, 0, 1, 1, 0.1);
	}
	wait(lua_state, 1.0);
	if is_excute(agent) {
		COL_NORMAL(agent);
	}
}

unsafe extern "C" fn effect_specialairlwend (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("ness_psimagnet_end"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 0.5, false);
		FLASH(agent, 0.5, 1, 1, 0.4);
	}
	wait(lua_state, 5.0);
	if is_excute(agent) {
		FLASH_FRM(agent, 10, 0, 1, 1, 0.1);
	}
	wait(lua_state, 1.0);
	if is_excute(agent) {
		COL_NORMAL(agent);
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnfire", special_n_fire_game);
    agent.acmd("game_specialairnfire", special_air_n_fire_game);
    agent.acmd("sound_specials", sound_specials);
    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialairs", game_specialairs);
    agent.acmd("sound_specialairs", sound_specialairs);
    agent.acmd("game_specialairhi", ness_special_air_hi_game);
    agent.acmd("game_speciallwhold", special_lw_hold_game);
    agent.acmd("game_specialairlwhold", special_air_lw_hold_game);
    agent.acmd("game_speciallwend", game_speciallwend);
    agent.acmd("game_specialairlwend", game_speciallwend);
    agent.acmd("effect_speciallwstart", effect_speciallwstart);
    agent.acmd("effect_specialairlwstart", effect_speciallwstart);
    agent.acmd("effect_speciallwend", effect_speciallwend);
    agent.acmd("effect_specialairlwend", effect_specialairlwend);
}
