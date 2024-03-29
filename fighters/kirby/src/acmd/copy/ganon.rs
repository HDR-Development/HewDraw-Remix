use super::*;

unsafe extern "C" fn game_ganonfloatstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_GROUND_DECIDE_ANGLE);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_GROUND_CHANGE_KINETIC);
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_ganonfloatstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

unsafe extern "C" fn expression_ganonfloatstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn sound_ganonfloatstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_jump01"));
    }
}

unsafe extern "C" fn game_ganonfloatairstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0 / 10.0);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_ganonfloatairstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
    }
    frame(lua_state, 2.0);
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

unsafe extern "C" fn expression_ganonfloatairstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn sound_ganonfloatairstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
}

unsafe extern "C" fn game_ganonfloat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 20.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_FALL_SPEED_Y_INCREASE);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_ganonfloat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_final_hand_triforce"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_entry_aura"), false, false);
    }
}

unsafe extern "C" fn expression_ganonfloat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_ganonfloatstart", game_ganonfloatstart);
    agent.acmd("effect_ganonfloatstart", effect_ganonfloatstart);
    agent.acmd("expression_ganonfloatstart", expression_ganonfloatstart);
    agent.acmd("sound_ganonfloatstart", sound_ganonfloatstart);
    agent.acmd("game_ganonfloatairstart", game_ganonfloatairstart);
    agent.acmd("effect_ganonfloatairstart", effect_ganonfloatairstart);
    agent.acmd(
        "expression_ganonfloatairstart",
        expression_ganonfloatairstart,
    );
    agent.acmd("sound_ganonfloatairstart", sound_ganonfloatairstart);
    agent.acmd("game_ganonfloat", game_ganonfloat);
    agent.acmd("effect_ganonfloat", effect_ganonfloat);
    agent.acmd("expression_ganonfloat", expression_ganonfloat);
}