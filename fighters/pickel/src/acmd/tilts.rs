
use super::*;


#[acmd_script( agent = "pickel", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn pickel_attack_s3_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "pickel", script = "effect_attacks3" , category = ACMD_EFFECT , low_priority)]
unsafe fn pickel_attack_s3_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "pickel", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn pickel_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
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
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.775, 78, 78, 0, 56, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 8.775, 78, 78, 0, 56, 4.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.75);
        }
        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 78, 78, 0, 56, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.5, 78, 78, 0, 56, 4.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.75);
        }
        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.5, 78, 78, 0, 56, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.5, 78, 78, 0, 56, 4.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.75);
        }
        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.8, 78, 78, 0, 56, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.8, 78, 78, 0, 56, 4.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.75);
        }
        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 78, 78, 0, 56, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.5, 78, 78, 0, 56, 4.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.75);
        }
        // Punch
        else{
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.2, 78, 78, 0, 56, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.6);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter){
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
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

#[acmd_script( agent = "pickel", script = "effect_attackhi3" , category = ACMD_EFFECT , low_priority)]
unsafe fn pickel_attack_hi3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_axe_flare_diamond"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 0.9, false);
        }
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_axe_flare_gold"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 0.9, false);
        }
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_axe_flare_iron"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 0.9, false);
        }
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_axe_flare_stone"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 0.9, false);
        }
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_axe_flare_wood"), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 0.9, false);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_axe_diamond"), Hash40::new("pickel_atk_axe_diamond"), Hash40::new("top"), -3.2, 11.5, -1.89, -80, -80, -27, 0.9, false, *EF_FLIP_YZ);
        }
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_axe_gold"), Hash40::new("pickel_atk_axe_gold"), Hash40::new("top"), -3.2, 11.5, -1.89, -80, -80, -27, 0.9, false, *EF_FLIP_YZ);
        }
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_axe_iron"), Hash40::new("pickel_atk_axe_iron"), Hash40::new("top"), -3.2, 11.5, -1.89, -80, -80, -27, 0.9, false, *EF_FLIP_YZ);
        }
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_axe_stone"), Hash40::new("pickel_atk_axe_stone"), Hash40::new("top"), -3.2, 11.5, -1.89, -80, -80, -27, 0.9, false, *EF_FLIP_YZ);
        }
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pickel_atk_axe_wood"), Hash40::new("pickel_atk_axe_wood"), Hash40::new("top"), -3.2, 11.5, -1.89, -80, -80, -27, 0.9, false, *EF_FLIP_YZ);
        }
        else {
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 11, -3, -20, -60, -70, 0.7, false, *EF_FLIP_YZ, 0.05);
            LAST_EFFECT_SET_COLOR(fighter, 1, 1, 1);
        }
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_axe_flare_diamond"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_axe_flare_gold"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_axe_flare_iron"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_axe_flare_stone"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_axe_flare_wood"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), false, true);
    }
}

#[acmd_script( agent = "pickel", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn pickel_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    }
    FT_MOTION_RATE(fighter, 8.0);
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK){
            VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        }
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 12.0/(3.0-2.0));
        }
        else{
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma,  *FIGHTER_PICKEL_GENERATE_ARTICLE_FIRE){
            ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FIRE, false, 0);
        }
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 1.0);
            //FT_MOTION_RATE(fighter, 25.0/(30.0-5.0));
        }
        else{
            FT_MOTION_RATE(fighter, 15.0/(30.0-5.0));
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            //FT_MOTION_RATE(fighter, 2.0);
            FT_MOTION_RATE(fighter, 0.5);
        }
        else{
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
}

#[acmd_script( agent = "pickel", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn pickel_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT(fighter, Hash40::new("pickel_flint"), Hash40::new("haver"), 1, 7.2, 1, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.075, false);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, false);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
        }
        else{
            EFFECT(fighter, Hash40::new("pickel_flint"), Hash40::new("haver"), 1, 6.2, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        else{
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("sys_damage_aura"), -1);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_aura"), false, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        //pickel_attack_s3_game,
        //pickel_attack_s3_effect,
        pickel_attack_hi3_game,
        pickel_attack_hi3_effect,
        pickel_attack_lw3_game,
        pickel_attack_lw3_effect,
    );
}

