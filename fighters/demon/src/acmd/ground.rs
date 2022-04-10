
use super::*;

#[acmd_script( agent = "demon", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn game_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::demon::DEVASTATOR){
            MotionModule::change_motion(boma, Hash40::new("flash_punch"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 10, 0, 30, 2.0, 0.0, 14.5, 9.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 10, 0, 30, 3.2, 0.0, 13.5, 8.75, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 10, 0, 30, 3.2, 0.0, 13.5, 3.25, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 361, 10, 0, 30, 3.2, 0.0, 11.0, 8.75, Some(0.0), Some(13.5), Some(8.75), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 3.0, 361, 10, 0, 30, 3.2, 0.0, 11.0, 3.25, Some(0.0), Some(13.5), Some(3.25), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 3.0, 33, 15, 0, 35, 3.6, 0.0, 13.2, 3.25, Some(0.0), Some(13.2), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 4, 8.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.3);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.3);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.3);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
    
}

#[acmd_script( agent = "demon", script = "game_flashpunch" , category = ACMD_GAME , low_priority)]
unsafe fn game_flashpunch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::demon::DEVASTATOR){
            ATTACK(fighter, 0, 1, Hash40::new("top"), 0.0, 368, 100, 40, 0, 7.0, 0.0, 15.0, 5.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(fighter, 1, 1, Hash40::new("top"), 0.0, 368, 100, 40, 0, 7.0, 0.0, 15.0, 0.0, Some(0.0), Some(7.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_vec_target_pos(boma, 0, smash::phx::Hash40::new("top"), &Vector2f::new(5.0, 15.0), 5, false);
            AttackModule::set_vec_target_pos(boma, 1, smash::phx::Hash40::new("top"), &Vector2f::new(5.0, 15.0), 5, false);
        }
        else{
            ATTACK(fighter, 0, 1, Hash40::new("top"), 0.0, 361, 100, 35, 0, 4.0, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::demon::DEVASTATOR){
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 40, 40, 0, 76, 2.0, 0.0, 9.5, 9.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 40, 40, 0, 76, 4.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(8.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 40, 40, 0, 76, 4.0, 0.0, 10.5, 4.0, Some(0.0), Some(10.5), Some(8.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 43, 70, 0, 76, 2.0, 0.0, 9.5, 9.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 43, 70, 0, 76, 4.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(8.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 43, 70, 0, 76, 4.0, 0.0, 10.5, 4.0, Some(0.0), Some(10.5), Some(8.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        }
        
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "demon", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn game_attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        //JostleModule::set_push_speed_x_overlap_rate_status(boma, 0.05);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 14.0, 44, 57, 0, 65, 3.5, 0.0, 0.0, -1.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 44, 57, 0, 65, 2.5, 0.0, 12.0, 6.0, Some(0.0), Some(2.5), Some(1.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 16.0, 44, 62, 0, 65, 2.5, 0.5, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("footl"), 16.0, 44, 62, 0, 65, 4.5, 0.5, -1.0, -1.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        //ATTACK(fighter, 4, 0, Hash40::new("top"), 14.0, 44, 57, 0, 65, 2.6, 1.0, 2.6, 1.8, Some(-1.0), Some(2.6), Some(1.8), 0.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 5, 0, Hash40::new("hip"), 14.0, 44, 57, 0, 65, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        //AttackModule::set_down_only(boma, 4, true);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
        //AttackModule::clear(boma, 4);
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 10.0, 94, 70, 0, 84, 3.5, 0.0, 0.0, -1.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 10.0, 94, 70, 0, 84, 2.5, 0.5, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("footl"), 10.0, 94, 70, 0, 84, 4.5, 0.5, -1.0, -1.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 5, 0, Hash40::new("hip"), 10.0, 94, 70, 0, 84, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        //JostleModule::set_push_speed_x_overlap_rate_status(boma, 0);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "demon", script = "game_attackstand1" , category = ACMD_GAME , low_priority)]
unsafe fn kazuya_left_splits_kick_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_DEMON_REFLECTOR_KIND_ATTACK_STAND1, *FIGHTER_DEMON_REFLECTOR_GROUP_ATTACK_STAND1);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(1.0, 0.0, 0.0));
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.2, 0.0, 0.0));
        // Ground-only
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 13.0, 275, 40, 0, 95, 3.8, 6.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 13.0, 275, 40, 0, 95, 3.8, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("legl"), 13.0, 275, 40, 0, 95, 4.2, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        // Air-only
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 13.0, 84, 40, 0, 78, 3.8, 6.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 4, 0, Hash40::new("kneel"), 13.0, 84, 40, 0, 78, 3.8, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 5, 0, Hash40::new("legl"), 13.0, 84, 40, 0, 78, 4.2, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 4, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 5, 10.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 5, 1.1);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.2, 0.0, 0.0));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.2, 0.0, 0.0));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.2, 0.0, 0.0));
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.2, 0.0, 0.0));
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_DEMON_REFLECTOR_KIND_ATTACK_STAND1, *FIGHTER_DEMON_REFLECTOR_GROUP_ATTACK_STAND1);
    }
    
}

