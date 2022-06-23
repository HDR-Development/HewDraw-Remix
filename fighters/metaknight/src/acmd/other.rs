
use super::*;

#[acmd_script( agent = "metaknight", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn metaknight_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 6.5, 0.0, Some(0.0), Some(6.5), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.217);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "metaknight", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "metaknight", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.73, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footl"), 5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "metaknight", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

/*** METAQUICK ACMD ***/
#[acmd_script( agent = "metaknight", script = "sound_metaquicksummon", category = ACMD_SOUND, low_priority)]
unsafe fn metaquick_sound(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        // plays the sword slash effect
        PLAY_SE(fighter, Hash40::new("se_metaknight_final01"));

        if VarModule::is_flag(fighter.battle_object, vars::metaknight::META_QUICK_PLAY_VC) {
            PLAY_SE(fighter, Hash40::new("vc_metaknight_final02"));
        }
    }
}

#[acmd_script( agent = "metaknight", script = "effect_metaquicksummon", category = ACMD_EFFECT, low_priority)]
unsafe fn metaquick_effect(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        VarModule::set_int(fighter.battle_object, vars::metaknight::META_QUICK_CHARGE_EFFECT_HANDLE, -1);
        lua_args! {
            fighter,
            Hash40::new("sys_bg_black"),
            0,
            0,
            0,
            0,
            0,
            0,
            1
        };
        smash::app::sv_animcmd::EFFECT_GLOBAL_BACK_GROUND(fighter.lua_state_agent);
        let handle = EffectModule::req_follow(
            fighter.boma(),
            Hash40::new("sys_aura_light"),
            Hash40::new("head"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            6.0,
            true,
            0,
            0,
            0,
            0,
            0,
            true,
            true
        ) as u32;

        let handle2 = EffectModule::req_follow(
            fighter.boma(),
            Hash40::new("sys_aura_light"),
            Hash40::new("head"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            6.0,
            true,
            0,
            0,
            0,
            0,
            0,
            true,
            true
        ) as u32;

        EffectModule::set_alpha(fighter.module_accessor, handle, 1.0);
        EffectModule::set_rgb(fighter.module_accessor, handle, 101.0 / 255.0, 32.0 / 255.0, 153.0 / 255.0);
        EffectModule::set_alpha(fighter.module_accessor, handle2, 1.0);
        EffectModule::set_rgb(fighter.module_accessor, handle2, 101.0 / 255.0, 32.0 / 255.0, 153.0 / 255.0);
        VarModule::set_int(fighter.battle_object, vars::metaknight::META_QUICK_EFFECT_HANDLE, handle as i32);
        VarModule::set_int(fighter.battle_object, vars::metaknight::META_QUICK_EFFECT_HANDLE2, handle2 as i32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        metaknight_catch_game,
        dash_game,
        //dash_effect,
        turn_dash_game,
        metaquick_sound,
        metaquick_effect
    );
}

