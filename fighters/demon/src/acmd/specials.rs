
use super::*;


#[acmd_script( agent = "demon", script = "game_attackstep2s" , category = ACMD_GAME , low_priority)]
unsafe fn demon_attack_step_2_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 4.0, 60, 100, 45, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.0, 72, 100, 45, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 4.0, 85, 100, 45, 0, 4.0, 5.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, 12.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 12.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 12.0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.8);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.5, 35, 30, 0, 75, 4.0, 0.0, 15.0, 7.0, None, None, None, 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.5, 35, 30, 0, 75, 5.0, 0.0, 14.0, 8.0, Some(0.0), Some(8.5), Some(8.0), 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 11.5, 35, 30, 0, 75, 5.0, 0.0, 13.0, 3.5, Some(0.0), Some(5.5), Some(3.5), 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("shoulderl"), 11.5, 35, 30, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("shoulderr"), 11.5, 35, 30, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 8.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 8.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 8.0, false);
        AttackModule::set_add_reaction_frame(boma, 3, 8.0, false);
        AttackModule::set_add_reaction_frame(boma, 4, 8.0, false);
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
            ATTACK(fighter, 5, 1, Hash40::new("top"), 0.0, 235, 100, 3, 0, 10.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        }
        
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.6);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        demon_attack_step_2_s_game,
    );
}

