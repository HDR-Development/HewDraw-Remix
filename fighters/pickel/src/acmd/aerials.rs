
use super::*;

// Note: Neutral air is handled in tilts.rs, as it shares a script with forward tilt/jab

#[acmd_script( agent = "pickel", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn game_attackairf(fighter: &mut L2CAgentBase) {
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
    if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_MINI_FRAME) != 0 {
        // sword hitboxes
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        wait(lua_state, 1.0);
        if material_kind == gold {
            if is_excute(fighter) {
                MotionModule::set_rate(boma, 6.0);
                MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 6.0);
            }
        } else {
            if is_excute(fighter) {
                MotionModule::set_rate(boma, 3.0);
                MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 3.0);
            }
        }
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
            MotionModule::set_rate(boma, 1.5);
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.5);
            if [wood, stone, iron, gold, diamond].contains(&material_kind) {
                let mut damage = 3.4; // default damage, used for wood and gold
                if material_kind == stone {
                    damage = 3.74; // damage for stone
                } else if material_kind == iron {
                    damage = 4.0; // damage for iron
                } else if material_kind == diamond {
                    damage = 4.5; // damage for diamond
                }
                let mut sfx = *COLLISION_SOUND_ATTR_PUNCH; // collision sound
                if [iron, gold, diamond].contains(&material_kind) {
                    sfx = *COLLISION_SOUND_ATTR_CUTUP;
                }
                // air hitboxes
                ATTACK(fighter, 0, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"),damage, 45, 134, 0, 27, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 3, 0, Hash40::new("top"), damage, 45, 134, 0, 27, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                // ground hitboxes
                ATTACK(fighter, 4, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 5, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 6, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 7, 0, Hash40::new("top"), damage, 57, 134, 0, 20, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
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
            } else {
                // fist hitbox
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.72, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                AttackModule::set_add_reaction_frame_revised(boma, 0, -10.0, false);
            }
            wait(lua_state, 5.0);
            if is_excute(fighter) {
                AttackModule::clear_all(boma);
            }
            frame(lua_state, 19.0);
            if is_excute(fighter) {
                if material_kind == gold {
                    MotionModule::set_rate(boma, 3.625);
                    MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 3.625);
                } else {
                    MotionModule::set_rate(boma, 3.23);
                    MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 3.23);
                }
            }
        }
    } else {
    //pickaxe hitboxes
        frame(lua_state, 0.0);
        if is_excute(fighter) {
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
        material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            let mut damage: [f32;4] = [8.0, 9.0, 10.0, 11.0]; // default damage, used for wood and gold
            if material_kind == stone {
                damage = [9.0, 10.625, 11.0, 12.25]; // damage for stone
            } else if material_kind == iron {
                damage = [10.0, 11.875, 12.0, 13.5]; // damage for iron
            } else if material_kind == diamond {
                damage = [11.0, 14.0, 12.5, 15.0]; // damage for diamond
            }
            if is_excute(fighter) {
                FT_MOTION_RATE(fighter, 1.0);
                ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), damage[0], 361, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), damage[0], 361, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), damage[1], 361, 69, 0, 56, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"), damage[1], 361, 69, 0, 56, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), damage[2], 55, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), damage[2], 55, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), damage[3], 275, 55, 0, 25, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"), damage[3], 275, 65, 0, 80, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.5, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        } else {
            // fist hitboxes
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 8.0, 361, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 9.5, 55, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.5, 55, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            if material_kind == gold {
                FT_MOTION_RATE(fighter, 0.8);
            }
        }
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
unsafe fn effect_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
    let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
    let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
    let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
    let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
    // sword effects
    if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_KIND) == *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD {
        frame(lua_state, 7.0);
        material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            let mut effect: [&str;2] = ["pickel_sword_flare_wood", "pickel_atk_slash_wood"]; // default effects for wood
            if material_kind == stone {
                effect = ["pickel_sword_flare_stone", "pickel_atk_slash_stone"];
            } else if material_kind == iron {
                effect = ["pickel_sword_flare_iron", "pickel_atk_slash_iron"];
            } else if material_kind == gold {
                effect = ["pickel_sword_flare_gold", "pickel_atk_slash_gold"];
            } else if material_kind == diamond {
                effect = ["pickel_sword_flare_diamond", "pickel_atk_slash_diamond"];
            }
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new(effect[0]), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.25, true);
            }
            frame(lua_state, 8.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new(effect[1]), Hash40::new(effect[1]), Hash40::new("top"), 1.85, 6.5, 0.6, -13, -33, -83, 1.15, true, *EF_FLIP_YZ);
            }
            frame(lua_state, 13.0);
            if is_excute(fighter) {
                EFFECT_OFF_KIND(fighter, Hash40::new(effect[0]), false, true);
            }
        } else {
            // fist effect
            frame(lua_state, 8.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 10.7, 2.6, -10, -20, -90, 0.6, true, *EF_FLIP_YZ, 0.06);
                LAST_EFFECT_SET_COLOR(fighter, 1, 1, 1);
            }
            frame(lua_state, 13.0);
            if is_excute(fighter) {
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), true, true);
            }
        }
    } else {
    // pickaxe effects
        frame(lua_state, 0.0);
        material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            let mut effect: [&str;2] = ["pickel_pick_flare_wood", "pickel_atk_pick_wood"]; // default effects for wood
            if material_kind == stone {
                effect = ["pickel_pick_flare_stone", "pickel_atk_pick_stone"];
            } else if material_kind == iron {
                effect = ["pickel_pick_flare_iron", "pickel_atk_pick_iron"];
            } else if material_kind == gold {
                effect = ["pickel_pick_flare_gold", "pickel_atk_pick_gold"];
            } else if material_kind == diamond {
                effect = ["pickel_pick_flare_diamond", "pickel_atk_pick_diamond"];
            }
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new(effect[0]), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            frame(lua_state, 8.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new(effect[1]), Hash40::new(effect[1]), Hash40::new("top"), 1, 8.5, 4, -10, -35, -75.7, 1, true, *EF_FLIP_YZ);
                if material_kind != gold {
                    LAST_EFFECT_SET_RATE(fighter, 1.1);
                } else {
                    LAST_EFFECT_SET_RATE(fighter, 0.6);
                }
            }
            frame(lua_state, 13.0);
            if is_excute(fighter) {
                EFFECT_OFF_KIND(fighter, Hash40::new(effect[0]), false, true);
            }
        } else {
        // fist effect
            frame(lua_state, 8.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 10.7, 2.6, -10, -20, -90, 0.6, true, *EF_FLIP_YZ, 0.06);
                LAST_EFFECT_SET_COLOR(fighter, 1, 1, 1);
            }
            frame(lua_state, 13.0);
            if is_excute(fighter) {
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), true, true);
            }
        }
    }
}

