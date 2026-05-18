use super::*;

// ================================================================================================
// ======================================== ULTIMATE UPPERCUT =====================================
// ================================================================================================

unsafe extern "C" fn game_specialn2start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 8.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn expression_specialn2start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

unsafe extern "C" fn effect_specialn2hold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_hyakuretsukick"), Hash40::new("haver"), -0.75, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 2.0);
    for h in 0..=12 {
        if is_excute(agent) {
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 12, 0, 12, 0, 0, 0, false);
            }
            if h % 4 == 0 {
                EFFECT_OFF_KIND(agent, Hash40::new("miifighter_hyakuretsukick"), true, true);
                EFFECT_FOLLOW(agent, Hash40::new("miifighter_hyakuretsukick"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true);
            }
        }
        wait(lua_state, 12.0);
    }
}

unsafe extern "C" fn sound_specialn2hold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_miifighter_special_n2_charge"));
    }
}

unsafe extern "C" fn expression_specialn2hold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    for _ in 0..=150 {
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 12.0);
    }
}

unsafe extern "C" fn expression_specialn2end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

unsafe extern "C" fn game_specialn2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let ground_start = boma.is_situation(*SITUATION_KIND_GROUND);
    let charge = VarModule::get_int(agent.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        let damage = 10.0 + (0.1 * charge as f32);
        let kbg = if ground_start { 58 } else { 50 };
        let sound_lvl = if charge <= 100 { *ATTACK_SOUND_LEVEL_M } else { *ATTACK_SOUND_LEVEL_L };
        ATTACK(agent, 0, 0, Hash40::new("handr"), damage, 90, kbg, 0, 57, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_lvl, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), damage, 90, kbg, 0, 57, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_lvl, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        let charge = VarModule::get_int(agent.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT);
        let damage = 8.0 + (0.05 * charge as f32);
        let kbg = if ground_start { 58 } else { 50 };
        ATTACK(agent, 0, 0, Hash40::new("handr"), damage, 90, kbg, 0, 57, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), damage, 90, kbg, 0, 57, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    if charge >= ParamModule::get_int(agent.battle_object, ParamType::Agent, "special_n2.charge_frame") {
        FT_MOTION_RATE_RANGE(agent, 15.0, 30.0, 20.0);
        frame(lua_state, 16.0);
    }
    else {
        frame(lua_state, 17.0);
    }
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialn2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = VarModule::get_int(agent.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT)
        >= ParamModule::get_int(agent.battle_object, ParamType::Agent, "special_n2.charge_frame");
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if charged {
            EFFECT_FOLLOW(agent, Hash40::new("miifighter_hyakuretsukick"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.65, true);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if charged {
            EFFECT_FOLLOW(agent, Hash40::new("miifighter_pistonpunch_impact"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_COLOR(agent, 2.0, 0.5, 0.5);
            LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_tenchi_arc"), Hash40::new("top"), 0, 8, -5, 0, 15, 90, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 3.0, 0.25, 0.25);
        if !charged {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.7, true);
        }
    }
    let mut clearFrame1 = 16.0;
    let mut clearFrame2 = 17.0;
    if charged {
        clearFrame1 = 15.5;
        clearFrame2 = 16.0;
    }
    frame(lua_state, clearFrame1);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miifighter_tenchi_arc"), true, true);
    }
    frame(lua_state, clearFrame2);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miifighter_hyakuretsukick"), false, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miifighter_pistonpunch_impact"), true, true);
    }
}

unsafe extern "C" fn sound_specialn2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        let charge = VarModule::get_int(agent.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT);
        if charge < ParamModule::get_int(agent.battle_object, ParamType::Agent, "special_n2.charge_frame") {
            PLAY_SE(agent, Hash40::new("se_miifighter_special_n2_swing"));
        }
        else {
            PLAY_SE(agent, Hash40::new("se_miifighter_special_n2_swing_max"));
        }
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_attack03"));
    }
}

unsafe extern "C" fn expression_specialn2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = VarModule::get_int(agent.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT)
        >= ParamModule::get_int(agent.battle_object, ParamType::Agent, "special_n2.charge_frame");
    frame(lua_state, 1.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if charged {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        else {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if charged {
            QUAKE(agent, *CAMERA_QUAKE_KIND_L);
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
        else {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn effect_specialn2landing(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialn2landing(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_miifighter_landing03"));
    }
}

unsafe extern "C" fn expression_specialn2landing(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn2start", game_specialn2start, Priority::Low);
    agent.acmd("game_specialairn2start", game_specialn2start, Priority::Low);
    agent.acmd("effect_specialn2start", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairn2start", acmd_stub, Priority::Low);
    agent.acmd("sound_specialn2start", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairn2start", acmd_stub, Priority::Low);
    agent.acmd("expression_specialn2start", expression_specialn2start, Priority::Low);
    agent.acmd("expression_specialairn2start", expression_specialn2start, Priority::Low);

    agent.acmd("game_specialn2hold", acmd_stub, Priority::Low);
    agent.acmd("game_specialairn2hold", acmd_stub, Priority::Low);
    agent.acmd("effect_specialn2hold", effect_specialn2hold, Priority::Low);
    agent.acmd("effect_specialairn2hold", effect_specialn2hold, Priority::Low);
    agent.acmd("sound_specialn2hold", sound_specialn2hold, Priority::Low);
    agent.acmd("sound_specialairn2hold", sound_specialn2hold, Priority::Low);
    agent.acmd("expression_specialn2hold", expression_specialn2hold, Priority::Low);
    agent.acmd("expression_specialairn2hold", expression_specialn2hold, Priority::Low);

    agent.acmd("game_specialn2end", acmd_stub, Priority::Low);
    agent.acmd("game_specialairn2end", acmd_stub, Priority::Low);
    agent.acmd("effect_specialn2end", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairn2end", acmd_stub, Priority::Low);
    agent.acmd("sound_specialn2end", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairn2end", acmd_stub, Priority::Low);
    agent.acmd("expression_specialn2end", expression_specialn2end, Priority::Low);
    agent.acmd("expression_specialairn2end", expression_specialn2end, Priority::Low);

    agent.acmd("game_specialn2attack", game_specialn2attack, Priority::Low);
    agent.acmd("game_specialairn2attack", game_specialn2attack, Priority::Low);
    agent.acmd("effect_specialn2attack", effect_specialn2attack, Priority::Low);
    agent.acmd("effect_specialairn2attack", effect_specialn2attack, Priority::Low);
    agent.acmd("sound_specialn2attack", sound_specialn2attack, Priority::Low);
    agent.acmd("sound_specialairn2attack", sound_specialn2attack, Priority::Low);
    agent.acmd("expression_specialn2attack", expression_specialn2attack, Priority::Low);
    agent.acmd("expression_specialairn2attack", expression_specialn2attack, Priority::Low);

    agent.acmd("game_specialn2landing", acmd_stub, Priority::Low);
    agent.acmd("game_specialairn2landing", acmd_stub, Priority::Low);
    agent.acmd("effect_specialn2landing", effect_specialn2landing, Priority::Low);
    agent.acmd("effect_specialairn2landing", effect_specialn2landing, Priority::Low);
    agent.acmd("sound_specialn2landing", sound_specialn2landing, Priority::Low);
    agent.acmd("sound_specialairn2landing", sound_specialn2landing, Priority::Low);
    agent.acmd("expression_specialn2landing", expression_specialn2landing, Priority::Low);
    agent.acmd("expression_specialairn2landing", expression_specialn2landing, Priority::Low);
}