
use super::*;


#[acmd_script( agent = "lucas", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 83, 68, 0, 60, 2.5, 0.0, 5.5, 1.2, Some(0.0), Some(5.5), Some(6.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 83, 68, 0, 70, 5.8, 0.0, 5.5, 12.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 361, 90, 0, 50, 2.5, 0.0, 5.5, 1.2, Some(0.0), Some(5.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 361, 90, 0, 40, 5.8, 0.0, 5.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
    }
    
}

#[acmd_script( agent = "lucas", script = "effect_attackdash" , category = ACMD_EFFECT , low_priority)]
unsafe fn lucas_attack_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x0e74ad60dd), Hash40::new("haver"), -0.200000003, 0, 0.200000003, 0, 0, 0, 0.800000012, true);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new_raw(0x0d0679b24d), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.899999976, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0.0, 5.0, 12.5, 0, 0, 0, 0.899999976, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0e74ad60dd), false, false);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
    
}

#[acmd_script( agent = "lucas", script = "sound_attackdash" , category = ACMD_SOUND , low_priority)]
unsafe fn lucas_attack_dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new_raw(0x14b1c72abf));
        PLAY_SE(fighter, Hash40::new_raw(0x13f2de7da4));
    }
    wait(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new_raw(0x148a41094f));
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        lucas_attack_dash_game,
        lucas_attack_dash_effect,
        lucas_attack_dash_sound,
    );
}

