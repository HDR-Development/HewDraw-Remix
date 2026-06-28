use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 8.0);//2f more startu
}

unsafe extern "C" fn game_specialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.7);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_REFLET_STATUS_SPECIAL_N_SHOOT_FLAG_TRY);
    }
    frame(lua_state, 13.0);
    if !agent.is_flag(*FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) {
        FT_MOTION_RATE_RANGE(agent, 13.0, 42.0, 26.0);//45 empty
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
    }
}

unsafe extern "C" fn game_specialntronend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();//FAF is frame 80/86
    frame(lua_state, 34.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
    }
}

unsafe extern "C" fn game_specialairntronend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 1.0, 65.0, 47.0);  //FAF is frame 80/86
    frame(lua_state, 4.0);
    if is_excute(agent) {//wait to start falling until she starts to move arm (end status starts rly early in anim)
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let gravity_mul = agent.get_param_float("param_special_n", "special_n_air_invoke_fall_speed_mul");
        let air_accel_y = agent.get_param_float("air_accel_y", "");
        sv_kinetic_energy!(set_accel, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * gravity_mul);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if !agent.is_flag(*FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) {
        FT_MOTION_RATE_RANGE(agent, 5.0, 54.0, 40.0);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_TRY);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        agent.dec_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
        if agent.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
            FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        }
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 8.0);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        agent.on_flag(*FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
        agent.on_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FAILURE_HOP);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        VarModule::on_flag(agent.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 12.0);
    MotionModule::set_rate(boma, 2.0);
    wait(lua_state, 1.0);
    for _ in 0..30 {
        if is_excute(agent) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                agent.on_flag( *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
            } else if boma.motion_frame() >= 30.0 && VarModule::is_flag(agent.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS) {
                VarModule::off_flag(agent.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
            }
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        agent.dec_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
        if agent.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
            FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, -1);
        agent.on_flag(*FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
    }
}

unsafe extern "C" fn game_specialhifail(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE_RANGE(agent, 35.0, 52.0, 10.0);
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if !agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
        FT_MOTION_RATE_RANGE(agent, 1.0, 20.0, 17.0);//+3
    } else {
        FT_MOTION_RATE_RANGE(agent, 1.0, 20.0, 11.0);//-5 total
    }
    frame(lua_state, 20.0);
    if !agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
        FT_MOTION_RATE_RANGE(agent, 20.0, 23.0, 7.0);//+5
        if is_excute(agent) {
            CATCH(agent, 0, Hash40::new("top"), 5.5, 0.0, 9.5, 29.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 1, Hash40::new("top"), 5.5, 0.0, 9.5, 29.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *COLLISION_SITUATION_MASK_A);
        }
    }
    frame(lua_state, 22.5);
    if !agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
        if is_excute(agent) {//fix last frame grab cancelling out of grab (status checks flag before it gets set?)
            agent.clear_lua_stack();
            lua_args!(agent, MA_MSC_CMD_GRAB_CLEAR, 0);
            sv_module_access::grab(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, MA_MSC_CMD_GRAB_CLEAR, 1);
            sv_module_access::grab(agent.lua_state_agent);
        }
    }
}

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
        frame(lua_state, 5.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 9.5, 29.0, 0, 0, 0, 0.50, true);
		    LAST_EFFECT_SET_COLOR(agent, 0.30, 0.0, 1.0);
            LAST_EFFECT_SET_RATE(agent, 8.0/12.0);
        }
        frame(lua_state, 10.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 14.0);
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_flash"), Hash40::new("havel"), -1, 1, 0, 0, 0, 0, 0.45, true);
        }
        frame(lua_state, 16.5);
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("reflet_rizaia"), Hash40::new("top"), 0, 5, 29.0, 0, 0, 0, 0.92, true);
            LAST_EFFECT_SET_RATE(agent, 3.9);
        }
    }
}

