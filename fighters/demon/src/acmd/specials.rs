use super::*;

unsafe extern "C" fn game_attackstep(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 0.8);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        JostleModule::set_team(boma, 1);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn kazuya_wind_god_fist_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        JostleModule::set_team(boma, 1);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.7);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(0.15, 0.0, 0.0));
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.15, 0.0, 0.0));
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 13.5, 88, 10, 0, 98, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.5, 88, 10, 0, 98, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 13.5, 88, 10, 0, 98, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("bust"), 13.5, 88, 10, 0, 98, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 13.5, 88, 10, 0, 98, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 13.5, 88, 10, 0, 98, 3.0, -1.0, 9.0, 3.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 5, 1.2);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 4, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 5, 15.0, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_M, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_M, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 4, *CAMERA_QUAKE_KIND_M, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 5, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 0.9);
    if is_excute(fighter) {
        AttackModule::clear(boma, 4, false);
        AttackModule::clear(boma, 5, false);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(fighter, 0.8);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn kazuya_electric_wind_god_fist_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        JostleModule::set_team(boma, 1);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.7);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(0.15, 0.0, 0.0));
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.15, 0.0, 0.0));
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 14.5, 86, 20, 0, 85, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.31, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 14.5, 86, 20, 0, 85, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.31, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 14.5, 86, 20, 0, 85, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.31, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("bust"), 14.5, 86, 20, 0, 85, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.31, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 14.5, 86, 20, 0, 85, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.31, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 14.5, 86, 20, 0, 85, 3.0, -1.0, 9.0, 3.5, None, None, None, 0.31, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2.8);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2.8);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2.8);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2.8);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2.8);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 5, 2.8);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 5, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2.8);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2.8);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 4, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 5, 12.0, false);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_S, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_S, false);
        AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_S, false);
        AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_S, false);
        AttackModule::set_attack_camera_quake_forced(boma, 4, *CAMERA_QUAKE_KIND_S, false);
        AttackModule::set_attack_camera_quake_forced(boma, 5, *CAMERA_QUAKE_KIND_S, false);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 0.9);
    if is_excute(fighter) {
        AttackModule::clear(boma, 4, false);
        AttackModule::clear(boma, 5, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 0.9);
    if is_excute(fighter) {
        AttackModule::clear(boma, 6, false);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }

}

