use super::*;

#[acmd_script( agent = "jack", script = "game_attacks3hi" , category = ACMD_GAME , low_priority)]
unsafe fn jack_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("knife"), 3.0, 180, 100, 10, 0, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("knife"), 3.0, 35, 100, 20, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 35, 100, 25, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("shoulderr"), 3.0, 35, 100, 25, 0, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("knife"), 5.0, 40, 110, 0, 45, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("knife"), 5.0, 40, 110, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 5.0, 40, 110, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("shoulderr"), 5.0, 40, 110, 0, 45, 3.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            AttackModule::set_size(boma, 2, 0.0);
            ATTACK(fighter, 0, 0, Hash40::new("knife"), 10.0, 45, 65, 0, 60, 4.5, 0.0, 0.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 1, 0, Hash40::new("knife"), 10.0, 45, 65, 0, 60, 4.5, 0.0, 3.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 10.0, 45, 65, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("shoulderr"), 10.0, 45, 65, 0, 60, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 18.0);
    if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        FT_MOTION_RATE(fighter, 19.0/(35.0-18.0));
    }
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "jack", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn jack_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("knife"), 3.0, 93, 100, 20, 0, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("knife"), 3.0, 40, 100, 20, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 35, 100, 25, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("shoulderr"), 3.0, 35, 100, 25, 0, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("knife"), 5.0, 40, 110, 0, 45, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("knife"), 5.0, 40, 110, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 5.0, 40, 110, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("shoulderr"), 5.0, 40, 110, 0, 45, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            AttackModule::set_size(boma, 2, 0.0);
            ATTACK(fighter, 0, 0, Hash40::new("knife"), 10.0, 40, 65, 0, 60, 4.5, 0.0, 0.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 1, 0, Hash40::new("knife"), 10.0, 40, 65, 0, 60, 4.5, 0.0, 3.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 10.0, 40, 65, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("shoulderr"), 10.0, 40, 65, 0, 60, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 18.0);
    if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        FT_MOTION_RATE(fighter, 19.0/(35.0-18.0));
    }
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

#[acmd_script( agent = "jack", script = "game_attacks3lw" , category = ACMD_GAME , low_priority)]
unsafe fn jack_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("knife"), 3.0, 93, 100, 10, 0, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("knife"), 3.0, 40, 100, 20, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 35, 100, 25, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("shoulderr"), 3.0, 35, 100, 25, 0, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("knife"), 5.0, 40, 105, 0, 45, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("knife"), 5.0, 40, 105, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 5.0, 40, 105, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("shoulderr"), 5.0, 40, 105, 0, 45, 3.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            AttackModule::set_size(boma, 2, 0.0);
            ATTACK(fighter, 0, 0, Hash40::new("knife"), 10.0, 35, 61, 0, 60, 4.5, 0.0, 0.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 1, 0, Hash40::new("knife"), 10.0, 35, 61, 0, 60, 4.5, 0.0, 3.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 10.0, 35, 61, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("shoulderr"), 10.0, 35, 61, 0, 60, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 18.0);
    if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        FT_MOTION_RATE(fighter, 19.0/(35.0-18.0));
    }
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

#[acmd_script( agent = "jack", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn jack_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(9.0-1.0));
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"),2.0, 115, 100, 80, 0, 2.5, -1.0, 0.0, 0.0, Some(1.0), Some(0.0), Some(0.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("knife"), 2.0, 115, 100, 80, 0, 2.0, 0.0, 1.2, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("knife"), 2.0, 115, 100, 80, 0, 2.0, 0.0, 3.3, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("knife"), 5.0, 90, 116, 0, 75, 5.0, 0.0, 1.7, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
            ATTACK(fighter, 2, 1, Hash40::new("knife"), 5.0, 90, 144, 0, 55, 5.0, 0.0, 1.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE_RANGE(fighter, 22.0, 27.0, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

#[acmd_script( agent = "jack", script = "effect_attackhi3" , category = ACMD_EFFECT , low_priority)]
unsafe fn jack_attack_hi3_effect (fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_jack_sword1"), Hash40::new("tex_jack_sword2"), 4, Hash40::new("knife"), 0.0, 0.25, 0.15, Hash40::new("knife"), 0.0, 5.8, 0.0, true, Hash40::new("jack_knife"), Hash40::new("knife"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("jack_knife_front"), Hash40::new("knife"), 0, 0, 0, 0, 180, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 0);
        EFFECT_OFF_KIND(fighter, Hash40::new("jack_knife_front"), false, true);
        EFFECT_FOLLOW(fighter, Hash40::new("jack_knife_spin"), Hash40::new("knife"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("jack_doyle_lightning"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("jack_knife_spin"), false, true);
    }

}

#[acmd_script( agent = "jack", script = "expression_attackhi3" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn jack_attack_hi3_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("knife"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashs"), 0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "jack", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn jack_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 35, 35, 0,60, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(2.0), Some(18.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-1.0), Some(26.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-4.0), Some(34.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

#[acmd_script( agent = "jack", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn jack_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 4, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("jack_gun_muzzle"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }

}

#[acmd_script( agent = "jack", script = "expression_attacklw3" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn jack_attack_lw3_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "jack", script = "game_attacklw3_ex" , category = ACMD_GAME , low_priority)]
unsafe fn jack_attack_lw3_ex_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if fighter.is_flag(*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        FT_MOTION_RATE(fighter, 8.0/(7.0-1.0));
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 20, 0, 20, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(2.0), Some(18.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-1.0), Some(26.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-4.0), Some(34.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE_RANGE(fighter, 12.0, 16.0, 2.0);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 20, 0, 20, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(2.0), Some(18.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-1.0), Some(26.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-4.0), Some(34.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 35, 50, 0, 60, 2.5, 0.0, 7.0, 5.0, Some(0.0), Some(2.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-1.0), Some(26.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-4.0), Some(34.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }  

}

#[acmd_script( agent = "jack", script = "effect_attacklw3_ex", category = ACMD_EFFECT, low_priority )]
unsafe fn jack_attack_lw3_ex_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_gunspecial_muzzle"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_gunspecial_muzzle"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_gunspecial_muzzle"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }

}

#[acmd_script( agent = "jack", script = "expression_attacklw3_ex", category = ACMD_EXPRESSION, low_priority )]
unsafe fn jack_attack_lw3_ex_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "jack_doyle", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn jack_doyle_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("jack_doyle_magic_flash"), Hash40::new("handl"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_doyle_magic_flash2"), Hash40::new("top"), 2, 10, 11, 0, 0, 0, 0.07, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_doyle_magic_flash2"), Hash40::new("handl"), 2, 0, 0, 0, 0, 0, 0.07, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_doyle_magic_flash2"), Hash40::new("handl"), 2, 0, 0, 0, 0, 0, 0.07, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        jack_attack_s3_hi_game,
        jack_attack_s3_s_game,
        jack_attack_s3_lw_game,
        jack_attack_hi3_game,
        jack_attack_hi3_effect,
        jack_attack_hi3_expression,
        jack_attack_lw3_game,
        jack_attack_lw3_effect,
        jack_attack_lw3_expression,
        jack_attack_lw3_ex_game,
        jack_attack_lw3_ex_effect,
        jack_attack_lw3_ex_expression,
        jack_doyle_attack_lw3_effect,
    );
}