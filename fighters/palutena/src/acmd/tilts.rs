use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 11.0/(16.0-0.0));
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::palutena::status::CAN_INCREASE_COLOR);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.667);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(agent, 1, 0, Hash40::new("stick"), 6.0, 90, 100, 25, 0, 4.3, 0.0, 5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("stick"), 6.0, 45, 100, 45, 0, 3.3, 0.0, -5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("stick"), 6.0, 285, 100, 25, 0, 4.3, 0.0, 5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("stick"), 6.0, 100, 100, 40, 0, 3.3, 0.0, -5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("stick"), 6.0, 20, 100, 35, 0, 4.3, 0.0, 5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("stick"), 6.0, 85, 100, 30, 0, 3.3, 0.0, -5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 1, Hash40::new("stick"), 7.0, 40, 87, 0, 78, 5.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 1, Hash40::new("stick"), 7.0, 40, 87, 0, 78, 4.3, 0.0, -5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.000);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
    }

}

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light4"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.85, 0.40, 0.001);
    }
    frame(lua_state, 14.0);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_01"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_02"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_03"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_04"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_05"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_06"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_07"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_08"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 40.0);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_01"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_02"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_03"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_04"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_05"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_06"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_07"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_08"), false, false);
        }
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light4"), false, false);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::palutena::status::CAN_INCREASE_COLOR);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.778);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.000);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.571);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.4, 367, 100, 20, 0, 3.0, 0.0, 18.0, 4.0, None, None, None, 0.75, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.4, 130, 100, 50, 0, 2.0, 0.0, 15.0, 10.5, Some(0.0), Some(15.0), Some(-5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.4, 190, 100, 30, 0, 2.0, 0.0, 20.0, 10.5, Some(0.0), Some(20.0), Some(-5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.4, 90, 100, 35, 0, 5.0, 0.0, 8.0, 3.5, None, None, None, 0.75, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(agent, 1.000);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 85, 136, 0, 75, 3.0, 0.0, 23.0, 10.5, Some(0.0), Some(23.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 85, 136, 0, 75, 3.0, 0.0, 17.0, 10.5, Some(0.0), Some(17.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
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
    frame(lua_state, 10.0);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_01"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_02"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_03"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_04"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_05"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_06"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_07"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_08"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light4"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.05, 0.90);
    }
    frame(lua_state, 32.0);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_01"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_02"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_03"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_04"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_05"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_06"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_07"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_08"), false, false);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light4"), false, false);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::palutena::status::CAN_INCREASE_COLOR);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.693);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.870);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        // Ground only hitboxes
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 67, 90, 0, 45, 4.0, 0.0, 2.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("stick"), 8.5, 83, 90, 0, 45, 4.0, -0.5, 8.0, 0.0, Some(-0.5), Some(-7.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        // Air only hitboxes
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 67, 90, 0, 45, 4.0, 0.0, 2.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("stick"), 8.5, 76, 90, 0, 45, 4.0, -0.5, 8.0, 0.0, Some(-0.5), Some(-7.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        // Ground only hitboxes
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 67, 90, 0, 45, 4.0, 0.0, 2.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("stick"), 8.5, 83, 90, 0, 45, 4.0, -0.5, 8.0, 0.0, Some(-0.5), Some(-7.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.5, 83, 90, 0, 45, 4.0, 0.0, 2.0, 24.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        // Air only hitboxes
        ATTACK(agent, 3, 0, Hash40::new("top"), 7.0, 67, 90, 0, 45, 4.0, 0.0, 2.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 4, 0, Hash40::new("stick"), 8.5, 76, 90, 0, 45, 4.0, -0.5, 8.0, 0.0, Some(-0.5), Some(-7.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 5, 0, Hash40::new("top"), 8.5, 76, 90, 0, 45, 4.0, 0.0, 2.0, 24.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 12.0/(35.0-25.0));
        AttackModule::clear_all(boma);
    }

}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_01"), Hash40::new("stick"), 0, 0, 0, 0, 180, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_02"), Hash40::new("stick"), 0, 0, 0, 0, 180, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_03"), Hash40::new("stick"), 0, 0, 0, 0, 180, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_04"), Hash40::new("stick"), 0, 0, 0, 0, 180, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_05"), Hash40::new("stick"), 0, 0, 0, 0, 180, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_06"), Hash40::new("stick"), 0, 0, 0, 0, 180, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_07"), Hash40::new("stick"), 0, 0, 0, 0, 180, 0, 1, true);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_trace_08"), Hash40::new("stick"), 0, 0, 0, 0, 180, 0, 1, true);
        }
    }
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light4"), Hash40::new("stick"), 0, 8.65, 0, 0, 180, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 2.0, 0.03, 0.0);
    }
    frame(lua_state, 29.0);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_01"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_02"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_03"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_04"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_05"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_06"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_07"), false, false);
        }
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_trace_08"), false, false);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light4"), false, false);
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
    agent.acmd("game_attacks3", game_attacks3);
    agent.acmd("effect_attacks3", effect_attacks3);

    agent.acmd("game_attackhi3", game_attackhi3);
    agent.acmd("effect_attackhi3", effect_attackhi3);

    agent.acmd("game_attacklw3", game_attacklw3);
    agent.acmd("effect_attacklw3", effect_attacklw3);
    agent.acmd("expression_attacklw3", expression_attacklw3);
}