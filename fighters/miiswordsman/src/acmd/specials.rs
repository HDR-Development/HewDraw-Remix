use super::*;

unsafe extern "C" fn game_specialn1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 17.0, 14.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_TORNADOSHOT, false, 0);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 48.0, 17.0);
    frame(lua_state, 48.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 14.0, 9.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
        }
        else {
            let x_vel = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            agent.clear_lua_stack();
            lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, x_vel * 0.6, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(agent.lua_state_agent);
            let air_speed_x_stable = WorkModule::get_param_float(agent.module_accessor, hash40("air_speed_x_stable"), 0);
            agent.clear_lua_stack();
            lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * 0.5, 100.0);
            app::sv_kinetic_energy::set_stable_speed(agent.lua_state_agent);
        }
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    if boma.is_button_on(Buttons::Special) {
        FT_MOTION_RATE_RANGE(agent, 14.0, 17.0, 13.0);
    }
    else {
        FT_MOTION_RATE(agent, 1.0);
    }
    if is_excute(agent) {
        if boma.is_button_on(Buttons::Special) {
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
    frame(lua_state, 17.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
                agent.clear_lua_stack();
                lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.8);
                app::sv_kinetic_energy::set_speed(agent.lua_state_agent);
            }
        }
        // light
        if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            MotionModule::set_rate(boma, 1.0);
            ATTACK(agent, 0, 0, Hash40::new("haver"), 11.0, 361, 100, 0, 39, 3.5, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 11.0, 361, 100, 0, 39, 3.5, 0.0, 6.75, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 361, 100, 0, 39, 3.5, 0.0, 11.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        // heavy
        else {
            ATTACK(agent, 0, 0, Hash40::new("haver"), 13.0, 361, 100, 0, 39, 3.5, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 13.0, 361, 100, 0, 39, 3.5, 0.0, 6.75, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("haver"), 14.0, 361, 100, 0, 39, 3.5, 0.0, 11.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            ArticleModule::generate_article(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, false, 0);
            ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 33.0);
    if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        FT_MOTION_RATE_RANGE(agent, 33.0, 36.0, 10.0);
    }
    frame(lua_state, 36.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_miiswordsman_sword03"), Hash40::new("tex_miiswordsman_sword04"), 8, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 14.2);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 7.5, 0.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), -1, 10, 2, -10.6, -140, 90.0, 1.3, true);
	    LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4.0);
    }
}

unsafe extern "C" fn sound_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_special_c2_l01"));
        PLAY_SE(agent, Hash40::new("se_miiswordsman_special_l03"));
    }
}

unsafe extern "C" fn expression_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x26769bd1de), 0, 0, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        } else {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x26769bd1de), 0, 30, 20);
    }
}

// ================================================================================================
// ======================================== BLURRING BLADE ========================================
// ================================================================================================

unsafe extern "C" fn game_specialn3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let turn = agent.is_motion_one_of(&[Hash40::new("special_n3_end_turn"), Hash40::new("special_n3_end_max_turn")]);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if turn {
            REVERSE_LR(agent);
        }
    }
    for _ in 0..4 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("haver"), 1.0, 180, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 1.0, 92, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("haver"), 1.0, 91, 100, 21, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 33.0, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 33.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let sfx = if agent.is_motion_one_of(&[Hash40::new("special_n3_end_max"), Hash40::new("special_n3_end_max_turn")]) { *COLLISION_SOUND_ATTR_FIRE } else { *COLLISION_SOUND_ATTR_KICK };
        let offset = if turn { -10.0 } else { 9.5 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 65, 0, 80, 10.0, 0.0, 10.0, offset, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, sfx, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(agent, 40.0, 80.0, 26.0);
    frame(lua_state, 80.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairn3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let turn = agent.is_motion_one_of(&[Hash40::new("special_air_n3_end_turn"), Hash40::new("special_air_n3_end_max_turn")]);
    if is_excute(agent) {
        let flip = if turn { -1.0 } else { 1.0 };
        SET_SPEED_EX(agent, 0.5 * flip, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if turn {
            REVERSE_LR(agent);
        }
    }
    for _ in 0..2 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("haver"), 1.0, 90, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("haver"), 1.0, 285, 100, 10, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 1.0, 120, 100, 20, 0, 4.0, 0.0, 8.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 1.0, 45, 100, 25, 0, 4.0, 0.0, -1.2, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 2.0);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 33.0, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(0.0, 2.0, 0.0));
    }
    frame(lua_state, 33.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let sfx = if agent.is_motion_one_of(&[Hash40::new("special_air_n3_end_max"), Hash40::new("special_air_n3_end_max_turn")]) { *COLLISION_SOUND_ATTR_FIRE } else { *COLLISION_SOUND_ATTR_KICK };
        let offset = if turn { -9.5 } else { 9.5 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 65, 0, 80, 10.0, 0.0, 10.0, offset, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, sfx, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(agent, 40.0, 80.0, 26.0);
    frame(lua_state, 80.0);
    FT_MOTION_RATE(agent, 1.0);
}

// ==================================================================================================
// ======================================== AIRBORNE ASSAULT ========================================
// ==================================================================================================

unsafe extern "C" fn game_specials1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.4);
    }
    
}