unsafe extern "C" fn kazuya_spinning_demon_to_left_hook_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 9.0, 6.0);
    if is_excute(fighter) {
        JostleModule::set_team(boma, 1);
        KineticModule::add_speed(boma, &Vector3f::new(0.1, 0.0, 0.0));
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 3.0, 60, 100, 50, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 3.0, 72, 100, 50, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 3.0, 85, 100, 50, 0, 4.0, 5.0, 0.0, 0.0, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, 20.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 20.0, false);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE_RANGE(fighter, 14.0, 33.0, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 33.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.1, 0.0, 0.0));
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.5, 25, 20, 0, 79, 4.0, 0.0, 15.0, 7.0, None, None, None, 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.5, 25, 20, 0, 79, 5.0, 0.0, 14.0, 9.0, Some(0.0), Some(8.5), Some(9.0), 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 11.5, 25, 20, 0, 79, 5.0, 0.0, 13.0, 3.5, Some(0.0), Some(5.5), Some(3.5), 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("shoulderl"), 11.5, 25, 20, 0, 79, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("shoulderr"), 11.5, 25, 20, 0, 79, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, -10.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, -10.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, -10.0, false);
        AttackModule::set_add_reaction_frame(boma, 3, -10.0, false);
        AttackModule::set_add_reaction_frame(boma, 4, -10.0, false);
        AttackModule::set_attack_camera_quake(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake(boma, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake(boma, 2, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
        AttackModule::clear(boma, 4, false);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            ATTACK(fighter, 5, 1, Hash40::new("top"), 0.0, 235, 100, 16, 0, 10.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(20.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 45.0);
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
        FT_MOTION_RATE(fighter, 0.2);
    }
    else{
        FT_MOTION_RATE(fighter, 0.6);
    }
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn kazuya_lightning_uppercut_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.9);
    if is_excute(fighter) {
        JostleModule::set_team(boma, 1);
    }
    frame(lua_state, 7.0);
    if VarModule::is_flag(boma.object(), vars::demon::instance::LIGHTNING_SCREW_UPPERCUT){
        FT_MOTION_RATE_RANGE(fighter, 7.0, 19.0, 10.0);
    }
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        AttackModule::set_damage_shake_scale(boma, 1.5);
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 22.0, 60, 65, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 22.0, 60, 65, 0, 80, 5.25, 0.0, 10.5, 4.75, Some(0.0), Some(16.0), Some(7.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 22.0, 60, 65, 0, 80, 5.25, 0.0, 5.0, 2.5, Some(0.0), Some(10.5), Some(4.75), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 0.5);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 2, false);
        AttackModule::set_damage_shake_scale(boma, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 18.0, 70, 60, 0, 80, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.1), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 70, 60, 0, 80, 5.0, 0.0, 13.0, 3.0, Some(0.0), Some(23.0), Some(3.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 0.5);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 16.0, 70, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 70, 50, 0, 60, 5.0, 0.0, 14.0, 3.0, Some(0.0), Some(24.0), Some(3.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 0.5);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 14.0, 70, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 5.0, 0.0, 21.0, 3.0, Some(0.0), Some(24.0), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 5.0, 0.0, 21.5, 3.0, Some(0.0), Some(24.5), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 5.0, 0.0, 22.0, 3.0, Some(0.0), Some(25.0), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 28.0);
    FT_MOTION_RATE(fighter, 0.9);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn kazuya_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        JostleModule::set_team(boma, 1);
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.5);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 4.0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 60, 50, 0, 80, 4.0, 0.0, 8.0, 3.0, Some(0.0), Some(8.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 60, 50, 0, 80, 4.0, 0.0, 8.0, 3.0, Some(0.0), Some(8.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_S_FLAG_CHANGE_HIT);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        //FT_MOTION_RATE(fighter, 30.0/(62.0-15.0));
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 1.0);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 2.0);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    }
    frame(lua_state, 58.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    }
    frame(lua_state, 61.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 8.0);
    }
    frame(lua_state, 62.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
        //FT_MOTION_RATE(fighter, 1.0);
    }
}

unsafe extern "C" fn kazuya_special_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 49.0, 40.0);
    if is_excute(fighter) {
        JostleModule::set_team(boma, 1);
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 1.0);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 2.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 8.0);
    }
    frame(lua_state, 49.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    }
}

unsafe extern "C" fn game_specialairs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.5);
    FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_s");
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(fighter, 1.0);
    FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    frame(lua_state, 12.0);
    FighterSpecializer_Demon::set_devil(boma, true, 4.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 14.0);
    FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 60, 50, 0, 80, 4.0, 0.0, 8.0, 3.0, Some(0.0), Some(8.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 60, 50, 0, 80, 4.0, 0.0, 8.0, 3.0, Some(0.0), Some(8.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_S_FLAG_CHANGE_HIT);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 16.0);
    FighterSpecializer_Demon::set_devil(boma, true, 1.0);
    frame(lua_state, 19.0);
    FighterSpecializer_Demon::set_devil(boma, true, 2.0);
    frame(lua_state, 24.0);
    FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    frame(lua_state, 41.0);
    FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    frame(lua_state, 58.0);
    FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    frame(lua_state, 60.0);
    FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    frame(lua_state, 61.0);
    FighterSpecializer_Demon::set_devil(boma, true, 8.0);
    frame(lua_state, 62.0);
    FighterSpecializer_Demon::set_devil(boma, false, 0.0);
}

unsafe extern "C" fn kazuya_special_air_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_s");
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 1.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 49.0, 30.0);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 2.0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_S_FLAG_AIR_CONTROL);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_S_FLAG_LANDING);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 8.0);
    }
    frame(lua_state, 49.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    }
}

