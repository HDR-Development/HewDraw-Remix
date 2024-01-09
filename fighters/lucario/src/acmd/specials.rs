
use super::*;


#[acmd_script( agent = "lucario", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 5.0);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if ArticleModule::is_generatable(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG) {
            ArticleModule::generate_article(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, 0);
        }
    }

}

#[acmd_script( agent = "lucario", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 5.0);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if ArticleModule::is_generatable(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG) {
            ArticleModule::generate_article(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, 0);
        }
    }
}

#[acmd_script(agent = "lucario", script = "game_specialairsthrow", category = ACMD_GAME)]
unsafe fn lucario_special_air_s_throw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        // able to angle sideB dunk based on stick position on f22

        let throw_angle_min = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "force_palm_air.throw_angle_min") as u64;
        let throw_angle_max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "force_palm_air.throw_angle_max") as u64;
        let kb_angle = if VarModule::get_float(fighter.object(), vars::lucario::status::AURA_OVERRIDE) > 0.0 
        && fighter.stick_y() < 0.25 {
            let angle = (270.0 + PostureModule::lr(boma) * fighter.stick_x().atan2(-fighter.stick_y()).to_degrees()) as u64;
            angle.clamp(throw_angle_min, throw_angle_max)
        } else {
            270 as u64
        };

        // calculate angle to rotate Lucario's model based on knockback angle
        let rot = 270 - kb_angle;
        VarModule::set_int(fighter.battle_object, vars::lucario::status::FORCE_PALM_ROT_ANGLE, rot as i32);
        
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.0, kb_angle, 66, 0, 10, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
        CHECK_FINISH_CAMERA(fighter, 15, 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_CRITICAL_HIT);
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 10.0, 52, 90, 0, 60, 5.0, 0.0, 8.5, 14.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        ATTACK_IGNORE_THROW(fighter, 1, 0, Hash40::new("top"), 10.0, 52, 90, 0, 60, 4.0, 0.0, 8.5, 19.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, false);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::resume_energy_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialsthrow", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.0, 52, 86, 0, 66, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 10.0, 52, 90, 0, 60, 5.0, 0.0, 8.5, 14.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        ATTACK_IGNORE_THROW(fighter, 1, 0, Hash40::new("top"), 10.0, 52, 90, 0, 60, 4.0, 0.0, 8.5, 19.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
        CHECK_FINISH_CAMERA(fighter, 15, 0);
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_CRITICAL_HIT);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
}

#[acmd_script(agent = "lucario", script = "effect_specialairsthrow", category = ACMD_EFFECT)]
unsafe fn lucario_special_air_s_throw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucario_bflash_l"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 180, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("lucario_bflash_r"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
}

#[acmd_script(agent = "lucario", script = "sound_specialairsthrow", category = ACMD_SOUND)]
unsafe fn lucario_special_air_s_throw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucario_005"));
        let curr_aurapower = WorkModule::get_float(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
        let middle_aurapower = WorkModule::get_float(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER);
        let high_aurapower = WorkModule::get_float(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER);
        if curr_aurapower < middle_aurapower {
            PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_s"));
        }
        else if curr_aurapower < high_aurapower {
            PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_m"));
        }
        else {
            PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_l"));
        }
    }
}

#[acmd_script(agent = "lucario", script = "expression_specialairsthrow", category = ACMD_EXPRESSION)]
unsafe fn lucario_special_air_s_throw_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 0, false, 0x50000000 /* default value */);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_impact"), 0);
    }
    frame(lua_state, 66.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_hi_l_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_DESIRED_RATE(fighter, 30.0, 34.0);
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }

}

#[acmd_script( agent = "lucario", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_hi_r_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_DESIRED_RATE(fighter, 30.0, 34.0);
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }

}

#[acmd_script( agent = "lucario", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_air_hi_l_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_DESIRED_RATE(fighter, 30.0, 34.0);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }

}

#[acmd_script( agent = "lucario", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_air_hi_r_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_DESIRED_RATE(fighter, 30.0, 34.0);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }

}

#[acmd_script( agent = "lucario", script = "game_specialhimove" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_hi_move_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, false);
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 0.5, 367, 100, 0, 35, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        GroundModule::set_passable_check(boma, true);
    }

}

#[acmd_script( agent = "lucario", script = "expression_specialhimove", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucario_special_hi_move_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 5);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialhiend" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_hi_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 70, 10, 0, 96, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, false);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }

}

#[acmd_script( agent = "lucario", script = "game_specialairhiend" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_air_hi_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 70, 10, 0, 96, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, false);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }

}

#[acmd_script( agent = "lucario", script = "game_specialnbomb", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnbomb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        ArticleModule::shoot(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        MeterModule::drain_direct(fighter.battle_object, 2.0 * MeterModule::meter_per_level(fighter.battle_object));
        let frames = 120.max(VarModule::get_int(fighter.object(), vars::lucario::instance::METER_PAUSE_REGEN_FRAME));
        VarModule::set_int(fighter.object(), vars::lucario::instance::METER_PAUSE_REGEN_FRAME, frames);
    }
}