unsafe extern "C" fn game_specialairs1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.4);
    }
    
}

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {

    }
    
}

unsafe extern "C" fn game_specialairs1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {

    }
    
}
/*
#[acmd_script( agent = "miiswordsman", script = "game_specials1hit" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_s1_hit_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.5, 44, 75, 0, 56, 4.5, 0.0, 2.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.5, 44, 75, 0, 56, 4.5, 0.0, 6.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.5, 44, 75, 0, 56, 4.5, 0.0, 11.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    
}
*/

// =============================================================================================
// ======================================== GALE STAB ==========================================
// =============================================================================================

unsafe extern "C" fn game_specials2dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    frame(lua_state, 10.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.5);
    }
}

unsafe extern "C" fn game_specials2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 5.0, 0.0, 5.5, 16.0, Some(0.0), Some(5.5), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 3.5, 0.0, 5.5, 15.0, Some(0.0), Some(5.5), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_ATTACK_END);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_specials2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        // Vanilla Effects
        //EFFECT_FOLLOW_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(agent, 1, 0.72, 1.1);
        
        //EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_blastwind_stab"), Hash40::new("top"), -0.0, 5, 12, 0, 0, 0, 1.1, true);
        //EFFECT_DETACH_KIND(agent, Hash40::new("miiswordsman_blastwind_stab"), -1);
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 11.4, 0, 0, 0, 0, 2, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_smash_flash_s"), -1);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, 7, 0, 0, 0, 1, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, -3, 0, 0, 0, 0.8, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        

        // Sword Trail
        //AFTER_IMAGE4_ON_WORK_arg29(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 5, Hash40::new("haver"), 0, 0.2, 0, Hash40::new("haver"), -0.0, 10.8, 0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), -1, 7, 7.5, 0.0, -180.0, 25.3, 1.1, true);
	    LAST_EFFECT_SET_RATE(agent, 2.5);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0.0, 10, 6, -10.6, -159.5, 55.3, 1.1, true);
	    //LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 9.25, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 5);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(agent, 2.5, 2.5, 2.5);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        //EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 6, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        //EFFECT_OFF_KIND_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, false, false);
    }
}

unsafe extern "C" fn game_specialairs2dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.5);
    }
}

unsafe extern "C" fn game_specialairs2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 5.0, 0.0, 5.5, 16.0, Some(0.0), Some(5.5), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 3.5, 0.0, 5.5, 15.0, Some(0.0), Some(5.5), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_ATTACK_END);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_specialairs2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        // Vanilla Effects
        //EFFECT_FOLLOW_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(agent, 1, 0.72, 1.1);
        
    
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 11.4, 0, 0, 0, 0, 2, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_smash_flash_s"), -1);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, 7, 0, 0, 0, 1, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, -3, 0, 0, 0, 0.8, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        

        // Sword Trail
        //AFTER_IMAGE4_ON_WORK_arg29(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 5, Hash40::new("haver"), 0, 0.2, 0, Hash40::new("haver"), -0.0, 10.8, 0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);

    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), -1, 7, 7.5, 0.0, -180.0, 25.3, 1.1, true);
	    LAST_EFFECT_SET_RATE(agent, 2.5);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0.0, 10, 6, -10.6, -159.5, 55.3, 1.1, true);
	    //LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 9.25, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 5);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(agent, 2.5, 2.5, 2.5);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        //EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 6, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        //EFFECT_OFF_KIND_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, false, false);
    }
}