unsafe extern "C" fn game_speciallwcapture(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 70, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 1, Hash40::new("top"), 2.0, 70, 100, 50, 0, 7.0, 0.0, 9.5, 29.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 8, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK_IGNORE_THROW(agent, 2, 1, Hash40::new("top"), 2.0, 70, 100, 100, 0, 7.0, 0.0, 9.5, 29.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(agent.module_accessor, 1, 81, 20, 0.5, false);
        AttackModule::set_poison_param(agent.module_accessor, 2, 81, 20, 0.5, false);
        agent.set_int(1, *FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_INT_ATTACK_ID);
        AttackModule::set_catch_only_all(boma, true, false);
    }
}

unsafe extern "C" fn effect_speciallwcapture(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    loop {
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("reflet_rizaia_capture"), Hash40::new("top"), 0, 5, 29.0, 0, 0, 0, 1.02, true);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_INT_EFFECT_HANDLE);
            smash::app::sv_animcmd::LAST_EFFECT_SET_WORK_INT(lua_state);
        }
        wait(lua_state, 30.0);
    }
}


unsafe extern "C" fn game_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
        FT_MOTION_RATE_RANGE(agent, 1.0, 38.0, 32.0);//faf 58 faf -> 45 empty
    } else if !agent.is_flag(*FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_FLAG_MISS) {
        if is_excute(agent) {//idk how to code this to work with whatever capture status is used and however it decides when to mash out of the grab
            ATTACK(agent, 0, 1, Hash40::new("top"), 2.0, 70, 200, 0, 40, 8.0, 0.0, 9.5, 29.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);//van
    }
    frame(lua_state, 18.0);
    if !agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
        FT_MOTION_RATE_RANGE(agent, 18.0, 38.0, 15.0);//ending 40 faf -> 40, 65 faf whiff (+7)
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
    }
    frame(lua_state, 38.0);//46 end
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !agent.is_flag(*FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_FLAG_MISS) 
    && !agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("reflet_rizaia"), Hash40::new("top"), 0, 5, 29.0, 0, 0, 0, 1.03, true);
            LAST_EFFECT_SET_RATE(agent, 4.5);
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            EFFECT_DETACH_KIND(agent, Hash40::new("reflet_rizaia"), -1);
        }
    } else {
        if is_excute(agent) {
            EffectModule::kill_kind(boma, Hash40::new("sys_flash"), true, true);
            EFFECT_DETACH_KIND(agent, Hash40::new("reflet_rizaia"), -1);
        }

    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);//8 -> 10
    agent.acmd("game_specialairnstart", game_specialnstart, Priority::Low);

    agent.acmd("game_specialnshoot", game_specialnshoot, Priority::Low);
    agent.acmd("game_specialairnshoot", game_specialnshoot, Priority::Low);

    agent.acmd("game_specialntronstart", acmd_stub, Priority::Low);
    agent.acmd( "game_specialairntronstart",acmd_stub, Priority::Low);//17 -> 20
    agent.acmd("game_specialntronend", game_specialntronend, Priority::Low);//faf 80/86
    agent.acmd("game_specialairntronend", game_specialairntronend, Priority::Low);

    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairs", game_specials, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);

    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);
    agent.acmd("game_specialairhi2", game_specialhi2, Priority::Low);

    agent.acmd("game_specialhifail", game_specialhifail, Priority::Low);
    agent.acmd("game_specialairhifail", game_specialhifail, Priority::Low);

    agent.acmd("game_speciallwstart", game_speciallwstart, Priority::Low);
    agent.acmd("effect_speciallwstart", effect_speciallwstart, Priority::Low);
    agent.acmd("game_specialairlwstart", game_speciallwstart, Priority::Low);
    agent.acmd("effect_specialairlwstart", effect_speciallwstart, Priority::Low);

    agent.acmd("game_speciallwcapture", game_speciallwcapture, Priority::Low);
    agent.acmd("effect_speciallwcapture", effect_speciallwcapture, Priority::Low);
    agent.acmd("game_specialairlwcapture", game_speciallwcapture, Priority::Low);
    agent.acmd("effect_specialairlwcapture", effect_speciallwcapture, Priority::Low);

    agent.acmd("game_speciallwend", game_speciallwend, Priority::Low);
    agent.acmd("effect_speciallwend", effect_speciallwend, Priority::Low);
    agent.acmd("game_specialairlwend", game_speciallwend, Priority::Low);
    agent.acmd("effect_specialairlwend", effect_speciallwend, Priority::Low);
}