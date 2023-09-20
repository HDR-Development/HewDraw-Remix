
use super::*;

#[acmd_script( agent = "ness", script = "game_specialnfire" , category = ACMD_GAME , low_priority)]
unsafe fn special_n_fire_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 75, 10, 0, 150, 6.0, 0.0, 5.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ness", script = "game_specialairnfire" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_n_fire_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 75, 10, 0, 150, 3.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ness", script = "sound_specials" , category = ACMD_SOUND )]
unsafe fn sound_specials(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);

    if rng == 0 {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_ness_002"));
            PLAY_SE(fighter, Hash40::new("se_ness_special_s03"));
        }
    }
    else {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_ness_special_s03"));
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_ness_attack04"));
        }
    }
}

#[acmd_script( agent = "ness", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn game_specials (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(fighter, 0.85);
	frame(lua_state, 20.0);
	FT_MOTION_RATE(fighter, 1);
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, 0);
		//WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
	}
	FT_MOTION_RATE(fighter, 1);
}

#[acmd_script( agent = "ness", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn game_specialairs (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(fighter, 0.85);
	frame(lua_state, 20.0);
	FT_MOTION_RATE(fighter, 1);
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, 0);
		//WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
	}
	FT_MOTION_RATE(fighter, 1);
}

#[acmd_script( agent = "ness", script = "sound_specialairs" , category = ACMD_SOUND )]
unsafe fn sound_specialairs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);

    if rng == 0 {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_ness_002"));
        }
    }
    else {
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_ness_attack04"));
        }
    }
}

#[acmd_script( agent = "ness", script = "game_speciallwhold" , category = ACMD_GAME , low_priority)]
unsafe fn special_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    for _ in 0..999 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 55, 90, 0, 27, 2.5, 0.0, 6.5, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 55, 90, 0, 27, 8.5, 0.0, 6.5, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 14.0);
    }

}

#[acmd_script( agent = "ness", script = "game_specialairlwhold" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
   //wait_loop_clear(lua_state);
    for _ in 0..999 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 55, 90, 0, 27, 2.5, 0.0, 6.5, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 55, 90, 0, 27, 8.5, 0.0, 6.5, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
        }   
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 14.0);
    }

}




//Implemented to remove release windbox
#[acmd_script( agent = "ness", scripts = ["game_speciallwend", "game_specialairlwend"], category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

#[acmd_script( agent = "ness", scripts = ["effect_speciallwstart", "effect_specialairlwstart"] , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_speciallwstart (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("ness_psimagnet_start"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 0.4, false);
	}
}

#[acmd_script( agent = "ness", script = "effect_speciallwend" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_speciallwend (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		LAST_EFFECT_SET_RATE(fighter, 1.3);
		EFFECT_FOLLOW(fighter, Hash40::new("ness_psimagnet_end"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 0.5, false);
		FLASH(fighter, 0.5, 1, 1, 0.4);
	}
	wait(lua_state, 5.0);
	if is_excute(fighter) {
		FLASH_FRM(fighter, 10, 0, 1, 1, 0.1);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		COL_NORMAL(fighter);
	}
}

#[acmd_script( agent = "ness", script = "effect_specialairlwend" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialairlwend (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("ness_psimagnet_end"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 0.5, false);
		FLASH(fighter, 0.5, 1, 1, 0.4);
	}
	wait(lua_state, 5.0);
	if is_excute(fighter) {
		FLASH_FRM(fighter, 10, 0, 1, 1, 0.1);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		COL_NORMAL(fighter);
	}
}


pub fn install() {
    install_acmd_scripts!(
        game_specials,
        game_specialairs,
        sound_specials,
        sound_specialairs,
        special_n_fire_game,
        special_air_n_fire_game,
        special_lw_hold_game,
        special_air_lw_hold_game,
        game_speciallwend,
        effect_speciallwstart,
        effect_speciallwend,
        effect_specialairlwend,
    );
}