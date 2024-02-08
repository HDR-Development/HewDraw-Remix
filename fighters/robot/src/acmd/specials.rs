use super::*;

#[acmd_script( agent = "robot", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn robot_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_ROBOT_STATUS_GYRO_FLAG_SHOOT);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_ROBOT_GENERATE_ARTICLE_GYRO, false,  app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 21.0/(37.0-10.0));
    }
}

#[acmd_script( agent = "robot", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn robot_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_ROBOT_STATUS_GYRO_FLAG_SHOOT);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_ROBOT_GENERATE_ARTICLE_GYRO, false,  app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 21.0/(37.0-10.0));
    }
}

#[acmd_script( agent = "robot", scripts = ["game_specials", "game_specialairs"] , category = ACMD_GAME , low_priority)]
unsafe fn robot_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -3.0, Some(0.0), Some(10.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -13.0, Some(0.0), Some(10.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        VarModule::set_float(fighter.battle_object, vars::robot::instance::STICK_ANGLE, ControlModule::get_stick_y(fighter.module_accessor));
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr1"), 9.0, 361, 95, 0, 55, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 9.0, 361, 95, 0, 55, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "robot", scripts = ["effect_specials", "effect_specialairs"], category = ACMD_EFFECT, low_priority )]
unsafe fn robot_special_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_armspin_wind"), Hash40::new("body"), 1, 0, 0, 0, -100, -90, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("robot_armspin"), Hash40::new("body"), 1, 0, 0, 0, 60, 90, 1, true);
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
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_RATE(fighter, 2.0);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("robot_armspin_wind"), 5);
        EffectModule::kill_kind(boma, Hash40::new("robot_armspin"), false, false);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 11.5, 3, 0, -15, 0, 1.5, true, *EF_FLIP_YZ);
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
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
    }
    
}

#[acmd_script( agent = "robot", scripts = ["game_specialshi", "game_specialairshi"] , category = ACMD_GAME , low_priority)]
unsafe fn robot_special_s_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -3.0, Some(0.0), Some(10.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -13.0, Some(0.0), Some(10.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        VarModule::set_float(fighter.battle_object, vars::robot::instance::STICK_ANGLE, ControlModule::get_stick_y(fighter.module_accessor));
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr1"), 7.0, 105, 85, 0, 55, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 7.0, 105, 85, 0, 55, 5.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "robot", scripts = ["effect_specialshi", "effect_specialairshi"], category = ACMD_EFFECT, low_priority )]
unsafe fn robot_special_s_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_armspin_wind"), Hash40::new("body"), 1, 0, 0, 0, -100, -90, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("robot_armspin"), Hash40::new("body"), 1, 0, 0, 0, 60, 90, 1, true);
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
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_RATE(fighter, 2.0);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("robot_armspin_wind"), 5);
        EffectModule::kill_kind(boma, Hash40::new("robot_armspin"), false, false);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 15.5, 4, 45, 0, -225, 1.5, true, *EF_FLIP_NONE);
        }
        else {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 15.5, 4, -45, 0, 45, 1.5, true, *EF_FLIP_NONE);
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
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        LAST_EFFECT_SET_SCALE_W(fighter, 0.8, 1.5, 1.5);
    }

}

#[acmd_script( agent = "robot", scripts = ["game_specialslw", "game_specialairslw"] , category = ACMD_GAME , low_priority)]
unsafe fn robot_special_s_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -3.0, Some(0.0), Some(10.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 100, 40, 0, 2.5, 0.0, 10.5, -13.0, Some(0.0), Some(10.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        VarModule::set_float(fighter.battle_object, vars::robot::instance::STICK_ANGLE, ControlModule::get_stick_y(fighter.module_accessor));
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr1"), 5.0, 25, 35, 0, 55, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 5.0, 25, 35, 0, 55, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "robot", scripts = ["effect_specialslw", "effect_specialairslw"], category = ACMD_EFFECT, low_priority )]
unsafe fn robot_special_s_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_armspin_wind"), Hash40::new("body"), 1, 0, 0, 0, -100, -90, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("robot_armspin"), Hash40::new("body"), 1, 0, 0, 0, 60, 90, 1, true);
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
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_RATE(fighter, 2.0);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("robot_armspin_wind"), 5);
        EffectModule::kill_kind(boma, Hash40::new("robot_armspin"), false, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 10.5, 5, 30, -15, -20, 1.5, true, *EF_FLIP_YZ);
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
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
    }

}

