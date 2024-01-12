
use super::*;

#[acmd_script( agent = "ike", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn ike_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 4.0, 65, 100, 20, 1, 3.0, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 4.0, 65, 100, 20, 1, 3.0, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 4.0, 70, 100, 20, 1, 3.5, 3.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 361, 100, 20, 1, 3.0, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        //WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

#[acmd_script( agent = "ike", script = "game_attack12" , category = ACMD_GAME , low_priority)]
unsafe fn ike_attack_12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 3.0, 65, 100, 30, 0, 2.75, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 3.0, 65, 100, 30, 0, 3.5, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 3.0, 65, 100, 30, 0, 4.0, 4.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 361, 100, 30, 0, 3.0, 0.0, 4.0, 11.0, Some(0.0), Some(4.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "ike", script = "game_attack13" , category = ACMD_GAME , low_priority)]
unsafe fn ike_attack_13_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 45, 100, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 6.0, 45, 100, 0, 45, 2.75, 1.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("sword"), 7.0, 45, 100, 0, 45, 4.0, 0.0, 1.0, -1.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("sword"), 6.0, 45, 100, 0, 45, 4.0, 0.0, 7.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("sword"), 5.0, 45, 100, 0, 45, 4.0, 0.0, 12.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ike", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn ike_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.1);
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 12.0, 361, 75, 0, 50, 2.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 361, 75, 0, 50, 2.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("sword"), 14.0, 361, 75, 0, 50, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("sword"), 12.0, 361, 75, 0, 50, 4.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("sword"), 10.0, 35, 75, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 9.0, 361, 75, 0, 30, 2.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.0, 361, 75, 0, 30, 2.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("sword"), 10.0, 361, 75, 0, 30, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("sword"), 9.0, 361, 75, 0, 30, 4.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("sword"), 7.0, 35, 75, 0, 30, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "ike", script = "expression_attackdash", category = ACMD_EXPRESSION, low_priority )]
unsafe fn ike_attack_dash_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_arg10(fighter, 0, 1, 80, 300, 0.8, 0, 12, 24, 24, 50);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(boma, 0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ike_attack_11_game,
        ike_attack_12_game,
        ike_attack_13_game,
        ike_attack_dash_game,
        ike_attack_dash_expression,
    );
}