unsafe extern "C" fn kazuya_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_HI_FLAG_AIR){
        if is_excute(fighter) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 2.0);
        }
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 76, 55, 0, 80, 6.0, 0.0, 6.0, 6.0, Some(0.0), Some(11.25), Some(6.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 76, 55, 0, 80, 6.0, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 18.0, 76, 55, 0, 80, 6.0, 0.0, 6.0, -2.5, Some(0.0), Some(10.25), Some(-2.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear(boma, 0, false);
            AttackModule::clear(boma,  2, false);
        }
        frame(lua_state, 4.0);
        if is_excute(fighter) {
            ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 71, 68, 0, 75, 5.15, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 7.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 7.0);
        if is_excute(fighter) {
            ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 63, 90, 0, 60, 4.6, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 60, 90, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 15.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            //FT_MOTION_RATE(fighter, 15.0/(56.0-15.0));
        }
        frame(lua_state, 46.0);
        if is_excute(fighter) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
        }
        frame(lua_state, 52.0);
        if is_excute(fighter) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 4.0);
        }
        frame(lua_state, 53.0);
        if is_excute(fighter) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
        }
        frame(lua_state, 54.0);
        if is_excute(fighter) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
        }
        frame(lua_state, 56.0);
        if is_excute(fighter) {
            //FT_MOTION_RATE(fighter, 1.0);
        }
    }
    else{
        if is_excute(fighter) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 2.0);
        }
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 76, 55, 0, 80, 6.0, 0.0, 6.0, 6.0, Some(0.0), Some(11.25), Some(6.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 76, 55, 0, 80, 6.0, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 18.0, 76, 55, 0, 80, 6.0, 0.0, 6.0, -2.5, Some(0.0), Some(10.25), Some(-2.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear(boma, 0, false);
            AttackModule::clear(boma,  2, false);
        }
        frame(lua_state, 4.0);
        if is_excute(fighter) {
            ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 71, 68, 0, 75, 5.15, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 7.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 7.0);
        if is_excute(fighter) {
            ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 63, 90, 0, 60, 4.6, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 60, 90, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 15.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            //FT_MOTION_RATE(fighter, 15.0/(56.0-15.0));
        }
        frame(lua_state, 46.0);
        if is_excute(fighter) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
        }
        frame(lua_state, 52.0);
        if is_excute(fighter) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 4.0);
        }
        frame(lua_state, 53.0);
        if is_excute(fighter) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
        }
        frame(lua_state, 54.0);
        if is_excute(fighter) {
            smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
        }
        frame(lua_state, 56.0);
        if is_excute(fighter) {
            //FT_MOTION_RATE(fighter, 1.0);
        }
    }
    
}

unsafe extern "C" fn kazuya_special_hi_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 2.0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 76, 53, 0, 80, 6.0, 0.0, 6.0, 6.0, Some(0.0), Some(11.25), Some(6.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 76, 53, 0, 80, 6.0, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 18.0, 76, 53, 0, 80, 6.0, 0.0, 6.0, -2.5, Some(0.0), Some(10.25), Some(-2.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma,  2, false);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 71, 65, 0, 75, 5.15, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 63, 85, 0, 60, 4.6, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 60, 85, 0, 60, 4.0, 0.0, 18.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        //FT_MOTION_RATE(fighter, 15.0/(56.0-15.0));
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    }
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 4.0);
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    }
    frame(lua_state, 56.0);
    if is_excute(fighter) {
        //FT_MOTION_RATE(fighter, 1.0);
    }
}

