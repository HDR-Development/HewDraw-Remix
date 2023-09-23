use super::*;

#[acmd_script( agent = "kamui", scripts = ["game_specialnend1", "game_specialairnend1"] , category = ACMD_GAME , low_priority)]
unsafe fn kamui_special_n_end1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 8.0, 5.0);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 17.0);
    FT_MOTION_RATE_RANGE(fighter, 17.0, 45.0, 19.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_N_FLAG_AIR_CONTROL);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "kamui_dragonhand", scripts = ["game_dhspecialnend1", "game_dhspecialairnend1"], category = ACMD_GAME, low_priority )]
unsafe fn kamui_dragonhand_special_n_end1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 8.0, 5.0);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        if WorkModule::is_flag(boma, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLAG_IS_KAMUI) {
            if sv_animcmd::get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
                let lerp = WorkModule::get_float(boma, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
                ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 100, 0, 50, 5.5, 0.0, 8.8, 15.0, Some(0.0), Some(8.8), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 50, 100, 0, 50, 6.5, 0.0, 8.8, 15.0, Some(0.0), Some(8.8), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
                ATK_LERP_RATIO(fighter, lerp);
            }
            else {
                let lerp = WorkModule::get_float(boma, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
                ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 100, 0, 50, 5.5, 0.0, 8.1, 15.0, Some(0.0), Some(8.1), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 50, 100, 0, 50, 6.5, 0.0, 8.1, 15.0, Some(0.0), Some(8.1), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
                ATK_LERP_RATIO(fighter, lerp);
            }
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "kamui", script = "game_specialsattack", category = ACMD_GAME, low_priority )]
unsafe fn kamui_special_s_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_s_attack"), false, -1.0);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_AIR_CONTROL);
    }
}

#[acmd_script( agent = "kamui", script = "game_specialairsattack", category = ACMD_GAME, low_priority )]
unsafe fn kamui_special_air_s_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_air_s_attack"), false, -1.0);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_AIR_CONTROL);
    }
}

#[acmd_script( agent = "kamui_spearhand", script = "game_specialsattack", category = ACMD_GAME, low_priority )]
unsafe fn kamui_spearhand_special_s_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("drag1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *WEAPON_KAMUI_SPEARHAND_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
        let tip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(boma, Hash40::new("drag8"), tip_pos, false);
        let base_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(boma, Hash40::new("drag2"), base_pos, false);
        let facing = PostureModule::lr(boma);
        let ground = &mut Vector2f{ x: 0.0, y: 0.0 };
        if GroundModule::line_segment_check(boma,
            &Vector2f{ x: base_pos.x + (10.0 * facing), y: base_pos.y - 8.0 },
            &Vector2f{ x: tip_pos.x + (35.0 * facing), y: tip_pos.y - 32.0 },
            &Vector2f::zero(),
            ground,
            true).is_null() {
            ATTACK(fighter, 1, 0, Hash40::new("drag8"), 17.0, 40, 90, 0, 32, 3.2, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        ATTACK(fighter, 0, 0, Hash40::new("drag2"), 7.0, 50, 100, 0, 25, 3.5, 4.0, 0.0, 0.0, Some(17.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_pierce"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *WEAPON_KAMUI_SPEARHAND_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
    }
}

#[acmd_script( agent = "kamui_spearhand", script = "game_specialairsattack", category = ACMD_GAME, low_priority )]
unsafe fn kamui_spearhand_special_air_s_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("drag1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *WEAPON_KAMUI_SPEARHAND_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
        let tip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(boma, Hash40::new("drag8"), tip_pos, false);
        let base_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(boma, Hash40::new("drag2"), base_pos, false);
        let facing = PostureModule::lr(boma);
        let ground = &mut Vector2f{ x: 0.0, y: 0.0 };
        // debug_draw_line(
        //     &Vector2f{ x: base_pos.x + (10.0 * facing), y: base_pos.y - 8.0 },
        //     &Vector2f{ x: tip_pos.x + (38.0 * facing), y: tip_pos.y - 35.0 },
        //     500
        // );
        // debug_draw_circle(&Vector2f{ x: base_pos.x + (10.0 * facing), y: base_pos.y - 8.0 }, 5.0, 100);
        // debug_draw_circle(&Vector2f{ x: tip_pos.x + (38.0 * facing), y: tip_pos.y - 35.0 }, 5.0, 100);
        // debug_set_draw_color(255.0, 0.0, 0.0, 1.0);
        if GroundModule::line_segment_check(boma,
            &Vector2f{ x: base_pos.x + (10.0 * facing), y: base_pos.y - 8.0 },
            &Vector2f{ x: tip_pos.x + (35.0 * facing), y: tip_pos.y - 32.0 },
            &Vector2f::zero(),
            ground,
            true).is_null() {
            ATTACK(fighter, 1, 0, Hash40::new("drag8"), 17.0, 40, 90, 0, 32, 3.2, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        ATTACK(fighter, 0, 0, Hash40::new("drag2"), 8.0, 50, 100, 0, 25, 3.5, 4.0, 0.0, 0.0, Some(17.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_pierce"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *WEAPON_KAMUI_SPEARHAND_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
    }
}

#[acmd_script( agent = "kamui", script = "game_specialswallattackf" , category = ACMD_GAME , low_priority)]
unsafe fn kamui_special_s_wall_attack_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, smash::phx::Hash40::new("special_s_wall_attack_f"), false, 0.0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 13.0, 55, 50, 0, 100, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 13.0, 55, 50, 0, 100, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 13.0, 55, 50, 0, 100, 4.5, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_MOVE_KINETIC_PARAM);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 9.0, 90, 50, 0, 100, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 9.0, 90, 50, 0, 100, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 9.0, 90, 50, 0, 100, 3.5, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_AIR_CONTROL);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
        
}

#[acmd_script( agent = "kamui", script = "effect_specialswallattackf", category = ACMD_EFFECT, low_priority )]
unsafe fn kamui_special_s_wall_attack_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, -6, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("kamui_tyousoutotu_wind"), Hash40::new("top"), 0, 0, -6, 0, 0, 0, 0.45, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
        EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("kamui_tyousoutotu_atk"), Hash40::new("top"), 0, 1.5, 15.5, 0, 0, 90, 1, true);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("kamui_tyousoutotu_wind"), false, false);
    }
}

