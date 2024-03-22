use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 15.0, 6.0);
    
}

unsafe extern "C" fn game_specialnhold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
}

unsafe extern "C" fn game_specialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_FLAG_SHOOT);
    }
}

unsafe extern "C" fn game_specialnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 9.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_bleath_hold"), Hash40::new("mouth1"), 5, 1, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ridley_bleath_hold"), false, false);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_specialnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_smash_s01"));
        PLAY_SE(agent, Hash40::new("vc_ridley_special_s02"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_smash_s02"));
    }
}

unsafe extern "C" fn expression_specialnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}

unsafe extern "C" fn effect_specialairnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 8.0, 6.0, 7.5, 7.5);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 21.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_START_JUMP);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 7.0, 6.0, 7.5, 5.5);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 7.8, 0.0, 10.0, 18.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        CATCH(agent, 1, Hash40::new("top"), 4.3, 0.0, 10.0, 18.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_A);
        CATCH(agent, 2, Hash40::new("top"), 5.0, 0.0, 8.0, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        GrabModule::set_constraint(boma, 0, true);
        GrabModule::set_constraint(boma, 1, true);
        GrabModule::set_constraint(boma, 2, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(lua_state, 28.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 30.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 37.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 39.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 49.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
}

unsafe extern "C" fn game_apecialairhichargef(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 40, 66, 0, 75, 6.5, 0.0, 5.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhichargeb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 78, 60, 0, 80, 4.0, 0.0, 16.3, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 78, 60, 0, 80, 6.0, 0.0, 13.5, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhichargehi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 64, 46, 0, 85, 4.0, 0.0, 25.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 64, 46, 0, 85, 6.0, 0.0, 15.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhichargelw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 295, 35, 0, 50, 6.5, 0.0, 4.5, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 44, 50, 0, 85, 6.5, 0.0, 4.5, 2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_speciallwstab(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 28.0);
    if is_excute(agent) {
        if boma.is_stick_backward() {
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        let grab_y = VarModule::get_float(agent.battle_object, vars::ridley::status::SKEWER_STICK_Y);
        let mut z_mod = -1.0 * grab_y;
        // no angle (normal Skewer)
        if grab_y == 0.0 {
            ATTACK(agent, 0, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(29.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 1, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(29.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.2, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(29.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        }
        // angle up
        else if grab_y > 0.0 {
            z_mod = 3.0 * grab_y;
            ATTACK(agent, 0, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, (grab_y * 7.0) + 8.0, 24.5 - z_mod, Some(0.0), Some((grab_y * 11.0) + 7.0), Some(29.5 - z_mod), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 1, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, (grab_y * 7.0) + 8.0, 24.5 - z_mod, Some(0.0), Some((grab_y * 11.0) + 7.0), Some(29.5 - z_mod), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.2, 0.0, 7.0, 8.0, Some(0.0), Some((grab_y * 11.0) + 7.0), Some(29.5 - z_mod), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        }
        // angle down
        else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, (grab_y * 5.0) + 7.0, 24.5 - z_mod, Some(0.0), Some((grab_y * 6.0) + 7.0), Some(29.5 - z_mod), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 1, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, (grab_y * 5.0) + 7.0, 24.5 - z_mod, Some(0.0), Some((grab_y * 6.0) + 7.0), Some(29.5 - z_mod), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.2, 0.0, 7.0, 8.0, Some(0.0), Some((grab_y * 6.0) + 7.0), Some(29.5 - z_mod), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        }
        if WorkModule::get_int(boma, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_INT_DISABLE_SPECIAL_LW_FINISH_COUNT) <= 0 {
            AttackModule::set_no_dead_all(boma, true, false);
        }
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 0.3);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallwstab(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        let grab_y = VarModule::get_float(agent.battle_object, vars::ridley::status::SKEWER_STICK_Y);
        let mut rot = 0 - ((grab_y * 25.0) as i32);
        let mut y_mod = if grab_y == 0.0 {0.0} else if grab_y > 0.0 {6.0} else {-3.0};
        if grab_y < 0.0 {
            rot = 0 - ((grab_y * 15.0) as i32);
        }
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("ridley_death_stab_line"), Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, 8.0 + y_mod, 18, rot, 0, 0, 0.9, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}

unsafe extern "C" fn game_speciallwfinish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 57, 100, 150, 0, 2.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_stab"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        AttackModule::set_no_finish_camera(boma, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
        VarModule::set_int(agent.battle_object, vars::ridley::instance::SPECIAL_LW_CATCH_ID, capture_id as i32);
        VarModule::on_flag(agent.battle_object, vars::ridley::instance::SPECIAL_LW_IS_THROW);
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW);
        JostleModule::set_status(boma, true);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_ENABLE_GRAVITY);
    }
}

unsafe extern "C" fn game_speciallwpogo(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 20.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_LANDING);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_BOUNCE);
    }
    frame(lua_state, 33.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        /* Ground-only */
        ATTACK(agent, 1, 0, Hash40::new("tail8"), 17.0, 280, 73, 0, 37, 3.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("tail8"), 17.0, 280, 73, 0, 37, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        /* Air-only */
        ATTACK(agent, 6, 0, Hash40::new("tail8"), 17.0, 305, 45, 0, 38, 3.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(agent, 7, 0, Hash40::new("tail8"), 17.0, 305, 45, 0, 38, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        
        ATTACK(agent, 3, 0, Hash40::new("tail7"), 12.0, 46, 70, 0, 70, 3.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(agent, 4, 0, Hash40::new("tail5"), 12.0, 46, 70, 0, 70, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(agent, 5, 0, Hash40::new("tail3"), 12.0, 46, 70, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 3, 4, 5, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 1, 2, 6, 7, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("tail8"), 12.0, 46, 70, 0, 70, 3.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("tail8"), 12.0, 46, 70, 0, 70, 3.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 1, 2, 0.5);
        AttackModule::clear(boma, 6, false);
        AttackModule::clear(boma, 7, false);
    }
    wait(lua_state, 3.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_BOUNCE);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 55.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_speciallwpogo(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, -7, -8, 60, 0, 0, 0.9, true);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}

unsafe extern "C" fn sound_speciallwpogo(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_special_l01"));
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_special_l02"));
    }
}

unsafe extern "C" fn expression_speciallwpogo(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_piercel"), 0);
    }
}

unsafe extern "C" fn game_speciallwpogolanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 17.0, 12.0);
    frame(lua_state, 17.0);
    FT_MOTION_RATE(agent, 1.0);
    
}

unsafe extern "C" fn effect_speciallwpogolanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
        EffectModule::set_visible_kind(boma, Hash40::new("ridley_death_stab_line"), false);
        EffectModule::set_visible_kind(boma, Hash40::new("sys_sp_flash"), false);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), -18, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallwpogolanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_landing03"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_attackair_f01"));
    }
}

