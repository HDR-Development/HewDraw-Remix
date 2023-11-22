
use super::*;

#[acmd_script( agent = "pickel", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn pickel_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
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

#[acmd_script( agent = "pickel", script = "effect_attackairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn pickel_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
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

#[acmd_script( agent = "pickel", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn pickel_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    // Pick fair
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 8.0 / (7.0 - 1.0));
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 11.0, 361, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 11.0, 361, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 14.0, 361, 69, 0, 56, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 14.0, 361, 69, 0, 56, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 8.0, 361, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 10.0, 361, 69, 0, 56, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 10.0, 361, 69, 0, 56, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 10.0, 361, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 11.875, 361, 69, 0, 56, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 11.875, 361, 69, 0, 56, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 9.0, 361, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.0, 361, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 10.625, 361, 69, 0, 56, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 10.625, 361, 69, 0, 56, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 8.0, 361, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 9.0, 361, 69, 0, 56, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 9.0, 361, 69, 0, 56, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        // Punch
        else {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 8.0, 361, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        // Strong hit starts here
        // Spike hit starts here
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 12.5, 55, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.5, 55, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 16.0, 275, 49, 0, 25, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 16.0, 275, 65, 0, 80, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 10.0, 55, 81, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 55, 81, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.0, 275, 55, 0, 25, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 12.0, 275, 65, 0, 80, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 12.0, 55, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 55, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 13.5, 275, 54, 0, 25, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 13.5, 275, 65, 0, 80, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 11.0, 55, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 11.0, 55, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.25, 275, 56, 0, 25, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 12.25, 275, 65, 0, 80, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 10.0, 55, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 55, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 11.0, 275, 59, 0, 25, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 11.0, 275, 65, 0, 80, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        // Punch
        else {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 9.5, 55, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.5, 55, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }   
}

#[acmd_script( agent = "pickel", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn pickel_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 0.0);
    if is_excute(fighter) {
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_pick_flare_diamond"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_pick_flare_gold"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_pick_flare_iron"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_pick_flare_stone"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_pick_flare_wood"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_pick_diamond"), Hash40::new("pickel_atk_pick_diamond"), Hash40::new("top"), 1, 8.5, 6, -10, -35, -75.7, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_pick_gold"), Hash40::new("pickel_atk_pick_gold"), Hash40::new("top"), 1, 8.5, 6, -10, -35, -75.7, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_pick_iron"), Hash40::new("pickel_atk_pick_iron"), Hash40::new("top"), 1, 8.5, 6, -10, -35, -75.7, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_pick_stone"), Hash40::new("pickel_atk_pick_stone"), Hash40::new("top"), 1, 8.5, 6, -10, -35, -75.7, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_pick_wood"), Hash40::new("pickel_atk_pick_wood"), Hash40::new("top"), 1, 8.5, 6, -10, -35, -75.7, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }       
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_pick_flare_diamond"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_pick_flare_gold"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_pick_flare_iron"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_pick_flare_stone"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_pick_flare_wood"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), true, true);
    }
}

