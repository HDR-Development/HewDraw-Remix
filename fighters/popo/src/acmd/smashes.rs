
use super::*;


#[acmd_script( agent = "popo", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn popo_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 12.5, 361, 110, 0, 50, 5.0, 1.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        FT_MOTION_RATE(fighter, 2.0);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 110, 0, 50, 5.0, 0.0, 4.5, 2.0, Some(0.0), Some(4.5), Some(12.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "popo", script = "effect_attacks4" , category = ACMD_EFFECT , low_priority)]
unsafe fn popo_attack_s4_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.5);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("popo_smash_arc"), Hash40::new("popo_smash_arc"), Hash40::new("top"), -1, 9, 5, -146, -160, 80, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -4.5, 0, 6, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), -4.5, 0, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 8, 4, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_crown"), Hash40::new("sys_crown"), Hash40::new("top"), 7, 0, 5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
}

#[acmd_script( agent = "popo", script = "expression_attacks4", category = ACMD_EXPRESSION, low_priority )]
unsafe fn popo_attack_s4_s_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(lua_state, 5.0);
    app::sv_animcmd::execute(lua_state, 5.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3, true);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 8);
    }
}

#[acmd_script( agent = "popo", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn popo_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 11.0, 83, 110, 0, 50, 3.0, -0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("havel"), 11.0, 83, 110, 0, 50, 3.0, -0.8, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 2, 0, Hash40::new("havel"), 11.0, 83, 110, 0, 50, 5.0, -0.8, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 83, 110, 0, 50, 3.0, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

#[acmd_script( agent = "popo", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn popo_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("havel"), 12.0, 40, 110, 0, 50, 3.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 2, 0, Hash40::new("havel"), 12.0, 40, 110, 0, 50, 3.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "popo", script = "effect_attacklw4" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 6, 7, -10, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("popo_smash_arc_b"), Hash40::new("popo_smash_arc_b"), Hash40::new("top"), 1.5, 2.5, 1, 180, -75, 180, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
}

#[acmd_script( agent = "popo", script = "expression_attacklw4", category = ACMD_EXPRESSION, low_priority )]
unsafe fn popo_attack_lw4_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 8, true);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 4);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_POPO_STATUS_ATTACK_LW4_FLAG_NANA_START_TURN);
    }
}

pub fn install() {
    install_acmd_scripts!(
        popo_attack_s4_s_game,
        popo_attack_s4_s_effect,
        popo_attack_s4_s_expression,
        popo_attack_hi4_game,
        popo_attack_lw4_game,
        effect_attacklw4,
        popo_attack_lw4_expression,
    );
}

