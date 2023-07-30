
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
    //wait_loop_clear(lua_state);
    for _ in 0..999 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 361, 100, 70, 0, 8.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 14.0);
    }

}
//We need to change magnet graphic/hitbox to center it at 4.5 instead of 6.5. This is where the center of ness's body is. Currently it is slightly offset vertically upwards
#[acmd_script( agent = "ness", script = "game_specialairlwhold" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    //wait_loop_clear(lua_state);
    for _ in 0..999 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 361, 100, 70, 0, 8.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 14.0);
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
        special_air_lw_hold_game
    );
}