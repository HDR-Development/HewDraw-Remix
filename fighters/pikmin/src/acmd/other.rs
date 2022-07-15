
use super::*;

#[acmd_script( agent = "pikmin", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let rng = app::sv_math::rand(hash40("fighter"), 3);
            if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pikmin_rnd_futtobi01"), Hash40::new("seq_pikmin_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let rng = app::sv_math::rand(hash40("fighter"), 3);
        if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pikmin_rnd_futtobi01"), Hash40::new("seq_pikmin_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "pikmin", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let rng = app::sv_math::rand(hash40("fighter"), 3);
            if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pikmin_rnd_futtobi01"), Hash40::new("seq_pikmin_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let rng = app::sv_math::rand(hash40("fighter"), 3);
        if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pikmin_rnd_futtobi01"), Hash40::new("seq_pikmin_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "pikmin", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let rng = app::sv_math::rand(hash40("fighter"), 3);
            if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pikmin_rnd_futtobi01"), Hash40::new("seq_pikmin_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let rng = app::sv_math::rand(hash40("fighter"), 3);
        if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pikmin_rnd_futtobi01"), Hash40::new("seq_pikmin_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "pikmin", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_pikmin_rnd_futtobi01"), Hash40::new("seq_pikmin_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pikmin_rnd_futtobi01"), Hash40::new("seq_pikmin_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "pikmin", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let rng = app::sv_math::rand(hash40("fighter"), 3);
            if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pikmin_rnd_futtobi01"), Hash40::new("seq_pikmin_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let rng = app::sv_math::rand(hash40("fighter"), 3);
        if rng == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pikmin_rnd_futtobi01"), Hash40::new("seq_pikmin_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "pikmin", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.3);
    }
	frame(lua_state, 11.0); // Effectively F14
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "pikmin", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.58, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "pikmin", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.1);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0); // Effectuvekt F14
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        dash_game,
        //dash_effect,
        turn_dash_game,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}

