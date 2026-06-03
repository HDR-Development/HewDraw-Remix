use super::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_HI_FLAG_AIR){
        if is_excute(agent) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 2.0);
        }
        frame(lua_state, 1.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 76, 55, 0, 80, 6.0, 0.0, 6.0, 6.0, Some(0.0), Some(11.25), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 76, 55, 0, 80, 6.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 76, 55, 0, 80, 6.0, 0.0, 6.0, -2.5, Some(0.0), Some(10.25), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear(boma, 0, false);
            AttackModule::clear(boma,  2, false);
        }
        frame(lua_state, 4.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 71, 68, 0, 75, 4.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 7.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 63, 90, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 10.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 60, 90, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 15.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            //FT_MOTION_RATE(agent, 15.0/(56.0-15.0));
        }
        frame(lua_state, 21.0);
        if is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        }
        frame(lua_state, 46.0);
        if is_excute(agent) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
        }
        frame(lua_state, 52.0);
        if is_excute(agent) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 4.0);
        }
        frame(lua_state, 53.0);
        if is_excute(agent) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
        }
        frame(lua_state, 54.0);
        if is_excute(agent) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
        }
        frame(lua_state, 56.0);
        if is_excute(agent) {
            //FT_MOTION_RATE(agent, 1.0);
        }
    }
    else{
        if is_excute(agent) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 2.0);
        }
        frame(lua_state, 1.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 76, 55, 0, 80, 6.0, 0.0, 6.0, 6.0, Some(0.0), Some(11.25), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 76, 55, 0, 80, 6.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 76, 55, 0, 80, 6.0, 0.0, 6.0, -2.5, Some(0.0), Some(10.25), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear(boma, 0, false);
            AttackModule::clear(boma,  2, false);
        }
        frame(lua_state, 4.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 71, 68, 0, 75, 4.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 7.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 63, 90, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 10.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 60, 90, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 15.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            //FT_MOTION_RATE(agent, 15.0/(56.0-15.0));
        }
        frame(lua_state, 21.0);
        if is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        }
        frame(lua_state, 46.0);
        if is_excute(agent) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
        }
        frame(lua_state, 52.0);
        if is_excute(agent) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 4.0);
        }
        frame(lua_state, 53.0);
        if is_excute(agent) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
        }
        frame(lua_state, 54.0);
        if is_excute(agent) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
        }
        frame(lua_state, 56.0);
        if is_excute(agent) {
            //FT_MOTION_RATE(agent, 1.0);
        }
    }
}