#[acmd_script( agent = "robot", scripts = ["expression_specials", "expression_specialairs", "expression_specialshi", "expression_specialairshi", "expression_specialslw", "expression_specialairslw"] , category = ACMD_EXPRESSION , low_priority)]
unsafe fn robot_special_s_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_spinattacks"), 5, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 4);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 5, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "robot", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME, low_priority )]
unsafe fn robot_special_hi_game (fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "robot", script = "game_specialhirise", category = ACMD_GAME, low_priority )]
unsafe fn robot_special_hi_rise_game (fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let robotFrames = VarModule::get_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB);

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, vars::robot::status::HELD_BUTTON);
        let mut workingDamage = robotFrames/3.5;

        if robotFrames <= 10.0 {
            MeterModule::drain_direct(fighter.object(), 20.0);
        } else {
            MeterModule::drain_direct(fighter.object(), (robotFrames * 2.0));
        }

        if (workingDamage < 4.0) {
            workingDamage = 0.0;
        }

        if workingDamage != 0.0 {
            /* Ground-only */
            ATTACK(fighter, 0, 0, Hash40::new("knee"), workingDamage, 280, 80, 0, 15, 5.0, -4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 1, 0, Hash40::new("knee"), workingDamage, 280, 80, 0, 15, 7.5, 5.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 2, 0, Hash40::new("knee"), workingDamage, 280, 80, 0, 15, 5.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            /* Air-only */
            ATTACK(fighter, 3, 0, Hash40::new("knee"), workingDamage, 280, 50, 0, 15, 5.0, -4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 4, 0, Hash40::new("knee"), workingDamage, 280, 50, 0, 15, 7.5, 5.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 5, 0, Hash40::new("knee"), workingDamage, 280, 50, 0, 15, 5.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }

    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);

        if VarModule::is_flag(fighter.battle_object, vars::robot::instance::GROUNDED_UPB) {
            FT_MOTION_RATE(fighter, 9.0/(22.0-4.0));
        }
    }

    /*frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::robot::instance::GROUNDED_UPB) {
            VarModule::on_flag(fighter.battle_object, vars::robot::instance::UPB_CANCEL);
        }
    }*/

    frame(lua_state, 22.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::robot::instance::UPB_CANCEL);
    }

}

#[acmd_script( agent = "robot", script = "effect_specialhirise" , category = ACMD_EFFECT , low_priority)]
unsafe fn robot_special_hi_rise_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let robotFrames = VarModule::get_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_nozzle_flare"), Hash40::new("knee1"), 1.5, 0, 0, 90, -90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.55, 0.55, 2.25);
    }
    if (robotFrames/3.5) > 4.0 {
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("robot_atk_lw_jet"), Hash40::new("knee"), 0, 0, 0, -90, -90, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 8.55);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.75, 1.0));
            
            EFFECT_FOLLOW(fighter, Hash40::new("robot_atk_lw_jet"), Hash40::new("knee1"), 0, 0, 0, -90, -90, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            LAST_EFFECT_SET_ALPHA(fighter, 0.75);
            LAST_EFFECT_SET_COLOR(fighter, 3.15, 0.55, 0.55);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.75, 1.0));
        }
        frame(lua_state, 2.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("robot_nozzle_flare"), Hash40::new("knee1"), 1.5, 0, 0, 90, -90, 0, 1, true);
        }
        frame(lua_state, 11.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("robot_nozzle_flare"), false, false);
        }
    } else {
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("knee"), 0, 0, 0, -90, -90, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.75, 1.0));
            
            EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("knee1"), 0, 0, 0, -90, -90, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            LAST_EFFECT_SET_ALPHA(fighter, 0.75);
            LAST_EFFECT_SET_COLOR(fighter, 0.55, 0.1, 0.1);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.75, 1.0));
        }
    }
}

