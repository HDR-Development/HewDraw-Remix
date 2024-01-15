
use super::*;


#[acmd_script( agent = "ridley", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME )]
unsafe fn ridley_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 6.0/(15.0 - 1.0));
    }
}

#[acmd_script( agent = "ridley", scripts = ["game_specialnhold", "game_specialairnhold"], category = ACMD_GAME )]
unsafe fn ridley_special_n_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
}

#[acmd_script( agent = "ridley", scripts = ["game_specialnshoot", "game_specialairnshoot"], category = ACMD_GAME )]
unsafe fn ridley_special_n_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_FLAG_SHOOT);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialnexplode", category = ACMD_GAME )]
unsafe fn ridley_special_n_explode_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 9.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ridley", script = "effect_specialnexplode", category = ACMD_EFFECT )]
unsafe fn ridley_special_n_explode_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_bleath_hold"), Hash40::new("mouth1"), 5, 1, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ridley_bleath_hold"), false, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}

#[acmd_script( agent = "ridley", script = "sound_specialnexplode", category = ACMD_SOUND )]
unsafe fn ridley_special_n_explode_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s01"));
        PLAY_SE(fighter, Hash40::new("vc_ridley_special_s02"));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s02"));
    }
}

#[acmd_script( agent = "ridley", script = "expression_specialnexplode", category = ACMD_EXPRESSION )]
unsafe fn ridley_special_n_explode_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairnexplode", category = ACMD_GAME )]
unsafe fn ridley_special_air_n_explode_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 9.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ridley", script = "effect_specialairnexplode", category = ACMD_EFFECT )]
unsafe fn ridley_special_air_n_explode_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}

#[acmd_script( agent = "ridley", script = "sound_specialairnexplode", category = ACMD_SOUND )]
unsafe fn ridley_special_air_n_explode_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s01"));
        PLAY_SE(fighter, Hash40::new("vc_ridley_special_s02"));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s02"));
    }
}

#[acmd_script( agent = "ridley", script = "expression_specialairnexplode", category = ACMD_EXPRESSION )]
unsafe fn ridley_special_air_n_explode_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}

#[acmd_script( agent = "ridley", scripts = ["game_specialsstart", "game_specialairsstart"], category = ACMD_GAME, low_priority )]
unsafe fn ridley_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            // Uncomment to make once-per-airtime
            // VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 8.0, 6.0, 7.5, 7.5);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.5);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_START_JUMP);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 7.0, 6.0, 7.5, 5.5);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 7.8, 0.0, 10.0, 18.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 4.3, 0.0, 10.0, 18.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, 2, Hash40::new("top"), 5.0, 0.0, 8.0, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        GrabModule::set_constraint(boma, 0, true);
        GrabModule::set_constraint(boma, 1, true);
        GrabModule::set_constraint(boma, 2, true);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.8);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 49.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargef", category = ACMD_GAME, low_priority )]
