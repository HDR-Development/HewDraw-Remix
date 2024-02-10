use super::*;

#[acmd_script( agent = "edge", scripts = ["game_specialn1", "game_specialairn1"] , category = ACMD_GAME , low_priority)]
unsafe fn edge_special_n1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 11.0, 13.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, 0);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(fighter, 15.0, 35.0, 5.0);
    frame(lua_state, 35.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }

}

#[acmd_script( agent = "edge", scripts = ["game_specialn2", "game_specialairn2"] , category = ACMD_GAME , low_priority)]
unsafe fn edge_special_n2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 11.0, 13.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, 0);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE(fighter, 0.4);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
    frame(lua_state, 60.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

#[acmd_script( agent = "edge", script = "game_specialhistart", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhistart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(boma.object(), vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if !boma.is_button_on(Buttons::Special) && boma.is_situation(*SITUATION_KIND_GROUND) {
            VarModule::on_flag(boma.object(), vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_DECIDED_RUSH);
    }
    frame(lua_state, 21.0);
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH) {
        FT_MOTION_RATE(fighter, 1.5);
    }
    else if !VarModule::is_flag(boma.object(), vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX) {
        FT_MOTION_RATE(fighter, 2.0);
    }

}

#[acmd_script( agent = "edge", script = "effect_specialhistart", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialhistart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_charge"), Hash40::new("swordl1"), 0, 0, 0, 0, 180, -90, 0.9, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX) {
            EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
            LAST_EFFECT_SET_RATE(fighter, 3);
            EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
            LAST_EFFECT_SET_RATE(fighter, 1.2);

            EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 10.0, 0, 0.8, 0, 0, 0, 1.7, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 17.0, 0, 0.8, 0, 0, 0, 1.7, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 10.0, 0, 0.8, 0, 0, 0, 1.7, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 17.0, 0, 0.8, 0, 0, 0, 1.7, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    }
}

#[acmd_script( agent = "edge", script = "game_specialairhistart", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhistart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(boma.object(), vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_DECIDED_RUSH);
    }
}


#[acmd_script( agent = "edge", script = "game_specialhi2", category = ACMD_GAME, low_priority )]
unsafe fn edge_special_hi2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    FighterSpecializer_Edge::set_special_hi_jostle_area(boma);
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 8.0, 0.0, 2.5, 1.0, Some(0.0), Some(-2.5), Some(1.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 2.3, 367, 100, 0, 0, 6.5, 0.0, 3.0, 10.0, Some(0.0), Some(-3.0), Some(10.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 8.0, 0.0, 2.5, 1.0, Some(0.0), Some(-2.5), Some(1.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("rot"), 2.3, 367, 100, 0, 0, 6.5, 0.0, 3.0, 10.0, Some(0.0), Some(-3.0), Some(10.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("rot"), &Vector2f{ x: 20.0, y: 0.0 }, 4, false);
        AttackModule::set_vec_target_pos(boma, 2, Hash40::new("rot"), &Vector2f{ x: 20.0, y: 0.0 }, 4, false);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 2, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 3, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 4.0);
    for _ in 0..6 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 6.5, 0.0, 2.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("rot"), 2.3, 367, 100, 0, 0, 6.5, 0.0, 3.0, 10.0, Some(0.0), Some(-3.0), Some(10.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 6.5, 0.0, 2.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("rot"), 2.3, 367, 100, 0, 0, 6.5, 0.0, 3.0, 10.0, Some(0.0), Some(-3.0), Some(10.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("rot"), &Vector2f{ x: 20.0, y: 5.0 }, 4, false);
            AttackModule::set_vec_target_pos(boma, 2, Hash40::new("rot"), &Vector2f{ x: 10.0, y: 0.0 }, 4, false);
            AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
            AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
            AttackModule::set_no_finish_camera_ex(boma, 2, true, false);
            AttackModule::set_no_finish_camera_ex(boma, 3, true, false);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 14.0);
    FighterSpecializer_Edge::clear_special_hi_jostle_area(boma);
}

#[acmd_script( agent = "edge", script = "effect_specialhi2", category = ACMD_EFFECT, low_priority )]
unsafe fn edge_special_hi2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_octaslash_charge"), false, false);
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
            EFFECT(fighter, Hash40::new("edge_octaslash_illution_wing"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        EFFECT(fighter, Hash40::new("edge_octaslash_illution_01"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_speed_line"), Hash40::new("rot"), 0, -1, 0, 0, 0, 0, 2, true);
        EffectModule::enable_sync_init_pos_last(boma);
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.8, 0, 0, 4, 0, 0, 0, false);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_arc2"), Hash40::new("rot"), 0, 4, 2, 0, -3.5, 196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.6, 0, 0, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_arc2"), Hash40::new("rot"), 0, 3, 2, -4, -19, 13, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.45, 0, 0, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_arc2"), Hash40::new("rot"), 0, 6, 2, -15, -18, -144.7, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.45, 0, 0, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_arc2"), Hash40::new("rot"), 0, 2.6, 2, 29, -55, -13, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.45, 0, 0, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_arc2"), Hash40::new("rot"), 0, 2.5, 2, -2.5, -20, -163, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.45, 0, 0, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_arc2"), Hash40::new("rot"), 0, 5.3, 10, -12, -38, 35, 0.3, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.45, 0, 0, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("edge_octaslash_arc2"), Hash40::new("rot"), 0, 7.5, 7, -2.7, -24.5, -141, 0.25, 0, 0, 0, 0, 0, 0, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_arc"), Hash40::new("rot"), -3.2, 3.8, -3, -5.7, -0.5, 23, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_octaslash_sword_flare"), false, true);
    }
}

