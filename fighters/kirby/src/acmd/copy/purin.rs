use super::*;

unsafe extern "C" fn game_purinspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 20.0, 361, 66, 0, 106, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
        if(AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)){
            if (DamageModule::damage(boma, 0) > 5.0) {
                DamageModule::add_damage(boma, -5.0, 0);
            }
        }
    }
}

unsafe extern "C" fn effect_purinspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("purin_nemuru_start"), Hash40::new("top"), 0, 9, 6.5, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("sys_sleep"), Hash40::new("body"), 0, 3, 6, 0, 0, 0, 1, false);
    }
    frame(lua_state, 40.0);
    for _ in 0..3 {
        if is_excute(agent) {
            FLASH(agent, 0.502, 0.314, 0.392, 0.196);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            FLASH_FRM(agent, 12, 0.941, 0.235, 0.549, 0.392);
        }
        wait(lua_state, 12.0);
        if is_excute(agent) {
            FLASH_FRM(agent, 12, 0.941, 0.118, 0.549, 0);
        }
        wait(lua_state, 12.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 6.0);
    }
    frame(lua_state, 130.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("sys_sleep"), Hash40::new("body"), 0, 3, 6, 0, 0, 0, 1, false);
        FLASH(agent, 0.502, 0.314, 0.392, 0.196);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 12, 0.941, 0.235, 0.549, 0.392);
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 12, 0.941, 0.118, 0.549, 0);
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 185.0);
}

unsafe extern "C" fn sound_purinspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 34.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("vc_purin_001"));
        PLAY_STATUS(agent, Hash40::new("se_purin_sleep"));
    }
    wait(lua_state, 100.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_purin_sleep"));
    }
    wait(lua_state, 53.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_purin_sleep"));
        PLAY_STATUS(agent, Hash40::new("vc_purin_002"));
    }
    wait(lua_state, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_purin_special_l01"));
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_purin_special_l01"));
    }
}

unsafe extern "C" fn expression_purinspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_sleep"), 30, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 134.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_sleep"), 30, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 187.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 210.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 223.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_purinspecialn", game_purinspecialn, Priority::Low);
    agent.acmd("game_purinspecialairn", game_purinspecialn, Priority::Low);
    agent.acmd("effect_purinspecialn", effect_purinspecialn, Priority::Low);
    agent.acmd("effect_purinspecialairn", effect_purinspecialn, Priority::Low);
    agent.acmd("sound_purinspecialn", sound_purinspecialn, Priority::Low);
    agent.acmd("sound_purinspecialairn", sound_purinspecialn, Priority::Low);
    agent.acmd("expression_purinspecialn", expression_purinspecialn, Priority::Low);
    agent.acmd("expression_purinspecialairn", expression_purinspecialn, Priority::Low);
}