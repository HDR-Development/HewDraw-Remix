use super::*;

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -3.0, Some(0.0), Some(10.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -13.0, Some(0.0), Some(10.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr1"), 9.0, 361, 95, 0, 55, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 9.0, 361, 95, 0, 55, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("robot_armspin_wind"), Hash40::new("body"), 1, 0, 0, 0, -100, -90, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("robot_armspin"), Hash40::new("body"), 1, 0, 0, 0, 60, 90, 1, true);
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let color_vec = match color {
            0 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.1, 0.01, 0.0) } else { Vector3f::new(0.196, 0.196, 0.216) },
            1 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.196, 0.196, 0.216) } else { Vector3f::new(0.22, 0.059, 0.039) },
            2 => Vector3f::new(0.176, 0.137, 0.059),
            3 => Vector3f::new(0.235, 0.196, 0.255),
            4 => Vector3f::new(0.098, 0.157, 0.196),
            5 => Vector3f::new(0.098, 0.059, 0.0),
            6 => Vector3f::new(0.098, 0.098, 0.157),
            7 => Vector3f::new(0.118, 0.039, 0.051),
            _ => Vector3f::new(0.196, 0.196, 0.216)
        };
        LAST_EFFECT_SET_COLOR(agent, color_vec.x, color_vec.y, color_vec.z);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_RATE(agent, 2.0);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("robot_armspin_wind"), 5);
        EffectModule::kill_kind(boma, Hash40::new("robot_armspin"), false, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 11.5, 3, 0, -15, 0, 1.5, true, *EF_FLIP_YZ);
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let color_vec = match color {
            0 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.1, 0.01, 0.0) } else { Vector3f::new(0.196, 0.196, 0.216) },
            1 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.196, 0.196, 0.216) } else { Vector3f::new(0.22, 0.059, 0.039) },
            2 => Vector3f::new(0.176, 0.137, 0.059),
            3 => Vector3f::new(0.235, 0.196, 0.255),
            4 => Vector3f::new(0.098, 0.157, 0.196),
            5 => Vector3f::new(0.098, 0.059, 0.0),
            6 => Vector3f::new(0.098, 0.098, 0.157),
            7 => Vector3f::new(0.118, 0.039, 0.051),
            _ => Vector3f::new(0.196, 0.196, 0.216)
        };
        LAST_EFFECT_SET_COLOR(agent, color_vec.x, color_vec.y, color_vec.z);
    }
}

unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_robot_special_s01"));
    }

    frame(lua_state, 22.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_robot_special_s04"));
    }
}

unsafe extern "C" fn sound_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_robot_special_s01"));
    }
}

unsafe extern "C" fn expression_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_spinattacks"), 5, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 4);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 5, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialshi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -3.0, Some(0.0), Some(10.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -13.0, Some(0.0), Some(10.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr1"), 7.0, 105, 85, 0, 55, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 7.0, 105, 85, 0, 55, 5.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialshi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("robot_armspin_wind"), Hash40::new("body"), 1, 0, 0, 0, -100, -90, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("robot_armspin"), Hash40::new("body"), 1, 0, 0, 0, 60, 90, 1, true);
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let color_vec = match color {
            0 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.1, 0.01, 0.0) } else { Vector3f::new(0.196, 0.196, 0.216) },
            1 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.196, 0.196, 0.216) } else { Vector3f::new(0.22, 0.059, 0.039) },
            2 => Vector3f::new(0.176, 0.137, 0.059),
            3 => Vector3f::new(0.235, 0.196, 0.255),
            4 => Vector3f::new(0.098, 0.157, 0.196),
            5 => Vector3f::new(0.098, 0.059, 0.0),
            6 => Vector3f::new(0.098, 0.098, 0.157),
            7 => Vector3f::new(0.118, 0.039, 0.051),
            _ => Vector3f::new(0.196, 0.196, 0.216)
        };
        LAST_EFFECT_SET_COLOR(agent, color_vec.x, color_vec.y, color_vec.z);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_RATE(agent, 2.0);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("robot_armspin_wind"), 5);
        EffectModule::kill_kind(boma, Hash40::new("robot_armspin"), false, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 15.5, 4, 45, 0, -225, 1.5, true, *EF_FLIP_NONE);
        }
        else {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 15.5, 4, -45, 0, 45, 1.5, true, *EF_FLIP_NONE);
        }
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let color_vec = match color {
            0 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.1, 0.01, 0.0) } else { Vector3f::new(0.196, 0.196, 0.216) },
            1 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.196, 0.196, 0.216) } else { Vector3f::new(0.22, 0.059, 0.039) },
            2 => Vector3f::new(0.176, 0.137, 0.059),
            3 => Vector3f::new(0.235, 0.196, 0.255),
            4 => Vector3f::new(0.098, 0.157, 0.196),
            5 => Vector3f::new(0.098, 0.059, 0.0),
            6 => Vector3f::new(0.098, 0.098, 0.157),
            7 => Vector3f::new(0.118, 0.039, 0.051),
            _ => Vector3f::new(0.196, 0.196, 0.216)
        };
        LAST_EFFECT_SET_COLOR(agent, color_vec.x, color_vec.y, color_vec.z);
        LAST_EFFECT_SET_SCALE_W(agent, 0.8, 1.5, 1.5);
    }
}

