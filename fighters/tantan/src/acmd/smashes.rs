
use super::*;


#[acmd_script( agent = "tantan", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn tantan_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_A, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        //shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_B, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 14.0, 75, 100, 0, 48, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 16.0, 75, 100, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 16.0, 75, 100, 0, 48, 5.0, 0.0, 1.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_A, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 12.0, 82, 103, 0, 48, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 14.0, 82, 103, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 14.0, 82, 103, 0, 48, 5.0, 0.0, 1.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 10.0, 82, 103, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 12.0, 82, 103, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 12.0, 82, 103, 0, 48, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_B, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "tantan", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn tantan_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 4.5);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 4.5);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 40, 80, 0, 50, 4.5, 0.0, 2.5, 7.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 40, 80, 0, 50, 4.5, 0.0, 2.5, -7.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 30, 77, 0, 50, 5.0, 0.0, 2.5, 12.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 16.0, 30, 77, 0, 50, 5.0, 0.0, 2.5, -12.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    
}


#[acmd_script( agent = "tantan", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn game_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 5.0, 26, 100, 47, 0, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 5.0, 45, 100, 47, 0, 3.0, -0.7, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 5.0, 80, 100, 30, 0, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 8.0, 50, 109, 0, 60, 4.8, -1.6, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 8.0, 50, 109, 0, 60, 4.6, 5.6, -0.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "tantan", script = "effect_attacks4", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 3, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 1.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    let lr = PostureModule::lr(boma);
    if is_excute(fighter) {
        if(lr>=0.0){
            EFFECT_FLW_POS(fighter, Hash40::new("tantan_attack_dash"), Hash40::new("kneer"), 0, -1, 0, -4, 0, 0, 0.9, true);
            LAST_EFFECT_SET_COLOR(fighter, 3, 1.2, 0.5);
            LAST_EFFECT_SET_RATE(fighter, 2.4);
        }
        else{
            EFFECT_FLW_POS(fighter, Hash40::new("tantan_attack_dash"), Hash40::new("kneer"), 0, 1, 0, -4, 0, 0, 0.9, true);
            LAST_EFFECT_SET_RATE(fighter, 2.4);
        }
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("tantan_smash_line"), Hash40::new("top"), -0.5, 10, -24.0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "tantan", script = "sound_attacks4", category = ACMD_SOUND, low_priority)]
unsafe fn sound_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_tantan_punch_01"));
        PLAY_SE(fighter, Hash40::new("se_tantan_swing_m01"));
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_tantan_step_right_m"));
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_tantan_punch_06"));
        PLAY_SE(fighter, Hash40::new("se_tantan_smash_h01"));
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_tantan_landing01"));
    }
}
#[acmd_script( agent = "tantan", script = "expression_attacks4", category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}


#[acmd_script( agent = "tantan", script = "effect_attacks4charge", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_attacks4charge(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    for i in 1..i32::MAX{
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 5.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
    }
}
#[acmd_script( agent = "tantan", script = "sound_attacks4charge", category = ACMD_SOUND, low_priority)]
unsafe fn sound_attacks4charge(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    }
}
#[acmd_script( agent = "tantan", script = "expression_attacks4charge", category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_attacks4charge(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter,*MA_MSC_CMD_PHYSICS_START_CHARGE, 0.8, 0.6, -1, 0.8, 0.8, -1, Hash40::new("invalid"));
        smash::app::sv_module_access::physics(lua_state);
        fighter.pop_lua_stack(1);
        
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 61.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        tantan_attack_hi4_game,
        tantan_attack_lw4_game,
        
        game_attacks4,
        effect_attacks4,
        sound_attacks4,
        expression_attacks4,

        effect_attacks4charge,
        sound_attacks4charge,
        expression_attacks4charge,
    );
}

