
use super::*;

#[acmd_script( agent = "yoshi", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let rng = app::sv_math::rand(hash40("fighter"), 3);
            if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_yoshi_rnd_futtobi01"), Hash40::new("seq_yoshi_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let rng = app::sv_math::rand(hash40("fighter"), 3);
        if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_yoshi_rnd_futtobi01"), Hash40::new("seq_yoshi_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "yoshi", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let rng = app::sv_math::rand(hash40("fighter"), 3);
            if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_yoshi_rnd_futtobi01"), Hash40::new("seq_yoshi_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let rng = app::sv_math::rand(hash40("fighter"), 3);
        if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_yoshi_rnd_futtobi01"), Hash40::new("seq_yoshi_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "yoshi", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let rng = app::sv_math::rand(hash40("fighter"), 3);
            if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_yoshi_rnd_futtobi01"), Hash40::new("seq_yoshi_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let rng = app::sv_math::rand(hash40("fighter"), 3);
        if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_yoshi_rnd_futtobi01"), Hash40::new("seq_yoshi_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "yoshi", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_yoshi_rnd_futtobi01"), Hash40::new("seq_yoshi_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_yoshi_rnd_futtobi01"), Hash40::new("seq_yoshi_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "yoshi", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let rng = app::sv_math::rand(hash40("fighter"), 3);
            if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_yoshi_rnd_futtobi01"), Hash40::new("seq_yoshi_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let rng = app::sv_math::rand(hash40("fighter"), 3);
        if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_yoshi_rnd_futtobi01"), Hash40::new("seq_yoshi_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "yoshi", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1.5, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "yoshi", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn yoshi_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "yoshi_tamago", script = "game_throwed" , category = ACMD_GAME , low_priority)]
unsafe fn yoshi_tamago_throwed_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 45, 0, 55, 5.5, 0.0, -1.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_YOSHI_EGG_HIT, *ATTACK_REGION_OBJECT);
    }
    
}

#[acmd_script( agent = "yoshi_tamago", script = "game_burst" , category = ACMD_GAME , low_priority)]
unsafe fn yoshi_tamago_burst_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 45, 0, 55, 9.5, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_YOSHI_EGG_HIT, *ATTACK_REGION_OBJECT);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
        
    
}

pub fn install() {
    install_acmd_scripts!(
      //dash_effect,
		yoshi_turn_dash_game,
        yoshi_tamago_throwed_game,
        yoshi_tamago_burst_game,
	    damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}

