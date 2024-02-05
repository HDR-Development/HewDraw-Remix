use super::*;


unsafe extern "C" fn gaogaen_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 16.0, 361, 84, 0, 65, 3.0, 0.0, 0.6, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 16.0, 361, 84, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("legr"), 20.0, 361, 79, 0, 65, 6.0, 9.0, 1.8, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_FOLLOW_THROUGH);
    }
    
}


unsafe extern "C" fn gaogaen_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 17.0, 88, 63, 0, 94, 5.5, 5.4, -0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 17.0, 88, 63, 0, 94, 3.6, 3.4, 0.0, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 14.0, 88, 63, 0, 94, 5.5, 5.4, -0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 14.0, 88, 63, 0, 94, 3.6, 3.4, 0.0, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_FOLLOW_THROUGH);
    }
    
}


unsafe extern "C" fn gaogaen_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        let top_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(boma, Hash40::new("top"), top_pos, false);
        let hip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(boma, Hash40::new("hip"), hip_pos, false);
        let facing = PostureModule::lr(boma);
        let ground = &mut Vector2f{ x: 0.0, y: 0.0 };
        if GroundModule::line_segment_check(boma,
            &Vector2f{ x: hip_pos.x, y: hip_pos.y },
            &Vector2f{ x: top_pos.x, y: top_pos.y - 5.0 },
            &Vector2f::zero(),
            ground,
            true).is_null() {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 3.0, 270, 100, 250, 0, 5.4, -4.0, -3.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 3.0, 270, 100, 250, 0, 5.4, 1.5, -5.5, 3.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
            ATTACK(fighter, 2, 0, Hash40::new("hip"), 3.0, 270, 100, 250, 0, 5.4, 7.0, -8.0, 7.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 4.0, 270, 100, 50, 0, 5.4, -4.0, -3.0, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 4.0, 270, 100, 50, 0, 5.4, 1.5, -5.5, 3.5, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
            ATTACK(fighter, 2, 0, Hash40::new("hip"), 4.0, 270, 100, 50, 0, 5.4, 7.0, -8.0, 7.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 270, 62, 0, 103, 5.4, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(-4.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 270, 62, 0, 103, 2.4, 0.0, 2.0, 11.0, Some(0.0), Some(2.0), Some(-6.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_FOLLOW_THROUGH);
    }
}


pub fn install() {
    smashline::Agent::new("gaogaen")
        .acmd("game_attacks4", gaogaen_attack_s4_s_game)
        .acmd("game_attackhi4", gaogaen_attack_hi4_game)
        .acmd("game_attacklw4", gaogaen_attack_lw4_game)
        .install();
}