#[acmd_script( agent = "edge", script = "game_specialhi1", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhi1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        if !VarModule::is_flag(boma.object(), vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX) {
            if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
                ATTACK(fighter, 0, 0, Hash40::new("rot"), 9.0, 89, 60, 0, 60, 3.0, 0.0, -2.0, 9.0, Some(0.0), Some(-2.0), Some(-4.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
            else{
                ATTACK(fighter, 0, 0, Hash40::new("rot"), 7.0, 89, 60, 0, 60, 3.0, 0.0, -2.0, 9.0, Some(0.0), Some(-2.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        }
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
}

#[acmd_script( agent = "edge", script = "effect_specialhi1", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialhi1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_octaslash_charge"), false, false);
        if !VarModule::is_flag(boma.object(), vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX) {
            EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("handr"), 2, 0, 0, 0, 0, 0, 1.25, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash2"), Hash40::new("handr"), 2, 1, 0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("handr"), 2, 1, 0, 0, 0, 0, 0.7, true);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("kneer"), 1, 0, 0, 0, 0, 0, 0.7, true);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
            EffectModule::enable_sync_init_pos_last(boma);
        }
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let eff_hash = match color {
            0 => Hash40::new("edge_octaslash_illution_01"),
            1 => Hash40::new("edge_octaslash_illution_02"),
            2 => Hash40::new("edge_octaslash_illution_03"),
            3 => Hash40::new("edge_octaslash_illution_04"),
            4 => Hash40::new("edge_octaslash_illution_05"),
            5 => Hash40::new("edge_octaslash_illution_06"),
            6 => Hash40::new("edge_octaslash_illution_07"),
            7 => Hash40::new("edge_octaslash_illution_08"),
            _ => Hash40::new("edge_octaslash_illution_01")
        };
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
            EFFECT(fighter, Hash40::new("edge_octaslash_illution_wing"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        EFFECT(fighter, eff_hash, Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.8, 0, 0, 4, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.2);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_octaslash_line"), Hash40::new("rot"), 0, 0, 7, 0, 0, 0, 0.85, true);
        EffectModule::enable_sync_init_pos_last(boma);
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.6, 0, 0, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.45, 0, 0, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.45, 0, 0, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_octaslash_sword_flare"), false, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SITUATION_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.45, 0, 0, 3, 0, 0, 0, false);
        }
    }
}


#[acmd_script( agent = "edge", script = "game_specialhi1end" , category = ACMD_GAME , low_priority)]
unsafe fn edge_special_hi1_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.08);
        // if VarModule::is_flag(boma.object(), vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX) {
        //     MotionModule::set_rate(boma, 2.0);
        // }
        // else{
        //     MotionModule::set_rate(boma, 0.9);
        // }
    }
}

