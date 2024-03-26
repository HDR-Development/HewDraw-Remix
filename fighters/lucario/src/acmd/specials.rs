
use super::*;

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 5.0);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        CATCH(agent, 0, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if ArticleModule::is_generatable(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG) {
            ArticleModule::generate_article(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, 0);
        }
    }

}

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 5.0);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        CATCH(agent, 0, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if ArticleModule::is_generatable(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG) {
            ArticleModule::generate_article(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, 0);
        }
    }
}

unsafe extern "C" fn game_specialairsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 22.0);
    if is_excute(agent) {
        // able to angle sideB dunk based on stick position on f22

        let throw_angle_min = ParamModule::get_int(agent.battle_object, ParamType::Agent, "force_palm_air.throw_angle_min") as u64;
        let throw_angle_max = ParamModule::get_int(agent.battle_object, ParamType::Agent, "force_palm_air.throw_angle_max") as u64;
        let kb_angle = if VarModule::get_float(agent.object(), vars::lucario::status::AURA_OVERRIDE) > 0.0 
        && agent.stick_y() < 0.25 {
            let angle = (270.0 + PostureModule::lr(boma) * agent.stick_x().atan2(-agent.stick_y()).to_degrees()) as u64;
            angle.clamp(throw_angle_min, throw_angle_max)
        } else {
            270 as u64
        };

        // calculate angle to rotate Lucario's model based on knockback angle
        let rot = 270 - kb_angle;
        VarModule::set_int(agent.battle_object, vars::lucario::status::FORCE_PALM_ROT_ANGLE, rot as i32);
        
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.0, kb_angle, 66, 0, 10, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
        CHECK_FINISH_CAMERA(agent, 15, 0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_CRITICAL_HIT);
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 10.0, 52, 90, 0, 60, 5.0, 0.0, 8.5, 14.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        ATTACK_IGNORE_THROW(agent, 1, 0, Hash40::new("top"), 10.0, 52, 90, 0, 60, 4.0, 0.0, 8.5, 19.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, false);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::resume_energy_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.0, 52, 86, 0, 66, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 10.0, 52, 90, 0, 60, 5.0, 0.0, 8.5, 14.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        ATTACK_IGNORE_THROW(agent, 1, 0, Hash40::new("top"), 10.0, 52, 90, 0, 60, 4.0, 0.0, 8.5, 19.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
        CHECK_FINISH_CAMERA(agent, 15, 0);
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_CRITICAL_HIT);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        MeterModule::watch_damage(agent.battle_object, false);
    }
}

unsafe extern "C" fn effect_specialairsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_l"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 180, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_r"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_specialairsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 23.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_005"));
        let curr_aurapower = WorkModule::get_float(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
        let middle_aurapower = WorkModule::get_float(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER);
        let high_aurapower = WorkModule::get_float(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER);
        if curr_aurapower < middle_aurapower {
            PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            PLAY_SE(agent, Hash40::new("se_lucario_special_s02_s"));
        }
        else if curr_aurapower < high_aurapower {
            PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            PLAY_SE(agent, Hash40::new("se_lucario_special_s02_m"));
        }
        else {
            PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            PLAY_SE(agent, Hash40::new("se_lucario_special_s02_l"));
        }
    }
}

unsafe extern "C" fn expression_specialairsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 0, false, 0x50000000 /* default value */);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_impact"), 0);
    }
    frame(lua_state, 66.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_DESIRED_RATE(agent, 30.0, 34.0);
    frame(lua_state, 21.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }

}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_DESIRED_RATE(agent, 30.0, 34.0);
    frame(lua_state, 21.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }

}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_DESIRED_RATE(agent, 30.0, 34.0);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }

}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_DESIRED_RATE(agent, 30.0, 34.0);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }

}

unsafe extern "C" fn game_specialhimove(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, false);
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 0.5, 367, 100, 0, 35, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        GroundModule::set_passable_check(boma, true);
    }

}

unsafe extern "C" fn expression_specialhimove(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 5);
    }
}

unsafe extern "C" fn game_specialhiend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    wait(lua_state, 1.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 6.0, 70, 10, 0, 96, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, false);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }

}

unsafe extern "C" fn game_specialairhiend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    wait(lua_state, 1.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 6.0, 70, 10, 0, 96, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, false);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }

}

unsafe extern "C" fn game_specialnbomb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 37.0);
    if is_excute(agent) {
        ArticleModule::shoot(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        MeterModule::drain_direct(agent.battle_object, MeterModule::meter_per_level(agent.battle_object));
        let frames = 120.max(VarModule::get_int(agent.object(), vars::lucario::instance::METER_PAUSE_REGEN_FRAME));
        VarModule::set_int(agent.object(), vars::lucario::instance::METER_PAUSE_REGEN_FRAME, frames);
    }
}

