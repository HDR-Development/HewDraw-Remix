
use super::*;

#[acmd_script( agent = "gamewatch", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_gamewatch_rnd_futtobi01"), Hash40::new("seq_gamewatch_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "gamewatch", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_gamewatch_rnd_futtobi01"), Hash40::new("seq_gamewatch_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "gamewatch", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_gamewatch_rnd_futtobi01"), Hash40::new("seq_gamewatch_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "gamewatch", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_gamewatch_rnd_futtobi01"), Hash40::new("seq_gamewatch_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "gamewatch", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_gamewatch_rnd_futtobi01"), Hash40::new("seq_gamewatch_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "gamewatch", script = "game_landingairf" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_landing_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "gamewatch", script = "game_landingairb" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_landing_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("landing_air_b"), false, 1.0);
    }
    
}

#[acmd_script( agent = "gamewatch", script = "expression_landingairlw" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn gamewatch_landing_air_lw_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        VisibilityModule::set_int64(boma, Hash40::new("head").hash as i64, Hash40::new("head_close").hash as i64);
        VisibilityModule::set_int64(boma, Hash40::new("hand").hash as i64, Hash40::new("hand_hold_lr").hash as i64);
        VisibilityModule::set_int64(boma, Hash40::new("lhand").hash as i64, Hash40::new("lhand_key").hash as i64);
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *DAMAGE_NO_REACTION_MODE_NORMAL, *WEAPON_INKLING_ROLLER_INSTANCE_WORK_ID_FLOAT_B);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, 0 as u32);
    }
    
}

#[acmd_script( agent = "gamewatch", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_catch_game(fighter: &mut L2CAgentBase) {
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
        CATCH(fighter, 0, Hash40::new("top"), 4.2, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(9.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.220);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "gamewatch", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "gamewatch", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "gamewatch", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "gamewatch", script = "sound_appealhil", category = ACMD_SOUND, low_priority)]
unsafe fn sound_appealhil(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

#[acmd_script( agent = "gamewatch", script = "sound_appealhir", category = ACMD_SOUND, low_priority)]
unsafe fn sound_appealhir(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        gamewatch_landing_air_f_game,
        gamewatch_landing_air_b_game,
        gamewatch_landing_air_lw_expression,
        gamewatch_catch_game,
        dash_game,
        //dash_effect,
        turn_dash_game,
        sound_appealhil,
        sound_appealhir,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}