#[acmd_script( agent = "kamui", script = "game_specialswallattackb" , category = ACMD_GAME , low_priority)]
unsafe fn kamui_special_s_wall_attack_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, smash::phx::Hash40::new("special_s_wall_attack_b"), false, 0.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 6.0, 125, 70, 0, 100, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 125, 70, 0, 100, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 6.0, 125, 70, 0, 100, 4.5, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_MOVE_KINETIC_PARAM);
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_WALL_ATTACK_B_REVERSE_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 13.0, 55, 70, 0, 100, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 13.0, 55, 70, 0, 100, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 13.0, 55, 70, 0, 100, 4.5, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 9.0, 55, 50, 0, 100, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 9.0, 55, 50, 0, 100, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 9.0, 55, 50, 0, 100, 3.5, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_AIR_CONTROL);
    }
        
}

#[acmd_script( agent = "kamui", script = "effect_specialswallattackb", category = ACMD_EFFECT, low_priority )]
unsafe fn kamui_special_s_wall_attack_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, -4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 3, 14, 0, 12, 210, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("kamui_tyousoutotu_wind"), Hash40::new("top"), 0, 0, 5, 0, 180, 0, 0.45, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("kamui_tyousoutotu_atk"), Hash40::new("top"), 0, 1, -12, 0, 180, 90, 1, true);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("kamui_tyousoutotu_wind"), false, false);
    }
}

#[acmd_script( agent = "kamui", script = "game_specialswalljump" , category = ACMD_GAME , low_priority)]
unsafe fn kamui_special_s_wall_jump_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 8.0, 2.0);
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_s_wall_jump"), true, 0.0);
        ArticleModule::set_rate(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, 1.5);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 85, 58, 0, 100, 5.5, 6.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 85, 58, 0, 100, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_MOVE_KINETIC_PARAM);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_CHANGE_JUMP_ACCEL_Y);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_JUMP_CONTROL);
    }
}

#[acmd_script( agent = "kamui", script = "effect_specialswalljump", category = ACMD_EFFECT, low_priority )]
unsafe fn kamui_special_s_wall_jump_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, -6, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_c"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 8, 6, 0, 0, 90, 1.2, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 21, -16, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
        EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
    }
}

#[acmd_script( agent = "kamui", script = "game_specialswallend" , category = ACMD_GAME , low_priority)]
unsafe fn kamui_special_s_wall_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_s_wall_end"), true, 0.0);
        ArticleModule::change_motion(boma, 0, Hash40::new("fall"), false, 0.0);
    }
}

