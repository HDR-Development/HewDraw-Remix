use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 0.444);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 160, 40, 0, 12, 9.5, 0.0, 7.0, -0.5, Some(0.0), Some(7.0), Some(0.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 160, 40, 0, 12, 4.0, 0.0, 8.0, -10.0, Some(0.0), Some(8.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 50, 7.0, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 4.0, 0.0, 8.0, -11.0, Some(0.0), Some(8.0), Some(11.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END);
    }
}

unsafe extern "C" fn game_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 0.444);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START);  
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 77, 80, 0, 50, 9.5, 0.0, 7.0, -0.5, Some(0.0), Some(7.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 77, 80, 0, 40, 4.0, 0.0, 8.0, -10.0, Some(0.0), Some(8.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 60, 100, 0, 50, 9.5, 0.0, 7.0, -0.5, Some(0.0), Some(7.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 60, 100, 0, 40, 4.0, 0.0, 8.0, -11.0, Some(0.0), Some(8.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END);
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_1);
        //VarModule::on_flag(agent.battle_object, vars::zelda::instance::DEIN_ACTIVE);
    }
}

unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("zelda_appeal_s_fire"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, true);    
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 8.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 90, 30, 0, 125, 9.5, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 8.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 80, 85, 0, 60, 10.5, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let lr = sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR);
    if is_excute(agent) {
        if lr < 0.0 {
            EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("zelda_flor_start_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        } else {
            EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("zelda_flor_start_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
        }
        LAST_EFFECT_SET_SCALE_W(agent, 0.85, 0.95, 0.85);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            if lr < 0.0 {
                EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, false);
            }
            else {
                EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, false);
            }
            LAST_EFFECT_SET_SCALE_W(agent, 0.55, 0.8, 0.55);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("zelda_flor_teleport"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, false);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.15);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        FLASH(agent, 0.18, 0.87, 0, 0.6);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        FLASH(agent, 0.18, 0.87, 0, 0.6);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        FLASH(agent, 0.6, 1, 1, 0.5);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 1.0);
    if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        FT_MOTION_RATE_RANGE(agent, 1.0, 35.0, 13.0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            ATTACK(agent, 0, 0, Hash40::new("rot"), 10.0, 361, 89, 0, 80, 7.5, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            ATTACK(agent, 1, 0, Hash40::new("rot"), 7.0, 361, 87, 0, 60, 11.2, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 0.75);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0); //f4
    if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        FT_MOTION_RATE_RANGE(agent, 11.0, 35.0, 29.0);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            ATTACK(agent, 0, 0, Hash40::new("rot"), 12.0, 55, 93, 0, 80, 7.5, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            ATTACK(agent, 1, 0, Hash40::new("rot"), 8.0, 55, 85, 0, 60, 11.2, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 0.75);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_DIVE);
        WorkModule::on_flag(boma, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    let lr = sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR);
    if is_excute(agent) {
        FLASH(agent, 0.62, 0.94, 0.9, 0.6);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT_FOLLOW(agent, Hash40::new("zelda_atk"), Hash40::new("top"), 4.5 * lr, 10.0, -4.0, 0, 0, 0, 1.65, true);
            LAST_EFFECT_SET_COLOR(agent, 0.95, 3.0, 0.6);
            LAST_EFFECT_SET_ALPHA(agent, 0.75);
            LAST_EFFECT_SET_RATE(agent, 1.15);
        } else {
            if lr < 0.0 {
                EFFECT_FOLLOW(agent, Hash40::new("zelda_flor_end_l"), Hash40::new("top"), 0, 9, -0.75, 0, 0, 0, 1.0, false);
            } else {
                EFFECT_FOLLOW(agent, Hash40::new("zelda_flor_end_r"), Hash40::new("top"), 0, 9, -0.75, 0, 0, 0, 1.0, false);
            }
            LAST_EFFECT_SET_SCALE_W(agent, 0.5, 0.77, 0.5);
            LAST_EFFECT_SET_RATE(agent, 1.31);
            EffectModule::set_disable_render_offset_last(boma);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("zelda_flor_teleport"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 1.5);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FLASH(agent, 0.33, 0.83, 0.9, 0.2);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FLASH(agent, 0.6, 1, 1, 0.5);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FLASH(agent, 0.33, 0.83, 0.9, 0.2);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            if lr < 0.0 {
                EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 0.75, false);
            }
            else {
                EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 0.75, false);
            }
            LAST_EFFECT_SET_SCALE_W(agent, 0.55, 0.8, 0.55);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("zelda_flor_end_l"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("zelda_flor_end_r"), true, true);
    }
}

unsafe extern "C" fn game_landingfallspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        FT_MOTION_RATE(agent, 0.5);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Low);
    agent.acmd("game_specialairn", game_specialairn, Priority::Low);

    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart, Priority::Low);
    agent.acmd("effect_specialsend", effect_specialsend, Priority::Low);
    agent.acmd("effect_specialairsend", effect_specialsend, Priority::Low);

    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhistart", game_specialairhistart, Priority::Low);
    agent.acmd("effect_specialhistart", effect_specialhistart, Priority::Low);
    agent.acmd("effect_specialairhistart", effect_specialhistart, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialairhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialhi, Priority::Low);
    
    agent.acmd("game_landingfallspecial", game_landingfallspecial, Priority::Low);
}
