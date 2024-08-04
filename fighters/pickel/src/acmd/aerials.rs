use super::*;

// Note: Neutral air is handled in tilts.rs, as it shares a script with forward tilt/jab

// shorthand for referencing steve's different MATERIAL_KIND constants
const WOOD: i32 = 0x1;
const STONE: i32 = 0x2;
const IRON: i32 = 0x3;
const GOLD: i32 = 0x4;
const DIAMOND: i32 = 0x6;

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    if agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_MINI_FRAME) != 0 {
    // sword hitboxes
        frame(lua_state, 1.0);
        if is_excute(agent) {
            agent.set_int(*FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            let rate = if material_kind == GOLD { 6.0 } else { 3.0 };
            MotionModule::set_rate(boma, rate);
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, rate);
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            agent.on_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 8.0);
        if is_excute(agent) {
            material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
            MotionModule::set_rate(boma, 1.5);
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.5);
            if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
                let damage = match material_kind {
                    /* WOOD/GOLD */ _  => 3.0,
                    STONE => 3.5,
                    IRON => 4.0,
                    DIAMOND => 4.5
                };
                let sfx = match material_kind {
                    ( IRON | GOLD | DIAMOND ) => *COLLISION_SOUND_ATTR_CUTUP,
                    _ => *COLLISION_SOUND_ATTR_PUNCH
                };
                // air hitboxes
                ATTACK(agent, 0, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 2, 0, Hash40::new("haver"),damage, 45, 134, 0, 27, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 3, 0, Hash40::new("top"), damage, 45, 134, 0, 27, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.25);
                for id in 0..=3 { AttackModule::set_add_reaction_frame_revised(boma, id, -5.0, false); }
                // ground hitboxes
                ATTACK(agent, 4, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 5, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 6, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 7, 0, Hash40::new("top"), damage, 57, 134, 0, 20, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 4, 5, 6, 7, 0.25);
                for id in 4..=7 { AttackModule::set_add_reaction_frame_revised(boma, id, -3.0, false); }
                agent.set_float(2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            } else {
                // fist hitbox
                ATTACK(agent, 0, 0, Hash40::new("haver"), 2.5, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.25);
                AttackModule::set_add_reaction_frame_revised(boma, 0, -10.0, false);
            }
            wait(lua_state, 5.0);
            if is_excute(agent) {
                AttackModule::clear_all(boma);
            }
            frame(lua_state, 19.0);
            if is_excute(agent) {
                let rate = if material_kind == GOLD { 3.625 } else { 3.23 };
                MotionModule::set_rate(boma, rate);
                MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, rate);
            }
        }
    } else {
    //pickaxe hitboxes
        frame(lua_state, 0.0);
        if is_excute(agent) {
            agent.set_int(*FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        frame(lua_state, 1.0);
        if is_excute(agent) {
            FT_MOTION_RATE(agent, 8.0 / (7.0 - 1.0));
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            agent.on_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 8.0);
        material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
            let damage: [f32;4] = match material_kind {
                /* WOOD/GOLD */ _  => [8.0, 9.0, 10.0, 11.0],
                STONE => [9.0, 10.5, 11.0, 12.25],
                IRON => [10.0, 11.5, 12.0, 13.5],
                DIAMOND => [11.0, 14.0, 12.5, 15.0]
            };
            if is_excute(agent) {
                FT_MOTION_RATE(agent, 1.0);
                ATTACK(agent, 0, 0, Hash40::new("shoulderr"), damage[0], 361, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 0, Hash40::new("armr"), damage[0], 361, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 2, 0, Hash40::new("haver"), damage[1], 361, 75, 0, 56, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 3, 0, Hash40::new("haver"), damage[1], 361, 75, 0, 56, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                agent.set_float(6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(lua_state, 3.0);
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("shoulderr"), damage[2], 55, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 0, Hash40::new("armr"), damage[2], 55, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 2, 0, Hash40::new("haver"), damage[3], 275, 55, 0, 25, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 3, 0, Hash40::new("haver"), damage[3], 275, 65, 0, 80, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.25, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                agent.set_float(6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        } else {
            // fist hitboxes
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 8.0, 361, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 1, 0, Hash40::new("armr"), 8.0, 361, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.15, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
            wait(lua_state, 3.0);
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 9.5, 55, 63, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 1, 0, Hash40::new("armr"), 9.5, 55, 63, 0, 56, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.15, 1.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            if material_kind == GOLD {
                FT_MOTION_RATE(agent, 0.8);
            }
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }   
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    // sword effects
    if agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_KIND) == *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD {
        frame(lua_state, 7.0);
        material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
            let effect: [&str;2] = match material_kind {
                /* WOOD */ _ => ["pickel_sword_flare_wood", "pickel_atk_slash_wood"],
                STONE => ["pickel_sword_flare_stone", "pickel_atk_slash_stone"],
                IRON => ["pickel_sword_flare_iron", "pickel_atk_slash_iron"],
                GOLD => ["pickel_sword_flare_gold", "pickel_atk_slash_gold"],
                DIAMOND => ["pickel_sword_flare_diamond", "pickel_atk_slash_diamond"]
            };
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new(effect[0]), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.25, true);
            }
            frame(lua_state, 8.0);
            if is_excute(agent) {
                EFFECT_FOLLOW_FLIP(agent, Hash40::new(effect[1]), Hash40::new(effect[1]), Hash40::new("top"), 1.85, 6.5, 0.6, -13, -33, -83, 1.15, true, *EF_FLIP_YZ);
            }
            frame(lua_state, 13.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, Hash40::new(effect[0]), false, true);
            }
        } else {
            // fist effect
            frame(lua_state, 8.0);
            if is_excute(agent) {
                EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 10.7, 2.6, -10, -20, -90, 0.6, true, *EF_FLIP_YZ, 0.06);
                LAST_EFFECT_SET_COLOR(agent, 1, 1, 1);
            }
            frame(lua_state, 13.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_b"), true, true);
            }
        }
    } else {
    // pickaxe effects
        frame(lua_state, 0.0);
        material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
            let effect: [&str;2] = match material_kind {
                /* WOOD */ _ => ["pickel_pick_flare_wood", "pickel_atk_pick_wood"],
                STONE => ["pickel_sword_flare_stone", "pickel_atk_pick_stone"],
                IRON => ["pickel_pick_flare_iron", "pickel_atk_pick_iron"],
                GOLD => ["pickel_pick_flare_gold", "pickel_atk_pick_gold"],
                DIAMOND => ["pickel_pick_flare_diamond", "pickel_atk_pick_diamond"]
            };
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new(effect[0]), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            frame(lua_state, 8.0);
            if is_excute(agent) {
                EFFECT_FOLLOW_FLIP(agent, Hash40::new(effect[1]), Hash40::new(effect[1]), Hash40::new("top"), 1, 8.5, 4, -10, -35, -75.7, 1, true, *EF_FLIP_YZ);
                LAST_EFFECT_SET_RATE(agent, if material_kind == GOLD { 1.1 } else { 0.6 });
            }
            frame(lua_state, 13.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, Hash40::new(effect[0]), false, true);
            }
        } else {
        // fist effect
            frame(lua_state, 8.0);
            if is_excute(agent) {
                EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 10.7, 2.6, -10, -20, -90, 0.6, true, *EF_FLIP_YZ, 0.06);
                LAST_EFFECT_SET_COLOR(agent, 1, 1, 1);
            }
            frame(lua_state, 13.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_b"), true, true);
            }
        }
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    if agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_MINI_FRAME) != 0 {
    // sword hitboxes
        frame(lua_state, 1.0);
        if is_excute(agent) {
            agent.set_int(*FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            let rate = if material_kind == GOLD { 10.0 } else { 5.0 };
            MotionModule::set_rate(boma, rate);
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, rate);
        }
        frame(lua_state, 5.0);
        if is_excute(agent) {
            agent.on_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 12.0);
        if is_excute(agent) {
            MotionModule::set_rate(boma, 1.5);
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.5);
            material_kind = agent.get_int( *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
            if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
                let damage = match material_kind {
                    /* WOOD/GOLD */ _  => 3.0,
                    STONE => 3.5,
                    IRON => 4.0,
                    DIAMOND => 4.5
                };
                let sfx = match material_kind {
                    ( IRON | GOLD | DIAMOND ) => *COLLISION_SOUND_ATTR_CUTUP,
                    _ => *COLLISION_SOUND_ATTR_PUNCH
                };
                // air hitboxes
                ATTACK(agent, 0, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 2, 0, Hash40::new("haver"),damage, 45, 134, 0, 27, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 3, 0, Hash40::new("top"), damage, 45, 134, 0, 27, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.25);
                for id in 0..=3 { AttackModule::set_add_reaction_frame_revised(boma, id, -5.0, false); }
                // ground hitboxes
                ATTACK(agent, 4, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 5, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 5.5, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 6, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 11.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATTACK(agent, 7, 0, Hash40::new("top"), damage, 57, 134, 0, 20, 2.3, 0.0, 6.8, -5.4, Some(0.0), Some(6.8), Some(-10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 4, 5, 6, 7, 0.25);
                for id in 4..=7 { AttackModule::set_add_reaction_frame_revised(boma, id, -3.0, false); }
                agent.set_float(2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            } else {
                // fist hitbox
                ATTACK(agent, 0, 0, Hash40::new("haver"), 2.5, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.25);
                AttackModule::set_add_reaction_frame_revised(boma, 0, -10.0, false);
            }
            wait(lua_state, 5.0);
            if is_excute(agent) {
                AttackModule::clear_all(boma);
            }
            frame(lua_state, 19.0);
            if is_excute(agent) {
                let rate = if material_kind == GOLD { 2.15 } else { 2.0 };
                MotionModule::set_rate(boma, rate);
                MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, rate);
            }
        }
    } else {
    // pickaxe hitboxes
        frame(lua_state, 0.0);
        if is_excute(agent) {
            agent.set_int(*FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        frame(lua_state, 5.0);
        if is_excute(agent) {
            agent.on_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 12.0);
        material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
            let damage: [f32;2] = match material_kind {
                /* WOOD/GOLD */ _  => [9.5, 13.0],
                STONE => [10.5, 14.0],
                IRON => [12.0, 15.0],
                DIAMOND => [13.5, 17.0]
            };
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("armr"), damage[0], 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.25, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 0, Hash40::new("armr"), damage[0], 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.25, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 2, 0, Hash40::new("haver"), damage[1], 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.25, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 3, 0, Hash40::new("haver"), damage[1], 361, 85, 0, 51, 5.4, 0.0, 4.4, 0.0, None, None, None, 1.25, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                agent.set_float(6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        } else {
            // fist hitboxes
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("armr"), 9.7, 50, 85, 0, 56, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.25, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        wait(lua_state, 5.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            if material_kind == GOLD {
                FT_MOTION_RATE(agent, 0.8);
            }
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
}
 
unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    // sword effects
    if agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_KIND) == *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD {
        frame(lua_state, 11.0);
        material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
            let effect: [&str;2] = match material_kind {
                /* WOOD */ _ => ["pickel_sword_flare_wood", "pickel_atk_slash_wood"],
                STONE => ["pickel_sword_flare_stone", "pickel_atk_slash_stone"],
                IRON => ["pickel_sword_flare_iron", "pickel_atk_slash_iron"],
                GOLD => ["pickel_sword_flare_gold", "pickel_atk_slash_gold"],
                DIAMOND => ["pickel_sword_flare_diamond", "pickel_atk_slash_diamond"]
            };
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new(effect[0]), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.25, true);
            }
            frame(lua_state, 12.0);
            if is_excute(agent) {
                EFFECT_FOLLOW_FLIP(agent, Hash40::new(effect[1]), Hash40::new(effect[1]), Hash40::new("top"), 1.85, 5.9, -2.0, -140, 30, -55, 1.15, true, *EF_FLIP_YZ);
            }
            frame(lua_state, 17.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, Hash40::new(effect[0]), false, true);
            }
        } else {
            // fist effects
            frame(lua_state, 12.0);
            if is_excute(agent) {
                EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 11.2, -3, 190, 15, -90, 0.6, true, *EF_FLIP_YZ, 0.06);
                LAST_EFFECT_SET_COLOR(agent, 1, 1, 1);
                LAST_EFFECT_SET_RATE(agent, 1);
            }
            frame(lua_state, 17.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_b"), true, true);
            }
        }
    } else { 
    // pickaxe effects
        frame(lua_state, 0.0);
        material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
            let effect: [&str;2] = match material_kind {
                /* WOOD */ _ => ["pickel_pick_flare_wood", "pickel_atk_pick_wood"],
                STONE => ["pickel_sword_flare_stone", "pickel_atk_pick_stone"],
                IRON => ["pickel_pick_flare_iron", "pickel_atk_pick_iron"],
                GOLD => ["pickel_pick_flare_gold", "pickel_atk_pick_gold"],
                DIAMOND => ["pickel_pick_flare_diamond", "pickel_atk_pick_diamond"]
            };
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new(effect[0]), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            frame(lua_state, 12.0);
            if is_excute(agent) {
                EFFECT_FOLLOW_FLIP(agent, Hash40::new(effect[1]), Hash40::new(effect[1]), Hash40::new("top"), 0, 9.7, -4, 180, 35, -100, 1, true, *EF_FLIP_YZ);
                LAST_EFFECT_SET_RATE(agent, if material_kind == GOLD { 0.95 } else { 0.55 });
            }
            frame(lua_state, 16.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, Hash40::new(effect[0]), false, true);
            }
            frame(lua_state, 17.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, Hash40::new(effect[1]), true, true);
            }
        } else {
        // fist effects
            frame(lua_state, 12.0);    
            if is_excute(agent) {
                EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 11.2, -3, 190, 15, -90, 0.6, true, *EF_FLIP_YZ, 0.06);
                LAST_EFFECT_SET_COLOR(agent, 1, 1, 1);
                LAST_EFFECT_SET_RATE(agent, 1);
            }
            frame(lua_state, 17.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_b"), true, true);
            }
        }
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
       agent.set_int(*FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let rate = if material_kind == GOLD { 0.9 } else { 0.8 };
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, rate);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
            let damage = match material_kind {
                /* WOOD/GOLD */ _  => 5.5,
                STONE => 6.5,
                IRON => 7.5,
                DIAMOND => 8.5
            };
            agent.set_float(5.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            ATTACK(agent, 0, 0, Hash40::new("armr"), damage, 71, 76, 0, 48, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 71, 76, 0, 48, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        } else {
            // fist hitbox
            ATTACK(agent, 0, 0, Hash40::new("armr"), 4.2, 71, 76, 0, 48, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if material_kind != GOLD {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.8);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI_ENABLE_LANDING);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0)
    }
}

unsafe extern "C" fn game_attackairlw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        agent.on_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }      
}

unsafe extern "C" fn game_aircatch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, -1);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 2.35); // the higher the number, the longer the line will end up being cast
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, *WEAPON_PICKEL_FISHINGROD_STATUS_KIND_SHOOT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("air_catch"), false, -1.0);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {        
        ArticleModule::set_visibility(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::set_flag(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairf", game_attackairf, Priority::Low);
    agent.acmd("effect_attackairf", effect_attackairf, Priority::Low);

    agent.acmd("game_attackairb", game_attackairb, Priority::Low);
    agent.acmd("effect_attackairb", effect_attackairb, Priority::Low);

    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);

    agent.acmd("game_attackairlw2", game_attackairlw2, Priority::Low);

    agent.acmd("game_aircatch", game_aircatch, Priority::Low);
}