#[acmd_script( agent = "kamui", scripts = ["game_specialhi", "game_specialairhi"] , category = ACMD_GAME , low_priority)]
unsafe fn kamui_special_hi_game (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		SA_SET(fighter, *SITUATION_KIND_AIR);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 1, Hash40::new("top"), 4.5, 85, 100, 150, 0, 5.0, 0.0, 4.0, -5.0, Some(0.0), Some(4.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_TILT_BODY_ON);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		ATTACK(fighter, 0, 1, Hash40::new("rot"), 1.2, 80, 100, 140, 0, 2.0, 0.0, 5.5, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 1, Hash40::new("rot"), 1.2, 105, 100, 140, 0, 5.5, 0.0, 4.0, 6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
		ATTACK(fighter, 2, 1, Hash40::new("rot"), 1.2, 68, 100, 140, 0, 5.5, 0.0, 4.0, -6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
		ATTACK(fighter, 3, 1, Hash40::new("rot"), 1.2, 100, 100, 155, 0, 5.0, 0.0, -1.0, 5.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
		ATTACK(fighter, 4, 1, Hash40::new("rot"), 1.2, 73, 100, 155, 0, 5.0, 0.0, -1.0, -5.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
		AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, 0, 2, Hash40::new("rot"), 3.0, 65, 170, 0, 70, 6.5, 0.0, 3.5, -7.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 2, Hash40::new("rot"), 3.0, 65, 170, 0, 70, 6.5, 0.0, 3.5, 7.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_TILT_BODY_ON);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_AIR_CONTROL);
	}
	frame(lua_state, 49.0);
	FT_MOTION_RATE(fighter, 0.8);
}

#[acmd_script( agent = "kamui", scripts = ["game_speciallwhit", "game_specialairlwhit"] , category = ACMD_GAME , low_priority)]
unsafe fn kamui_special_lw_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_DESIRED_RATE(fighter, 26.0, 30.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, 0);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12.0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(boma, false);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 31.0);
    FT_MOTION_RATE_RANGE(fighter, 31.0, 66.0, 37.0);
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_whole(boma, true);
    }
}

#[acmd_script( agent = "kamui", scripts = ["effect_speciallwhit", "effect_specialairlwhit"] , category = ACMD_EFFECT , low_priority)]
unsafe fn kamui_special_lw_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_guard_mark"), true, true);
        EFFECT(fighter, Hash40::new("kamui_counter_success"), Hash40::new("top"), 0, 14, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("kamui_water_hamon"), Hash40::new("top"), 0, 0.5, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    if WorkModule::is_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 14.8, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
    EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
    WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        if sv_animcmd::get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(fighter, Hash40::new("kamui_counter_splash"), Hash40::new("top"), -7, 0, 15, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("kamui_counter_splash"), Hash40::new("top"), -7, 0, 9, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            EFFECT(fighter, Hash40::new("kamui_counter_splash"), Hash40::new("top"), 7, 0, 15, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("kamui_counter_splash"), Hash40::new("top"), 7, 0, 9, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, true);
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT(fighter, Hash40::new("kamui_counter_ripple"), Hash40::new("top"), 0, 0.5, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
    }
}

#[acmd_script( agent = "kamui_waterdragon", scripts = ["game_speciallwhit", "game_specialairlwhit"] , category = ACMD_GAME , low_priority)]
unsafe fn kamui_waterdragon_special_lw_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_DESIRED_RATE(fighter, 26.0, 30.0);
    frame(lua_state, 26.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 80, 60, 0, 90, 8.0, 0.0, 8.0, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 80, 60, 0, 90, 6.0, 0.0, 6.0, -3.0, Some(0.0), Some(6.0), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 2, true, false);
        if WorkModule::is_flag(boma, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
            AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_kamui_criticalhit"));
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 80, 60, 0, 90, 5.0, 0.0, 21.0, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 80, 60, 0, 90, 6.0, 0.0, 6.0, -3.0, Some(0.0), Some(6.0), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(boma, 1, true, false);
        AttackModule::set_force_reaction(boma, 2, true, false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 80, 60, 0, 90, 7.0, 0.0, 10.0, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 80, 60, 0, 90, 5.0, 0.0, 22.0, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
    }
    frame(lua_state, 31.0);
    FT_MOTION_RATE_RANGE(fighter, 31.0, 66.0, 37.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        kamui_special_n_end1_game,
        kamui_dragonhand_special_n_end1_game,
        kamui_special_s_attack_game,
        kamui_special_air_s_attack_game,
        kamui_spearhand_special_s_attack_game,
        kamui_spearhand_special_air_s_attack_game,
        kamui_special_s_wall_attack_f_game,
        kamui_special_s_wall_attack_f_effect,
        kamui_special_s_wall_attack_b_game,
        kamui_special_s_wall_attack_b_effect,
        kamui_special_s_wall_jump_game,
        kamui_special_s_wall_jump_effect,
        kamui_special_s_wall_end_game,
        kamui_special_hi_game,
        kamui_special_lw_hit_game,
        kamui_special_lw_hit_effect,
        kamui_waterdragon_special_lw_hit_game,
    );
}