unsafe extern "C" fn game_specialslw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -3.0, Some(0.0), Some(10.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -13.0, Some(0.0), Some(10.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr1"), 5.0, 25, 35, 0, 55, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 5.0, 25, 35, 0, 55, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialslw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("robot_armspin_wind"), Hash40::new("body"), 1, 0, 0, 0, -100, -90, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("robot_armspin"), Hash40::new("body"), 1, 0, 0, 0, 60, 90, 1, true);
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let color_vec = match color {
            0 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.1, 0.01, 0.0) } else { Vector3f::new(0.196, 0.196, 0.216) },
            1 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.196, 0.196, 0.216) } else { Vector3f::new(0.22, 0.059, 0.039) },
            2 => Vector3f::new(0.176, 0.137, 0.059),
            3 => Vector3f::new(0.235, 0.196, 0.255),
            4 => Vector3f::new(0.098, 0.157, 0.196),
            5 => Vector3f::new(0.098, 0.059, 0.0),
            6 => Vector3f::new(0.098, 0.098, 0.157),
            7 => Vector3f::new(0.118, 0.039, 0.051),
            _ => Vector3f::new(0.196, 0.196, 0.216)
        };
        LAST_EFFECT_SET_COLOR(agent, color_vec.x, color_vec.y, color_vec.z);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_RATE(agent, 2.0);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("robot_armspin_wind"), 5);
        EffectModule::kill_kind(boma, Hash40::new("robot_armspin"), false, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 10.5, 5, 30, -15, -20, 1.5, true, *EF_FLIP_YZ);
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let color_vec = match color {
            0 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.1, 0.01, 0.0) } else { Vector3f::new(0.196, 0.196, 0.216) },
            1 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.196, 0.196, 0.216) } else { Vector3f::new(0.22, 0.059, 0.039) },
            2 => Vector3f::new(0.176, 0.137, 0.059),
            3 => Vector3f::new(0.235, 0.196, 0.255),
            4 => Vector3f::new(0.098, 0.157, 0.196),
            5 => Vector3f::new(0.098, 0.059, 0.0),
            6 => Vector3f::new(0.098, 0.098, 0.157),
            7 => Vector3f::new(0.118, 0.039, 0.051),
            _ => Vector3f::new(0.196, 0.196, 0.216)
        };
        LAST_EFFECT_SET_COLOR(agent, color_vec.x, color_vec.y, color_vec.z);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("robot_nozzle_flare"), Hash40::new("knee1"), 1.5, 0, 0, 90, -90, 0, 1, true);
    }
    frame(lua_state, 15.0);
    for _ in 0..20 {
        if is_excute(agent) {
            if agent.is_situation(*SITUATION_KIND_AIR) {
                LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 2, 2, 0, 160, 0, 0, 1.0, 10, 0, 4, 0, 0, 0, true);
            } else {
                LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 10, 0, 4, 0, 0, 0, true);
            }
            LAST_EFFECT_SET_RATE(agent, 0.75);
        }
        wait(lua_state, 15.0);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_fire_s_damage"));
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialhirise(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charge_frame = VarModule::get_int(agent.battle_object, vars::robot::instance::SPECIAL_HI_CHARGE_FRAME) as f32;

    frame(lua_state, 1.0);
    if is_excute(agent) {
        let mut damage = charge_frame / 3.0;
        if charge_frame <= 10.0 {
            MeterModule::drain_direct(agent.object(), 20.0);
        } else {
            MeterModule::drain_direct(agent.object(), charge_frame * 2.0);
        }
        if charge_frame >= 10.0 {
            let rot = VarModule::get_float(boma.object(), vars::robot::instance::SPECIAL_HI_ROT_X);
            let angle = (270.0 - (rot * agent.lr() * 0.5)) as u64;
            //println!("{}", angle);
            /* Ground-only */
            ATTACK(agent, 0, 0, Hash40::new("knee"), damage, angle, 80, 0, 15, 5.0, -4.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 1, 0, Hash40::new("knee"), damage, angle, 80, 0, 15, 7.5, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 2, 0, Hash40::new("knee"), damage, angle, 80, 0, 15, 5.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            /* Air-only */
            ATTACK(agent, 3, 0, Hash40::new("knee"), damage, angle, 50, 0, 15, 5.0, -4.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 4, 0, Hash40::new("knee"), damage, angle, 50, 0, 15, 7.5, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 5, 0, Hash40::new("knee"), damage, angle, 50, 0, 15, 5.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 
        if VarModule::is_flag(agent.battle_object, vars::robot::instance::GROUNDED_UPB) { 0.25 }
        else { 0.35 }
    );
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::robot::instance::UPB_CANCEL);
    }
}

unsafe extern "C" fn effect_specialhirise(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charge_frame = VarModule::get_int(agent.battle_object, vars::robot::instance::SPECIAL_HI_CHARGE_FRAME);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("robot_nozzle_flare"), Hash40::new("knee1"), 1.5, 0, 0, 90, -90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.55, 2.25);
    }
    if charge_frame >= 10 {
        frame(lua_state, 1.0);
        let color: [f32;3] = [ // fades from red to blue as the charge increases
                /* R */ (1.0 - ((charge_frame - 10) as f32 * 0.02)).clamp(0.15, 1.0),
                /* G */ 0.55,
                /* B */ (0.0 + ((charge_frame - 10) as f32 * 0.25)).clamp(0.0, 10.0)
            ];
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("robot_atk_lw_jet"), Hash40::new("knee"), 0, 0, 0, -90, -90, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            if charge_frame >= 30 { LAST_EFFECT_SET_COLOR(agent, color[0], color[1], color[2]); }
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.75, 1.0));
            
            EFFECT_FOLLOW(agent, Hash40::new("robot_atk_lw_jet"), Hash40::new("knee1"), 0, 0, 0, -90, -90, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 1.5);
            LAST_EFFECT_SET_ALPHA(agent, 0.75);
            if charge_frame >= 30 { LAST_EFFECT_SET_COLOR(agent, color[0], color[1], color[2]); }
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.75, 1.0));
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("robot_nozzle_flare"), Hash40::new("knee1"), 1.5, 0, 0, 90, -90, 0, 1, true);
            if charge_frame >= 30 { LAST_EFFECT_SET_COLOR(agent, color[0], color[1], color[2]); }
        }
        frame(lua_state, 11.0);
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("robot_nozzle_flare"), false, false);
        }
    } else {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("knee"), 0, 0, 0, -90, -90, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.75, 1.0));
            
            EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("knee1"), 0, 0, 0, -90, -90, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 1.5);
            LAST_EFFECT_SET_ALPHA(agent, 0.75);
            LAST_EFFECT_SET_COLOR(agent, 0.55, 0.1, 0.1);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.75, 1.0));
        }
    }
}