// =============================================================================================
// ======================================== CHAKRAM ============================================
// =============================================================================================

unsafe extern "C" fn game_specials3_1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }  
}

unsafe extern "C" fn game_specials3_1hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }     
}

unsafe extern "C" fn game_specials3_1lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }    
}

unsafe extern "C" fn game_specialairs3_1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }     
}

unsafe extern "C" fn game_specialairs3_1hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }    
}

unsafe extern "C" fn game_specialairs3_1lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }     
}

// =============================================================================================
// ======================================== STONE SCABBARD ========================================
// =============================================================================================

unsafe extern "C" fn game_specialhi1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.7);
    }
}

unsafe extern "C" fn game_specialairhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let addSpeed = Vector3f { x: 0.0, y: 0.5, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 46, 95, 0, 60, 4.0, 0.0, 7.0, 6.0, Some(0.0), Some(4.0), Some(6.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 46, 95, 0, 60, 3.5, 0.0, 7.0, 10.0, Some(0.0), Some(4.0), Some(10.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 46, 95, 0, 60, 3.5, 0.0, 7.0, 14.0, Some(0.0), Some(4.0), Some(14.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 92, 100, 60, 0, 4.0, 0.0, 7.0, 6.0, Some(0.0), Some(4.0), Some(6.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 92, 100, 60, 0, 3.5, 0.0, 7.0, 10.0, Some(0.0), Some(4.0), Some(10.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 92, 100, 60, 0, 3.5, 0.0, 7.0, 14.0, Some(0.0), Some(4.0), Some(14.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 7.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 7.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 7.0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        // Tip
        ATTACK(agent, 0, 0, Hash40::new("haver"), 3.0, 268, 180, 30, 0, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 3.0, 361, 100, 20, 0, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // Middle
        ATTACK(agent, 2, 0, Hash40::new("haver"), 3.0, 268, 180, 30, 0, 5.0, 0.0, 7.25, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 3.0, 361, 100, 20, 0, 5.0, 0.0, 7.25, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // Hilt
        ATTACK(agent, 4, 0, Hash40::new("haver"), 3.0, 268, 180, 30, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("haver"), 3.0, 361, 100, 20, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_ROKET_UNDER_DISABLE_CONTROL_X);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialhi1end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 55, 116, 0, 103, 8.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(4.5), 1.55, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.8);
    }
}

unsafe extern "C" fn game_specialairhi1end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 55, 116, 0, 103, 8.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(4.5), 1.55, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.8);
    } 
}

// ====================================================================================================
// ======================================== SKYWARD SLASH DASH ========================================
// ====================================================================================================

unsafe extern "C" fn game_specialhi2hold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        FT_MOTION_RATE(agent, 0.6);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            //VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            FT_MOTION_RATE(agent, 2.5);
        }
        else{
            FT_MOTION_RATE(agent, 0.8);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.2, 91, 100, 140, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.2, 100, 100, 80, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
}

unsafe extern "C" fn game_specialhi2holdair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        FT_MOTION_RATE(agent, 0.6);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            //VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            FT_MOTION_RATE(agent, 2.5);
        }
        else{
            FT_MOTION_RATE(agent, 0.8);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("rot"), 2.2, 91, 100, 140, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("rot"), 2.2, 100, 100, 80, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    for _ in 0..5 {
        frame(lua_state, 2.0);
        if is_excute(agent) {
            // Charged
            //if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
                ATTACK(agent, 0, 0, Hash40::new("rot"), 3.0, 367, 40, 0, 40, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("rot"), 3.0, 55, 40, 0, 40, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 2, 0, Hash40::new("rot"), 3.0, 367, 100, 100, 0, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 3, 0, Hash40::new("rot"), 3.0, 363, 100, 120, 0, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
            // Uncharged
            else{
                ATTACK(agent, 0, 0, Hash40::new("rot"), 1.0, 367, 40, 0, 40, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("rot"), 1.0, 55, 40, 0, 40, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 2, 0, Hash40::new("rot"), 1.0, 367, 100, 100, 0, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 3, 0, Hash40::new("rot"), 1.0, 363, 100, 120, 0, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(agent) {
        
        let h_speed = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) as f32;
        let v_speed = KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) as f32;
        //let angle_calc = v_speed / h_speed as f32;
        //println!("Angle: {}", angle_calc);
        
        let mut launch_angle = if v_speed > 0.0 {(v_speed / h_speed.abs()).atan().to_degrees() as u64} else {361 as u64};

        //println!("H speed: {}", h_speed);
        //println!("V speed: {}", v_speed);

        //println!("Launch Angle (pre-calc): {}", launch_angle);

        if h_speed == 0.0 {
            launch_angle = 90 as u64;
        }

        //println!("Launch Angle: {}", launch_angle);
        
        // Charged
        //if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            ATTACK(agent, 0, 0, Hash40::new("rot"), 6.0, launch_angle, 110, 0, 70, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("rot"), 6.0, launch_angle, 110, 0, 70, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        // Uncharged
        else{
            ATTACK(agent, 0, 0, Hash40::new("rot"), 4.0, 79, 40, 0, 90, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("rot"), 4.0, 79, 40, 0, 90, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.3, 1.23, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash"), Hash40::new("rot"), -2, -4, 0, 0, 0, 0, 0.9, true);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0 ,0, 0, false);
        }
        else if agent.is_situation(*SITUATION_KIND_AIR) {
            EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0 ,0, 0, false);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash"), Hash40::new("rot"), 0, 0, 9, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash2"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash3"), Hash40::new("rot"), 0, -5, 0, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash4"), Hash40::new("rot"), 0, -7.5, 0, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash5"), Hash40::new("rot"), -1, -10, 3.5, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash6"), Hash40::new("rot"), 0, 0, -1.5, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_DETACH_KIND(agent, Hash40::new("miiswordsman_sdash_hit"), -1);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_sdash"), false, true);
    }
}

unsafe extern "C" fn game_specialhi2landing(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 0.8);
        }
    }
}

