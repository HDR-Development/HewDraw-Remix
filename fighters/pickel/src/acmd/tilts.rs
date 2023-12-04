
use super::*;

// this script is used for jab, forward tilt, and neutral air
#[acmd_script( agent = "pickel", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn game_attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
    let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
    let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
    let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
    let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 2.0);
    if material_kind == gold {
        if is_excute(fighter) {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            // sword hitboxes
            let mut damage = 3.4; // default damage, used for wood and gold
            let mut hitstun: [f32; 2] = [0.0, 0.0]; // hitstun modifier
            if material_kind == wood {
                hitstun = [-7.0, -5.0];  
            } else if material_kind == stone {
                damage = 3.74; // damage for stone
                hitstun = [-5.0, -3.0];
            } else if material_kind == iron {
                damage = 4.0; // damage for iron
                hitstun = [-3.0, -1.0];
            } else if material_kind == gold {
                hitstun = [-4.0, -2.0];
            } else {
                damage = 4.5; // damage for diamond
            }
            // air hitboxes
            ATTACK(fighter, 0, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 3.1, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 6.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("top"), damage, 45, 134, 0, 27, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            // ground hitboxes
            if boma.is_situation(*SITUATION_KIND_GROUND) 
            && (-0.1..0.1).contains(&fighter.left_stick_x()) { // locking hitbox on jab
                ATTACK(fighter, 4, 0, Hash40::new("top"), (damage / 2.0), 361, 30, 0, 20, 2.5, 0.0, 3.3, 5.4, Some(0.0), Some(3.3), Some(11.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                AttackModule::set_down_only(boma, 4, false);
            } else { // non-locking hitbox on ftilt/neutral air
                ATTACK(fighter, 4, 0, Hash40::new("top"), damage, 57, 134, 0, 20, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            }
            ATTACK(fighter, 5, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 6, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 3.1, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 7, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 6.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
            AttackModule::set_add_reaction_frame_revised(boma, 0, hitstun[0], false);
            AttackModule::set_add_reaction_frame_revised(boma, 1, hitstun[0], false);
            AttackModule::set_add_reaction_frame_revised(boma, 2, hitstun[0], false);
            AttackModule::set_add_reaction_frame_revised(boma, 3, hitstun[0], false);
            AttackModule::set_add_reaction_frame_revised(boma, 4, hitstun[1], false);
            AttackModule::set_add_reaction_frame_revised(boma, 5, hitstun[1], false);
            AttackModule::set_add_reaction_frame_revised(boma, 6, hitstun[1], false);
            AttackModule::set_add_reaction_frame_revised(boma, 7, hitstun[1], false);
            WorkModule::set_float(boma, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        } else {
            // fist hitboxes
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.72, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            if boma.is_situation(*SITUATION_KIND_GROUND) { // ground-only locking hitbox
                ATTACK(fighter, 7, 0, Hash40::new("top"), 1.36, 361, 30, 0, 20, 2.5, 0.0, 3.3, 5.4, Some(0.0), Some(3.3), Some(11.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
            AttackModule::set_add_reaction_frame_revised(boma, 0, -10.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 7, -10.0, false);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if material_kind == gold {
        if is_excute(fighter) {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.25);
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        } 
    }
}

#[acmd_script( agent = "pickel", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn game_attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
    let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
    let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
    let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
    let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if material_kind != gold {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.8);
        } else {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            let mut damage = 5.5; // default damage, used for wood and gold
            if material_kind == stone {
                damage = 6.8; // damage for stone
            } else if material_kind == iron {
                damage = 7.5; // damage for iron
            } else if material_kind == diamond {
                damage = 8.7; // damage for diamond
            }
            WorkModule::set_float(boma, 5.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), damage, 78, 78, 0, 56, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), damage, 78, 78, 0, 56, 4.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        } else {
            // fist hitbox
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 4.2, 78, 78, 0, 56, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if material_kind != gold {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.5);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
}

#[acmd_script( agent = "pickel", script = "effect_attackhi3" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
    let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
    let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
    let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
    let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
    frame(lua_state, 5.0);
    material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if [wood, stone, iron, gold, diamond].contains(&material_kind) {
        let mut effect: [&str;2] = ["pickel_axe_flare_wood", "pickel_atk_axe_wood"]; // default effects for wood
        if material_kind == stone {
            effect = ["pickel_axe_flare_stone", "pickel_atk_axe_stone"];
        } else if material_kind == iron {
            effect = ["pickel_axe_flare_iron", "pickel_atk_axe_iron"];
        } else if material_kind == gold {
            effect = ["pickel_axe_flare_gold", "pickel_atk_axe_gold"];
        } else if material_kind == diamond {
            effect = ["pickel_axe_flare_diamond", "pickel_atk_axe_diamond"];
        }
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new(effect[0]), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 0.9, false);
        }
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new(effect[1]), Hash40::new(effect[1]), Hash40::new("top"), -3.2, 11.5, -1.89, -80, -80, -27, 0.9, false, *EF_FLIP_YZ);
        }
    } else {
        frame(lua_state, 6.0);
        // fist effect
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 11, -3, -20, -60, -70, 0.7, false, *EF_FLIP_YZ, 0.05);
            LAST_EFFECT_SET_COLOR(fighter, 1, 1, 1); 
        }
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

#[acmd_script( agent = "pickel", script = "expression_attackhi3", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_77_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        //let mut rotation_vec = Vector3f{ x:0.0, y: 0.0, z: -8.0 };
        //let rotation_vec_ptr: *mut Vector3f = &mut rotation_vec;
		//ModelModule::joint_rotate(boma, Hash40::new("haver"), rotation_vec_ptr);
        WorkModule::set_float(boma, 5.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        // Diamond
        if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            RUMBLE_HIT(fighter, Hash40::new("rbkind_77_attackm"), 0);
        }
        // Gold
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            RUMBLE_HIT(fighter, Hash40::new("rbkind_77_attacks"), 9);
        }
        // Iron
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            RUMBLE_HIT(fighter, Hash40::new("rbkind_77_attackm"), 6);
        }
        // Stone
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            RUMBLE_HIT(fighter, Hash40::new("rbkind_77_attacks"), 0);
        }
        // Wood
        else if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) ==  *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            RUMBLE_HIT(fighter, Hash40::new("rbkind_77_attacks"), 8);
        }
        // Punch
        else {
            RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 8);
        }
    }
}