unsafe fn ridley_special_air_hi_charge_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 40, 66, 0, 75, 6.5, 0.0, 5.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargeb", category = ACMD_GAME, low_priority )]
unsafe fn ridley_special_air_hi_charge_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 78, 60, 0, 80, 4.0, 0.0, 16.3, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 78, 60, 0, 80, 6.0, 0.0, 13.5, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargehi" , category = ACMD_GAME , low_priority)]
unsafe fn ridley_special_air_hi_charge_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 64, 46, 0, 85, 4.0, 0.0, 25.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 64, 46, 0, 85, 6.0, 0.0, 15.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargelw" , category = ACMD_GAME , low_priority)]
unsafe fn ridley_special_air_hi_charge_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 295, 35, 0, 50, 6.5, 0.0, 4.5, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 44, 50, 0, 85, 6.5, 0.0, 4.5, 2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ridley", scripts = ["game_speciallwstab", "game_specialairlwstab"], category = ACMD_GAME , low_priority )]
unsafe fn ridley_special_lw_stab_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        if boma.is_stick_backward() {
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        let grab_y = VarModule::get_float(fighter.battle_object, vars::ridley::status::SKEWER_STICK_Y);
        let mut z_mod = -1.0 * grab_y;
        // no angle (normal Skewer)
        if grab_y == 0.0 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(29.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(29.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.2, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(29.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        }
        // angle up
        else if grab_y > 0.0 {
            z_mod = 3.0 * grab_y;
            ATTACK(fighter, 0, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, (grab_y * 7.0) + 8.0, 24.5 - z_mod, Some(0.0), Some((grab_y * 11.0) + 7.0), Some(29.5 - z_mod), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, (grab_y * 7.0) + 8.0, 24.5 - z_mod, Some(0.0), Some((grab_y * 11.0) + 7.0), Some(29.5 - z_mod), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.2, 0.0, 7.0, 8.0, Some(0.0), Some((grab_y * 11.0) + 7.0), Some(29.5 - z_mod), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        }
        // angle down
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, (grab_y * 5.0) + 7.0, 24.5 - z_mod, Some(0.0), Some((grab_y * 6.0) + 7.0), Some(29.5 - z_mod), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, (grab_y * 5.0) + 7.0, 24.5 - z_mod, Some(0.0), Some((grab_y * 6.0) + 7.0), Some(29.5 - z_mod), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.2, 0.0, 7.0, 8.0, Some(0.0), Some((grab_y * 6.0) + 7.0), Some(29.5 - z_mod), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        }
        if WorkModule::get_int(boma, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_INT_DISABLE_SPECIAL_LW_FINISH_COUNT) <= 0 {
            AttackModule::set_no_dead_all(boma, true, false);
        }
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.3);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ridley", scripts = ["effect_speciallwstab", "effect_specialairlwstab"], category = ACMD_EFFECT , low_priority )]
unsafe fn ridley_special_lw_stab_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        let grab_y = VarModule::get_float(fighter.battle_object, vars::ridley::status::SKEWER_STICK_Y);
        let mut rot = 0 - ((grab_y * 25.0) as i32);
        let mut y_mod = if grab_y == 0.0 {0.0} else if grab_y > 0.0 {6.0} else {-3.0};
        if grab_y < 0.0 {
            rot = 0 - ((grab_y * 15.0) as i32);
        }
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ridley_death_stab_line"), Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, 8.0 + y_mod, 18, rot, 0, 0, 0.9, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}

#[acmd_script( agent = "ridley", scripts = ["game_speciallwfinish", "game_specialairlwfinish"], category = ACMD_GAME , low_priority )]
unsafe fn ridley_special_lw_finish_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 57, 100, 150, 0, 2.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_stab"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        AttackModule::set_no_finish_camera(boma, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
        VarModule::set_int(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_CATCH_ID, capture_id as i32);
        VarModule::on_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_IS_THROW);
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW);
        JostleModule::set_status(boma, true);
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_ENABLE_GRAVITY);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairlwpogo", category = ACMD_GAME )]
unsafe fn ridley_special_lw_pogo_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.5);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_LANDING);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_BOUNCE);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        /* Ground-only */
        ATTACK(fighter, 1, 0, Hash40::new("tail8"), 17.0, 280, 73, 0, 37, 3.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 2, 0, Hash40::new("tail8"), 17.0, 280, 73, 0, 37, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        /* Air-only */
        ATTACK(fighter, 6, 0, Hash40::new("tail8"), 17.0, 305, 45, 0, 38, 3.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 7, 0, Hash40::new("tail8"), 17.0, 305, 45, 0, 38, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        
        ATTACK(fighter, 3, 0, Hash40::new("tail7"), 12.0, 48, 70, 0, 70, 3.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 4, 0, Hash40::new("tail5"), 7.0, 361, 70, 0, 70, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 5, 0, Hash40::new("tail3"), 7.0, 361, 70, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 3, 4, 5, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 1, 2, 6, 7, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("tail8"), 12.0, 48, 70, 0, 70, 3.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 2, 0, Hash40::new("tail8"), 12.0, 48, 70, 0, 70, 3.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 1, 2, 0.5);
;        AttackModule::clear(boma, 6, false);
        AttackModule::clear(boma, 7, false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.5);
    }
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_BOUNCE);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        VarModule::off_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "ridley", script = "effect_specialairlwpogo", category = ACMD_EFFECT , low_priority )]
unsafe fn ridley_special_lw_pogo_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, -7, -8, 60, 0, 0, 0.9, true);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}

#[acmd_script( agent = "ridley", script = "sound_specialairlwpogo", category = ACMD_SOUND , low_priority )]
unsafe fn ridley_special_lw_pogo_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_special_l01"));
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_special_l02"));
    }
}

#[acmd_script( agent = "ridley", script = "expression_specialairlwpogo", category = ACMD_EXPRESSION , low_priority )]
unsafe fn ridley_special_lw_pogo_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_piercel"), 0);
    }
}

#[acmd_script( agent = "ridley", script = "game_speciallwpogolanding", category = ACMD_GAME , low_priority )]
unsafe fn ridley_special_lw_pogo_landing_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 12.0/(17.0 - 1.0));
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "ridley", script = "effect_speciallwpogolanding", category = ACMD_EFFECT , low_priority )]
unsafe fn ridley_special_lw_pogo_landing_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ridley_death_stab_flare"), false, true);
        EffectModule::set_visible_kind(boma, Hash40::new("ridley_death_stab_line"), false);
        EffectModule::set_visible_kind(boma, Hash40::new("sys_sp_flash"), false);
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), -18, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "ridley", script = "sound_speciallwpogolanding", category = ACMD_SOUND , low_priority )]
unsafe fn ridley_special_lw_pogo_landing_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_landing03"));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_attackair_f01"));
    }
}

#[acmd_script( agent = "ridley", script = "expression_speciallwpogolanding", category = ACMD_EXPRESSION , low_priority )]
unsafe fn ridley_special_lw_pogo_landing_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ridley_special_n_start_game,
        ridley_special_n_hold_game,
        ridley_special_n_shoot_game,
        ridley_special_n_explode_game,
        ridley_special_n_explode_effect,
        ridley_special_n_explode_sound,
        ridley_special_n_explode_expression,
        ridley_special_air_n_explode_game,
        ridley_special_air_n_explode_effect,
        ridley_special_air_n_explode_sound,
        ridley_special_air_n_explode_expression,
        ridley_special_s_start_game,
        ridley_special_air_hi_charge_f_game,
        ridley_special_air_hi_charge_b_game,
        ridley_special_air_hi_charge_hi_game,
        ridley_special_air_hi_charge_lw_game,
        ridley_special_lw_stab_game,
        ridley_special_lw_stab_effect,
        ridley_special_lw_finish_game,
        ridley_special_lw_pogo_game,
        ridley_special_lw_pogo_effect,
        ridley_special_lw_pogo_sound,
        ridley_special_lw_pogo_expression,
        ridley_special_lw_pogo_landing_game,
        ridley_special_lw_pogo_landing_effect,
        ridley_special_lw_pogo_landing_sound,
        ridley_special_lw_pogo_landing_expression,
    );
}

