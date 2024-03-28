
use super::*;

unsafe extern "C" fn game_floatstart(agent: &mut L2CAgentBase) {
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
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_GROUND_CHANGE_KINETIC);
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_floatstart(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn expression_floatstart(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn sound_floatstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
}

unsafe extern "C" fn game_floatairstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0 / 10.0);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
    KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

unsafe extern "C" fn effect_floatairstart(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn expression_floatairstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn sound_floatairstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
}

unsafe extern "C" fn game_float(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 20.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_FALL_SPEED_Y_INCREASE);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_float(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn expression_float(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 350, 100, 30, 0, 5.0, 0.0, 9.0, 1.0, Some(0.0), Some(9.0), Some(6.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        CATCH(agent, 0, Hash40::new("top"), 5.25, 0.0, 10.75, 7.25, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairscatch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        let lr = PostureModule::lr(boma);
        let stick_x = ControlModule::get_stick_x(boma) * lr;
        let speed_x = 0.5 * stick_x;
        WorkModule::set_float(boma, speed_x + 1.0, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_X);
        WorkModule::set_float(boma, 2.0, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_Y);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_GANON_EXPLOSION_FALL_SETTING_CATCH, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_FALL_SETTING);
    }
}

unsafe extern "C" fn game_specialairsfall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        let speed_x = WorkModule::get_float(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_X);
        SET_SPEED_EX(agent, speed_x, -5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_GANON_EXPLOSION_FALL_SETTING_FALL, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_FALL_SETTING);
    }
}

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 15.0, 361, 82, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 3.0, 6.0, 8.5, 9.5);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 2.0, 6.0, 8.5, 10.0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 9.0, 0.0, 10.5, 6.0, 0.0, 12.5, -7.0, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        ATTACK(agent, 0, 0, Hash40::new("legr"), 14.0, 42, 70, 0, 65, 4.0, 0.0, 0.0, 0.0, Some(-2.7), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 14.0, 42, 70, 0, 65, 4.0, 1.0, 0.0, 0.0, Some(-2.7), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 16.0, 42, 70, 0, 65, 4.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 12.0, 42, 70, 0, 65, 4.0, 0.0, 0.0, 0.0, Some(-2.7), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 12.0, 42, 70, 0, 65, 4.0, 1.0, 0.0, 0.0, Some(-2.7), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 14.0, 42, 70, 0, 65, 4.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 8.0, 8.0, 8.0, 4.0);
    }
    frame(lua_state, 36.0);
    FT_MOTION_RATE(agent, 30.0/(61.0 - 36.0));
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }

}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
        JostleModule::set_status(boma, false);
        /* Ground-only */
        ATTACK(agent, 0, 0, Hash40::new("legl"), 15.0, 290, 100, 0, 50, 5.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 15.0, 290, 100, 0, 50, 3.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        /* Air-only */
        ATTACK(agent, 2, 0, Hash40::new("legl"), 15.0, 290, 36, 0, 50, 5.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("legl"), 15.0, 290, 36, 0, 50, 3.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
        ATTACK(agent, 0, 0, Hash40::new("legl"), 14.0, 80, 100, 0, 50, 5.0, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 14.0, 80, 100, 0, 50, 3.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn game_specialairlwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.790);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 80, 35, 0, 80, 4.5, 0.0, 3.2, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 80, 35, 0, 80, 4.8, 0.0, 3.2, -9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 80, 35, 0, 80, 4.8, 0.0, 3.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 19.0);
    if is_excute(agent) {  }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 6.5, 0.0, 16.0, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 5.0, 0.0, 6.75, 7.75, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
        CATCH(agent, 0, Hash40::new("bust"), 5.25, 0.0, 0.0, 2.25, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        //ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 70, 9.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        //ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 70, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        sv_kinetic_energy!(
            reset_energy,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let air_accel_x_mul = WorkModule::get_param_float(agent.module_accessor, hash40("air_accel_x_mul"), 0);
        let special_hi_speed_x_mul = WorkModule::get_param_float(
            agent.module_accessor,
            hash40("param_special_hi"),
            hash40("special_hi_speed_x_mul")
        );
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            agent,
            air_accel_x_mul * special_hi_speed_x_mul
        );
        let air_speed_x_stable = WorkModule::get_param_float(agent.module_accessor, hash40("air_speed_x_stable"), 0);
        let special_hi_speed_max_x_mul = WorkModule::get_param_float(
            agent.module_accessor,
            hash40("param_special_hi"),
            hash40("special_hi_speed_max_x_mul")
        );
        sv_kinetic_energy!(
            set_stable_speed,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * special_hi_speed_max_x_mul,
            0.0
        );
    }
    frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_raijin_hold"), Hash40::new("head"), 2, 0, 0, 0, 0, 0, 1.3, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    /* vanilla electric effects
    for _ in 0..4 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("ganon_attack_elec"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 42.0);
    */
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_raijin_hold"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_floatstart", game_floatstart);
    agent.acmd("effect_floatstart", effect_floatstart);
    agent.acmd("expression_floatstart", expression_floatstart);
    agent.acmd("sound_floatstart", sound_floatstart);

    agent.acmd("game_floatairstart", game_floatairstart);
    agent.acmd("effect_floatairstart", effect_floatairstart);
    agent.acmd("expression_floatairstart", expression_floatairstart);
    agent.acmd("sound_floatairstart", sound_floatairstart);

    agent.acmd("game_float", game_float);
    agent.acmd("effect_float", effect_float);
    agent.acmd("expression_float", expression_float);

    agent.acmd("game_specialairsstart", game_specialairsstart);

    agent.acmd("game_specialairscatch", game_specialairscatch);

    agent.acmd("game_specialairsfall", game_specialairsfall);

    agent.acmd("game_specialairs", game_specialairs);

    agent.acmd("game_speciallw", game_speciallw);

    agent.acmd("game_specialairlw", game_specialairlw);

    agent.acmd("game_specialairlwend", game_specialairlwend);
    
    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_specialairhi", game_specialhi);
    agent.acmd("effect_specialhi", effect_specialhi);
    agent.acmd("effect_specialairhi", effect_specialhi);
}