#[acmd_script( agent = "demon", script = "game_attackstand21" , category = ACMD_GAME , low_priority)]
unsafe fn kazuya_triple_spin_kicks_1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.5);
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 0, 50, 60, 0, 4.0, 0.0, 11.0, 10.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 0, 50, 60, 0, 4.0, 0.0, 12.0, 2.0, Some(0.0), Some(11.5), Some(7.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 10, 40, 100, 0, 4.0, 0.0, 14.0, 3.0, Some(0.0), Some(13.0), Some(9.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 10.0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
        
        
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::demon::LIGHTNING_SCREW_UPPERCUT){
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP);
        }
        
    }
}

#[acmd_script( agent = "demon", script = "game_attackstand22" , category = ACMD_GAME , low_priority)]
unsafe fn kazuya_triple_spin_kicks_2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::demon::LIGHTNING_SCREW_UPPERCUT){
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::demon::LIGHTNING_SCREW_UPPERCUT){
            KineticModule::add_speed(boma, &Vector3f::new(-0.1, 0.0, 0.0));
            //FT_MOTION_RATE(fighter, 9.0/(15.0-2.0));
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::demon::LIGHTNING_SCREW_UPPERCUT){
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 65, 100, 40, 0, 2.0, 0.0, 4.0, 9.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 65, 100, 40, 0, 4.0, 0.0, 5.25, 3.0, Some(0.0), Some(4.0), Some(7.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 15, 100, 30, 0, 4.0, 0.0, 8.5, 3.0, Some(0.0), Some(7.5), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2.0);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2.0);
        }
        
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::demon::LIGHTNING_SCREW_UPPERCUT){
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 65, 100, 40, 0, 2.0, 0.0, 4.5, 12.0, Some(0.0), Some(4.4), Some(12.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 65, 100, 40, 0, 4.0, 0.0, 5.75, 3.0, Some(0.0), Some(4.2), Some(11.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 15, 100, 30, 0, 4.0, 0.0, 8.0, 3.0, Some(0.0), Some(7.0), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2.0);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2.0);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::demon::LIGHTNING_SCREW_UPPERCUT){
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP);
        }
    }
}

#[acmd_script( agent = "demon", script = "game_attackstand5" , category = ACMD_GAME , low_priority)]
unsafe fn kazuya_flash_tornado_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::demon::SLAUGHTER_HIGH_KICK){
            ATTACK(fighter, 0, 0, Hash40::new("legr"), 8.0, 62, 65, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("kneer"), 8.0, 62, 65, 0, 80, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("kneer"), 8.0, 62, 65, 0, 80, 3.5, 6.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 62, 65, 0, 80, 5.2, 0.0, 13.5, 5.5, Some(0.0), Some(16.72), Some(10.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);    
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("legr"), 11.0, 46, 70, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("legr"), 11.0, 46, 70, 0, 60, 3.0, 2.0, 0.0, 0.0, None, None, None, 0.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("kneer"), 13.0, 46, 80, 0, 55, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 3, 0, Hash40::new("kneer"), 13.0, 46, 80, 0, 55, 3.5, 6.0, 0.0, 0.0, None, None, None, 0.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 4, 0, Hash40::new("top"), 11.0, 46, 70, 0, 60, 4.0, 0.0, 9.5, 4.5, None, None, None, 0.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 5, 0, Hash40::new("top"), 13.0, 46, 80, 0, 55, 5.2, 0.0, 10.5, 5.5, Some(0.0), Some(13.72), Some(10.5), 0.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        }
        
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "demon", script = "game_attacksquat4" , category = ACMD_GAME , low_priority)]
unsafe fn kazuya_demon_god_fist_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);

        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.0, 361, 0, 0, 0, 6.0, 0.0, 6.25, 13.0, Some(0.0), Some(6.25), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        //ATTACK(fighter, 1, 1, Hash40::new("top"), 0.0, 361, 0, 0, 0, 6.0, 0.0, 6.25, 13.0, Some(0.0), Some(6.25), Some(18.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD){
            KineticModule::add_speed(boma, &Vector3f::new(1.0, 0.0, 0.0));
        }
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD){
            KineticModule::add_speed(boma, &Vector3f::new(-1.0, 0.0, 0.0));
        }
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 50, 0, 60, 2.0, 0.0, 11.25, 9.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x19f2214801), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 50, 0, 60, 4.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(9.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x19f2214801), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 361, 60, 0, 40, 4.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(9.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
}


pub fn install() {
    install_acmd_scripts!(
        game_attack11,
        game_flashpunch,
        game_attackdash,
        kazuya_left_splits_kick_game,
        kazuya_triple_spin_kicks_1_game,
        kazuya_triple_spin_kicks_2_game,
        kazuya_flash_tornado_game,
        kazuya_demon_god_fist_game,
    );
}

