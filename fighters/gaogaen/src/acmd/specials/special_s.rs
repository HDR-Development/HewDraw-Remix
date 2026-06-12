use super::*;

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_START);
        VarModule::off_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_ALTERNATE_GRAB);
        VarModule::off_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_LOW_GRAB);
        VarModule::off_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_HIGH_GRAB);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if ControlModule::get_stick_y(boma) < -0.5 {
            VarModule::on_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_ALTERNATE_GRAB);
            VarModule::on_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_LOW_GRAB);
        }
        else if ControlModule::get_stick_y(boma) > 0.5 {
            VarModule::on_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_ALTERNATE_GRAB);
            VarModule::on_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_HIGH_GRAB);
        }
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_ALTERNATE_GRAB) {
            // Spawn the special windboxes if performing OTG grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_LOW_GRAB) {
                let offset = if agent.is_situation(*SITUATION_KIND_GROUND) { 4.0 } else { 3.0 };
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 70, 100, 1, 0, 4.0, 0.0, offset, 2.0, Some(0.0), Some(offset), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                AttackModule::set_down_only(boma, 0, true);
            }
        }
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 17.0);
    FT_MOTION_RATE_RANGE(agent, 17.0, 35.0, 17.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_ALTERNATE_GRAB) {
            // Spawn the air grab/OTG grab box
            let (low_offset, size_high, collision_mask) = if agent.is_situation(*SITUATION_KIND_GROUND)
                { (4.0, 5.5, *COLLISION_SITUATION_MASK_GA) } else { (3.0, 3.5, *COLLISION_SITUATION_MASK_G) };
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_LOW_GRAB) {
                CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, low_offset, 2.0, Some(0.0), Some(low_offset), Some(8.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, collision_mask);
            }
            else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_HIGH_GRAB) {
                CATCH(agent, 0, Hash40::new("top"), size_high, 0.0, 15.0, 0.0, Some(0.0), Some(15.0), Some(6.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_A);
            }
        }
        // If the regular grab, then just spawn the grab box
        else {
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(8.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_GA);
            }
            else {
                CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(8.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 1, Hash40::new("top"), 2.5, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(9.5), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_A);
            }
        }
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if !VarModule::is_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_LOW_GRAB) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
        GrabModule::set_rebound(boma, false);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_LOW_GRAB) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_END);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
        }
    }
    frame(lua_state, 44.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 67.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
    frame(lua_state, 74.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 10, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        // OTG/Anti-air grab effects
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_ALTERNATE_GRAB) {
            // OTG grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_LOW_GRAB) {
                EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.6, true);
                LAST_EFFECT_SET_RATE(agent, 1.5);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::SPECIAL_S_HIGH_GRAB) {
                EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.6, true);
                LAST_EFFECT_SET_COLOR(agent, 0.25, 0.5, 3.0);
                LAST_EFFECT_SET_RATE(agent, 1.5);
            }
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 18.0);
    for _ in 0..8{
        if is_excute(agent) {
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(lua_state, 2.0);
    }
}

unsafe extern "C" fn game_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 37.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_THROW_ROPE);
    }
    frame(lua_state, 60.0);
    FT_MOTION_RATE(agent, 1);
    frame(lua_state, 95.0);
}

unsafe extern "C" fn effect_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_catch"), Hash40::new("sys_catch"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.9, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_specialslariat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        if agent.is_flag(*FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) 
        || agent.is_situation(*SITUATION_KIND_AIR) {
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 145, 474, 0, 20, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        else {
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 179, 0, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        AttackModule::set_force_reaction(boma, 0, true, true);
        WorkModule::set_float(boma, 9.0, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLOAT_LARIAT_HIT_FRAME);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE)
        || agent.is_situation(*SITUATION_KIND_AIR) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 145, 40, 0, 88, 7.0, 0.0, 8.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 50, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 105, 34, 0, 88, 7.0, 0.0, 8.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 10, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 50.0, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        REVERSE_LR(agent);
    }
    frame(lua_state, 50.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

unsafe extern "C" fn game_specialsshoulder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 98, 68, 0, 68, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_force_reaction(boma, 0, true, true);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 98, 34, 0, 128, 9.0, 0.0, 10.0, -5.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 10, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        REVERSE_LR(agent);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart, Priority::Low);
    agent.acmd("effect_specialsstart", effect_specialsstart, Priority::Low);
    agent.acmd("effect_specialairsstart", effect_specialsstart, Priority::Low);

    agent.acmd("game_specialsthrow", game_specialsthrow, Priority::Low);
    agent.acmd("game_specialairsthrow", game_specialsthrow, Priority::Low);
    agent.acmd("effect_specialsthrow", effect_specialsthrow, Priority::Low);
    agent.acmd("effect_specialairsthrow", effect_specialsthrow, Priority::Low);

    agent.acmd("game_specialslariat", game_specialslariat, Priority::Low);
    agent.acmd("game_specialairslariat", game_specialslariat, Priority::Low);
    
    agent.acmd("game_specialsshoulder", game_specialsshoulder, Priority::Low);
    agent.acmd("game_specialairsshoulder", game_specialsshoulder, Priority::Low);
}