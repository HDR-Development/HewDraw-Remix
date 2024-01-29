use super::*;

#[acmd_script( agent = "kamui", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn kamui_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, smash::phx::Hash40::new("attack_air_n"), false, 0.0);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE_RANGE(fighter, 5.0, 5.5, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 60, 93, 0, 47, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.0, 60, 93, 0, 47, 3.25, -1.0, 4.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 10.0, 50, 88, 0, 47, 2.25, -1.0, 11.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 7.0, 60, 93, 0, 47, 3.25, -1.0, 7.5, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("haver"), 7.0, 60, 93, 0, 47, 3.0, -1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 6, 0, Hash40::new("handl"), 7.0, 60, 93, 0, 47, 3.0, 7.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 7, 0, Hash40::new("handl"), 7.0, 60, 93, 0, 47, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 5.5);
    FT_MOTION_RATE_RANGE(fighter, 5.5, 6.0, 1.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(fighter, 6.0, 15.0, 8.0);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.0, 60, 88, 0, 51, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.0, 60, 88, 0, 51, 3.25, -1.0, 4.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 8.0, 50, 88, 0, 51, 2.25, -1.0, 11.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 5.0, 60, 88, 0, 51, 3.25, -1.0, 7.5, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("haver"), 5.0, 60, 88, 0, 51, 3.0, -1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 6, 0, Hash40::new("handl"), 5.0, 60, 88, 0, 51, 3.0, 7.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 7, 0, Hash40::new("handl"), 5.0, 60, 88, 0, 51, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "kamui", script = "expression_attackairn", category = ACMD_EXPRESSION, low_priority )]
unsafe fn kamui_attack_air_n_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        VisibilityModule::set_int64(boma, hash40("dragon") as i64, hash40("dragon_horn") as i64);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("hair") as i64, hash40("hair_hide") as i64);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("hair") as i64, hash40("hair_normal") as i64);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("dragon") as i64, hash40("dragon_none") as i64);
    }
}


#[acmd_script( agent = "kamui", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn kamui_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    FT_MOTION_RATE_RANGE(fighter, 4.0, 8.85, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.85);
    FT_MOTION_RATE_RANGE(fighter, 8.85, 14.0, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 67, 95, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 8.0, 67, 95, 0, 40, 3.0, -1.0, 4.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 11.0, 50, 75, 0, 57, 2.25, -1.0, 11.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 8.0, 67, 95, 0, 40, 3.0, -1.0, 7.5, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("haver"), 8.0, 67, 95, 0, 40, 3.0, -1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 59.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }

}

#[acmd_script( agent = "kamui", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn kamui_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_sword1"), Hash40::new("tex_kamui_sword2"), 5, Hash40::new("haver"), 0.0, 0.1, 0.0, Hash40::new("haver"), -0.0, 15.0, 0.0, true, Hash40::new("kamui_sword_flare"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

#[acmd_script( agent = "kamui", script = "expression_attackairf", category = ACMD_EXPRESSION, low_priority )]
unsafe fn kamui_attack_air_f_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.85);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.85);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
}