unsafe extern "C" fn kazuya_special_hi_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("demon_devil_sign_flash"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
        EFFECT_FOLLOW(fighter, Hash40::new("demon_ragedrive_start"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.75, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.5, 1.0);
        //FLASH(fighter, 0.25, 0.08, 0.6, 0);
        //FLASH_SET_DIRECTION(fighter, -1, 0, 0);
        //BURN_COLOR(fighter, 8, 3, 36, 0);
        //FLASH_FRM(fighter, 2, 0.25, 0.08, 0.6, 0.4);
        //BURN_COLOR_FRAME(fighter, 2, 8, 3, 36, 0.4);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("demon_devil_start"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("demon_devil_start_aura"), Hash40::new("bust"), 0, 0, 1, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("demon_devil_start_elec"), Hash40::new("bust"), 0, 0, 1, 0, 0, 0, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        //FLASH(fighter, 0.25, 0.08, 0.6, 0.4);
        //BURN_COLOR(fighter, 8, 3, 36, 0.4);
        //FLASH_FRM(fighter, 3, 0.25, 0.08, 0.6, 0);
        //BURN_COLOR_FRAME(fighter, 3, 8, 3, 36, 0);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_attackstep2fhitshield(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        //macros::ATTACK_FP(fighter, 6, 1, Hash40::new("top"), 0, 361, 100, 65, 0, 12, 0, 10, 10, Hash40::new("collision_attr_normal"), 0, 0, 0, false, false, 0, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *COLLISION_SITUATION_MASK_G, true, *ATTACK_REGION_NONE, *COLLISION_CATEGORY_MASK_FIGHTER, false, *COLLISION_PART_MASK_ALL, false, false, false, false, 0, false, false, *ATTACK_LR_CHECK_POS, false, false, true, true, false, *COLLISION_SHAPE_TYPE_SPHERE);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 0.6);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 7.75, 12.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        GrabModule::set_constraint(boma, 0, true);
        GrabModule::set_constraint(boma, 1, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 19.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        GrabModule::set_constraint(boma, 0, true);
        GrabModule::set_constraint(boma, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(agent, 0.9);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.0, 4.5, Some(0.0), Some(23.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 64, 50, 0, 75, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(5.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(boma, 2, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.5, 4.5, Some(0.0), Some(23.5), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 20.0, 4.5, Some(0.0), Some(24.0), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    FighterSpecializer_Demon::set_devil(boma, false, 0.0);
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 0.6);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 7.75, 12.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        GrabModule::set_constraint(boma, 0, true);
        GrabModule::set_constraint(boma, 1, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 15.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 20.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_GA);
        GrabModule::set_constraint(boma, 0, true);
        GrabModule::set_constraint(boma, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(agent, 0.9);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 21.0, 4.5, Some(0.0), Some(25.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 14.0, 6.0, Some(0.0), Some(18.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 64, 50, 0, 75, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(5.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(boma, 2, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 21.5, 4.5, Some(0.0), Some(25.5), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 14.0, 6.0, Some(0.0), Some(18.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 22.0, 4.5, Some(0.0), Some(26.0), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 0.75);
}

pub fn install() {
    smashline::Agent::new("demon")
        .acmd("game_attackstep", game_attackstep)
        .acmd("game_attackstep2", kazuya_wind_god_fist_game)
        .acmd("game_attackstep2f", kazuya_electric_wind_god_fist_game)
        .acmd("game_attackstep2s", kazuya_spinning_demon_to_left_hook_game)
        .acmd("game_attackstep2l", kazuya_lightning_uppercut_game)
        .acmd("game_specials", kazuya_special_s_game)
        .acmd("game_specialsend", kazuya_special_s_end_game)
        .acmd("game_specialairs", game_specialairs)
        .acmd("game_specialairsend", kazuya_special_air_s_end_game)
        .acmd("game_specialhi", kazuya_special_hi_game)
        .acmd("game_specialhiair", kazuya_special_hi_air_game)
        .acmd("effect_specialhistart", kazuya_special_hi_start_effect)
        .acmd("effect_specialairhistart", kazuya_special_hi_start_effect)
        .acmd("game_attackstep2fhitshield", game_attackstep2fhitshield)
        .acmd("game_speciallw", game_speciallw)
        .acmd("game_specialairlw", game_speciallw)
        .acmd("game_specialairlw", game_specialairlw)
        .acmd("game_specialairlw", game_specialairlw)
        .install();
}
