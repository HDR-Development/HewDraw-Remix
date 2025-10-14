use super::*;

unsafe extern "C" fn game_superspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 24.0, 80, 52, 0, 79, 1.0, 0.0, 2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 80, 52, 0, 79, 3.0, 0.0, 3.0, -8.0, Some(0.0), Some(3.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 24.0, 80, 52, 0, 79, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(10.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 22.0, 80, 53, 0, 78, 11.0, 0.0, 25.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 20.0, 80, 54, 0, 77, 8.0, 0.0, 40.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn game_superspecialtriple1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let scale = 0.81;
    if is_excute(agent) {
        PostureModule::set_scale(boma, scale, false);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 368, 100, 90, 0, 11.0, 0.0, 10.0, -1.0, None, None, None, 1.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 8, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 368, 100, 90, 0, 11.0, 0.0, 25.0,  3.0, None, None, None, 1.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 8, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 368, 100, 90, 0,  8.0, 0.0, 40.0,  5.0, None, None, None, 1.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 8, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 23.0 / scale, y: 22.0 / scale}, 12, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 24.0 / scale, y: 31.0 / scale}, 12, false);
        AttackModule::set_vec_target_pos(boma, 2, Hash40::new("top"), &Vector2f{x: 25.0 / scale, y: 40.0 / scale}, 12, false);
        AttackModule::set_no_dead_all(boma, true, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_superspecialtriple1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

unsafe extern "C" fn sound_superspecialtriple1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_dolly_final_superspecial01_03"));
    }
}

unsafe extern "C" fn game_superspecialtriple2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let scale = 0.9;
    if is_excute(agent) {
        PostureModule::set_scale(boma, scale, false);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 368, 100, 90, 0, 11.0, 0.0, 10.0, -1.0, None, None, None, 2.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 8, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 368, 100, 90, 0, 11.0, 0.0, 25.0,  3.0, None, None, None, 2.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 8, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 368, 100, 90, 0,  8.0, 0.0, 40.0,  5.0, None, None, None, 2.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 8, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 23.0 / scale, y: 38.0 / scale}, 16, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 24.0 / scale, y: 38.0 / scale}, 16, false);
        AttackModule::set_vec_target_pos(boma, 2, Hash40::new("top"), &Vector2f{x: 25.0 / scale, y: 44.0 / scale}, 16, false);
        AttackModule::set_no_dead_all(boma, true, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_superspecialtriple2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
}

unsafe extern "C" fn sound_superspecialtriple2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_dolly_final_superspecial01_03"));
    }
}

unsafe extern "C" fn game_superspecialtriple3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let lr = PostureModule::lr(boma);
        PostureModule::add_pos(boma, &Vector3f::new(-7.0 * lr, 0.0, 0.0));
        ATTACK(agent, 0, 0, Hash40::new("top"), 24.0, 80, 52, 0, 79, 1.0, 0.0, 2.0, -2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 80, 52, 0, 79, 3.0, 0.0, 3.0, -8.0, Some(0.0), Some(3.0), Some(4.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 24.0, 80, 52, 0, 79, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(10.0), Some(-1.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 22.0, 80, 53, 0, 78, 11.0, 0.0, 25.0, 3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 20.0, 80, 54, 0, 77, 8.0, 0.0, 40.0, 5.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_superspecialtriple3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_superspecialtriple3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_dolly_final_superspecial01_03_last"));
    }
}


pub fn install(agent: &mut Agent) {
    agent.acmd("game_superspecial", game_superspecial, Priority::Low);

    agent.acmd("game_superspecialtriple1", game_superspecialtriple1, Priority::Low);
    agent.acmd("effect_superspecialtriple1", effect_superspecialtriple1, Priority::Low);
    agent.acmd("sound_superspecialtriple1", sound_superspecialtriple1, Priority::Low);

    agent.acmd("game_superspecialtriple2", game_superspecialtriple2, Priority::Low);
    agent.acmd("effect_superspecialtriple2", effect_superspecialtriple2, Priority::Low);
    agent.acmd("sound_superspecialtriple2", sound_superspecialtriple2, Priority::Low);

    agent.acmd("game_superspecialtriple3", game_superspecialtriple3, Priority::Low);
    agent.acmd("effect_superspecialtriple3", effect_superspecialtriple3, Priority::Low);
    agent.acmd("sound_superspecialtriple3", sound_superspecialtriple3, Priority::Low);
}