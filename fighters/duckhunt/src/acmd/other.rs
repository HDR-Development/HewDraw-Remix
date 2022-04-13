
use super::*;

#[acmd_script( agent = "duckhunt", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_catch_game(fighter: &mut L2CAgentBase) {
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
        CATCH(fighter, 0, Hash40::new("top"), 4.3, 0.0, 6.6, 0.0, Some(0.0), Some(6.6), Some(11.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "duckhunt", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "duckhunt", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footl"), 2, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "duckhunt", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "duckhunt_can", script = "game_explode" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_can_explode_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 60, 100, 0, 40, 13.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 60, 95, 0, 40, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        AREA_WIND_2ND_RAD_arg9(fighter, 0, 2, 0.05, 300, 1, 0, 0, 26, 60);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "duckhunt_clay" , script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_clay_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  2.0,  75,  50,  0,  20,  1.0,  0.0,  0.0,  0.0,  None,  None,  None,  4.0,  0.0,  *ATTACK_SETOFF_KIND_THRU,  *ATTACK_LR_CHECK_F,  false,  -1,  0.0,  0,  true,  false,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *WEAPON_DUCKHUNT_CLAY_INSTANCE_WORK_ID_FLAG_IS_ADD_ACCEL_Y);
    }
}

#[acmd_script( agent = "duckhunt_gunman" , scripts = ["sound_readyr", "sound_readyl"] , category = ACMD_SOUND , low_priority)]
unsafe fn duckhunt_gunman_ready_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_duckhunt_special_l02"));
    }
    frame(lua_state, 275.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_duckhunt_special_l09"));
    } 
}

#[acmd_script( agent = "duckhunt_gunman" , script = "effect_readyl" , category = ACMD_EFFECT , low_priority)]
unsafe fn duckhunt_gunman_ready_effectl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 275.0);
    let gunman_kind = WorkModule::get_int(fighter.boma(), *WEAPON_DUCKHUNT_GUNMAN_INSTANCE_WORK_ID_KIND);
    if is_excute(fighter) {
        match gunman_kind {
            0 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 13.3, 0.74, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 13.3, -0.78, 0, 0, 0, 1, true);
            }
            1 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 15.66, 0.42, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 15.66, -0.5, 0, 0, 0, 1, true);
            }
            2 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 16.92, 0.26, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 16.92, -1.29, 0, 0, 0, 1, true);
            }
            3 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 10.9, 0.85, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 10.9, -0.64, 0, 0, 0, 1, true);
            }
            4 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 14.17, 0.4, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 14.16, -1.36, 0, 0, 0, 1, true);
            }
            _ => {
                return
            }
        }
    }
}

#[acmd_script( agent = "duckhunt_gunman" , script = "effect_readyr" , category = ACMD_EFFECT , low_priority)]
unsafe fn duckhunt_gunman_ready_effectr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 275.0);
    let gunman_kind = WorkModule::get_int(fighter.boma(), *WEAPON_DUCKHUNT_GUNMAN_INSTANCE_WORK_ID_KIND);
    if is_excute(fighter) {
        match gunman_kind {    
            0 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 13.3, 0.74, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 13.3, -0.78, 0, 0, 0, 1, true);
            }
            1 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 15.66, 0.42, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 15.66, -0.5, 0, 0, 0, 1, true);
            }
            2 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 16.92, 0.26, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 16.92, -1.29, 0, 0, 0, 1, true);
            }
            3 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 10.9, 0.85, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 10.9, -0.64, 0, 0, 0, 1, true);
            }
            4 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 14.17, 0.4, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 14.16, -1.36, 0, 0, 0, 1, true);
            }
            _ => {
                return
            }
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        duckhunt_catch_game,
        dash_game,
        dash_effect,
        turn_dash_game,
        duckhunt_can_explode_game,
        duckhunt_clay_fly_game,
        duckhunt_gunman_ready_effectr,
        duckhunt_gunman_ready_effectl,
        duckhunt_gunman_ready_sound,
    );
}

