use super::*;

unsafe extern "C" fn game_trailspecialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        ArticleModule::remove_exist(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER, app::ArticleOperationTarget(0));
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER,  Hash40::new("special_n2"), false, 0.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.7);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 78, 60, 0, 70, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 78, 60, 0, 70, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 90.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
    }
}

unsafe extern "C" fn effect_trailspecialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_sword_ice"), Hash40::new("haver"), 0, 0, 0, -90, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("trail_ice_hold"), Hash40::new("haver"), 0, 10, -1, -90, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("trail_ice_sword_flare"), Hash40::new("haver"), 0, 10, -1, -90, 0, 0, 1, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), 0, 3.7, 0, 0, 0, 180, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 0.25);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), 0, 3.7, 0, 0, 180, 180, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 0.25);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("trail_sword_ice"), -1);
        EFFECT_OFF_KIND(agent, Hash40::new("trail_sword_ice"), false, true);
    }
}

unsafe extern "C" fn sound_trailspecialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_trail_special_n_b01"));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_copy_trail_special_n02"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_trail_special_n_b02"));
    }
}

unsafe extern "C" fn expression_trailspecialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_81_special_n2"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 43.0);
    if ItemModule::is_have_item(boma, 0) {
        if is_excute(agent) {
            //FT_MOTION_INTP_WAIT_ITEM(agent, true, 0.05);
        }
    }
    frame(lua_state, 49.0);
    if is_excute(agent) {
        //FT_MOTION_INTP_WAIT_ITEM(agent, true, 0.08);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        //FT_MOTION_INTP_WAIT_ITEM(agent, true, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_trailspecialn2", game_trailspecialn2, Priority::Low);
    agent.acmd("effect_trailspecialn2", effect_trailspecialn2, Priority::Low);
    agent.acmd("sound_trailspecialn2", sound_trailspecialn2, Priority::Low);   
    agent.acmd("expression_trailspecialn2", expression_trailspecialn2, Priority::Low);
    agent.acmd("game_trailspecialairn2", game_trailspecialn2, Priority::Low);
    agent.acmd("effect_trailspecialairn2", effect_trailspecialn2, Priority::Low);
    agent.acmd("sound_trailspecialairn2", sound_trailspecialn2, Priority::Low);
    agent.acmd("expression_trailspecialairn2", expression_trailspecialn2, Priority::Low);
}