#[acmd_script( agent = "lucario", script = "effect_specialnbomb", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialnbomb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucario", script = "sound_specialnbomb", category = ACMD_SOUND, low_priority )]
unsafe fn sound_specialnbomb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucario_013"));
    }
}

#[acmd_script( agent = "lucario", script = "expression_specialnbomb", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_specialnbomb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beaml"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairnbomb", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairnbomb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        ArticleModule::shoot(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        MeterModule::drain_direct(fighter.battle_object, 2.0 * MeterModule::meter_per_level(fighter.battle_object));
        let frames = 120.max(VarModule::get_int(fighter.object(), vars::lucario::instance::METER_PAUSE_REGEN_FRAME));
        VarModule::set_int(fighter.object(), vars::lucario::instance::METER_PAUSE_REGEN_FRAME, frames);
    }
}

#[acmd_script( agent = "lucario", script = "effect_specialairnbomb", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialairnbomb(fighter: &mut L2CAgentBase) {

}

#[acmd_script( agent = "lucario", script = "sound_specialairnbomb", category = ACMD_SOUND, low_priority )]
unsafe fn sound_specialairnbomb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucario_013"));
    }
}

#[acmd_script( agent = "lucario", script = "expression_specialairnbomb", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_specialairnbomb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beaml"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "lucario", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 10.0, 4.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "lucario", script = "effect_speciallw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("lucario_kagebunshin_flash"), Hash40::new("top"), 0, 15, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        COL_PRI(fighter, 200);
        FLASH(fighter, 0.67, 0, 0.78, 0.31);
        FLASH(fighter, 1, 1, 1, 0.75);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("lucario_shinsoku1"), Hash40::new("lucario_shinsoku2"), 13, Hash40::new("handl"), 0.0, 0.0, 0.0, Hash40::new("handr"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("waist"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, -1.0);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -20, 0, 0, 0, 1.0, true);
        // LAST_EFFECT_SET_RATE(fighter, 0.4);
    }
    wait(lua_state, 1.0);
    for _ in 0..4 {
        if is_excute(fighter) {
            FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            FLASH(fighter, 0.67, 0, 0.78, 0.31);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.67, 0, 0.78, 0.31);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        AFTER_IMAGE_OFF(fighter, 4);
    }
}

#[acmd_script( agent = "lucario", scripts = ["sound_speciallw", "sound_specialairlw"], category = ACMD_SOUND, low_priority )]
unsafe fn sound_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucario_special_l02"));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucario_special_l03_s"));
    }
}

#[acmd_script( agent = "lucario", scripts = ["expression_speciallw", "expression_specialairlw"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 57.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 10.0, 4.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "lucario", script = "effect_specialairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("lucario_kagebunshin_flash"), Hash40::new("top"), 0, 15, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        COL_PRI(fighter, 200);
        FLASH(fighter, 0.67, 0, 0.78, 0.31);
        FLASH(fighter, 1, 1, 1, 0.75);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("lucario_shinsoku1"), Hash40::new("lucario_shinsoku2"), 13, Hash40::new("handl"), 0.0, 0.0, 0.0, Hash40::new("handr"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("waist"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, -1.0);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 19, -12, 45, 0, 0, 1.0, true);
        // LAST_EFFECT_SET_RATE(fighter, 0.4);
    }
    wait(lua_state, 1.0);
    for _ in 0..4 {
        if is_excute(fighter) {
            FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            FLASH(fighter, 0.67, 0, 0.78, 0.31);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.67, 0, 0.78, 0.31);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        AFTER_IMAGE_OFF(fighter, 4);
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucario_special_s_game,
        game_specialsthrow,
        lucario_special_air_s_game,
        lucario_special_air_s_throw_game,
        lucario_special_air_s_throw_effect,
        lucario_special_air_s_throw_sound,
        lucario_special_air_s_throw_expression,
        lucario_special_hi_l_game,
        lucario_special_hi_r_game,
        lucario_special_air_hi_l_game,
        lucario_special_air_hi_r_game,
        lucario_special_hi_move_game,
        lucario_special_hi_move_expression,
        lucario_special_hi_end_game,
        lucario_special_air_hi_end_game,
        game_specialnbomb,
        effect_specialnbomb,
        sound_specialnbomb,
        expression_specialnbomb,
        game_specialairnbomb,
        effect_specialairnbomb,
        sound_specialairnbomb,
        expression_specialairnbomb,
        game_speciallw,
        effect_speciallw,
        sound_speciallw,
        expression_speciallw,
        game_specialairlw,
        effect_specialairlw,
    );
}