unsafe extern "C" fn game_specialhi2fall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 0.8);
        }
        if VarModule::is_flag(agent.battle_object, vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT) {
            FT_MOTION_RATE(agent, 0.5);
        }
    }
}

// =============================================================================================
// ======================================== HERO'S SPIN ========================================
// =============================================================================================

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let start_x_mul = ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_hi3.start_x_mul");
        KineticModule::mul_speed(agent.module_accessor, &Vector3f{x: start_x_mul, y: 1.0, z: 1.0}, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 14.0, 40, 64, 0, 90, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 13.0, 40, 64, 0, 90, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 10.0, 40, 64, 0, 90, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 361, 64, 0, 80, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 11.0, 361, 64, 0, 80, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 361, 64, 0, 80, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 9.0, 361, 64, 0, 75, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 9.0, 361, 64, 0, 75, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 7.0, 361, 64, 0, 75, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 7.0, 361, 64, 0, 70, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 7.0, 361, 64, 0, 70, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 5.0, 361, 64, 0, 70, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn game_specialairhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let start_speed = agent.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let start_x_mul = ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_hi3.start_x_mul");
    agent.clear_lua_stack();
    lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_speed * start_x_mul);
    app::sv_kinetic_energy::set_speed(agent.lua_state_agent);
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 100, 115, 0, 6.0, 0.0, 9.0, 7.0, Some(0.0), Some(6.0), Some(14.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 60, 100, 80, 0, 6.0, 0.0, 10.0, -14.0, Some(0.0), Some(10.0), Some(-9.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 90, 0, 6.5, 0.0, 8.0, 9.0, Some(0.0), Some(7.0), Some(14.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 60, 100, 90, 0, 6.5, 0.0, 5.0, -15.0, Some(0.0), Some(6.0), Some(-11.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 75, 0, 6.0, 0.0, 8.0, 9.0, Some(0.0), Some(6.0), Some(14.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 60, 100, 90, 0, 6.5, 0.0, 8.0, -13.0, Some(0.0), Some(8.0), Some(-10.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 80, 100, 60, 0, 6.3, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(15.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 60, 100, 100, 0, 6.0, 0.0, 9.0, -14.0, Some(0.0), Some(9.0), Some(-10.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 39.0, 47.0, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 208, 0, 30, 7.5, 0.0, 14.0, 4.0, Some(0.0), Some(18.0), Some(11.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 208, 0, 30, 7.5, 0.0, 15.0, 3.0, Some(0.0), Some(19.0), Some(7.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    
}

// ===============================================================================================
// ======================================== KINESIS BLADE ========================================
// ===============================================================================================

unsafe extern "C" fn game_speciallw1hit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 21.0, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 80, 60, 0, 85, 8.8, 0.0, 8.0, 15.0, Some(0.0), Some(8.0), Some(3.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 47.0, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairlw1hit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 21.0, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 73, 60, 0, 85, 10.5, 0.0, 7.0, 14.0, Some(0.0), Some(7.0), Some(4.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 47.0, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
}

// Kinesis Blade - Charge Storage
// #[acmd_script( agent = "miiswordsman", script = "game_speciallw1hit" , category = ACMD_GAME , low_priority)]
// unsafe fn miiswordsman_special_lw1_hit_game(agent: &mut L2CAgentBase) {
//     let lua_state = agent.lua_state_agent;
//     let boma = agent.boma();
//     if is_excute(agent) {
//         let current_level = VarModule::get_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL);
//         // Attack transition
//         if VarModule::is_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER) {
//             VarModule::off_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER);
//             if current_level > 0{
//                 if current_level == 1 {
//                     // println!("Changing to level 1");
//                     MotionModule::change_motion(boma, Hash40::new("special_lw1_hit_lv1"), 0.0, 1.0, false, 0.0, false, false);
//                 }
//                 else{
//                     // println!("Changing to level 2");
//                     MotionModule::change_motion(boma, Hash40::new("special_lw1_hit_lv2"), 0.0, 1.0, false, 0.0, false, false);
//                 }
//             }
//         }
//         else{
//             // Increment the level
//             if current_level < 2 {
//                 VarModule::set_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, current_level + 1); // Add a charge level
//                 // println!("Kinesis increment");
//             }
//             // println!("Kinesis Level: {}", VarModule::get_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL));
//         }
//     }
//     frame(lua_state, 23.0);
//     if is_excute(agent) {
//         WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
//         //FT_MOTION_RATE(agent, 0.65);
//     }
// }

// #[acmd_script( agent = "miiswordsman", script = "game_specialairlw1hit" , category = ACMD_GAME , low_priority)]
// unsafe fn miiswordsman_special_air_lw1_hit_game(agent: &mut L2CAgentBase) {
//     let lua_state = agent.lua_state_agent;
//     let boma = agent.boma();
//     if is_excute(agent) {
//         let current_level = VarModule::get_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL);
//         // Attack transition
//         if VarModule::is_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER) {
//             VarModule::off_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER);
//             if current_level > 0{
//                 if current_level == 1 {
//                     // println!("Changing to level 1");
//                     MotionModule::change_motion(boma, Hash40::new("special_air_lw1_hit_lv1"), 0.0, 1.0, false, 0.0, false, false);
//                 }
//                 else{
//                     // println!("Changing to level 2");
//                     MotionModule::change_motion(boma, Hash40::new("special_air_lw1_hit_lv2"), 0.0, 1.0, false, 0.0, false, false);
//                 }
//             }
//         }
//         else{
//             // Increment the level
//             if current_level < 2 {
//                 VarModule::set_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, current_level + 1); // Add a charge level
//                 // println!("Kinesis increment");
//             }
//             // println!("Kinesis Level: {}", VarModule::get_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL));
//         }
//     }
//     frame(lua_state, 23.0);
//     if is_excute(agent) {
//         WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
//         //FT_MOTION_RATE(agent, 0.65);
//     }
// }

// Kinesis Blade - 1 Charge

unsafe extern "C" fn miiswordsman_special_lw1_hit_lv1_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, 0); // Reset charge level
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 50, 6.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(-30.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        //WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
}

unsafe extern "C" fn miiswordsman_special_air_lw1_hit_lv1_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, 0); // Reset charge level
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 11.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 50, 6.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(-30.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        //WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
}

// Kinesis Blade - 2 Charges

unsafe extern "C" fn miiswordsman_special_lw1_hit_lv2_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.1, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 63.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 73.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn miiswordsman_special_air_lw1_hit_lv2_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.1, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 63.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 73.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

// ================================================================================================================
// ======================================== DEFLECTING DRAFT / SHOCK SPELL ========================================
// ================================================================================================================

unsafe extern "C" fn game_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 12.0, 16.0);
    if is_excute(agent) {
        if boma.is_button_on(Buttons::Special) {
            VarModule::on_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD);
        }
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
        let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD) { 15.0 } else { 0.0 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 55, 0, 40, 5.0, 0.0, 9.0, 15.0 + hold, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 60, 55, 0, 40, 2.0, 0.0, 2.0, 15.0 + hold, Some(0.0), Some(25.0), Some(15.0 + hold), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(agent, 16.0, 23.0, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
    }
    
}

unsafe extern "C" fn effect_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -1.5, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_reflect_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0.0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.65);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let mut offset = 0;
        EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("arml"), 4, 0, 0, 0, 0, 0, 0.4, true);
        if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD) {
            offset = 13;
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15.0, 8.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        }
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_smokescreen"), Hash40::new("top"), 0, 25, 13 + offset, 0, 0.0, 0, 0.4, true);
            LAST_EFFECT_SET_SCALE_W(agent, 0.4, 0.25, 0.4);
            LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.1);
            LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD) { 13 } else { 0 };
            EFFECT_FOLLOW(agent, Hash40::new("sys_smokescreen"), Hash40::new("top"), 0, 25, 13 + hold, 0, 0.0, 0, 0.4, true);
            LAST_EFFECT_SET_SCALE_W(agent, 0.4, 0.25, 0.4);
            LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.1);
            LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    frame(lua_state, 10.5);
    if is_excute(agent) {
        let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD) { 13 } else { 0 };
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0.5, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        }
        EFFECT_FOLLOW(agent, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 15, 13 + hold, 0, 0, 180, 0.15, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD) { 13 } else { 0 };
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_paralysis"), Hash40::new("arml"), 4, 0, 0, 0, 0, 0, 0.35, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_smokescreen"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 8, 13 + hold, 0, 0, 0, 0.4, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.84, 0.17);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_paralysis"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder_flash"), true, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_hit_elec"), -1);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_reflect_sword"), false, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("miiswordsman_reflect1"), -1);
    }
    
}

