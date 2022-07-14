
use super::*;

#[acmd_script( agent = "eflame", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_eflame_rnd_futtobi01"), Hash40::new("seq_eflame_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "eflame", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_eflame_rnd_futtobi01"), Hash40::new("seq_eflame_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "eflame", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_eflame_rnd_futtobi01"), Hash40::new("seq_eflame_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "eflame", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_eflame_rnd_futtobi01"), Hash40::new("seq_eflame_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "eflame", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_eflame_rnd_futtobi01"), Hash40::new("seq_eflame_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "eflame", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }    
}

pub fn install() {
    install_acmd_scripts!(
        //dash_effect,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}

