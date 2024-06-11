use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, (11.0-1.0)/3.0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        //MotionModule::set_rate(boma, 2.0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 361, 115, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 361, 115, 0, 30, 3.5, 0.0, 2.75, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 361, 115, 0, 30, 3.5, 0.0, 5.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 361, 115, 0, 30, 4.0, 0.0, 8.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        
        wait(lua_state, 6.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 361, 100, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 361, 100, 0, 25, 3.5, 0.0, 2.75, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("haver"), 10.0, 361, 100, 0, 25, 3.5, 0.0, 5.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 8.0, 361, 100, 0, 25, 4.0, 0.0, 8.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::trail::status::SHOULD_PRORATE_ATTACK);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 0.5);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
}

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("trail_air_lw_fall"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("rot"), 0, 0, 1.5, 0, 0, 90, 1.15, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.8, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -1.0, 1.0, 0, 0, 90, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.8, 0.1);
        //EFFECT_FOLLOW(agent, Hash40::new("sys_heart"), Hash40::new("haver"), 0, 4.0, 0, 0, 0, 90.0, 0.5, true);
        //LAST_EFFECT_SET_COLOR(agent, 4.0, 5.0, 1.0);
        //EFFECT_FOLLOW(agent, Hash40::new("sys_heart"), Hash40::new("haver"), 0, 8.0, 0, 0, 0, 90.0, 0.5, true);
        //LAST_EFFECT_SET_COLOR(agent, 4.0, 5.0, 1.0);
        EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"), Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn sound_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_trail_attackair_l01"));
        //STOP_SE_arg2(agent, Hash40::new("se_trail_jump02"), 50);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_trail_rnd_attack03"));
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_trail_attackair_l02"));
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 0.6);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, (1.0));
        ATTACK(agent, 0, 0, Hash40::new("haver"), 4.5, 75, 4, 0, 60, 4.0, 0.0, 9.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 4.5, 82, 4, 0, 55, 4.0, 0.0, 4.5, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 4.5, 82, 4, 0, 55, 4.0, 0.0, -1.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 4.0, false);
    }
    wait(lua_state, 2.0);
    FT_MOTION_RATE(agent, 1.4);
    wait(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_F_FLAG_CHECK_COMBO_BUTTON_ON);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"), Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn game_attackairf2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CHECK_COMBO_BUTTON_ON);
        //MotionModule::set_rate(boma, 1.25);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 0.6);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 4.5, 78, 8, 0, 75, 4.0, 0.0, 9.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 4.5, 85, 8, 0, 65, 4.0, 0.0, 4.5, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 4.5, 85, 8, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(boma, 0, -1.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, -5.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, -5.0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CHECK_COMBO_BUTTON_ON);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, (37.0-20.0)/12.0);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
}