#[acmd_script( agent = "pickel", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn pickel_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_MINI_FRAME) != 0{
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
    }
    else{
        frame(lua_state, 0.0);
        if is_excute(fighter) {
            WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
    }
    wait(lua_state,1.0);
    //Sword hitboxes
    if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND) == *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD{
        //setting up motion rating (vanilla values)
        if WorkModule::get_int(boma , *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD{
            if is_excute(fighter) {
                MotionModule::set_rate(boma, 10.0);
                MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 10.0);
            }
        }
        else{
            if is_excute(fighter) {
                MotionModule::set_rate(boma, 5.0);
                MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 5.0);
            }
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, 1.75);
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.75);
        }
        if WorkModule::get_int(boma , *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND{
            if is_excute(fighter) {
                /*Air Only Hitboxes */
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.5, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.5, 45, 134, 0, 27, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"),4.5, 45, 134, 0, 27, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 3, 0, Hash40::new("top"), 4.5, 45, 134, 0, 27, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                /*Ground only Hitboxes */
                ATTACK(fighter, 4, 0, Hash40::new("haver"), 4.5, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 5, 0, Hash40::new("haver"), 4.5, 57, 134, 0, 20, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 6, 0, Hash40::new("haver"), 4.5, 57, 134, 0, 20, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 7, 0, Hash40::new("top"), 4.5, 57, 134, 0, 20, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);

                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(boma, 0, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 1, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 2, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 3, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 4, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 5, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 6, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 7, -3.0, false);
                WorkModule::set_float(boma, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        }
        if WorkModule::get_int(boma , *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD{
            if is_excute(fighter) {
                /*Air-Only Hitboxes */
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 3, 0, Hash40::new("top"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                /*Ground-only Hitboxes */
                ATTACK(fighter, 4, 0, Hash40::new("haver"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 5, 0, Hash40::new("haver"),3.4, 57, 134, 0, 20, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 6, 0, Hash40::new("haver"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 7, 0, Hash40::new("top"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);

                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(boma, 0, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 1, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 2, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 3, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 4, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 5, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 6, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 7, -3.0, false);
                WorkModule::set_float(boma, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        }
        if WorkModule::get_int(boma , *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON{
            if is_excute(fighter) {
                /* Air-Only Hitboxes */
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.0, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.0, 45, 134, 0, 27, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.0, 45, 134, 0, 27, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 45, 134, 0, 27, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                /* Ground-Only Hitboxes */
                ATTACK(fighter, 4, 0, Hash40::new("haver"), 4.0, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 5, 0, Hash40::new("haver"), 4.0, 57, 134, 0, 20, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 6, 0, Hash40::new("haver"), 4.0, 57, 134, 0, 20, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 7, 0, Hash40::new("top"), 4.0, 57, 134, 0, 20, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);

                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(boma, 0, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 1, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 2, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 3, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 4, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 5, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 6, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 7, -3.0, false);
                WorkModule::set_float(boma, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        }
        if WorkModule::get_int(boma , *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE{
            if is_excute(fighter) {
                /* Air-only Hitboxes */
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.74, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.74, 45, 134, 0, 27, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.74, 45, 134, 0, 27, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 3, 0, Hash40::new("top"), 3.74, 45, 134, 0, 27, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                /* Ground-only Hitboxes */
                ATTACK(fighter, 4, 0, Hash40::new("haver"), 3.74, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 5, 0, Hash40::new("haver"), 3.74, 57, 134, 0, 20, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 6, 0, Hash40::new("haver"), 3.74, 57, 134, 0, 20, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 7, 0, Hash40::new("top"), 3.74, 57, 134, 0, 20, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);

                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(boma, 0, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 1, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 2, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 3, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 4, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 5, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 6, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 7, -3.0, false);
                WorkModule::set_float(boma, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        }
        if WorkModule::get_int(boma , *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD{
            if is_excute(fighter) {
                /* Air-only Hitboxes */
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 3, 0, Hash40::new("top"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                /* Ground-only Hitboxes */
                ATTACK(fighter, 4, 0, Hash40::new("haver"),3.4, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 5, 0, Hash40::new("haver"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 6, 0, Hash40::new("haver"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 7, 0, Hash40::new("top"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);

                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(boma, 0, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 1, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 2, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 3, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 4, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 5, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 6, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 7, -3.0, false);
                WorkModule::set_float(boma, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        }
        else{
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.72, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATTACK(fighter, 4, 0, Hash40::new("haver"), 2.72, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(boma, 0, -10.0, false);
                AttackModule::set_add_reaction_frame_revised(boma, 4, -10.0, false);
            }
        }
        wait(lua_state, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 22.0);
        if is_excute(fighter) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 47.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, 1.0);
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
    }
    //Pick Hitboxes
    else{
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }   
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, 1.75);
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.75);
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND{
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 15.525, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), 15.525, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 15.525, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"), 15.525, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(lua_state, 2.0);
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 17.55, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), 17.55, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 17.55, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"), 17.55, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD{
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 11.5, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"),  11.5, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"),  11.5, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"),  11.5, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
                FT_MOTION_RATE(fighter, 0.8);
            }
            wait(lua_state, 2.0);
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.0, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.0, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 13.0, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"), 13.0, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON{
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.8, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"),  13.8, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"),  13.8, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"),  13.8, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(lua_state, 2.0);
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 15.6, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), 15.6, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 15.6, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"), 15.6, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE{
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.65, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.65, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"),  12.65, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"),  12.65, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(lua_state, 2.0);
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 14.3, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), 14.3, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 14.3, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"), 14.3, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD{
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 11.5, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"),  11.5, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"),  11.5, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"),  11.5, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(lua_state, 2.0);
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.0, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.0, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 13.0, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"), 13.0, 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
        else{
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.2, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.2, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
            wait(lua_state, 2.0);
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.4, 50, 79, 0, 52, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.4, 50, 79, 0, 52, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 22.0);
        if is_excute(fighter) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 47.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, 1.0);
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
    }
}

#[acmd_script( agent = "pickel", script = "effect_attackairb", category = ACMD_EFFECT, low_priority )] 
unsafe fn pickel_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    //Sword effects
    if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_KIND) == *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD{
        frame(lua_state, 11.0);
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND{
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("pickel_sword_flare_diamond"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.25, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_slash_diamond"), Hash40::new("pickel_atk_slash_diamond"), Hash40::new("top"), -1, 12.0, 1.0, -177, 28, -100, 1.25, true, *EF_FLIP_YZ);
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD{
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("pickel_sword_flare_gold"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.25, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_slash_gold"), Hash40::new("pickel_atk_slash_gold"), Hash40::new("top"), -1, 12.0, 1.0, -177, 28, -100, 1.25, true, *EF_FLIP_YZ);
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON{
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("pickel_sword_flare_iron"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.25, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_slash_iron"), Hash40::new("pickel_atk_slash_iron"), Hash40::new("top"), -1, 12.0, 1.0, -177, 28, -100, 1.25, true, *EF_FLIP_YZ);
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE{
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("pickel_sword_flare_stone"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.25, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_slash_stone"), Hash40::new("pickel_atk_slash_stone"), Hash40::new("top"), -1, 12.0, 1.0, -177, 28, -100, 1.25, true, *EF_FLIP_YZ);
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD{
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("pickel_sword_flare_wood"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.25, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_slash_wood"), Hash40::new("pickel_atk_slash_wood"), Hash40::new("top"), -1, 12.0, 1.0, -177, 28, -100, 1.25, true, *EF_FLIP_YZ);
            }
        }
        frame(lua_state, 17.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("pickel_sword_flare_diamond"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("pickel_sword_flare_gold"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("pickel_sword_flare_iron"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("pickel_sword_flare_stone"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("pickel_sword_flare_wood"), false, true);
        }
    }
    //Pickaxe effects                                         
    else{
        frame(lua_state, 0.0);
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND{
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("pickel_pick_flare_diamond"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_pick_diamond"), Hash40::new("pickel_atk_pick_diamond"), Hash40::new("top"), 0, 9.7, -4, 180, 35, -100, 1, true, *EF_FLIP_YZ);
                LAST_EFFECT_SET_RATE(fighter, 0.95);
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD{
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("pickel_pick_flare_gold"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_pick_gold"), Hash40::new("pickel_atk_pick_gold"), Hash40::new("top"), 0, 9.7, -4, 180, 35, -100, 1, true, *EF_FLIP_YZ);
                LAST_EFFECT_SET_RATE(fighter, 0.95);
            }
            if is_excute(fighter) {
                LAST_EFFECT_SET_RATE(fighter, 0.55);
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON{
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("pickel_pick_flare_iron"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_pick_iron"), Hash40::new("pickel_atk_pick_iron"), Hash40::new("top"), 0, 9.7, -4, 180, 35, -100, 1, true, *EF_FLIP_YZ);
                LAST_EFFECT_SET_RATE(fighter, 0.95);
            }
        }           
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE{
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("pickel_pick_flare_stone"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_pick_stone"), Hash40::new("pickel_atk_pick_stone"), Hash40::new("top"), 0, 9.7, -4, 180, 35, -100, 1, true, *EF_FLIP_YZ);
                LAST_EFFECT_SET_RATE(fighter, 0.95);
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD{
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("pickel_pick_flare_wood"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_pick_wood"), Hash40::new("pickel_atk_pick_wood"), Hash40::new("top"), 0, 9.7, -4, 180, 35, -100, 1, true, *EF_FLIP_YZ);
                LAST_EFFECT_SET_RATE(fighter, 0.95);
            }
        }
        else{
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 11.2, -3, 190, 15, -90, 0.6, true, *EF_FLIP_YZ, 0.06);
                LAST_EFFECT_SET_COLOR(fighter, 1, 1, 1);
                LAST_EFFECT_SET_RATE(fighter, 1);
            }
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_pick_flare_diamond"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_pick_flare_gold"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_pick_flare_iron"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_pick_flare_stone"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_pick_flare_wood"), false, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_atk_pick_diamond"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_atk_pick_gold"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_atk_pick_iron"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_atk_pick_stone"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_atk_pick_wood"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), true, true);
    }
}

#[acmd_script( agent = "pickel", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn pickel_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_FORGE_GENERATE_ENABLE);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        let article = ArticleModule::get_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE);
        let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);
        let anvil_pos_y = PostureModule::pos_y(article_boma);
        VarModule::set_float(fighter.battle_object, vars::pickel::instance::FORGE_START_Y_POS, anvil_pos_y);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
        
        
}

#[acmd_script( agent = "pickel", script = "game_attackairlw2" , category = ACMD_GAME , low_priority)]
unsafe fn pickel_attack_air_lw2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        //WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        //WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
        
}
pub fn install() {
    install_acmd_scripts!(
        //pickel_attack_air_n_game,
        //pickel_attack_air_n_effect,
        pickel_attack_air_f_game,
        pickel_attack_air_f_effect,
        pickel_attack_air_lw_game,
        pickel_attack_air_lw2_game,
        pickel_attack_air_b_effect,
        pickel_attack_air_b_game,
    );
}

