
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

#[acmd_script( agent = "sonic", script = "game_speciallwhold" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if VarModule::is_flag(fighter.battle_object, vars::sonic::status::PULSE_HITBOX) {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 1.0, 365, 0, 0, 58, 8.3, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            VarModule::off_flag(fighter.battle_object, vars::sonic::status::PULSE_HITBOX)
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
        ATTACK(fighter, 0, 0, bone_hash, 8.0, 72, 120, 0, 15, 5.0, 0.5, 1.5, 0.0, Some(0.5), Some(1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
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
    FT_MOTION_RATE(fighter, 0.6);
    
}

#[acmd_script( agent = "sonic", script = "effect_specialnhit" , category = ACMD_EFFECT , low_priority)]
unsafe fn sonic_special_n_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
}

pub fn install() {
    install_acmd_scripts!(
        sonic_special_hi_game,
        sonic_special_lw_hold_game,
        sonic_special_lw_start_game,
        sonic_special_air_lw_start_game,
        sonic_special_n_homing_game,
        sonic_special_n_hit_game,
        sonic_special_n_hit_effect,
        sonic_special_n_homing_sound
    );
}