#[acmd_script( agent = "pickel", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairb(fighter: &mut L2CAgentBase) {
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
    if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_MINI_FRAME) != 0 {
        // sword hitboxes
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        wait(lua_state, 1.0);
        if material_kind == gold {
            if is_excute(fighter) {
                MotionModule::set_rate(boma, 10.0);
                MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 10.0);
            }
        } else {
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
            MotionModule::set_rate(boma, 1.5);
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.5);
            material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
            if [wood, stone, iron, gold, diamond].contains(&material_kind) {
                let mut damage = 3.4; // default damage, used for wood and gold
                if material_kind == stone {
                    damage = 3.74; // damage for stone
                } else if material_kind == iron {
                    damage = 4.0; // damage for iron
                } else if material_kind == diamond {
                    damage = 4.5; // damage for diamond
                }
                let mut sfx = *COLLISION_SOUND_ATTR_PUNCH; // collision sound
                if [iron, gold, diamond].contains(&material_kind) {
                    sfx = *COLLISION_SOUND_ATTR_CUTUP;
                }
                // air hitboxes
                ATTACK(fighter, 0, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"),damage, 45, 134, 0, 27, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 3, 0, Hash40::new("top"), damage, 45, 134, 0, 27, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                // ground hitboxes
                ATTACK(fighter, 4, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 5, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 6, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 7, 0, Hash40::new("top"), damage, 57, 134, 0, 20, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
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
            } else {
                // fist hitbox
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.72, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                AttackModule::set_add_reaction_frame_revised(boma, 0, -10.0, false);
            }
            wait(lua_state, 5.0);
            if is_excute(fighter) {
                AttackModule::clear_all(boma);
            }
            frame(lua_state, 19.0);
            if is_excute(fighter) {
                if material_kind == gold {
                    MotionModule::set_rate(boma, 2.15);
                    MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.15);
                } else {
                    MotionModule::set_rate(boma, 2.0);
                    MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
                }
            }
        }
    } else {
        // pickaxe hitboxes
        frame(lua_state, 0.0);
        if is_excute(fighter) {
            WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }   
        frame(lua_state, 12.0);
        material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            let mut damage: [f32;2] = [9.5, 13.0]; // default damage, used for wood and gold
            if material_kind == stone {
                damage = [10.75, 14.0]; // damage for stone
            } else if material_kind == iron {
                damage = [12.0, 15.0]; // damage for iron
            } else if material_kind == diamond {
                damage = [13.5, 17.0]; // damage for diamond
            }
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), damage[0], 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("armr"), damage[0], 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), damage[1], 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 3, 0, Hash40::new("haver"), damage[1], 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(boma, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        } else {
            // fist hitboxes
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.7, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        wait(lua_state, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            if material_kind == gold {
                FT_MOTION_RATE(fighter, 0.8);
            }
        }
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

#[acmd_script( agent = "pickel", script = "effect_attackairb", category = ACMD_EFFECT, low_priority )] 
unsafe fn effect_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
    let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
    let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
    let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
    let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
    // sword effects
    if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_KIND) == *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD {
        frame(lua_state, 11.0);
        material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            let mut effect: [&str;2] = ["pickel_sword_flare_wood", "pickel_atk_slash_wood"]; // default effects for wood
            if material_kind == stone {
                effect = ["pickel_sword_flare_stone", "pickel_atk_slash_stone"];
            } else if material_kind == iron {
                effect = ["pickel_sword_flare_iron", "pickel_atk_slash_iron"];
            } else if material_kind == gold {
                effect = ["pickel_sword_flare_gold", "pickel_atk_slash_gold"];
            } else if material_kind == diamond {
                effect = ["pickel_sword_flare_diamond", "pickel_atk_slash_diamond"];
            }
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new(effect[0]), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.25, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new(effect[1]), Hash40::new(effect[1]), Hash40::new("top"), 1.85, 5.9, -2.0, -140, 30, -55, 1.15, true, *EF_FLIP_YZ);
            }
            frame(lua_state, 17.0);
            if is_excute(fighter) {
                EFFECT_OFF_KIND(fighter, Hash40::new(effect[0]), false, true);
            }
        } else {
            // fist effects
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 11.2, -3, 190, 15, -90, 0.6, true, *EF_FLIP_YZ, 0.06);
                LAST_EFFECT_SET_COLOR(fighter, 1, 1, 1);
                LAST_EFFECT_SET_RATE(fighter, 1);
            }
            frame(lua_state, 17.0);
            if is_excute(fighter) {
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), true, true);
            }
        }
    } else { 
    // pickaxe effects
        frame(lua_state, 0.0);
        material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        let mut effect: [&str;2] = ["pickel_pick_flare_wood", "pickel_atk_pick_wood"]; // default effects for wood
        if material_kind == stone {
            effect = ["pickel_pick_flare_stone", "pickel_atk_pick_stone"];
        } else if material_kind == iron {
            effect = ["pickel_pick_flare_iron", "pickel_atk_pick_iron"];
        } else if material_kind == gold {
            effect = ["pickel_pick_flare_gold", "pickel_atk_pick_gold"];
        } else if material_kind == diamond {
            effect = ["pickel_pick_flare_diamond", "pickel_atk_pick_diamond"];
        }
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new(effect[0]), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            frame(lua_state, 12.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new(effect[1]), Hash40::new(effect[1]), Hash40::new("top"), 0, 9.7, -4, 180, 35, -100, 1, true, *EF_FLIP_YZ);
                if material_kind != gold {
                    LAST_EFFECT_SET_RATE(fighter, 0.95);
                } else {
                    LAST_EFFECT_SET_RATE(fighter, 0.55);
                }
            }
            frame(lua_state, 16.0);
            if is_excute(fighter) {
                EFFECT_OFF_KIND(fighter, Hash40::new(effect[0]), false, true);
            }
            frame(lua_state, 17.0);
            if is_excute(fighter) {
                EFFECT_OFF_KIND(fighter, Hash40::new(effect[1]), true, true);
            }
        } else {
        // fist effects
            frame(lua_state, 12.0);    
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 11.2, -3, 190, 15, -90, 0.6, true, *EF_FLIP_YZ, 0.06);
                LAST_EFFECT_SET_COLOR(fighter, 1, 1, 1);
                LAST_EFFECT_SET_RATE(fighter, 1);
            }
            frame(lua_state, 17.0);
            if is_excute(fighter) {
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), true, true);
            }
        }
    }
}

