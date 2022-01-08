
use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::utility::*;
use smash_script::*;
use smashline::*;
use smashline::*;
use smash_script::macros::*;
use crate::utils::hdr;
use crate::vars::*;


#[acmd_script( agent = "pickel", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn pickel_attack_s3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_SITUATION_KIND) == *SITUATION_KIND_GROUND {
            WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_SITUATION_KIND) != *SITUATION_KIND_GROUND {
            WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }

        WorkModule::set_float(boma, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY); // Subtract 2 points of durability per attack

        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND { //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.75, 48, 128, 0, 27, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.75, 48, 128, 0, 27, 3.0, 0.0, 5.1, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.75, 48, 128, 0, 27, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            // Copy of 2
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 4.75, 48, 128, 0, 27, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);

            ATTACK(fighter, 4, 0, Hash40::new("haver"), 4.75, 60, 128, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 5, 0, Hash40::new("haver"), 4.75, 60, 128, 0, 20, 3.0, 0.0, 5.1, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 6, 0, Hash40::new("haver"), 4.75, 60, 128, 0, 20, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            // Copy of 6
            ATTACK(fighter, 7, 0, Hash40::new("haver"), 4.75, 60, 128, 0, 20, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.75);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.75);
        }

        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {  //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.75, 48, 128, 0, 27, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.75, 48, 128, 0, 27, 3.0, 0.0, 5.1, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.75, 48, 128, 0, 27, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            // Copy of 2
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 3.75, 48, 128, 0, 27, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);

            ATTACK(fighter, 4, 0, Hash40::new("haver"), 3.75, 60, 128, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 5, 0, Hash40::new("haver"), 3.75, 60, 128, 0, 20, 3.0, 0.0, 5.1, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 6, 0, Hash40::new("haver"), 3.75, 60, 128, 0, 20, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            // Copy of 6
            ATTACK(fighter, 7, 0, Hash40::new("haver"), 3.75, 60, 128, 0, 20, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.75);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.75);
        }

        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {  //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.25, 48, 128, 0, 27, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.25, 48, 128, 0, 27, 3.0, 0.0, 5.1, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.25, 48, 128, 0, 27, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            // Copy of 2
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 4.25, 48, 128, 0, 27, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);

            ATTACK(fighter, 4, 0, Hash40::new("haver"), 4.25, 60, 128, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 5, 0, Hash40::new("haver"), 4.25, 60, 128, 0, 20, 3.0, 0.0, 5.1, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 6, 0, Hash40::new("haver"), 4.25, 60, 128, 0, 20, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            // Copy of 6
            ATTACK(fighter, 7, 0, Hash40::new("haver"), 4.25, 60, 128, 0, 20, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.75);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.75);
        }

        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {  //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.0, 48, 128, 0, 27, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.0, 48, 128, 0, 27, 3.0, 0.0, 5.1, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.0, 48, 128, 0, 27, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            // Copy of 2
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 4.0, 48, 128, 0, 27, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            
            ATTACK(fighter, 4, 0, Hash40::new("haver"), 4.0, 60, 128, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 5, 0, Hash40::new("haver"), 4.0, 60, 128, 0, 20, 3.0, 0.0, 5.1, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 6, 0, Hash40::new("haver"), 4.0, 60, 128, 0, 20, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            // Copy of 6
            ATTACK(fighter, 7, 0, Hash40::new("haver"), 4.0, 60, 128, 0, 20, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.75);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.75);
        }

        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {  //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.75, 48, 128, 0, 27, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.75, 48, 128, 0, 27, 3.0, 0.0, 5.1, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.75, 48, 128, 0, 27, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            // Copy of 2
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 3.75, 48, 128, 0, 27, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            
            ATTACK(fighter, 4, 0, Hash40::new("haver"), 3.75, 60, 128, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 5, 0, Hash40::new("haver"), 3.75, 60, 128, 0, 20, 3.0, 0.0, 5.1, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 6, 0, Hash40::new("haver"), 3.75, 60, 128, 0, 20, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            // Copy of 6
            ATTACK(fighter, 7, 0, Hash40::new("haver"), 3.75, 60, 128, 0, 20, 3.0, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.75);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.75);
        }

        // Punch
        else{
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.75, 60, 50, 0, 72, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 2.75, 60, 50, 0, 72, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {  // if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            // Capsule Hitboxes
            ATTACK(fighter, 3, 0, Hash40::new("top"), 4.75, 48, 128, 0, 27, 3.0, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(14.2), 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 7, 0, Hash40::new("top"),  4.75, 60, 128, 0, 20, 3.0, 0.0, 3.8, 5.4, Some(0.0), Some(3.8), Some(14.2), 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        }

        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {  // if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            // Capsule Hitboxes
            ATTACK(fighter, 3, 0, Hash40::new("top"), 3.75, 48, 128, 0, 27, 3.0, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(14.2), 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 7, 0, Hash40::new("top"), 3.75, 60, 128, 0, 20, 3.0, 0.0, 3.8, 5.4, Some(0.0), Some(3.8), Some(14.2), 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        }

        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {  //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            // Capsule Hitboxes
            ATTACK(fighter, 3, 0, Hash40::new("top"), 4.25, 48, 128, 0, 27, 3.0, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(14.2), 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 7, 0, Hash40::new("top"), 4.25, 60, 128, 0, 20, 3.0, 0.0, 3.8, 5.4, Some(0.0), Some(3.8), Some(14.2), 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        }

        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {  //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            // Capsule Hitboxes
            ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 48, 128, 0, 27, 3.0, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(14.2), 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 7, 0, Hash40::new("top"), 4.0, 60, 128, 0, 20, 3.0, 0.0, 3.8, 5.4, Some(0.0), Some(3.8), Some(14.2), 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        }

        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {  //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            // Capsule Hitboxes
            ATTACK(fighter, 3, 0, Hash40::new("top"), 3.75, 48, 128, 0, 27, 3.0, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(14.2), 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 7, 0, Hash40::new("top"), 3.75, 60, 128, 0, 20, 3.0, 0.0, 3.8, 5.4, Some(0.0), Some(3.8), Some(14.2), 1.0, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.25);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
}

#[acmd_script( agent = "pickel", script = "effect_attacks3" , category = ACMD_EFFECT , low_priority)]
unsafe fn pickel_attack_s3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {  //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_sword_flare_diamond"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.4, true);
        }
        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {  //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_sword_flare_gold"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.4, true);
        }

        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON { // if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_sword_flare_iron"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.4, true);
        }

        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {  //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_sword_flare_stone"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.4, true);
        }

        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {  //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_sword_flare_wood"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.4, true);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND { //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atks3_slash_diamond"), Hash40::new("pickel_atks3_slash_diamond"), Hash40::new("top"), -3, -3.6, 3.7, -18.2, 0, -90, 1.4, true, *EF_FLIP_YZ);
        }

        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD { //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atks3_slash_gold"), Hash40::new("pickel_atks3_slash_gold"), Hash40::new("top"), -3, -3.6, 3.7, -18.2, 0, -90, 1.4, true, *EF_FLIP_YZ);
        }

        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON { //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atks3_slash_iron"), Hash40::new("pickel_atks3_slash_iron"), Hash40::new("top"), -3, -3.6, 3.7, -18.2, 0, -90, 1.4, true, *EF_FLIP_YZ);
        }

        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE { //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atks3_slash_stone"), Hash40::new("pickel_atks3_slash_stone"), Hash40::new("top"), -3, -3.6, 3.7, -18.2, 0, -90, 1.4, true, *EF_FLIP_YZ);
        }

        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD { //if ArticleModule::get_int(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_SWORD, *WEAPON_PICKEL_SWORD_INSTANCE_WORK_ID_INT_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atks3_slash_wood"), Hash40::new("pickel_atks3_slash_wood"), Hash40::new("top"), -3, -3.6, 3.7, -18.2, 0, -90, 1.4, true, *EF_FLIP_YZ);
        }
    }
}

#[acmd_script( agent = "pickel", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn pickel_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
        else{
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.5);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        //let mut rotation_vec = Vector3f{ x:0.0, y: 0.0, z: -8.0 };
        //let rotation_vec_ptr: *mut Vector3f = &mut rotation_vec;
		//ModelModule::joint_rotate(boma, Hash40::new("haver"), rotation_vec_ptr);
        WorkModule::set_float(boma, 5.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.775, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 8.775, 86, 78, 0, 42, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 8.775, 86, 78, 0, 42, 4.0, 0.0, 5.5, -7.0, Some(0.0), Some(5.5), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.5, 86, 78, 0, 42, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 6.5, 86, 78, 0, 42, 4.0, 0.0, 5.5, -7.0, Some(0.0), Some(5.5), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.8, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.8, 86, 78, 0, 42, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 7.8, 86, 78, 0, 42, 4.0, 0.0, 5.5, -7.0, Some(0.0), Some(5.5), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.15, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.15, 86, 78, 0, 42, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 7.15, 86, 78, 0, 42, 4.0, 0.0, 5.5, -7.0, Some(0.0), Some(5.5), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.5, 86, 78, 0, 42, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 6.5, 86, 78, 0, 42, 4.0, 0.0, 5.5, -7.0, Some(0.0), Some(5.5), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Punch
        else{
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.2, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.2, 86, 78, 0, 42, 3.0, 0.0, 5.5, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        
    }
    wait(lua_state, 2.0);
    if is_excute(fighter){
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        // Hitbox 2 becomes a copy of hitbox 0
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 8.775, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 6.5, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 7.8, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 7.15, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 6.5, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Punch
        else{
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 5.2, 86, 78, 0, 42, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter){
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.775, 98, 74, 0, 38, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 8.775, 98, 74, 0, 38, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 98, 74, 0, 38, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.5, 98, 74, 0, 38, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.8, 98, 74, 0, 38, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.8, 98, 74, 0, 38, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.15, 98, 74, 0, 38, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.15, 98, 74, 0, 38, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 98, 74, 0, 38, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.5, 98, 74, 0, 38, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Punch
        else{
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.2, 98, 74, 0, 38, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        // Hitbox 2 is a pickup hitbox again
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 8.775, 98, 74, 0, 38, 4.0, 0.0, 5.5, 7.0, Some(0.0), Some(5.5), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 6.5, 98, 74, 0, 38, 4.0, 0.0, 5.5, 7.0, Some(0.0), Some(5.5), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 7.8, 98, 74, 0, 38, 4.0, 0.0, 5.5, 7.0, Some(0.0), Some(5.5), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 7.15, 98, 74, 0, 38, 4.0, 0.0, 5.5, 7.0, Some(0.0), Some(5.5), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 6.5, 98, 74, 0, 38, 4.0, 0.0, 5.5, 7.0, Some(0.0), Some(5.5), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        // Punch
        else{
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.2, 98, 74, 0, 38, 3.0, 0.0, 7.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
        else{
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.5);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
        
}

pub fn install() {
    install_acmd_scripts!(
        //pickel_attack_s3_game,
        //pickel_attack_s3_effect,
        pickel_attack_hi3_game,
    );
}

