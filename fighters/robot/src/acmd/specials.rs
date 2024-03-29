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
    frame(lua_state, 18.0);
    if is_excute(agent) {
        VarModule::set_float(agent.battle_object, vars::robot::instance::STICK_ANGLE, ControlModule::get_stick_y(boma));
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
    frame(lua_state, 20.0);
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
    frame(lua_state, 18.0);
    if is_excute(agent) {
        VarModule::set_float(agent.battle_object, vars::robot::instance::STICK_ANGLE, ControlModule::get_stick_y(boma));
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
    frame(lua_state, 18.0);
    if is_excute(agent) {
        VarModule::set_float(agent.battle_object, vars::robot::instance::STICK_ANGLE, ControlModule::get_stick_y(boma));
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
    let robotFrames = VarModule::get_float(agent.battle_object, vars::robot::instance::FRAMES_SINCE_UPB);

    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, vars::robot::status::HELD_BUTTON);
        let mut workingDamage = robotFrames/3.5;
        if robotFrames <= 10.0 {
            MeterModule::drain_direct(agent.object(), 20.0);
        } else {
            MeterModule::drain_direct(agent.object(), (robotFrames * 2.0));
        }
        if (workingDamage < 4.0) {
            workingDamage = 0.0;
        }
        if workingDamage != 0.0 {
            /* Ground-only */
            ATTACK(agent, 0, 0, Hash40::new("knee"), workingDamage, 280, 80, 0, 15, 5.0, -4.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 1, 0, Hash40::new("knee"), workingDamage, 280, 80, 0, 15, 7.5, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 2, 0, Hash40::new("knee"), workingDamage, 280, 80, 0, 15, 5.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            /* Air-only */
            ATTACK(agent, 3, 0, Hash40::new("knee"), workingDamage, 280, 50, 0, 15, 5.0, -4.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 4, 0, Hash40::new("knee"), workingDamage, 280, 50, 0, 15, 7.5, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 5, 0, Hash40::new("knee"), workingDamage, 280, 50, 0, 15, 5.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);

        if VarModule::is_flag(agent.battle_object, vars::robot::instance::GROUNDED_UPB) {
            FT_MOTION_RATE(agent, 9.0/(22.0-4.0));
        }
    }
    /*frame(lua_state, 12.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::robot::instance::GROUNDED_UPB) {
            VarModule::on_flag(agent.battle_object, vars::robot::instance::UPB_CANCEL);
        }
    }*/
    frame(lua_state, 22.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::robot::instance::UPB_CANCEL);
    }
}

unsafe extern "C" fn effect_specialhirise(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let robotFrames = VarModule::get_float(agent.battle_object, vars::robot::instance::FRAMES_SINCE_UPB);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("robot_nozzle_flare"), Hash40::new("knee1"), 1.5, 0, 0, 90, -90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.55, 2.25);
    }
    if (robotFrames/3.5) > 4.0 {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("robot_atk_lw_jet"), Hash40::new("knee"), 0, 0, 0, -90, -90, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 8.55);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.75, 1.0));
            
            EFFECT_FOLLOW(agent, Hash40::new("robot_atk_lw_jet"), Hash40::new("knee1"), 0, 0, 0, -90, -90, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 1.5);
            LAST_EFFECT_SET_ALPHA(agent, 0.75);
            LAST_EFFECT_SET_COLOR(agent, 3.15, 0.55, 0.55);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.75, 1.0));
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("robot_nozzle_flare"), Hash40::new("knee1"), 1.5, 0, 0, 90, -90, 0, 1, true);
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
    let robotFrames = VarModule::get_float(agent.battle_object, vars::robot::instance::FRAMES_SINCE_UPB);

    frame(lua_state, 1.0);
    if is_excute(agent) {
        if (robotFrames/4.0) >= 12.0 {
            QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        } else if (robotFrames/4.0) >= 8.0 {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        } else if (robotFrames/4.0) >= 4.0 {
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

unsafe extern "C" fn stub(agent: &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialairs", game_specials);
    agent.acmd("effect_specials", effect_specials);
    agent.acmd("effect_specialairs", effect_specials);
    agent.acmd("sound_specials", sound_specials);
    agent.acmd("sound_specialairs", sound_specialairs);
    agent.acmd("expression_specials", expression_specials);
    agent.acmd("expression_specialairs", expression_specials);

    agent.acmd("game_specialshi", game_specialshi);
    agent.acmd("game_specialairshi", game_specialshi);
    agent.acmd("effect_specialshi", effect_specialshi);
    agent.acmd("effect_specialairshi", effect_specialshi);
    agent.acmd("expression_specialshi", expression_specials);
    agent.acmd("expression_specialairshi", expression_specials);

    agent.acmd("game_specialslw", game_specialslw);
    agent.acmd("game_specialairslw", game_specialslw);
    agent.acmd("effect_specialslw", effect_specialslw);
    agent.acmd("effect_specialairslw", effect_specialslw);
    agent.acmd("expression_specialslw", expression_specials);
    agent.acmd("expression_specialairslw", expression_specials);

    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_specialairhi", game_specialhi);
    agent.acmd("effect_specialhi", effect_specialhi);
    agent.acmd("effect_specialairhi", effect_specialhi);
    agent.acmd("sound_specialhi", sound_specialhi);
    agent.acmd("sound_specialairhi", sound_specialhi);
    agent.acmd("expression_specialhi", expression_specialhi);
    agent.acmd("expression_specialairhi", expression_specialhi);

    agent.acmd("game_specialhirise", game_specialhirise);
    agent.acmd("effect_specialhirise", effect_specialhirise);
    agent.acmd("expression_specialhirise", expression_specialhirise);

    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("game_specialairlw", game_speciallw);
    
    agent.acmd("sound_specialsstart", stub);
    agent.acmd("sound_specialairsstart", stub);
    agent.acmd("sound_specialsend", stub);
    agent.acmd("sound_specialairsend", stub);
}
