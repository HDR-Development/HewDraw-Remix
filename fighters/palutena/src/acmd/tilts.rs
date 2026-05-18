use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 9.0, 11.0);
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(agent, 9.0, 16.0, 2.0);
    frame(lua_state, 16.0); // 14
    FT_MOTION_RATE_RANGE(agent, 16.0, 44.0, 20.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::palutena::status::ENABLE_COLOR_INCREMENT);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(agent, 1, 0, Hash40::new("stick"), 11.0, 55, 111, 0, 44, 4.3, 0.0, 4.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 13, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("stick"), 11.0, 55, 111, 0, 44, 3.5, 0.0, -6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 13, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 23.0); // 20
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("stick"), 9.0, 50, 111, 0, 44, 4.3, 0.0, 4.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("stick"), 9.0, 50, 111, 0, 44, 3.5, 0.0, -6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 32.0); // 27
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("stick"), 7.0, 45, 111, 0, 44, 4.3, 0.0, 4.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("stick"), 7.0, 45, 111, 0, 44, 3.5, 0.0, -6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 44.0); // 34
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 50.0);
    FT_MOTION_RATE_RANGE(agent, 50.0, 64.0, 10.0);
    frame(lua_state, 64.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(agent, 1.0, 0.82, 0.0125);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light4_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.87, 0.0125);
        LAST_EFFECT_SET_RATE(agent, 6.0/9.0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        let hash = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Hash40::new("palutena_wand_trace_01"),
            1 => Hash40::new("palutena_wand_trace_02"),
            2 => Hash40::new("palutena_wand_trace_03"),
            3 => Hash40::new("palutena_wand_trace_04"),
            4 => Hash40::new("palutena_wand_trace_05"),
            5 => Hash40::new("palutena_wand_trace_06"),
            6 => Hash40::new("palutena_wand_trace_07"),
            7 => Hash40::new("palutena_wand_trace_08"),
            _ => Hash40::new("palutena_wand_trace_08"),
        };
        EFFECT_FOLLOW(agent, hash, Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light4_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.87, 0.0125);
        LAST_EFFECT_SET_RATE(agent, 6.0/9.0);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_01"), false, false),
            1 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_02"), false, false),
            2 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_03"), false, false),
            3 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_04"), false, false),
            4 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_05"), false, false),
            5 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_06"), false, false),
            6 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_07"), false, false),
            7 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_08"), false, false),
            _ => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_08"), false, false),
        };
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace_grey"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light4_grey"), false, false);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::palutena::status::ENABLE_COLOR_INCREMENT);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 9.0, 7.0);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 30.0, 11.0);
    FT_MOTION_RATE(agent, 0.571);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.4, 367, 100, 20, 0, 3.0, 0.0, 18.0, 4.0, None, None, None, 0.75, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.4, 130, 100, 50, 0, 2.0, 0.0, 15.0, 10.5, Some(0.0), Some(15.0), Some(-5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.4, 190, 100, 30, 0, 2.0, 0.0, 20.0, 10.5, Some(0.0), Some(20.0), Some(-5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.4, 90, 100, 35, 0, 5.0, 0.0, 8.0, 3.5, None, None, None, 0.75, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 85, 136, 0, 75, 3.0, 0.0, 21.0, 10.5, Some(0.0), Some(21.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 85, 136, 0, 75, 3.0, 0.0, 15.0, 10.5, Some(0.0), Some(15.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(agent, 0.05, 0.2, 0.95);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light4_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.2, 0.95);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        let hash = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Hash40::new("palutena_wand_trace_01"),
            1 => Hash40::new("palutena_wand_trace_02"),
            2 => Hash40::new("palutena_wand_trace_03"),
            3 => Hash40::new("palutena_wand_trace_04"),
            4 => Hash40::new("palutena_wand_trace_05"),
            5 => Hash40::new("palutena_wand_trace_06"),
            6 => Hash40::new("palutena_wand_trace_07"),
            7 => Hash40::new("palutena_wand_trace_08"),
            _ => Hash40::new("palutena_wand_trace_08"),
        };
        EFFECT_FOLLOW(agent, hash, Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light4_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.2, 0.95);
        LAST_EFFECT_SET_RATE(agent, 6.0/8.0);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_01"), false, false),
            1 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_02"), false, false),
            2 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_03"), false, false),
            3 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_04"), false, false),
            4 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_05"), false, false),
            5 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_06"), false, false),
            6 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_07"), false, false),
            7 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_08"), false, false),
            _ => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_08"), false, false),
        };
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace_grey"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light4_grey"), false, false);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::palutena::status::ENABLE_COLOR_INCREMENT);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 14.0, 9.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE_RANGE(agent, 14.0, 25.0, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 67, 90, 0, 45, 3.5, 0.0, 3.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        // Ground-only
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.5, 83, 90, 0, 45, 3.0, 0.0, 3.5, 6.0, Some(0.0), Some(3.0), Some(21.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        // Air-only
        ATTACK(agent, 3, 0, Hash40::new("top"), 8.5, 76, 90, 0, 45, 3.0, 0.0, 3.5, 6.0, Some(0.0), Some(3.0), Some(21.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE_RANGE(agent, 25.0, 35.0, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.05);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light4_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 180, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.05);
        LAST_EFFECT_SET_RATE(agent, 6.0/9.0);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        let hash = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Hash40::new("palutena_wand_trace_01"),
            1 => Hash40::new("palutena_wand_trace_02"),
            2 => Hash40::new("palutena_wand_trace_03"),
            3 => Hash40::new("palutena_wand_trace_04"),
            4 => Hash40::new("palutena_wand_trace_05"),
            5 => Hash40::new("palutena_wand_trace_06"),
            6 => Hash40::new("palutena_wand_trace_07"),
            7 => Hash40::new("palutena_wand_trace_08"),
            _ => Hash40::new("palutena_wand_trace_08"),
        };
        EFFECT_FOLLOW(agent, hash, Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_01"), false, false),
            1 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_02"), false, false),
            2 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_03"), false, false),
            3 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_04"), false, false),
            4 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_05"), false, false),
            5 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_06"), false, false),
            6 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_07"), false, false),
            7 => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_08"), false, false),
            _ => EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_08"), false, false),
        };
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace_grey"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light4_grey"), false, false);
    }
}

unsafe extern "C" fn expression_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 15);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3, Priority::Low);
    agent.acmd("effect_attacks3", effect_attacks3, Priority::Low);

    agent.acmd("game_attackhi3", game_attackhi3, Priority::Low);
    agent.acmd("effect_attackhi3", effect_attackhi3, Priority::Low);

    agent.acmd("game_attacklw3", game_attacklw3, Priority::Low);
    agent.acmd("effect_attacklw3", effect_attacklw3, Priority::Low);
    agent.acmd("expression_attacklw3", expression_attacklw3, Priority::Low);
}