unsafe extern "C" fn expression_specialhirise(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charge_frame = VarModule::get_int(agent.battle_object, vars::robot::instance::SPECIAL_HI_CHARGE_FRAME);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if charge_frame >= 48 {
            QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        } else if charge_frame >= 32{
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        } else if charge_frame >= 16 {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
        RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_ROBOT_STATUS_GYRO_FLAG_SHOOT);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_ROBOT_GENERATE_ARTICLE_GYRO, false,  app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 21.0/(37.0-10.0));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairs", game_specials, Priority::Low);
    agent.acmd("effect_specials", effect_specials, Priority::Low);
    agent.acmd("effect_specialairs", effect_specials, Priority::Low);
    agent.acmd("sound_specials", sound_specials, Priority::Low);
    agent.acmd("sound_specialairs", sound_specialairs, Priority::Low);
    agent.acmd("expression_specials", expression_specials, Priority::Low);
    agent.acmd("expression_specialairs", expression_specials, Priority::Low);

    agent.acmd("game_specialshi", game_specialshi, Priority::Low);
    agent.acmd("game_specialairshi", game_specialshi, Priority::Low);
    agent.acmd("effect_specialshi", effect_specialshi, Priority::Low);
    agent.acmd("effect_specialairshi", effect_specialshi, Priority::Low);
    agent.acmd("expression_specialshi", expression_specials, Priority::Low);
    agent.acmd("expression_specialairshi", expression_specials, Priority::Low);

    agent.acmd("game_specialslw", game_specialslw, Priority::Low);
    agent.acmd("game_specialairslw", game_specialslw, Priority::Low);
    agent.acmd("effect_specialslw", effect_specialslw, Priority::Low);
    agent.acmd("effect_specialairslw", effect_specialslw, Priority::Low);
    agent.acmd("expression_specialslw", expression_specials, Priority::Low);
    agent.acmd("expression_specialairslw", expression_specials, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialhi, Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi, Priority::Low);
    agent.acmd("sound_specialairhi", sound_specialhi, Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi, Priority::Low);
    agent.acmd("expression_specialairhi", expression_specialhi, Priority::Low);

    agent.acmd("game_specialhirise", game_specialhirise, Priority::Low);
    agent.acmd("effect_specialhirise", effect_specialhirise, Priority::Low);
    agent.acmd("expression_specialhirise", expression_specialhirise, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    
    agent.acmd("sound_specialsstart", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairsstart", acmd_stub, Priority::Low);
    agent.acmd("sound_specialsend", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairsend", acmd_stub, Priority::Low);
}
