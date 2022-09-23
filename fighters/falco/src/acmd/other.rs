
use super::*;

#[acmd_script( agent = "falco", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "falco", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn falco_dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_dash_start"));
    }
    wait(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_step_right_m"));
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_step_left_m"));
    }

}



#[acmd_script( agent = "falco", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn falco_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "falco", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn falco_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.200);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        CATCH(fighter, 0, Hash40::new("trans"), 4.1, 0.0, 7.2, 0.0, Some(0.0), Some(7.2), Some(9.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "falco_blaster_bullet", script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn falco_blaster_bullet_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 100, 20, 0, 1.5, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(0.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
	}
    wait(lua_state, 1.0);
    if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 100, 20, 0, 1.5, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
	}
    wait(lua_state, 1.0);
    if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 100, 20, 0, 1.5, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
	}
    wait(lua_state, 1.0);
    if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 100, 20, 0, 1.5, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
	}
    wait(lua_state, 1.0);
    if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 100, 20, 0, 1.5, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
	}
    
}

#[acmd_script( agent = "falco", script = "sound_escapeair" , category = ACMD_SOUND )]
unsafe fn escape_air_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_escapeair"));
        PLAY_SE(fighter, Hash40::new("vc_falco_ottotto"));
    }

}

#[acmd_script( agent = "falco", script = "sound_escapeairslide" , category = ACMD_SOUND )]
unsafe fn escape_air_slide_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("falco"));
        PLAY_SE(fighter, Hash40::new("vc_falco_ottotto"));
    }

}

#[acmd_script( agent = "falco", script = "sound_escapen" , category = ACMD_SOUND )]
unsafe fn escape_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_escapeair"));
        PLAY_SE(fighter, Hash40::new("vc_falco_ottotto"));
    }

}

#[acmd_script( agent = "falco", script = "sound_passivewalljump" , category = ACMD_SOUND )]
unsafe fn wall_jump_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_jump02"));
        PLAY_SE(fighter, Hash40::new("vc_falco_passive"));
    }

}

pub fn install() {
    install_acmd_scripts!(
        dash_effect,
        falco_dash_sound,
        falco_turn_dash_game,
		falco_catch_game,
        falco_blaster_bullet_fly_game,
        escape_air_sound,
        escape_air_slide_sound,
        escape_n_sound,
        wall_jump_sound,
    );
}