unsafe extern "C" fn game_specialhiair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 2.0);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 76, 53, 0, 80, 6.0, 0.0, 6.0, 6.0, Some(0.0), Some(11.25), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 76, 53, 0, 80, 6.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 76, 53, 0, 80, 6.0, 0.0, 6.0, -2.5, Some(0.0), Some(10.25), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma,  2, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 71, 65, 0, 75, 4.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 63, 85, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 60, 85, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        //FT_MOTION_RATE(agent, 15.0/(56.0-15.0));
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 4.0);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        //FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("demon_devil_sign_flash"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
        EFFECT_FOLLOW(agent, Hash40::new("demon_ragedrive_start"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.75, false);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.5, 1.0);
        //FLASH(agent, 0.25, 0.08, 0.6, 0);
        //FLASH_SET_DIRECTION(agent, -1, 0, 0);
        //BURN_COLOR(agent, 8, 3, 36, 0);
        //FLASH_FRM(agent, 2, 0.25, 0.08, 0.6, 0.4);
        //BURN_COLOR_FRAME(agent, 2, 8, 3, 36, 0.4);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("demon_devil_start"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("demon_devil_start_aura"), Hash40::new("bust"), 0, 0, 1, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("demon_devil_start_elec"), Hash40::new("bust"), 0, 0, 1, 0, 0, 0, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        //FLASH(agent, 0.25, 0.08, 0.6, 0.4);
        //BURN_COLOR(agent, 8, 3, 36, 0.4);
        //FLASH_FRM(agent, 3, 0.25, 0.08, 0.6, 0);
        //BURN_COLOR_FRAME(agent, 3, 8, 3, 36, 0);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 0.6);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 7.75, 12.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        GrabModule::set_constraint(boma, 0, true);
        GrabModule::set_constraint(boma, 1, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 19.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        GrabModule::set_constraint(boma, 0, true);
        GrabModule::set_constraint(boma, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(agent, 0.9);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.0, 4.5, Some(0.0), Some(23.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 64, 50, 0, 75, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(5.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(boma, 2, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.5, 4.5, Some(0.0), Some(23.5), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 20.0, 4.5, Some(0.0), Some(24.0), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    FighterSpecializer_Demon::set_devil(boma, false, 0.0);
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 0.6);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 7.75, 12.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        GrabModule::set_constraint(boma, 0, true);
        GrabModule::set_constraint(boma, 1, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 15.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 20.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        GrabModule::set_constraint(boma, 0, true);
        GrabModule::set_constraint(boma, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(agent, 0.9);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 21.0, 4.5, Some(0.0), Some(25.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 14.0, 6.0, Some(0.0), Some(18.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 64, 50, 0, 75, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(5.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(boma, 2, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 21.5, 4.5, Some(0.0), Some(25.5), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 14.0, 6.0, Some(0.0), Some(18.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 22.0, 4.5, Some(0.0), Some(26.0), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 0.75);
}

unsafe extern "C" fn game_attackragedrive(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) {
        // command input
        sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8);
        FighterSpecializer_Demon::set_devil(boma, true, 10.0);
        FT_MOTION_RATE(agent, 0.7);
        if is_excute(agent) {
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15.0);
        }
        frame(lua_state, 10.0);
        FT_MOTION_RATE(agent, 1.0);
        frame(lua_state, 11.0);
        if is_excute(agent) {
            GrabModule::set_rebound(boma, true);
        }
        frame(lua_state, 12.0);
        if is_excute(agent) {
            CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(boma, 0, true);
            GrabModule::set_constraint(boma, 1, true);
            GrabModule::set_constraint(boma, 2, true);
            GrabModule::set_constraint(boma, 3, true);
        }
        frame(lua_state, 13.0);
        if is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(boma, 0, true);
            GrabModule::set_constraint(boma, 1, true);
            GrabModule::set_constraint(boma, 2, true);
            GrabModule::set_constraint(boma, 3, true);
        }
        frame(lua_state, 14.0);
        if is_excute(agent) {
            CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(boma, 0, true);
            GrabModule::set_constraint(boma, 1, true);
            GrabModule::set_constraint(boma, 2, true);
            GrabModule::set_constraint(boma, 3, true);
        }
        frame(lua_state, 15.0);
        if is_excute(agent) {
            CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 22.0, 7.0, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 24.0, 6.5, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(boma, 0, true);
            GrabModule::set_constraint(boma, 1, true);
            GrabModule::set_constraint(boma, 2, true);
            GrabModule::set_constraint(boma, 3, true);
        }
        frame(lua_state, 16.0);
        if is_excute(agent) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            GrabModule::set_rebound(boma, false);
        }
        frame(lua_state, 38.0);
        FT_MOTION_RATE(agent, 1.3);
        frame(lua_state, 52.0);
        FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    }
    else {
        // no input
        FighterSpecializer_Demon::set_devil(boma, true, 10.0);
        if is_excute(agent) {
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 4.0);
        if is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10.0);
        }
        frame(lua_state, 12.0);
        if is_excute(agent) {
            GrabModule::set_rebound(boma, true);
        }
        frame(lua_state, 13.0);
        if is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(boma, 0, true);
            GrabModule::set_constraint(boma, 1, true);
            GrabModule::set_constraint(boma, 2, true);
            GrabModule::set_constraint(boma, 3, true);
        }
        frame(lua_state, 14.0);
        if is_excute(agent) {
            CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(boma, 0, true);
            GrabModule::set_constraint(boma, 1, true);
            GrabModule::set_constraint(boma, 2, true);
            GrabModule::set_constraint(boma, 3, true);
        }
        frame(lua_state, 15.0);
        if is_excute(agent) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            GrabModule::set_rebound(boma, false);
        }
        frame(lua_state, 38.0);
        FT_MOTION_RATE(agent, 1.3);
        frame(lua_state, 52.0);
        FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    }
}

