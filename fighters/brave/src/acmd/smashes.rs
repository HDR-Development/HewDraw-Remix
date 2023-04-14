
use super::*;

#[acmd_script( agent = "brave", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn brave_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT);
        let rng = app::sv_math::rand(hash40("fighter"), 10);
        if rng > 1 {
            VarModule::off_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT);
        }
        else{
            VarModule::on_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        //FT_MOTION_RATE(fighter, 9.0/(15.0-7.0));
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.0/(16.5-15.0));
    }
    frame(lua_state, 16.5);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 5.0/(21.0-16.5));
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
        if VarModule::is_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT){
            WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
            //ATTACK(fighter, 1, 0, Hash40::new("armr"), 21.0, 361, 76, 0, 62, 3.0, -1.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 21.0, 361, 76, 0, 62, 3.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword1"), 24.0, 361, 76, 0, 62, 3.5, 1.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("sword1"), 24.0, 361, 76, 0, 62, 3.5, 4.5, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("sword1"), 24.0, 361, 76, 0, 62, 3.5, 9.5, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
        }
        else{
            //ATTACK(fighter, 1, 0, Hash40::new("armr"), 15.0, 42, 76, 0, 62, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 15.0, 42, 76, 0, 62, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword1"), 18.0, 42, 76, 0, 62, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("sword1"), 18.0, 42, 76, 0, 62, 3.5, 4.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("sword1"), 18.0, 42, 76, 0, 62, 3.5, 9.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 4.0);
    }
    
}

#[acmd_script( agent = "brave", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn brave_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT);
        let rng = app::sv_math::rand(hash40("fighter"), 10);
        if rng > 1 {
            VarModule::off_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT);
        }
        else{
            VarModule::on_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
        if VarModule::is_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT){
            WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 87, 76, 0, 50, 4.0, 0.0, 6.0, -5.0, Some(0.0), Some(6.0), Some(5.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 18.0, 87, 76, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 18.0, 87, 76, 0, 50, 3.0, -1.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("sword1"), 21.0, 90, 76, 0, 50, 3.5, 1.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 4, 0, Hash40::new("sword1"), 21.0, 90, 76, 0, 50, 3.5, 4.5, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 5, 0, Hash40::new("sword1"), 21.0, 90, 76, 0, 50, 3.5, 9.5, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 87, 76, 0, 60, 4.0, 0.0, 6.0, -5.0, Some(0.0), Some(6.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 14.0, 87, 76, 0, 60, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 14.0, 87, 76, 0, 60, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("sword1"), 16.0, 90, 76, 0, 60, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 4, 0, Hash40::new("sword1"), 16.0, 90, 76, 0, 60, 3.5, 4.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 5, 0, Hash40::new("sword1"), 16.0, 90, 76, 0, 60, 3.5, 9.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "brave", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn brave_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT);
        let rng = app::sv_math::rand(hash40("fighter"), 10);
        if rng > 1 {
            VarModule::off_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT);
        }
        else{
            VarModule::on_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
        if VarModule::is_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT){
            WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 22.0, 56, 113, 0, 25, 2.5, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -11, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 22.0, 56, 113, 0, 25, 3.0, -1.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -11, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("sword1"), 26.0, 56, 100, 0, 25, 3.5, 1.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("sword1"), 26.0, 56, 100, 0, 25, 3.5, 4.5, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 4, 0, Hash40::new("sword1"), 26.0, 56, 100, 0, 25, 3.5, 9.5, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 11.0, 56, 113, 0, 45, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 11.0, 56, 113, 0, 45, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("sword1"), 13.0, 56, 100, 0, 45, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("sword1"), 13.0, 56, 100, 0, 45, 3.5, 4.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 4, 0, Hash40::new("sword1"), 13.0, 56, 100, 0, 45, 3.5, 9.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::brave::status::IS_CRITICAL_HIT){
            WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 22.0, 46, 113, 0, 25, 2.5, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -11, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 22.0, 46, 113, 0, 25, 3.0, -1.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -11, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("sword1"), 26.0, 46, 100, 0, 25, 3.5, 1.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("sword1"), 26.0, 46, 100, 0, 25, 3.5, 4.5, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 4, 0, Hash40::new("sword1"), 26.0, 46, 100, 0, 25, 3.5, 9.5, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 11.0, 46, 113, 0, 45, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 11.0, 46, 113, 0, 45, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("sword1"), 13.0, 46, 100, 0, 45, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("sword1"), 13.0, 46, 100, 0, 45, 3.5, 4.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 4, 0, Hash40::new("sword1"), 13.0, 46, 100, 0, 45, 3.5, 9.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        brave_attack_s4_s_game,
        brave_attack_hi4_game,
        brave_attack_lw4_game,
    );
}