#[acmd_script( agent = "pickel", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn game_attackairhi(fighter: &mut L2CAgentBase) {
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
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.9);
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
            ATTACK(fighter, 0, 0, Hash40::new("armr"), damage, 71, 76, 0, 48, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), damage, 71, 76, 0, 48, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        } else {
            // fist hitbox
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 4.2, 71, 76, 0, 48, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if material_kind != gold {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.8);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI_ENABLE_LANDING);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0)
    }
}

#[acmd_script( agent = "pickel", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn game_attackairlw(fighter: &mut L2CAgentBase) {
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
unsafe fn game_attackairlw2(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "pickel_forge", script = "game_fall", category = ACMD_GAME, low_priority )]
unsafe fn forge_game_fall(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.boma();
    if is_excute(weapon) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 8.0);
    if is_excute(weapon) {
        JostleModule::set_status(boma, true);
        WorkModule::on_flag(boma, *WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_UPDATE_ATTACK);
    }
}

#[acmd_script( agent = "pickel_forge", script = "game_fallattack", category = ACMD_GAME, low_priority )]
unsafe fn forge_game_fallattack(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.boma();
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let pickel = utils::util::get_battle_object_from_id(owner_id);
    let fall_distance = VarModule::get_float(pickel, vars::pickel::instance::FORGE_START_Y_POS) - PostureModule::pos_y(boma);
    if is_excute(weapon) {
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_PICKEL {
            let pickel = utils::util::get_battle_object_from_id(owner_id);
            let pickel_boma = &mut *(*pickel).module_accessor;
            if pickel_boma.is_motion_one_of(&[Hash40::new("attack_air_lw"),
            Hash40::new("attack_air_lw_2"),
            Hash40::new("attack_air_lw_fall"),]){
                //below hitbox shows for 1 frame if this isnt here lol
           } else {
                wait(lua_state, 2.0);
                ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 70, (80.0 - (fall_distance / 2.0)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
                ATTACK(weapon, 1, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 58, (80.0 - fall_distance), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
                AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
            }
        } else {
            ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 70, (80.0 - (fall_distance / 2.0)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
            ATTACK(weapon, 1, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 58, (80.0 - (fall_distance / 2.0)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
}

#[acmd_script( agent = "pickel_forge", script = "game_fallattackride", category = ACMD_GAME, low_priority )]
unsafe fn forge_game_fallattackride(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.boma();
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let pickel = utils::util::get_battle_object_from_id(owner_id);
    let pickel_boma = &mut *(*pickel).module_accessor;
    let fall_distance = VarModule::get_float(pickel, vars::pickel::instance::FORGE_START_Y_POS) - PostureModule::pos_y(boma);
    if is_excute(weapon) {
        ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0 + (fall_distance / 3.2), 70, (80.0 - (fall_distance / 2.0)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0 + (fall_distance / 3.2), 58, (80.0 - (fall_distance / 2.0)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_attackairf,
        effect_attackairf,
        game_attackairb,
        effect_attackairb,
        game_attackairhi,
        game_attackairlw,
        game_attackairlw2,
        forge_game_fall,
        forge_game_fallattack,
        forge_game_fallattackride
    );
}

