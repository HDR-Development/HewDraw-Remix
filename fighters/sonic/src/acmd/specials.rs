
use super::*;


#[acmd_script( agent = "sonic", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::shoot_exist(boma, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }   
}
#[acmd_script( agent = "sonic_gimmickjump", script = "game_fall" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_gimmick_jump_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
if is_excute(fighter) {
    ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 90, 90, 0, 30, 6.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "sonic" , scripts = ["game_speciallwhold", "game_specialairlwhold", "game_speciallwholdchargehi", "game_speciallwholdchargelw", "game_speciallwholdchargemiddle"] , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    for i in 1..50 {
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("rot"), 1.0, 90, 0, 0, 58, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
}

#[acmd_script( agent = "sonic", script = "game_speciallwstart" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, false);
    }
    
}

#[acmd_script( agent = "sonic", script = "game_specialairlwstart" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_air_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, false);
    }
    
}

#[acmd_script( agent = "sonic", script = 0x1b07509826, category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_lw_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        AttackModule::clear_all(boma);
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.0, 60, 40, 0, 100, 4.0, 0.0, 1.5, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_captured_same_time_attack(boma, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_ATTACK_ID_DEFAULT, true);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
}

#[acmd_script( agent = "sonic", script = "game_specialnhomingstart" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_n_homing_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
 if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 40.0, 0.0, 10.0, 10.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
        }

}

#[acmd_script( agent = "sonic", script = "game_specialnhoming" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_n_homing_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    let mut bone_hash = match WorkModule::is_flag(boma, *FIGHTER_SONIC_STATUS_SPECIAL_N_HOMING_FLAG_IS_KIRBY){
        true => Hash40::new("rot"),
        false => Hash40::new("hip")
    };

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        FT_MOTION_RATE(fighter, 0.013);
        ATTACK(fighter, 0, 0, bone_hash, 8.0, 80, 80, 0, 45, 5.0, 0.5, 1.5, 0.0, Some(0.5), Some(1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
        AttackModule::set_captured_same_time_attack(boma, 0, true);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
    
}

#[acmd_script( agent = "sonic", script = "sound_specialnhoming" , category = ACMD_SOUND )]
unsafe fn sonic_special_n_homing_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);
    let mut sound = "";

    if rng == 0 { sound = "vc_sonic_004"; }
    else { sound = "vc_sonic_attack02"; }

    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_sonic_rounddash"));
        PLAY_SE(fighter, Hash40::new(sound));
        PLAY_SE(fighter, Hash40::new("se_sonic_special_n01"));
    }
}

#[acmd_script( agent = "sonic", script = "game_specialnhit" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_n_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        let temp = Vector3f { x: -0.3, y: 1.0, z: 0.0 };
		KineticModule::add_speed(boma, &temp);
    }
    FT_MOTION_RATE(fighter, 0.5);
    
}

#[acmd_script( agent = "sonic", script = "effect_specialnhit" , category = ACMD_EFFECT , low_priority)]
unsafe fn sonic_special_n_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
}
#[acmd_script( agent = "sonic", script = "game_specialnlanding" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_n_landing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

pub fn install() {
    install_acmd_scripts!(
        sonic_special_hi_game,
        sonic_special_lw_hold_game,
        sonic_special_lw_start_game,
        sonic_special_air_lw_start_game,
        sonic_special_lw_dash_game,
        sonic_special_n_homing_game,
        sonic_special_n_hit_game,
        sonic_special_n_hit_effect,
        sonic_special_n_homing_sound,
        sonic_special_n_landing,
        sonic_gimmick_jump_game,
        sonic_special_n_homing_start_game
        
    );
}

