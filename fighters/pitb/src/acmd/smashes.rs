use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 21.0, 6.0);
    // without set_speed_mul 0.0 here, motion speed on this frame makes the 366 fail
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 100, 53, 0, 5.4, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(12.0), 1.0, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);    }
    frame(lua_state, 11.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 10.0, 361, 121, 0, 42, 6.0, 0.0, 8.0, 13.5, Some(0.0), Some(8.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 55.0, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, -5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 3, Hash40::new("swordr1"), 0, 0, -0.2, Hash40::new("swordr1"), 0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 5, Hash40::new("swordr1"), 0, 0.9, -0.2, Hash40::new("swordr1"), 0, 11, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0.9, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordl"), 0, -0.9, -0.2, Hash40::new("swordl"), 0, -11, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, -0.9, 0, 180, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 5);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 0);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 105, 159, 0, 62, 5.0, 0.0, 26.5, 0.0, None, None, None, 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 105, 159, 0, 62, 5.5, 0.0, 24.0, 7.0, Some(0.0), Some(24.0), Some(4.5), 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 105, 159, 0, 62, 5.5, 0.0, 24.0, -6.0, Some(0.0), Some(24.0), Some(-4.5), 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 105, 159, 0, 62, 5.5, 0.0, 24.0, 7.0, Some(0.0), Some(24.0), Some(-6.0), 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 4, 0, Hash40::new("top"), 5.0, 105, 159, 0, 62, 5.5, 0.0, 24.0, 7.0, Some(0.0), Some(24.0), Some(-6.0), 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 5, 0, Hash40::new("top"), 5.0, 105, 159, 0, 62, 5.5, 0.0, 18.0, -5.5, Some(0.0), Some(18.0), Some(-4.5), 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 6, 0, Hash40::new("top"), 5.0, 105, 159, 0, 62, 5.5, 0.0, 18.0, -5.5, Some(0.0), Some(18.0), Some(-4.5), 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE_RANGE(agent, 7.0, 9.0, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 105, 159, 0, 62, 6.0, 0.0, 14.0, 9.0, Some(0.0), Some(12.0), Some(7.0), 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 4, 0, Hash40::new("top"), 5.0, 105, 159, 0, 62, 6.0, 0.0, 14.0, 9.0, Some(0.0), Some(12.0), Some(7.0), 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 12.0, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 6.0, 94, 139, 0, 62, 5.5, 0.0, 28.0, 0.0, None, None, None, 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 1, 1, Hash40::new("top"), 6.0, 94, 139, 0, 62, 6.0, 0.0, 24.0, 6.0, None, None, None, 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 2, 1, Hash40::new("top"), 6.0, 94, 139, 0, 62, 6.0, 0.0, 24.0, -6.0, None, None, None, 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE_RANGE(agent, 12.0, 16.0, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 2, Hash40::new("top"), 7.0, 90, 123, 0, 62, 6.0, 0.0, 34.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 1, 2, Hash40::new("top"), 7.0, 90, 123, 0, 62, 7.0, 0.0, 24.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 2, 2, Hash40::new("top"), 7.0, 90, 123, 0, 62, 7.0, 0.0, 31.0, 6.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 3, 2, Hash40::new("top"), 7.0, 90, 123, 0, 62, 7.0, 0.0, 31.0, -6.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, -8, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 5, Hash40::new("swordr1"), 0, 0, -0.2, Hash40::new("swordr1"), 0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 5, Hash40::new("swordl"), 0, 0, -0.2, Hash40::new("swordl"), 0, -10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, 0, 0, 180, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 5, Hash40::new("swordr1"), 0, 0, -0.2, Hash40::new("swordr1"), 0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 48, 98, 0, 40, 3.7, 0.0, 3.3, 6.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 48, 98, 0, 40, 3.6, 0.0, 2.8, 12.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 48, 93, 0, 35, 3.4, 0.0, 2.0, 17.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 30, 93, 0, 25, 3.7, 0.0, 3.3, -8.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 30, 93, 0, 25, 3.6, 0.0, 2.8, -14.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 30, 93, 0, 25, 3.4, 0.0, 2.0, -19.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, -3, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0, 0, -0.2, Hash40::new("swordr1"), 0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 5.0);
    if PostureModule::lr(agent.module_accessor) < 0.0 {
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        else {
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, -2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    }
}
frame(lua_state, 7.0);
if is_excute(agent) {
    AFTER_IMAGE_OFF(agent, 3);
}
frame(lua_state, 16.0);
if is_excute(agent) {
    EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, 0, 0, 180, 90, 0, 1, true);
}
frame(lua_state, 17.0);
if PostureModule::lr(agent.module_accessor) < 0.0 {
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, -3.5, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    else {
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 3.5, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}
}
frame(lua_state, 18.0);
if is_excute(agent) {
EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 3, 3, -12, 23, 197, 6, 0.6, true);
}
frame(lua_state, 22.0);
if is_excute(agent) {
EFFECT_OFF_KIND(agent, Hash40::new("pitb_sword"), false, true);
}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);
    agent.acmd("effect_attacks4", effect_attacks4, Priority::Low);
    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    agent.acmd("effect_attackhi4", effect_attackhi4, Priority::Low);
    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
    agent.acmd("effect_attacklw4", effect_attacklw4, Priority::Low);
}