unsafe extern "C" fn game_attackragedriveground(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) {
        // command input
        FighterSpecializer_Demon::set_devil(boma, true, 2.0);
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        }
        frame(lua_state, 1.0);
        if is_excute(agent) {
            WorkModule::set_int(boma, *HIT_STATUS_NORMAL, *FIGHTER_DEMON_STATUS_ATTACK_RAGE_DRIVE_INT_TARGET_HIT_STATUS);
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 100, 100, 100, 50, 6.5, 0.0, 1.0, 5.0, Some(10.0), Some(1.0), Some(5.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_dead_all(boma, true, false);
            AttackModule::set_catch_only_all(boma, true, false);
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 16.0, 60, 95, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_NONE, true);
            AttackModule::set_no_finish_camera(boma, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            ATTACK_IGNORE_THROW(agent, 1, 0, Hash40::new("top"), 18.0, 361, 80, 0, 80, 7.5, 0.0, 7.5, -10.0, Some(0.0), Some(7.5), Some(14.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(boma, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
            FighterSpecializer_Demon::clear_past_log_throw(boma);
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_CHECK_DAMAGE);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_HIT);
            CAM_ZOOM_OUT(agent);
        }
        frame(lua_state, 4.0);
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        }
        frame(lua_state, 35.0);
        FighterSpecializer_Demon::set_devil(boma, true, 3.0);
        frame(lua_state, 38.0);
        FighterSpecializer_Demon::set_devil(boma, true, 4.0);
        frame(lua_state, 40.0);
        FighterSpecializer_Demon::set_devil(boma, true, 5.0);
        frame(lua_state, 41.0);
        FighterSpecializer_Demon::set_devil(boma, true, 6.0);
        frame(lua_state, 42.0);
        FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    }
    else {
        // no input
        FighterSpecializer_Demon::set_devil(boma, true, 2.0);
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        }
        frame(lua_state, 1.0);
        if is_excute(agent) {
            WorkModule::set_int(boma, *HIT_STATUS_NORMAL, *FIGHTER_DEMON_STATUS_ATTACK_RAGE_DRIVE_INT_TARGET_HIT_STATUS);
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.5, 100, 100, 100, 50, 6.5, 0.0, 1.0, 5.0, Some(10.0), Some(1.0), Some(5.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_dead_all(boma, true, false);
            AttackModule::set_catch_only_all(boma, true, false);
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.0, 60, 95, 0, 67, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_NONE, true);
            AttackModule::set_no_finish_camera(boma, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            ATTACK_IGNORE_THROW(agent, 1, 0, Hash40::new("top"), 15.0, 361, 80, 0, 80, 5.0, 0.0, 5.0, -8.5, Some(0.0), Some(5.0), Some(12.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(boma, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
            FighterSpecializer_Demon::clear_past_log_throw(boma);
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_CHECK_DAMAGE);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_HIT);
            CAM_ZOOM_OUT(agent);
        }
        frame(lua_state, 4.0);
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        }
        frame(lua_state, 35.0);
        FighterSpecializer_Demon::set_devil(boma, true, 3.0);
        frame(lua_state, 38.0);
        FighterSpecializer_Demon::set_devil(boma, true, 4.0);
        frame(lua_state, 40.0);
        FighterSpecializer_Demon::set_devil(boma, true, 5.0);
        frame(lua_state, 41.0);
        FighterSpecializer_Demon::set_devil(boma, true, 6.0);
        frame(lua_state, 42.0);
        FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialhiair", game_specialhiair, Priority::Low);
    agent.acmd("effect_specialhistart", effect_specialhistart, Priority::Low);
    agent.acmd("effect_specialairhistart", effect_specialhistart, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);

    agent.acmd("game_attackragedrive", game_attackragedrive, Priority::Low);
    agent.acmd("game_attackragedriveground", game_attackragedriveground, Priority::Low);
}
