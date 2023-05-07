
use super::*;


#[acmd_script( agent = "lucario", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_s_game(fighter: &mut L2CAgentBase) {
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
        CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
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
        CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
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
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        // able to angle sideB dunk based on stick position on f10

        // get stick angle for dunk
        let angle = fighter.stick_y().atan2(fighter.stick_x() * PostureModule::lr(boma)).to_degrees();
        let mut kb_angle = if angle >= 360.0 {
            angle - 360.0
        } else if angle < 0.0 {
            angle + 360.0
        } else {
            angle
        } as u64;

        // only able to angle the dunk within a certain angle range
        let throw_angle_min = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "force_palm_air.throw_angle_min") as u64;
        let throw_angle_max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "force_palm_air.throw_angle_max") as u64;
        kb_angle = if fighter.stick_y() < 0.0 {
            kb_angle.clamp(throw_angle_min, throw_angle_max)
        }
        else {
            270
        };

        // calculate angle to rotate Lucario's model based on knockback angle
        let rot = 270 - kb_angle;
        VarModule::set_int(fighter.battle_object, vars::lucario::status::FORCE_PALM_ROT_ANGLE, rot as i32);
        
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
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::resume_energy_all(fighter.module_accessor);
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
    frame(lua_state, 1.0);
    FT_DESIRED_RATE(fighter, 30.0, 25.0);
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }

}

#[acmd_script( agent = "lucario", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_hi_r_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_DESIRED_RATE(fighter, 30.0, 25.0);
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }

}

#[acmd_script( agent = "lucario", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_air_hi_l_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_DESIRED_RATE(fighter, 30.0, 25.0);
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
    frame(lua_state, 1.0);
    FT_DESIRED_RATE(fighter, 30.0, 25.0);
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
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 0.5, 367, 100, 0, 35, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        GroundModule::set_passable_check(boma, true);
    }

}

#[acmd_script( agent = "lucario", script = "game_specialhiend" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_special_hi_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 70, 10, 0, 96, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
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
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 70, 10, 0, 96, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
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

#[acmd_script( agent = "lucario", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    FT_MOTION_RATE(fighter, 25.0 / (55.0 - 5.0));
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 11.0, 0.0, 9.0, 0.0, 0.0, 9.0, 0.0, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 55.0);
    FT_MOTION_RATE(fighter, 10.0 / (80.0 - 55.0));
}

pub fn install() {
    install_acmd_scripts!(
        lucario_special_s_game,
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
        lucario_special_hi_end_game,
        lucario_special_air_hi_end_game,
        game_speciallw
    );
}
