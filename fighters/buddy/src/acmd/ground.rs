use super::*;


#[acmd_script( agent = "buddy", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 3.0, 80, 45, 0, 20, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 3.0, 80, 45, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 3.0, 80, 45, 0, 20, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        // Jab lock hitbox
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 361, 24, 0, 30, 3.0, 0.0, 5.0, 5.0, Some(0.0), Some(5.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);  
    }
}

#[acmd_script( agent = "buddy", script = "game_attack12" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 3.0, 75, 100, 30, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 75, 100, 30, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 75, 100, 30, 0, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        // Jab lock hitbox
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 361, 24, 0, 30, 3.0, 0.0, 5.0, 5.0, Some(0.0), Some(5.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
}

#[acmd_script( agent = "buddy", script = "game_attack13" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_13_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 4.0/(7.5-4.0));
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 4.0, 74, 98, 0, 72, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 4.0, 74, 98, 0, 72, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 4.0, 74, 98, 0, 72, 3.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("handl"), 4.0, 74, 98, 0, 72, 4.0, 4.0, 0.0, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 7.5);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0/(8.0-7.5));
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "buddy", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_FALL);
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_CONTINUE);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 66, 66, 0, 75, 6.0, 0.0, 5.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 66, 38, 0, 59, 5.5, 0.0, 5.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state,  23.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_DRIFT);
    }
}

#[acmd_script( agent = "buddy", script = "effect_attackdash", category = ACMD_EFFECT, low_priority )]
unsafe fn buddy_attack_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 0.85);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind_s"), Hash40::new("top"), 0, 6, 0, 0, 0, -90, 0.9, false, 0.2);
            LAST_EFFECT_SET_RATE(fighter, 1.6);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind_s"), Hash40::new("top"), 0, 5, 0, 0, 0, -90, 0.9, false, 0.5);
            LAST_EFFECT_SET_RATE(fighter, 1.6);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.6);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        buddy_attack_11_game,
        buddy_attack_12_game,
        buddy_attack_13_game,
        buddy_attack_dash_game,
        buddy_attack_dash_effect,
    );
}