unsafe extern "C" fn effect_specialnbomb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 35.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialnbomb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 37.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_013"));
    }
}

unsafe extern "C" fn expression_specialnbomb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beaml"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialairnbomb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 37.0);
    if is_excute(agent) {
        ArticleModule::shoot(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        MeterModule::drain_direct(agent.battle_object, MeterModule::meter_per_level(agent.battle_object));
        let frames = 120.max(VarModule::get_int(agent.object(), vars::lucario::instance::METER_PAUSE_REGEN_FRAME));
        VarModule::set_int(agent.object(), vars::lucario::instance::METER_PAUSE_REGEN_FRAME, frames);
    }
}

unsafe extern "C" fn effect_specialairnbomb(agent: &mut L2CAgentBase) {

}

unsafe extern "C" fn sound_specialairnbomb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 37.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_013"));
    }
}

unsafe extern "C" fn expression_specialairnbomb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beaml"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 10.0, 4.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("lucario_kagebunshin_flash"), Hash40::new("top"), 0, 15, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        COL_PRI(agent, 200);
        FLASH(agent, 0.67, 0, 0.78, 0.31);
        FLASH(agent, 1, 1, 1, 0.75);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("lucario_shinsoku1"), Hash40::new("lucario_shinsoku2"), 13, Hash40::new("handl"), 0.0, 0.0, 0.0, Hash40::new("handr"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("waist"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, -1.0);
        // EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -20, 0, 0, 0, 1.0, true);
        // LAST_EFFECT_SET_RATE(agent, 0.4);
    }
    wait(lua_state, 1.0);
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(agent) {
        FLASH(agent, 0.7, 0.7, 0.7, 0.5);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH(agent, 0.67, 0, 0.78, 0.31);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH(agent, 0.7, 0.7, 0.7, 0.5);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_lucario_special_l02"));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_lucario_special_l03_s"));
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 10.0, 4.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("lucario_kagebunshin_flash"), Hash40::new("top"), 0, 15, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        COL_PRI(agent, 200);
        FLASH(agent, 0.67, 0, 0.78, 0.31);
        FLASH(agent, 1, 1, 1, 0.75);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("lucario_shinsoku1"), Hash40::new("lucario_shinsoku2"), 13, Hash40::new("handl"), 0.0, 0.0, 0.0, Hash40::new("handr"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("waist"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, -1.0);
        // EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 19, -12, 45, 0, 0, 1.0, true);
        // LAST_EFFECT_SET_RATE(agent, 0.4);
    }
    wait(lua_state, 1.0);
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(agent) {
        FLASH(agent, 0.7, 0.7, 0.7, 0.5);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH(agent, 0.67, 0, 0.78, 0.31);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH(agent, 0.7, 0.7, 0.7, 0.5);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        AFTER_IMAGE_OFF(agent, 4);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialairs", game_specialairs);
    agent.acmd("game_specialairsthrow", game_specialairsthrow);
    agent.acmd("game_specialsthrow", game_specialsthrow);
    agent.acmd("effect_specialairsthrow", effect_specialairsthrow);
    agent.acmd("sound_specialairsthrow", sound_specialairsthrow);
    agent.acmd("expression_specialairsthrow", expression_specialairsthrow);
    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_specialairhi", game_specialairhi);
    agent.acmd("game_specialairhi", game_specialairhi);
    agent.acmd("game_specialhimove", game_specialhimove);
    agent.acmd("expression_specialhimove", expression_specialhimove);
    agent.acmd("game_specialhiend", game_specialhiend);
    agent.acmd("game_specialairhiend", game_specialairhiend);
    agent.acmd("game_specialnbomb", game_specialnbomb);
    agent.acmd("effect_specialnbomb", effect_specialnbomb);
    agent.acmd("sound_specialnbomb", sound_specialnbomb);
    agent.acmd("expression_specialnbomb", expression_specialnbomb);
    agent.acmd("game_specialairnbomb", game_specialairnbomb);
    agent.acmd("effect_specialairnbomb", effect_specialairnbomb);
    agent.acmd("sound_specialairnbomb", sound_specialairnbomb);
    agent.acmd("expression_specialairnbomb", expression_specialairnbomb);
    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("effect_speciallw", effect_speciallw);
    agent.acmd("sound_speciallw", sound_speciallw);
    agent.acmd("sound_specialairlw", sound_speciallw);
    agent.acmd("expression_speciallw", expression_speciallw);
    agent.acmd("expression_specialairlw", expression_speciallw);
    agent.acmd("game_specialairlw", game_specialairlw);
    agent.acmd("effect_specialairlw", effect_specialairlw);
}
