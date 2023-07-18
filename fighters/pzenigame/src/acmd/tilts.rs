
use super::*;


#[acmd_script( agent = "pzenigame", script = "game_attacks3hi" , category = ACMD_GAME , low_priority)]
unsafe fn pzenigame_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 38, 75, 0, 40, 3.5, 0.0, 7.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_WATER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 38, 75, 0, 40, 4.0, 0.0, 9.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_WATER);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 38, 75, 0, 40, 3.5, 0.0, 11.4, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_WATER);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "pzenigame", script = "effect_attacks3hi" , category = ACMD_EFFECT , low_priority)]
unsafe fn pzenigame_attack_s3_hi_effect(fighter: &mut L2CAgentBase) {    
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 3, 5, 0, -50, 12, 0.7, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pzenigame_water_attack"), Hash40::new("top"), 0, 0, 8, 0, 0, 0, 1.0, true);
    }

}

#[acmd_script( agent = "pzenigame", script = "sound_attacks3hi" , category = ACMD_SOUND , low_priority)]
unsafe fn pzenigame_attack_s3_hi_sound(fighter: &mut L2CAgentBase) {    
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pzenigame_attackhard_l01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pzenigame_rnd_attack"));
    }

}


#[acmd_script( agent = "pzenigame", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn pzenigame_attack_s3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 38, 75, 0, 40, 3.5, 0.0, 6.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_WATER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 38, 75, 0, 40, 4.0, 0.0, 6.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_WATER);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 38, 75, 0, 40, 3.5, 0.0, 6.4, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_WATER);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "pzenigame", script = "effect_attacks3" , category = ACMD_EFFECT , low_priority)]
unsafe fn pzenigame_attack_s3_effect(fighter: &mut L2CAgentBase) {    
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 3, 5, 0, -50, 12, 0.7, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pzenigame_water_attack"), Hash40::new("top"), 0, 0, 8, 0, 0, 0, 1.0, true);
    }

}

#[acmd_script( agent = "pzenigame", script = "sound_attacks3" , category = ACMD_SOUND , low_priority)]
unsafe fn pzenigame_attack_s3_sound(fighter: &mut L2CAgentBase) {    
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pzenigame_attackhard_l01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pzenigame_rnd_attack"));
    }

}

#[acmd_script( agent = "pzenigame", script = "game_attacks3lw" , category = ACMD_GAME , low_priority)]
unsafe fn pzenigame_attack_s3_lw_game(fighter: &mut L2CAgentBase) {    
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 38, 75, 0, 40, 3.5, 0.0, 3.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_WATER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 38, 75, 0, 40, 4.0, 0.0, 3.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_WATER);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 38, 75, 0, 40, 3.5, 0.0, 3.4, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_WATER);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

#[acmd_script( agent = "pzenigame", script = "effect_attacks3lw" , category = ACMD_EFFECT , low_priority)]
unsafe fn pzenigame_attack_s3_lw_effect(fighter: &mut L2CAgentBase) {    
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 3, 5, 0, -50, 12, 0.7, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pzenigame_water_attack"), Hash40::new("top"), 0, 0, 8, 0, 0, 0, 1.0, true);
    }

}

#[acmd_script( agent = "pzenigame", script = "sound_attacks3lw" , category = ACMD_SOUND , low_priority)]
unsafe fn pzenigame_attack_s3_lw_sound(fighter: &mut L2CAgentBase) {    
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pzenigame_attackhard_l01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pzenigame_rnd_attack"));
    }

}

#[acmd_script( agent = "pzenigame", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn pzenigame_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("head"), 6.0, 88, 90, 0, 50, 5.0, 1.7, 0.7, 0.7, Some(1.7), Some(0.7), Some(0.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 6.0, 88, 90, 0, 50, 5.2, 1.7, 1.2, 1.2, Some(1.7), Some(1.2), Some(1.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 88, 90, 0, 50, 4.0, 0.0, 3.0, -3.5, Some(0.0), Some(3.0), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 2, false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "pzenigame", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn pzenigame_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("tail1"), 6.0, 102, 45, 0, 75, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 1, 0, Hash40::new("tail2"), 6.0, 102, 45, 0, 75, 3.0, 0.5, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 2, 0, Hash40::new("tail2"), 6.0, 102, 45, 0, 75, 4.0, 4.2, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}
#[acmd_script( agent = "pzenigame", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn pzenigame_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), -2, 3, 5.5, 0, -30, 191, 0.95, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    if PostureModule::lr(boma)<0.0 {
        FOOT_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
    else {
        FOOT_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }

}

#[acmd_script( agent = "pzenigame", script = "sound_attacklw3" , category = ACMD_SOUND , low_priority)]
unsafe fn pzenigame_attack_lw3_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pzenigame_attackhard_s01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pzenigame_rnd_attack"));
    }

}

pub fn install() {
    install_acmd_scripts!(
        pzenigame_attack_s3_hi_game,
        pzenigame_attack_s3_hi_effect,
        pzenigame_attack_s3_hi_sound,
        pzenigame_attack_s3_game,
        pzenigame_attack_s3_effect,
        pzenigame_attack_s3_sound,
        pzenigame_attack_s3_lw_game,
        pzenigame_attack_s3_lw_effect,
        pzenigame_attack_s3_lw_sound,
        pzenigame_attack_hi3_game,
        pzenigame_attack_lw3_game,
        pzenigame_attack_lw3_effect,
        pzenigame_attack_lw3_sound,
    );
}