unsafe extern "C" fn game_attackairf3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.4);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 0.7);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("haver"), 4.5, 40, 110, 0, 50, 4.6, 0.0, 0.0, -1.8, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 4.5, 40, 110, 0, 50, 4.6, 0.0, 4.5, -1.8, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.5, 40, 110, 0, 50, 4.6, 0.0, 9.5, -1.8, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairf3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"),  Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::trail::status::SHOULD_PRORATE_ATTACK);
        VarModule::off_flag(boma.object(), vars::trail::instance::COMBO_PLUS_AIR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(boma, 2.0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 9.0, 361, 100, 0, 20, 4.5, 1.5, 6.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 13.5, 361, 100, 0, 30, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 13.5, 361, 100, 0, 30, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 13.5, 361, 100, 0, 30, 4.5, 0.0, 4.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("haver"), 13.5, 361, 100, 0, 30, 4.5, 0.0, 8.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::trail::status::SHOULD_PRORATE_ATTACK);
        if VarModule::is_flag(boma.object(), vars::trail::instance::COMBO_PLUS_AIR){
            VarModule::on_flag(boma.object(), vars::trail::status::SHOULD_PRORATE_ATTACK);
            VarModule::off_flag(boma.object(), vars::trail::instance::COMBO_PLUS_AIR);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(boma, 2.0);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, (9.5-5.0)/5.0);
    }
    frame(lua_state, 9.5);
    if is_excute(agent) {
        MotionModule::set_rate(boma, (20.0-9.5)/8.0);
        if VarModule::is_flag(boma.object(), vars::trail::status::SHOULD_PRORATE_ATTACK){
            ATTACK(agent, 0, 0, Hash40::new("haver"), 4.5, 75, 136, 0, 44, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 4.5, 75, 136, 0, 44, 4.0, 0.0, 4.25, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("haver"), 4.5, 75, 136, 0, 44, 4.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, *ATTACK_REGION_SWORD);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 75, 70, 0, 65, 3.8, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 75, 70, 0, 65, 3.8, 0.0, 4.1, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("haver"), 10.0, 75, 70, 0, 65, 3.8, 0.0, 8.2, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::trail::status::SHOULD_PRORATE_ATTACK);
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"),  Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let attack_start_frame = 15.5;
    let attack_end_frame = 25.0;
    let attack_duration = 7.0;
    let attack_duration_rate = (attack_end_frame-attack_start_frame)/attack_duration;
    let spike_start_frame = attack_start_frame + 5.0*attack_duration_rate;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, (10.0-1.0)/3.0);
        VarModule::off_flag(boma.object(), vars::trail::status::SHOULD_PRORATE_ATTACK);
        if VarModule::is_flag(boma.object(), vars::trail::instance::COMBO_PLUS_AIR){
            VarModule::on_flag(boma.object(), vars::trail::status::SHOULD_PRORATE_ATTACK);
            VarModule::off_flag(boma.object(), vars::trail::instance::COMBO_PLUS_AIR);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, (attack_start_frame-10.0)/4.0);
    }
    frame(lua_state, attack_start_frame);
    if is_excute(agent) {
        MotionModule::set_rate(boma, attack_duration_rate);
        if VarModule::is_flag(boma.object(), vars::trail::status::SHOULD_PRORATE_ATTACK){
            ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 4.5, 62, 117, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 4.5, 62, 117, 0, 60, 4.0, 0.4, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("haver"), 4.5, 62, 117, 0, 60, 4.0, 0.4, 4.25, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("haver"), 4.5, 62, 117, 0, 60, 4.0, 0.4, 8.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        } else {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 8.0, 62, 70, 0, 50, 3.8, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 62, 70, 0, 50, 3.8, 0.4, 0.0, 1.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
                ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 62, 70, 0, 50, 3.8, 0.4, 4.25, 1.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
                ATTACK(agent, 3, 0, Hash40::new("haver"), 8.0, 62, 70, 0, 50, 3.8, 0.4, 8.5, 1.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            }
        }
    }
    frame(lua_state, spike_start_frame);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::trail::status::SHOULD_PRORATE_ATTACK){
            ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 8.0, 62, 70, 0, 50, 3.8, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 300, 70, 0, 30, 3.8, 0.4, 0.0, 1.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 300, 70, 0, 30, 3.8, 0.4, 4.25, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("haver"), 8.0, 300, 70, 0, 30, 3.8, 0.4, 8.5, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 8.0, 58, 80, 0, 60, 3.8, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 300, 70, 0, 30, 3.8, 0.4, 0.0, 1.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 6, 0, Hash40::new("haver"), 8.0, 285, 100, 0, 45, 3.8, 0.4, 4.25, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 7, 0, Hash40::new("haver"), 8.0, 285, 100, 0, 45, 3.8, 0.4, 8.5, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);

            ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 58, 80, 0, 60, 3.8, 0.4, 0.0, 1.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            /* Ground-only */
            ATTACK(agent, 4, 0, Hash40::new("haver"), 8.0, 285, 100, 0, 45, 3.8, 0.4, 4.25, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 5, 0, Hash40::new("haver"), 8.0, 285, 100, 0, 45, 3.8, 0.4, 8.5, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            /* Air-only */
            ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 300, 70, 0, 30, 3.8, 0.4, 4.25, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("haver"), 8.0, 300, 70, 0, 30, 3.8, 0.4, 8.5, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        }
        //println!("spike start");
    }
    frame(lua_state, attack_end_frame);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::trail::status::SHOULD_PRORATE_ATTACK);
        AttackModule::clear_all(boma);
        MotionModule::set_rate(boma, 1.0);
        //println!("spike end");
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, (50.0-30.0)/13.0);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MotionModule::set_rate(boma, 1.0);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.25);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"), Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        AFTER_IMAGE_OFF(agent, 5);
    }
}

unsafe extern "C" fn sound_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_trail_attackair_n03"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_trail_rnd_attack03"));
    }
}

unsafe extern "C" fn expression_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.5);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 15.5);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_81_attackl"), 0);
    }
}

unsafe extern "C" fn game_landingairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn effect_landingairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_landingairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_trail_landing02"));
    }
}

unsafe extern "C" fn expression_landingairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_81_landingairlw"), 0, false, 0);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if ItemModule::is_have_item(boma, 0){
            //FT_MOTION_INTP_WAIT_ITEM(agent, true, 0);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        //FT_MOTION_INTP_WAIT_ITEM(agent, true, 0.08);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        //FT_MOTION_INTP_WAIT_ITEM(agent, true, 0.03);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn, Priority::Low);
    agent.acmd("effect_attackairn", effect_attackairn, Priority::Low);
    agent.acmd("sound_attackairn", sound_attackairn, Priority::Low);

    agent.acmd("game_attackairf", game_attackairf, Priority::Low);
    agent.acmd("effect_attackairf", effect_attackairf, Priority::Low);

    agent.acmd("game_attackairf2", game_attackairf2, Priority::Low);

    agent.acmd("game_attackairf3", game_attackairf3, Priority::Low);
    agent.acmd("effect_attackairf3", effect_attackairf3, Priority::Low);

    agent.acmd("game_attackairb", game_attackairb, Priority::Low);

    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi", effect_attackairhi, Priority::Low);

    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw", effect_attackairlw, Priority::Low);
    agent.acmd("sound_attackairlw", sound_attackairlw, Priority::Low);
    agent.acmd("expression_attackairlw", expression_attackairlw, Priority::Low);

    agent.acmd("game_landingairlw", game_landingairlw, Priority::Low);
    agent.acmd("effect_landingairlw", effect_landingairlw, Priority::Low);
    agent.acmd("sound_landingairlw", sound_landingairlw, Priority::Low);
    agent.acmd("expression_landingairlw", expression_landingairlw, Priority::Low);
}