#[acmd_script( agent = "edge", script = "game_specialairhi1end", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhi1end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
}

#[acmd_script( agent = "edge", script = "game_specialhi2end" , category = ACMD_GAME , low_priority)]
unsafe fn edge_special_hi2_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 7.0, 361, 104, 0, 68, 10.0, 0.0, 0.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 2.3, 60, 60, 0, 90, 8.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MotionModule::set_rate(boma, 1.256);
    }
}

#[acmd_script( agent = "edge", script = "game_specialairhi2end" , category = ACMD_GAME , low_priority)]
unsafe fn edge_special_air_hi_2_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut kb_angle = 0;
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("charged_special_hi");
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 3.0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if WorkModule::get_float(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE) <= 0.0 {
            kb_angle = (WorkModule::get_float(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE) * -1.0) as u64;
        }
        else {
            kb_angle = 361 as u64;
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 7.0, kb_angle, 104, 0, 68, 10.0, 0.0, 0.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 2.3, 60, 60, 0, 90, 8.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
}

#[acmd_script( agent = "edge", scripts = ["game_speciallw", "game_specialairlw"] , category = ACMD_GAME , low_priority)]
unsafe fn edge_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 27.0, 13.0);
    if is_excute(fighter) {
        if app::sv_math::rand(hash40("fighter"), 2) == 1 {
            WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_VOICE);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 11.0, 3.0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 3.0);
    }
}

#[acmd_script( agent = "edge", scripts = ["effect_speciallw", "effect_specialairlw"] , category = ACMD_EFFECT , low_priority)]
unsafe fn edge_special_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_senkou_start"), Hash40::new("waist"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 1, 0.75);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }

}

#[acmd_script( agent = "edge", scripts = ["game_speciallwhit", "game_specialairlwhit"] , category = ACMD_GAME , low_priority)]
unsafe fn edge_special_lw_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(fighter.object(), vars::edge::status::FLASH_HOLD);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FLASH, false, -1);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_X);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_ENABLE_FALL_SPEED);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(fighter, 1.15);
    frame(lua_state, 50.0);
    FT_MOTION_RATE(fighter, 1.0);

}

#[acmd_script( agent = "edge", scripts = ["effect_speciallwhit", "effect_specialairlwhit"] , category = ACMD_EFFECT , low_priority)]
unsafe fn edge_special_lw_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_senkou_shield"), true, true);
        let offset = if VarModule::is_flag(fighter.object(), vars::edge::status::FLASH_HOLD) { 35 } else { 0 };
        EFFECT(fighter, Hash40::new("edge_senkou_shield_break"), Hash40::new("top"), 0, 13, 12 + offset, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        edge_special_n1_game,
        edge_special_n2_game,
        //game_specialhistart,
        //effect_specialhistart,
        //game_specialairhistart,
        edge_special_hi2_game,
        edge_special_hi2_effect,
        //game_specialhi1,
        //effect_specialhi1,
        edge_special_hi1_end_game,
        game_specialairhi1end,
        edge_special_hi2_end_game,
        edge_special_air_hi_2_end_game,
        edge_special_lw_game,
        edge_special_lw_effect,
        edge_special_lw_hit_game,
        edge_special_lw_hit_effect,
    );
}