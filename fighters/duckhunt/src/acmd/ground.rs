
use super::*;
#[acmd_script( agent = "duckhunt", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attackdash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8);
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 6.0/(10.0-1.0));
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 40, 0, 100, 4.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 40, 0, 100, 4.0, 0.0, 7.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 60, 40, 0, 100, 4.0, 0.0, 7.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 105, 40, 0, 90, 3.5, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 105, 40, 0, 90, 3.5, 0.0, 7.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 105, 40, 0, 90, 3.5, 0.0, 7.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    
}


#[acmd_script( agent = "duckhunt", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  2.5,  68,  100,  30,  0,  3.0,  0.0,  4.0,  6.0,  None,  None,  None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA_d,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_PUNCH); 
        ATTACK(fighter,  1,  0,  Hash40::new("top"),  2.5,  75,  100,  27,  0,  3.0,  0.0,  4.0,  8.5,  None,  None,  None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA_d,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_PUNCH); 
        ATTACK(fighter,  2,  0,  Hash40::new("top"),  2.5,  82,  100,  25,  0,  3.0,  0.0,  4.0,  11.5,  None,  None,  None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA_d,  *COLLISION_CATEGORY_MASK_FIGHTER,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_PUNCH);
        //Locking hitboxes
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 361, 30, 0, 20, 3.0,  0.0,  4.0,  6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 2.5, 361, 20, 0, 15, 3.0,  0.0,  4.0,  8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 2.5, 361, 20, 0, 15, 3.0,  0.0,  4.0,  11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma,  *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    } 
}


#[acmd_script( agent = "duckhunt", script = "game_attack12" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attack_12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  2.5,  60,  100,  35,  0,  3.0,  0.0,  3.5,  2.0,  None,  None,  None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA_d,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
        ATTACK(fighter,  1,  0,  Hash40::new("top"),  2.5,  75,  100,  25,  0,  5.0,  0.0,  7.5,  5.5,  None,  None,  None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA_d,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
        //Locking hitboxes
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 30, 0, 20, 3.0,  0.0,  3.5,  2.0,  None,  None,  None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 361, 20, 0, 15, 5.0,  0.0,  7.5,  5.5,  None,  None,  None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "duckhunt", script = "game_attack13" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attack_13_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  5.0,  45,  70,  0,  50,  4.0,  0.0,  4.5,  4.5,  None, None, None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_M,  *COLLISION_SOUND_ATTR_KICK,  *ATTACK_REGION_KICK);
        ATTACK(fighter,  1,  0,  Hash40::new("top"),  5.0,  45,  70,  0,  50,  4.0,  0.0,  4.5,  9.0,  None, None, None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_M,  *COLLISION_SOUND_ATTR_KICK,  *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    FT_MOTION_RATE_RANGE(fighter, 6.0, 28.0, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "duckhunt", script = "game_attack100sub" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attack_100_sub_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 0.5);
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  0.4,  361,  15,  0,  16,  4.5,  0.0,  7.5,  4.5,  None,  None,  None,  0.5,  0.4,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_CUTUP,  *ATTACK_REGION_HEAD);
        ATTACK(fighter,  1,  0,  Hash40::new("top"),  0.4,  361,  10,  0,  12,  6.5,  0.0,  7.5,  14.0,  Some(0.0),  Some(7.5),  Some(6.5),  0.5,  0.4,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_CUTUP,  *ATTACK_REGION_HEAD);
        AttackModule::set_add_reaction_frame(boma,  0,  2.0,  false);
        AttackModule::set_add_reaction_frame(boma,  1,  2.0,  false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 5.0);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma,  *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

#[acmd_script( agent = "duckhunt", script = "game_attack100end" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attack_100_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 5.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  3.0,  80,  140,  0,  70,  5.5,  0.0,  6.5,  7.0,  None,  None,  None,  2.0,  1.0,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
        ATTACK(fighter,  1,  0,  Hash40::new("top"),  3.0,  80,  140,  0,  70,  5.5,  0.0,  6.5,  12.0,  None,  None,  None,  2.0,  1.0,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
        ATTACK(fighter,  2,  0,  Hash40::new("top"),  3.0,  80,  140,  0,  70,  5.5,  0.0,  6.5,  17.0,  None,  None,  None,  2.0,  1.0,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter,  36.3/(39.0-6.0));
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        duckhunt_attackdash_game,
        duckhunt_attack_11_game,
        duckhunt_attack_12_game,
        duckhunt_attack_13_game,
        duckhunt_attack_100_sub_game,
        duckhunt_attack_100_end_game,
    );
}