#[acmd_script( agent = "kamui", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn kamui_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 2.0);
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
	if is_excute(fighter) {
		let charge = VarModule::get_float(fighter.battle_object, vars::kamui::status::CURRENT_CHARGE);
        if (1.0..4.0).contains(&charge) {
            SET_SPEED_EX(fighter, 0.3 + (0.2 * charge), 0.1 + (0.03 * charge), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if charge >= 5.0 {
            SET_SPEED_EX(fighter, 1.3, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
	}
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        let charge = VarModule::get_float(fighter.battle_object, vars::kamui::status::CURRENT_CHARGE);
        let damage = 13.0 + (charge * 0.5);
        let kbg = 98 - (charge * 1.6) as i32;
        let sound_level = if charge > 0.0 { *ATTACK_SOUND_LEVEL_L } else { *ATTACK_SOUND_LEVEL_M };
        let sound = if charge > 0.0 { *COLLISION_SOUND_ATTR_WATER } else { *COLLISION_SOUND_ATTR_CUTUP };
        ATTACK(fighter, 0, 0, Hash40::new("top"), damage, 361, kbg, 0, 40, 7.0, 0.0, 10.0, -12.5, Some(0.0), Some(11.0), Some(-17.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), sound_level, sound, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 106, 0, 40, 5.7, 0.0, 10.7, -12.5, Some(0.0), Some(11.9), Some(-19.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "kamui", script = "effect_attackairb" , category = ACMD_EFFECT , low_priority)]
unsafe fn kamui_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 6.1);
    if is_excute(fighter) {
        if VarModule::get_float(fighter.battle_object, vars::kamui::status::CURRENT_CHARGE) > 0.0 {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 1.6, 17.0, -8.0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(fighter, 0.75);
            LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 1.0);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.3, -21, 0, 0, 0, 1.1, true);
        if VarModule::get_float(fighter.battle_object, vars::kamui::status::CURRENT_CHARGE) > 0.0 {
            EFFECT(fighter, Hash40::new("kamui_counter_splash"), Hash40::new("top"), 0.0, 11.0, -10.0, 270, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
    }

}

#[acmd_script( agent = "kamui", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn kamui_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE_RANGE(fighter, 7.0, 10.5, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.0, 70, 80, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 9.0, 70, 80, 0, 60, 3.25, -1.0, 4.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.0, 68, 70, 0, 85, 2.25, -1.0, 11.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 9.0, 70, 80, 0, 60, 3.25, -1.0, 7.5, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("haver"), 9.0, 70, 80, 0, 60, 3.0, -1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 10.5);
    FT_MOTION_RATE_RANGE(fighter, 10.5, 11.0, 1.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "kamui", script = "effect_attackairhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn kamui_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state =fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
	    AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kamui_sword1"), Hash40::new("tex_kamui_sword2"), 7, Hash40::new("haver"), 0.0, 0.1, 0.0, Hash40::new("haver"), -0.0, 15.0, 0.0, true, Hash40::new("kamui_sword_flare"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter){
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

#[acmd_script( agent = "kamui", script = "expression_attackairhi", category = ACMD_EXPRESSION, low_priority )]
unsafe fn kamui_attack_air_hi_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
}

#[acmd_script( agent = "kamui", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn kamui_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.769);
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 270, 49, 0, 33, 5.5, 0.0, 2.0, 0.0, None, None, None, 0.8, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 270, 49, 0, 33, 4.0, 0.0, -7.5, 0.0, None, None, None, 0.8, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        /* Air-only */
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 367, 100, 40, 0, 5.5, 0.0, 2.0, 0.0, None, None, None, 0.8, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 367, 100, 40, 0, 4.0, 0.0, -7.5, 0.0, None, None, None, 0.8, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 1.0, false);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 78, 45, 0, 90, 6.0, 0.0, 2.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 78, 45, 0, 90, 4.5, 0.0, -7.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "kamui", script = "effect_attackairlw" , category = ACMD_EFFECT , low_priority)]
unsafe fn kamui_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sheik_atk_lw"), Hash40::new("top"), 0, 6, 0, -90, 0, 0, 0.8, true, 0.9);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sheik_atk_lw"), Hash40::new("top"), 2, 6, 0, -90, 0, 0, 0.8, true, 0.9);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 13, -2.5, 78, 0, 0, 1.1, true, 0.9);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_club_tornado"), Hash40::new("top"), 0.0, -5.0, 0.0, 120, 70, 100, 0.65, true);
        LAST_EFFECT_SET_RATE(fighter, 6.0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_club_tornado"), false, true);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("top"), 0, -5.0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("kamui_ryusensya_shot"), Hash40::new("top"), 0, -5.0, 0, 0, 0, 0, 1.0, true);
        EFFECT(fighter, Hash40::new("kamui_counter_splash"), Hash40::new("top"), 0.0, -7.0, 0.0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.25);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0.0, -12.0, 0.0, 0, 0, 0, 1.25, true, 1.0);
    }

}

#[acmd_script( agent = "kamui", script = "game_landingairlw" , category = ACMD_GAME , low_priority)]
unsafe fn kamui_landing_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

#[acmd_script( agent = "kamui", script = "effect_landingairlw" , category = ACMD_EFFECT , low_priority)]
unsafe fn kamui_landing_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_club_tornado"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("kamui_ryusensya_shot"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("kamui_counter_splash"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("kamui_transform_splash_end"), false, true);
        EFFECT_FLW_POS(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        kamui_attack_air_n_game,
        kamui_attack_air_n_expression,
        kamui_attack_air_f_game,
        kamui_attack_air_f_effect,
        kamui_attack_air_f_expression,
        kamui_attack_air_b_game,
        kamui_attack_air_b_effect,
        kamui_attack_air_hi_game,
        kamui_attack_air_hi_effect,
        kamui_attack_air_hi_expression,
        kamui_attack_air_lw_game,
        kamui_attack_air_lw_effect,
        kamui_landing_air_lw_game,
        kamui_landing_air_lw_effect,
    );
}