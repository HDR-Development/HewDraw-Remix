use super::*;

// shorthand for referencing steve's different MATERIAL_KIND constants
const WOOD: i32 = 0x1;
const STONE: i32 = 0x2;
const IRON: i32 = 0x3;
const GOLD: i32 = 0x4;
const DIAMOND: i32 = 0x6;

// this script is used for jab, forward tilt, and neutral air
unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        agent.set_int(*FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 2.0);
    material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if material_kind == GOLD {
        if is_excute(agent) {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
            let damage = match material_kind {
                ( WOOD | GOLD ) => 3.0,
                STONE => 3.5,
                IRON => 4.0,
                /* DIAMOND */ _ => 4.5
            };
            let hitstun = match material_kind {
                WOOD => -6.0,
                STONE => -4.0,
                IRON => -2.0,
                GOLD => -3.0, 
                /* DIAMOND */ _ => 0.0
            };
            let sfx = match material_kind {
                ( IRON | GOLD | DIAMOND ) => *COLLISION_SOUND_ATTR_CUTUP,
                _ => *COLLISION_SOUND_ATTR_PUNCH
            };
            ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 55, 130, 0, 30, 2.3, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("haver"), damage, 55, 130, 0, 30, 2.3, 2.0, 4.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("haver"), damage, 55, 130, 0, 30, 2.3, 2.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            ATTACK(agent, 4, 0, Hash40::new("top"), damage, 55, 130, 0, 30, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 1, 2, 3, 4, 0.25);
            for id in 1..=4 { AttackModule::set_add_reaction_frame_revised(boma, id, hitstun, false); }
            /// locking hitbox on jab
            if boma.is_situation(*SITUATION_KIND_GROUND) 
            && (-0.1..0.1).contains(&agent.left_stick_x()) { // locking hitbox on jab
                ATTACK(agent, 0, 0, Hash40::new("top"), damage, 361, 100, 20, 0, 2.5, 0.0, 3.3, 5.4, Some(0.0), Some(3.3), Some(10.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
                AttackModule::set_down_only(boma, 0, false);
            }
            agent.set_float(2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        } else {
            // fist hitboxes
            ATTACK(agent, 1, 0, Hash40::new("haver"), 2.5, 45, 100, 35, 0, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(boma, 0, -10.0, false);
            // ground-only locking hitbox
            if boma.is_situation(*SITUATION_KIND_GROUND) { 
                ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 361, 100, 20, 0, 2.5, 0.0, 3.3, 3.0, Some(0.0), Some(3.3), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                AttackModule::set_down_only(boma, 0, false);
            }
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if material_kind == GOLD {
        if is_excute(agent) {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.25);
        }
        frame(lua_state, 16.0);
        if is_excute(agent) {
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        } 
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.75);
        agent.set_int(*FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 
            if material_kind == GOLD { 1.4 } else { 0.6 }
        );
    }
    frame(lua_state, 4.0); // this comes out frame 5 if gold, frame 6 for all other materials
    if is_excute(agent) {
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.3);
        if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
            let damage = match material_kind {
                 (WOOD | GOLD ) => 5.5,
                STONE => 6.5,
                IRON => 7.5,
                /* DIAMOND */ _ => 8.5
            };
            ATTACK(agent, 0, 0, Hash40::new("haver"), damage, 85, 70, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 85, 70, 0, 65, 4.5, -2.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            agent.set_float(5.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        } else { // fist hitbox
            ATTACK(agent, 0, 0, Hash40::new("armr"), 3.0, 75, 60, 0, 50, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 9.0);
    if is_excute(agent) { // clears after 7 frames
        AttackModule::clear_all(boma);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,
            if material_kind == GOLD { 1.1 } else { 1.0 }
        );
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    let material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
        let effect: [&str;2] = match material_kind {
            WOOD => ["pickel_axe_flare_wood", "pickel_atk_axe_wood"],
            STONE => ["pickel_axe_flare_stone", "pickel_atk_axe_stone"],
            IRON => ["pickel_axe_flare_iron", "pickel_atk_axe_iron"],
            GOLD => ["pickel_axe_flare_gold", "pickel_atk_axe_gold"],
            /* DIAMOND */ _ => ["pickel_axe_flare_diamond", "pickel_atk_axe_diamond"]
        };
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new(effect[0]), Hash40::new("weaponr"), 0, 0, 0, 0, 0, 0, 1.0, false);
        }
        frame(lua_state, 6.0);
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new(effect[1]), Hash40::new(effect[1]), Hash40::new("top"), -3.2, 12.0, -1.89, -90, -90, -27, 0.9, false, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    } else {
        // fist effect
        frame(lua_state, 5.0);
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 11, -3, -20, -60, -70, 0.7, false, *EF_FLIP_YZ, 0.05);
            LAST_EFFECT_SET_COLOR(agent, 1, 1, 1);
            LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        let effects: [&str;6] = [
            "pickel_axe_flare_wood",
            "pickel_axe_flare_stone",
            "pickel_axe_flare_iron",
            "pickel_axe_flare_gold",
            "pickel_axe_flare_diamond",
            "sys_attack_arc_b"
        ];
        for kind in effects {
            EFFECT_OFF_KIND(agent, Hash40::new(kind), false, true);
        }
    }
}

unsafe extern "C" fn expression_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_77_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        let (rumble, frames) = match material_kind {
            WOOD => ("rbkind_77_attacks", 8),
            STONE => ("rbkind_77_attacks", 0),
            IRON => ("rbkind_77_attackm", 6),
            GOLD => ("rbkind_77_attacks", 9),
            DIAMOND => ("rbkind_77_attackm", 0),
            /* NONE */ _ => ("rbkind_77_attacks", 8) 
        };
        RUMBLE_HIT(agent, Hash40::new(rumble), frames);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut is_soul_fire = false;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    FT_MOTION_RATE(agent, 8.0);
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK){
            is_soul_fire = true;
            VarModule::on_flag(agent.battle_object, vars::pickel::status::ATTACK_LW3_SOUL_FIRE);
        }
        FT_MOTION_RATE(agent, if is_soul_fire { 4.0 } else { 1.0 } );
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if !ArticleModule::is_exist(boma,  *FIGHTER_PICKEL_GENERATE_ARTICLE_FIRE){
            ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FIRE, false, 0);
        }
        FT_MOTION_RATE(agent, if is_soul_fire { 1.0 } else { 0.6 } );
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, if is_soul_fire { 0.5 } else { 1.0 } );
    }
}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut is_soul_fire = false;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::pickel::status::ATTACK_LW3_SOUL_FIRE) {
            is_soul_fire = true;
        }
        if is_soul_fire {
            EFFECT(agent, Hash40::new("sys_hit_aura"), Hash40::new("top"), 3.5, 3, 0, 0, 0, 0, 0.075, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 3.5, 3, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.8);
        } else {
            EFFECT(agent, Hash40::new("pickel_flint"), Hash40::new("haver"), 1, 6.2, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if is_soul_fire {
            FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_aura"), false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3, Priority::Low);

    agent.acmd("game_attackhi3", game_attackhi3, Priority::Low);
    agent.acmd("effect_attackhi3", effect_attackhi3, Priority::Low);
    agent.acmd("expression_attackhi3", expression_attackhi3, Priority::Low);
    
    agent.acmd("game_attacklw3", game_attacklw3, Priority::Low);
    agent.acmd("effect_attacklw3", effect_attacklw3, Priority::Low);
}
