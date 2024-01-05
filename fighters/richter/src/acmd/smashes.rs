
use super::*;


#[acmd_script( agent = "richter", script = "game_attacks4hi" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_s4_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 74, 0, 60, 6.5, 0.0, 9.5, 17.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 361, 80, 0, 60, 5.0, 0.0, 11.0, 28.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 361, 80, 0, 60, 4.0, 0.0, 12.5, 36.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 18.0, 361, 80, 0, 60, 3.5, 0.0, 13.5, 43.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 18.0, 361, 80, 0, 60, 3.0, 0.0, 14.5, 49.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        /*
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            ATTACK(fighter, 3, 0, Hash40::new("top"), 15.0, 115, 40, 0, 65, 5.0, 0.0, 12.0, 40.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
            ATTACK(fighter, 4, 0, Hash40::new("top"), 15.0, 115, 40, 0, 65, 5.0, 0.0, 13.0, 45.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        }
        */
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "richter", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 74, 0, 60, 6.5, 0.0, 7.5, 17.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 361, 80, 0, 60, 5.0, 0.0, 7.5, 28.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 361, 80, 0, 60, 4.0, 0.0, 7.5, 36.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 18.0, 361, 80, 0, 60, 3.5, 0.0, 7.5, 43.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 18.0, 361, 80, 0, 60, 3.0, 0.0, 7.5, 50.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        /*
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            ATTACK(fighter, 3, 0, Hash40::new("top"), 15.0, 115, 40, 0, 65, 5.0, 0.0, 7.0, 40.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
            ATTACK(fighter, 4, 0, Hash40::new("top"), 15.0, 115, 40, 0, 65, 5.0, 0.0, 7.0, 45.5, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        }
        */
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "richter", script = "game_attacks4lw" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_s4_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 74, 0, 60, 6.0, 0.0, 6.5, 17.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 361, 80, 0, 60, 5.0, 0.0, 4.5, 28.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 361, 80, 0, 60, 4.0, 0.0, 3.0, 36.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 18.0, 361, 80, 0, 60, 3.5, 0.0, 2.0, 43.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 18.0, 361, 80, 0, 60, 3.0, 0.0, 1.0, 49.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        /*
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            ATTACK(fighter, 3, 0, Hash40::new("top"), 15.0, 115, 40, 0, 65, 5.0, 0.0, 2.0, 40.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
            ATTACK(fighter, 4, 0, Hash40::new("top"), 15.0, 115, 40, 0, 65, 5.0, 0.0, 1.0, 45.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        }
        */
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "richter", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 93, 100, 0, 60, 6.5, 0.0, 27.5, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 93, 100, 0, 60, 5.5, 0.0, 27.5, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 93, 100, 0, 60, 5.5, 0.0, 27.5, 10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 93, 90, 0, 60, 5.5, 0.0, 17.5, 3.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 8.0, 93, 80, 0, 60, 5.5, 0.0, 10.5, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
      
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attackhi4charge" , category = ACMD_GAME , low_priority)]
unsafe fn richter_whip_attack_hi4_charge_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn richter_whip_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }    
}


#[acmd_script( agent = "richter", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 48, 85, 0, 65, 6.0, 0.0, 4.5, 10.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 48, 85, 0, 65, 4.5, 0.0, 3.5, 20.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 48, 82, 0, 65, 4.5, 0.0, 3.0, 29.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        /*
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 109, 57, 0, 70, 5.5, 0.0, 3.0, 28.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        }
        */
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 48, 85, 0, 65, 6.0, 0.0, 4.5, -6.2, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 48, 85, 0, 65, 4.5, 0.0, 3.5, -14.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 48, 82, 0, 65, 4.5, 0.0, 3.0, -23.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        /*
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 109, 62, 0, 70, 4.0, 0.0, 3.0, -21.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        }
        */
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "richter_whip", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn richter_whip_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_s4_hi_game,
        richter_attack_s4_s_game,
        richter_attack_s4_lw_game,
        richter_attack_hi4_game,
        richter_whip_attack_hi4_game,
        richter_whip_attack_hi4_charge_game,
        richter_attack_lw4_game,
        richter_whip_attack_lw4_game,
    );
}