unsafe extern "C" fn expression_speciallwpogolanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart);
    agent.acmd("game_specialairnstart", game_specialnstart);
    agent.acmd("game_specialnhold", game_specialnhold);
    agent.acmd("game_specialairnhold", game_specialnhold);
    agent.acmd("game_specialnshoot", game_specialnshoot);
    agent.acmd("game_specialairnshoot", game_specialnshoot);
    agent.acmd("game_specialnexplode", game_specialnexplode);
    agent.acmd("effect_specialnexplode", effect_specialnexplode);
    agent.acmd("sound_specialnexplode", sound_specialnexplode);
    agent.acmd("expression_specialnexplode", expression_specialnexplode);
    agent.acmd("game_specialairnexplode", game_specialnexplode);
    agent.acmd("effect_specialairnexplode", effect_specialairnexplode);
    agent.acmd("sound_specialairnexplode", sound_specialnexplode);
    agent.acmd("expression_specialairnexplode", expression_specialnexplode);
    agent.acmd("game_specialsstart", game_specialsstart);
    agent.acmd("game_specialairsstart", game_specialsstart);
    agent.acmd("game_specialairhichargef", game_apecialairhichargef);
    agent.acmd("game_specialairhichargeb", game_specialairhichargeb);
    agent.acmd("game_specialairhichargehi", game_specialairhichargehi);
    agent.acmd("game_specialairhichargelw", game_specialairhichargelw);
    agent.acmd("game_speciallwstab", game_speciallwstab);
    agent.acmd("game_specialairlwstab", game_speciallwstab);
    agent.acmd("effect_speciallwstab", effect_speciallwstab);
    agent.acmd("effect_specialairlwstab", effect_speciallwstab);
    agent.acmd("game_speciallwfinish", game_speciallwfinish);
    agent.acmd("game_specialairlwfinish", game_speciallwfinish);
    agent.acmd("game_specialairlwpogo", game_speciallwpogo);
    agent.acmd("effect_specialairlwpogo", effect_speciallwpogo);
    agent.acmd("sound_specialairlwpogo", sound_speciallwpogo);
    agent.acmd("expression_specialairlwpogo", expression_speciallwpogo);
    agent.acmd("game_speciallwpogolanding", game_speciallwpogolanding);
    agent.acmd("effect_speciallwpogolanding", effect_speciallwpogolanding);
    agent.acmd("sound_speciallwpogolanding", sound_speciallwpogolanding);
    agent.acmd("expression_speciallwpogolanding", expression_speciallwpogolanding);
}