unsafe extern "C" fn sound_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miiswordsman_special_c2_l01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_special_c2_l01"));
    }
    frame(lua_state, 10.5);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_m"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_s"));
    }

}

// ================================================================================================================
// ======================================== HURRICANE HEAVE =======================================================
// ================================================================================================================

unsafe extern "C" fn game_speciallw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 12.0, /*Angle*/ 35, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 77, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.8, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 35, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 77, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(19.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_WALL_CHECK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 42, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 42, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(19.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 50, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 50, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(19.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn miiswordsman_special_lw3_end_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {

    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //ATTACK(agent, 0, 1, Hash40::new("haver"), 1.0, 100, 10, 0, 10, 4.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 4.0, y: 5.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 0.0, y: 17.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 4, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        //REVERSE_LR(agent);
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 17.0, 0.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -6.0, y: 15.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 3, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        //REVERSE_LR(agent);
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 15.0, -6.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -12.0, y: 0.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 2, false);
    }
    /*
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 0.1, 365, 30, 0, 0, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        //let hit1 = Vector2f {x: 12.0, y: 0.0};
        //AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    */
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 2, Hash40::new("haver"), 5.0, 80, 100, 0, 60, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(11.5), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairlw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    sv_kinetic_energy!(set_speed_mul_2nd, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.4, 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_WALL_CHECK);
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 275, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 275, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0);
    sv_kinetic_energy!(set_speed_mul_2nd, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.0, 1.0);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairlw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.790);
        //WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_SP_BRAKE);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        //WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_SP_BRAKE);
    }
}