#[acmd_script( agent = "robot", script = "effect_specialhi", category = ACMD_EFFECT, low_priority )]
unsafe fn robot_special_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_nozzle_flare"), Hash40::new("knee1"), 1.5, 0, 0, 90, -90, 0, 1, true);
    }
}

#[acmd_script( agent = "robot", script = "effect_specialairhi", category = ACMD_EFFECT, low_priority )]
unsafe fn robot_special_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_nozzle_flare"), Hash40::new("knee1"), 1.5, 0, 0, 90, -90, 0, 1, true);
    }
}

#[acmd_script( agent = "robot", script = "sound_specialhi", category = ACMD_SOUND, low_priority )]
unsafe fn robot_special_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_fire_s_damage"));
    }
}

#[acmd_script( agent = "robot", script = "sound_specialairhi", category = ACMD_SOUND, low_priority )]
unsafe fn robot_special_air_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_fire_s_damage"));
    }
}


#[acmd_script( agent = "robot", scripts = ["expression_specialhi", "expression_specialairhi"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn robot_special_hi_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "robot", script = "sound_specialairsstart", category = ACMD_SOUND, low_priority )]
unsafe fn robot_special_air_s_start_sound(fighter: &mut L2CAgentBase) {
}
   
#[acmd_script( agent = "robot", script = "sound_specialairs", category = ACMD_SOUND, low_priority )]
unsafe fn robot_special_air_s_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_robot_special_s01"));
    }
}

#[acmd_script( agent = "robot", script = "sound_specialairsend", category = ACMD_SOUND, low_priority )]
unsafe fn robot_special_air_s_end_sound(fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "robot", script = "sound_specialsstart", category = ACMD_SOUND, low_priority )]
unsafe fn robot_special_s_start_sound(fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "robot", script = "sound_specials", category = ACMD_SOUND, low_priority )]
unsafe fn robot_special_s_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_robot_special_s01"));
    }

    frame(lua_state, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_robot_special_s04"));
    }
}

#[acmd_script( agent = "robot", script = "sound_specialsend", category = ACMD_SOUND, low_priority )]
unsafe fn robot_special_s_end_sound(fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "robot", script = "expression_specialhirise", category = ACMD_EXPRESSION, low_priority )]
unsafe fn robot_special_hi_rise_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let robotFrames = VarModule::get_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB);

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if (robotFrames/4.0) >= 12.0 {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        } else if (robotFrames/4.0) >= 8.0 {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        } else if (robotFrames/4.0) >= 4.0 {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
        
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        robot_special_lw_game,
        robot_special_air_lw_game,
        robot_special_s_game,
        robot_special_s_hi_game,
        robot_special_s_lw_game,
        robot_special_s_expression,
        robot_special_hi_game,
        robot_special_hi_rise_game,

        robot_special_s_effect,
        robot_special_s_lw_effect,
        robot_special_s_hi_effect,
        robot_special_hi_effect,
        robot_special_air_hi_effect,
        robot_special_hi_expression,
        robot_special_hi_rise_effect,

        robot_special_hi_sound,
        robot_special_air_hi_sound,
        robot_special_air_s_start_sound,
        robot_special_air_s_sound,
        robot_special_air_s_end_sound,
        robot_special_s_start_sound,
        robot_special_s_sound,
        robot_special_s_end_sound,

        robot_special_hi_rise_expression
    );
}