use super::*;

#[acmd_script( agent = "demon", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn game_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        JostleModule::set_team(boma, 1);
        WorkModule::set_int(boma, -1, *FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO);
    }
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10.0);
    }
    frame(lua_state, 10.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 7.0);

    frame(lua_state, 11.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    frame(lua_state, 13.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 4.0);
    frame(lua_state, 14.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    frame(lua_state, 21.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 25.0);
    FT_DESIRED_RATE(fighter, 28.0-25.0, 4.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        WorkModule::set_int(boma, 2, *FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 65, 0, 50, 3.0, 0.0, 6.0, 4.0, Some(0.0), Some(13.0), Some(4.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 20.0, 361, 65, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("handl"), 25.0, 361, 75, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 20.0, 361, 65, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);        
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.75);
    }
    frame(lua_state, 28.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if !WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_S4_FLAG_HIT) {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(lua_state, 51.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 4.0);
    frame(lua_state, 53.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    frame(lua_state, 57.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    frame(lua_state, 58.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    frame(lua_state, 59.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 8.0);
    frame(lua_state, 61.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
}

#[acmd_script( agent = "demon", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn game_attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    frame(lua_state, 1.0);
    FT_DESIRED_RATE(fighter, 14.0-1.0, 9.0);
    if is_excute(fighter) {
        JostleModule::set_team(boma, 1);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 9.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6.0);
    }
    frame(lua_state, 12.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 19.0, 90, 62, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 19.0, 90, 62, 0, 65, 4.75, 0.0, 9.0, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 19.0, 90, 62, 0, 65, 3.5, 0.0, 11.0, 3.5, Some(0.0), Some(8.0), Some(2.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 2, false);
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 19.0, 90, 62, 0, 65, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 19.0, 90, 62, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 19.0, 90, 62, 0, 65, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 51.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    frame(lua_state, 53.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 8.0);
    frame(lua_state, 54.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 9.0);
    frame(lua_state, 56.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
}

#[acmd_script( agent = "demon", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        JostleModule::set_team(boma, 1);
    }
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6.0);
    }
    frame(lua_state, 12.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 8.0);
    frame(lua_state, 13.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    frame(lua_state, 14.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    frame(lua_state, 15.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    frame(lua_state, 16.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 277, 10, 0, 100, 4.0, 0.0, 8.2, 11.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 277, 10, 0, 100, 3.0, 0.0, 8.0, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 277, 52, 0, 20, 4.0, 0.0, 8.2, 11.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 16.0, 277, 52, 0, 20, 3.0, 0.0, 8.0, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 5.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 1.2);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 277, 10, 0, 100, 4.0, 0.0, 6.3, 11.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 277, 10, 0, 100, 3.0, 0.0, 6.3, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 277, 52, 0, 20, 4.0, 0.0, 6.3, 11.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 16.0, 277, 52, 0, 20, 3.0, 0.0, 6.3, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 5.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 1.2);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 277, 10, 0, 100, 4.0, 0.0, 2.5, 11.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 277, 10, 0, 100, 3.0, 0.0, 3.0, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 277, 60, 0, 20, 4.0, -5.0, 2.5, 11.5, Some(5.0), Some(2.5), Some(11.5), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 13.0, 277, 60, 0, 20, 3.0, 0.0, 3.0, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 5.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 1.2);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 46.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    frame(lua_state, 47.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    frame(lua_state, 48.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    frame(lua_state, 50.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 8.0);
    frame(lua_state, 52.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, true, 9.0);
    frame(lua_state, 53.0);
    smash::app::FighterSpecializer_Demon::set_devil(boma, false, 0.0);
}

pub fn install() {
    install_acmd_scripts!(
        game_attacks4,
        game_attackhi4,
        game_attacklw4,
);
}

