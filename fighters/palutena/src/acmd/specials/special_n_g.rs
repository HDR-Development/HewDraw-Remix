use super::*;

unsafe extern "C" fn game_specialngstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 10.0, 8.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 13.0); // 12
    FT_MOTION_RATE_RANGE(agent, 14.0, 24.0, 6.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
}

unsafe extern "C" fn effect_specialngstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight_grey"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), -0.5*boma.lr(), -0.5, 0.5, 0, 0, 0, 0.95, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 1.0, 0.05);
        EFFECT_FOLLOW(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), -0.5*boma.lr(), -0.5, 0.5, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.05);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), -0.5*boma.lr(), -0.5, 0.5, 0, 0, 0, 1.15, true);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 0.70, 0.25);
        LAST_EFFECT_SET_ALPHA(agent, 0.3);
    }
}

unsafe extern "C" fn sound_specialngstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
        PLAY_SE(agent, Hash40::new("se_common_slip_01"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        PLAY_SE(agent, Hash40::new("se_palutena_throw"));
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_attack"));
        PLAY_STATUS(agent, Hash40::new("se_item_club_wind"));
    }
}

unsafe extern "C" fn expression_specialngstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_27_spinslash"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialngloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let loop_count = VarModule::get_int(agent.battle_object, vars::palutena::status::SPECIAL_N_GREEN_LOOP) as f32;
    let damage = 1.75 - loop_count/4.0; // 1.5-1.0
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 30.0, 19.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        // multihit segment
        ATTACK(agent, 0, 0, Hash40::new("top"), damage, 111, 100, 42, 0, 5.75, 0.0, 6.75, 0.75, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), damage, 145, 100, 40, 0, 5.75, 0.0, 11.75, -1.25, Some(0.0), Some(11.75), Some(2.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 2, 0, Hash40::new("top"), damage, 177, 100, 38, 0, 5.75, 0.0, 17.75, -2.25, Some(0.0), Some(17.75), Some(3.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
}

unsafe extern "C" fn expression_specialngloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashss"), 10);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_27_spinslash"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_27_spinslash"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialngend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let loop_count = VarModule::get_int(agent.battle_object, vars::palutena::status::SPECIAL_N_GREEN_LOOP) as f32;
    let damage = 1.75 - loop_count/4.0; // 1.5-0.75
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 13.0, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), damage, 111, 100, 42, 0, 5.75, 0.0, 6.75, 0.75, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), damage, 145, 100, 40, 0, 5.75, 0.0, 11.75, -1.25, Some(0.0), Some(11.75), Some(2.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 2, 0, Hash40::new("top"), damage, 177, 100, 38, 0, 5.75, 0.0, 17.75, -2.25, Some(0.0), Some(17.75), Some(3.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        let angle_mod = ((sv_math::rand(hash40("fighter"), 51) as i32) - 25) as u64;
        ATTACK(agent, 0, 1, Hash40::new("top"), 5.0, 90 + angle_mod, 123, 0, 66, 6.5, 0.0, 6.5, 0.75, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 1, Hash40::new("top"), 5.0, 90 + angle_mod, 123, 0, 66, 6.5, 0.0, 13.25, -2.5, Some(0.0), Some(13.25), Some(3.25), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 2, 1, Hash40::new("top"), 5.0, 90 + angle_mod, 123, 0, 66, 6.5, 0.0, 20.0, -4.5, Some(0.0), Some(20.0), Some(5.25), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 13.0); // 11
    FT_MOTION_RATE(agent, 1.0); // 42 faf
    //FT_MOTION_RATE_RANGE(agent, 29.0, 46.0, 15.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
        VarModule::set_int(agent.battle_object, vars::palutena::status::SPECIAL_N_GREEN_BUTTON_TIMER, 1);
    }
}

unsafe extern "C" fn effect_specialngend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight_grey"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), -0.5*boma.lr(), -0.5, 0.5, 0, 0, 0, 1.15, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 1.0, 0.05);
        LAST_EFFECT_SET_RATE(agent, 2.0);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), -0.5*boma.lr(), -0.5, 0.5, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 1.0, 0.25);
        LAST_EFFECT_SET_RATE(agent, 2.0);
        LAST_EFFECT_SET_ALPHA(agent, 0.4);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("sys_club_tornado"), false, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace_grey"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2_grey"), false, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("sys_club_tornado"), true, true);
    }
}

unsafe extern "C" fn sound_specialngend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_item_club_wind"));
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

unsafe extern "C" fn expression_specialngend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 10);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialngstart", game_specialngstart, Priority::Low);
    agent.acmd("effect_specialngstart", effect_specialngstart, Priority::Low);
    agent.acmd("sound_specialngstart", sound_specialngstart, Priority::Low);
    agent.acmd("expression_specialngstart", expression_specialngstart, Priority::Low);

    agent.acmd("game_specialngloop", game_specialngloop, Priority::Low);
    agent.acmd("effect_specialngloop", acmd_stub, Priority::Low);
    agent.acmd("sound_specialngloop", acmd_stub, Priority::Low);
    agent.acmd("expression_specialngloop", expression_specialngloop, Priority::Low);

    agent.acmd("game_specialngend", game_specialngend, Priority::Low);
    agent.acmd("game_specialairngend", game_specialngend, Priority::Low);
    agent.acmd("effect_specialngend", effect_specialngend, Priority::Low);
    agent.acmd("effect_specialairngend", effect_specialngend, Priority::Low);
    agent.acmd("sound_specialngend", sound_specialngend, Priority::Low);
    agent.acmd("sound_specialairngend", sound_specialngend, Priority::Low);
    agent.acmd("expression_specialngend", expression_specialngend, Priority::Low);
    agent.acmd("expression_specialairngend", expression_specialngend, Priority::Low);
}