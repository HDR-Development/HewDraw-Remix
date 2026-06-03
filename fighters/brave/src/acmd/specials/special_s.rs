use super::*;

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 9.0, 9.0);
    if is_excute(agent) {
        if boma.is_stick_backward() {
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 11.0, 6.0);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_ENABLE_SPARK);
    }
    if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_SUCCESS_SP) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 45, 100, 0, 35, 8.0, 0.0, 8.0, 11.5, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 45, 100, 0, 35, 6.0, 0.0, 10.5, 17.0, Some(0.0), Some(10.5), Some(17.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 45, 100, 0, 35, 6.0, 0.0, 10.5, 22.0, Some(0.0), Some(10.5), Some(22.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 45, 100, 0, 35, 6.0, 0.0, 10.5, 26.5, Some(0.0), Some(10.5), Some(26.5), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 45, 100, 0, 35, 5.5, 0.0, 10.5, 29.0, Some(0.0), Some(10.5), Some(29.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
    }
    else {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 45, 100, 0, 30, 7.0, 0.0, 9.0, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 6.0);
    }
}

unsafe extern "C" fn game_specials2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 10.0, 13.0);
    if is_excute(agent) {
        if boma.is_stick_backward() {
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_ENABLE_SPARK);
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_SUCCESS_SP) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 90, 0, 25, 4.5, 0.0, 11.5, 12.0, Some(0.0), Some(11.5), Some(50.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 10, 4);
        }   
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specials32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 11.0, 11.0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 120, 50, 50, 0, 14.0, 0.0, 11.0, 0.0, None, None, None, 1.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 100, 50, 50, 0, 8.5, 0.0, 8.5, 7.0, Some(0.0), Some(8.5), Some(10.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 100, 50, 50, 0, 8.5, 0.0, 8.5, -9.0, Some(0.0), Some(8.5), Some(-12.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_ENABLE_SPARK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 75, 9.0, 0.0, 8.5, 12.0, Some(0.0), Some(8.5), Some(-10.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.3);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 70, 0, 75, 9.0, 0.0, 8.5, 12.0, Some(0.0), Some(8.5), Some(-10.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 6.0);
    }
}

unsafe extern "C" fn effect_specials32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("brave_lightning3_hit2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_LIGHTNING_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        EFFECT(agent, Hash40::new("brave_lightning3_hit"), Hash40::new("top"), 0, 10.2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("brave_lightning3_lightning2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..6 {
    if is_excute(agent) {
        FLASH(agent, 1, 1, 0.3, 0.6);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_swordspark"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_zapsword"), Hash40::new("tex_brave_sword2"), 16, Hash40::new("sword1"), 3, 0, 0, Hash40::new("sword1"), 14, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 28.0);
    if PostureModule::lr(agent.module_accessor) < 0.0 {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_rolllightning"), Hash40::new("top"), 0, 10, 0, 0, 0, -5, 1, true);
        EFFECT_OFF_KIND(agent, Hash40::new("brave_lightning3_swordspark"), false, true);
    }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_swordspark"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    else {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_rolllightning"), Hash40::new("top"), 0, 10, 0, 0, 0, 4, 1, true);
    }
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("brave_lightning3_swordspark"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 10);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_LIGHTNING_SWORD_FLARE, false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("brave_lightning3_sword"), false, true);
    }
}

unsafe extern "C" fn effect_specialairs32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("brave_lightning3_hit2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_LIGHTNING_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        EFFECT(agent, Hash40::new("brave_lightning3_hit"), Hash40::new("top"), 0, 10.2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("brave_lightning3_lightning2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..6 {
    if is_excute(agent) {
        FLASH(agent, 1, 1, 0.3, 0.6);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_swordspark"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_zapsword"), Hash40::new("tex_brave_sword2"), 16, Hash40::new("sword1"), 3, 0, 0, Hash40::new("sword1"), 14, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 28.0);
    if PostureModule::lr(agent.module_accessor) < 0.0 {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_rolllightning"), Hash40::new("top"), 0, 10, 0, 0, 0, -5, 1, true);
        EFFECT_OFF_KIND(agent, Hash40::new("brave_lightning3_swordspark"), false, true);
    }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_swordspark"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    else {
    if is_excute(agent) {
    EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_rolllightning"), Hash40::new("top"), 0, 10, 0, 0, 0, 4, 1, true);
    }
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
    EFFECT_OFF_KIND(agent, Hash40::new("brave_lightning3_swordspark"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
    AFTER_IMAGE_OFF(agent, 10);
    EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_LIGHTNING_SWORD_FLARE, false, true);
    EFFECT_OFF_KIND(agent, Hash40::new("brave_lightning3_sword"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials1", game_specials1, Priority::Low);
    agent.acmd("game_specialairs1", game_specials1, Priority::Low);

    agent.acmd("game_specials2", game_specials2, Priority::Low);
    agent.acmd("game_specialairs2", game_specials2, Priority::Low);

    agent.acmd("game_specials32", game_specials32, Priority::Low);
    agent.acmd("effect_specials32", effect_specials32, Priority::Low);
    agent.acmd("game_specialairs32", game_specials32, Priority::Low);
    agent.acmd("effect_specialairs32", effect_specialairs32, Priority::Low);
}