unsafe extern "C" fn miiswordsman_special_air_lw3_end_air_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //ATTACK(agent, 0, 1, Hash40::new("haver"), 1.0, 100, 10, 0, 10, 4.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 4.0, y: 5.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 0.0, y: 17.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 4, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        //REVERSE_LR(agent);
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 17.0, 0.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -6.0, y: 15.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 3, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        //REVERSE_LR(agent);
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 15.0, -6.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -12.0, y: 0.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 2, false);
    }
    /*
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 0.1, 365, 30, 0, 0, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        //let hit1 = Vector2f {x: 12.0, y: 0.0};
        //AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    */
    frame(lua_state, 21.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        ATTACK(agent, 0, 2, Hash40::new("haver"), 5.0, 80, 100, 0, 60, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(11.5), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn1", game_specialn1);
    agent.acmd("game_specialairn1", game_specialn1);

    agent.acmd("game_specialn2", game_specialn2);
    agent.acmd("game_specialairn2", game_specialn2);
    agent.acmd("effect_specialn2", effect_specialn2);
    agent.acmd("effect_specialairn2", effect_specialn2);
    agent.acmd("sound_specialn2", sound_specialn2);
    agent.acmd("sound_specialairn2", sound_specialn2);
    agent.acmd("expression_specialn2", expression_specialn2);
    agent.acmd("expression_specialairn2", expression_specialn2);

    agent.acmd("game_specialn3end", game_specialn3end);
    agent.acmd("game_specialn3endturn", game_specialn3end);
    agent.acmd("game_specialn3endmax", game_specialn3end);
    agent.acmd("game_specialn3endmaxturn", game_specialn3end);
    agent.acmd("game_specialairn3end", game_specialairn3end);
    agent.acmd("game_specialairn3endturn", game_specialairn3end);
    agent.acmd("game_specialairn3endmax", game_specialairn3end);
    agent.acmd("game_specialairn3endmaxturn", game_specialairn3end);

    agent.acmd("game_specials1start", game_specials1start);
    agent.acmd("game_specialairs1start", game_specialairs1start);
    agent.acmd("game_specials1", game_specials1);
    agent.acmd("game_specialairs1", game_specialairs1);

    agent.acmd("game_specials2dash", game_specials2dash);
    agent.acmd("game_specials2attack", game_specials2attack);
    agent.acmd("effect_specials2attack", effect_specials2attack);
    agent.acmd("game_specialairs2dash", game_specialairs2dash);
    agent.acmd("game_specialairs2attack", game_specialairs2attack);
    agent.acmd("effect_specialairs2attack", effect_specialairs2attack);

    agent.acmd("game_specials3_1", game_specials3_1);
    agent.acmd("game_specials3_1hi", game_specials3_1hi);
    agent.acmd("game_specials3_1lw", game_specials3_1lw);
    agent.acmd("game_specialairs3_1", game_specialairs3_1);
    agent.acmd("game_specialairs3_1hi", game_specialairs3_1hi);
    agent.acmd("game_specialairs3_1lw", game_specialairs3_1lw);

    agent.acmd("game_specialhi1start", game_specialhi1start);
    agent.acmd("game_specialairhi1", game_specialairhi1);
    agent.acmd("game_specialhi1end", game_specialhi1end);
    agent.acmd("game_specialairhi1end", game_specialairhi1end);

    agent.acmd("game_specialhi2hold", game_specialhi2hold);
    agent.acmd("game_specialhi2holdair", game_specialhi2holdair);
    agent.acmd("game_specialhi2", game_specialhi2);
    agent.acmd("effect_specialhi2", effect_specialhi2);
    agent.acmd("game_specialhi2landing", game_specialhi2landing);
    agent.acmd("game_specialhi2fall", game_specialhi2fall);

    agent.acmd("game_specialhi3", game_specialhi3);
    agent.acmd("game_specialairhi3", game_specialairhi3);

    agent.acmd("game_speciallw1hit", game_speciallw1hit);
    agent.acmd("game_specialairlw1hit", game_specialairlw1hit);
    //agent.acmd("game_speciallw1hitlv1", miiswordsman_special_lw1_hit_lv1_game);
    //agent.acmd("game_specialairlw1hitlv1", miiswordsman_special_air_lw1_hit_lv1_game);
    //agent.acmd("game_speciallw1hitlv2", miiswordsman_special_lw1_hit_lv2_game);
    //agent.acmd("game_specialairlw1hitlv2", miiswordsman_special_air_lw1_hit_lv2_game);

    agent.acmd("game_speciallw2", game_speciallw2);
    agent.acmd("game_specialairlw2", game_speciallw2);
    agent.acmd("effect_speciallw2", effect_speciallw2);
    agent.acmd("effect_specialairlw2", effect_speciallw2);
    agent.acmd("sound_speciallw2", sound_speciallw2);
    agent.acmd("sound_specialairlw2", sound_speciallw2);
    
    agent.acmd("game_speciallw3", game_speciallw3);
    //agent.acmd("game_speciallw3end", miiswordsman_special_lw3_end_game);
    agent.acmd("game_specialairlw3", game_specialairlw3);
    agent.acmd("game_specialairlw3end", game_specialairlw3end);
    //agent.acmd("game_specialairlw3endair", miiswordsman_special_air_lw3_end_air_game);
}