#[acmd_script( agent = "pickel", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacklw3(fighter: &mut L2CAgentBase) {
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
        } else {
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
        } else {
            FT_MOTION_RATE(fighter, 15.0/(30.0-5.0));
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            //FT_MOTION_RATE(fighter, 2.0);
            FT_MOTION_RATE(fighter, 0.5);
        } else {
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
}

#[acmd_script( agent = "pickel", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT(fighter, Hash40::new("pickel_flint"), Hash40::new("haver"), 1, 7.2, 1, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.075, false);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, false);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
        } else {
            EFFECT(fighter, Hash40::new("pickel_flint"), Hash40::new("haver"), 1, 6.2, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        } else {
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

#[acmd_script( agent = "pickel_fire", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn fire_game_attacklw3(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.boma();
    let pickel_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let pickel = pickel_boma.object();
    if is_excute(weapon) {
        AttackModule::disable_tip(boma);
        VarModule::off_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE);
        if VarModule::is_flag(pickel, vars::common::instance::IS_HEAVY_ATTACK){
            VarModule::on_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE);
        } else {
            VarModule::off_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE);
        }
        if VarModule::is_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE) {
            //FT_MOTION_RATE(weapon, 1.25);
            ATTACK(weapon, 0, 0, Hash40::new("top"), 0.8, 366, 100, 40, 0, 3.2, 0.0, 2.8, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            ATTACK(weapon, 1, 0, Hash40::new("top"), 0.8, 366, 100, 40, 0, 3.2, 0.0, 2.8, -2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        } else {
            FT_MOTION_RATE(weapon, 0.75);
            ATTACK(weapon, 0, 0, Hash40::new("top"), 7.0, 70, 60, 0, 75, 3.2, 0.0, 2.8, 2.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            ATTACK(weapon, 1, 0, Hash40::new("top"), 7.0, 70, 60, 0, 75, 3.2, 0.0, 2.8, -2.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(weapon) {
        if VarModule::is_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE){
            FT_MOTION_RATE(weapon, 1.0);
            ATTACK(weapon, 0, 1, Hash40::new("top"), 6.4, 54, 116, 0, 42, 3.5, 0.0, 2.8, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            ATTACK(weapon, 1, 1, Hash40::new("top"), 6.4, 54, 116, 0, 42, 3.5, 0.0, 2.8, -2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(weapon) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 45.0);
    if is_excute(weapon) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

#[acmd_script( agent = "pickel_fire", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn fire_effect_attacklw3(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.boma();
    let pickel_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let pickel = pickel_boma.object();
    if is_excute(weapon) {
        if VarModule::is_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE){
            EFFECT_FOLLOW(weapon, Hash40::new("pickel_fire_soot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW(weapon, Hash40::new("pickel_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            LAST_EFFECT_SET_COLOR(weapon, 0.137, 0.85, 0.85);
        } else {
            EFFECT_FOLLOW(weapon, Hash40::new("pickel_fire_soot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW(weapon, Hash40::new("pickel_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            LAST_EFFECT_SET_COLOR(weapon, 1.0, 0.467, 0.0);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(weapon) {
        EFFECT_DETACH_KIND(weapon, Hash40::new("pickel_fire_soot"), -1);
    }
    frame(lua_state, 38.0);
    if is_excute(weapon) {
        EFFECT_DETACH_KIND(weapon, Hash40::new("pickel_fire"), -1);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_attacks3,
        game_attackhi3,
        effect_attackhi3,
        expression_attackhi3,
        game_attacklw3,
        effect_attacklw3,
        fire_game_attacklw3,
        fire_effect_attacklw3
    );
}