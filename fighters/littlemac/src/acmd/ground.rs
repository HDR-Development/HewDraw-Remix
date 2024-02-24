use super::*;

#[acmd_script( agent = "littlemac", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn littlemac_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 2.5, 82, 25, 0, 30, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 2.5, 82, 25, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("handl"), 2.5, 82, 25, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 361, 10, 0, 27, 2.5, 0.0, 4.5, 3.0, Some(0.0), Some(4.5), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_MIDDLE), false);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    
}

#[acmd_script( agent = "littlemac", script = "game_attack12" , category = ACMD_GAME , low_priority)]
unsafe fn littlemac_attack_12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::littlemac::instance::IS_DREAMLAND_EXPRESS);
        VarModule::off_flag(fighter.battle_object, vars::littlemac::instance::IS_LATE_DLE_INPUT);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 3.0, 83, 80, 0, 10, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.9, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 83, 80, 0, 10, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.9, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("handr"), 3.0, 83, 80, 0, 10, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.9, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 361, 25, 0, 25, 2.5, 0.0, 4.5, 5.0, Some(0.0), Some(4.5), Some(12.0), 1.0, 0.9, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::littlemac::instance::IS_LATE_DLE_INPUT);
    }
    
}

#[acmd_script( agent = "littlemac", script = "game_attack13" , category = ACMD_GAME , low_priority)]
unsafe fn littlemac_attack_13_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if VarModule::is_flag(fighter.battle_object, vars::littlemac::instance::IS_DREAMLAND_EXPRESS) {
        FT_MOTION_RATE_RANGE(fighter, 1.0, 4.0, 1.0);
    }
    frame(lua_state, 4.0);
    if VarModule::is_flag(fighter.battle_object, vars::littlemac::instance::IS_DREAMLAND_EXPRESS) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    if is_excute(fighter) {
        let is_dle = VarModule::is_flag(fighter.battle_object, vars::littlemac::instance::IS_DREAMLAND_EXPRESS);
        let angle = if is_dle { 70 } else { 62 };
        let bkb = if is_dle { 60 } else { 70 };
        let kbg = if is_dle { 52 } else { 88 };
        let hitlag = if is_dle { 1.5 } else { 1.0 };
        let sdi = if is_dle { 0.6 } else { 1.0 };
        let collision_attr = if is_dle { Hash40::new("collision_attr_elec") } else { Hash40::new("collision_attr_normal") };
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 5.0, angle, kbg, 0, bkb, 3.2, 2.0, 0.0, 0.0, None, None, None, hitlag, sdi, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 5.0, angle, kbg, 0, bkb, 3.0, 0.0, 0.0, 0.0, None, None, None, hitlag, sdi, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("bust"), 5.0, angle, kbg, 0, bkb, 3.0, 0.0, 0.0, 0.0, None, None, None, hitlag, sdi, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, angle, kbg, 0, bkb, 3.5, 0.0, 8.0, 6.0, None, None, None, hitlag, sdi, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        if is_dle {
            ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 2.0);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "littlemac", script = "effect_attack13", category = ACMD_EFFECT, low_priority)]
unsafe fn littlemac_attack_13_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::littlemac::instance::IS_DREAMLAND_EXPRESS) {
            if is_training_mode() {
                if VarModule::is_flag(fighter.battle_object, vars::littlemac::instance::IS_LATE_DLE_INPUT) {
                    FLASH(fighter, 1, 0, 0.05, 0.7);
                }
                else {
                    FLASH(fighter, 0, 0.24, 0.8, 0.7);
                }
            }
        }
        else {
            EFFECT(fighter, Hash40::new("littlemac_attack_arc_glove_b"), Hash40::new("top"), -1, 11, 5, -10, -45, 120, 0.8, 0, 0, 0, 0, 0, 0, true);
            LAST_PARTICLE_SET_COLOR(fighter, 0.43, 1, 0.3);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::littlemac::instance::IS_DREAMLAND_EXPRESS) {
            EFFECT(fighter, Hash40::new("littlemac_attack_arc_glove_b"), Hash40::new("top"), -1, 11, 1, -10, -45, 120, 0.8, 0, 0, 0, 0, 0, 0, true);
            LAST_PARTICLE_SET_COLOR(fighter, 0.43, 1, 0.3);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 10, 15, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false, 0.7);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::littlemac::instance::IS_DREAMLAND_EXPRESS) && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_level_up"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 0.1, true);
            LAST_EFFECT_SET_RATE(fighter, 1.2);
        }
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if is_training_mode() && !VarModule::is_flag(fighter.battle_object, vars::littlemac::instance::IS_DREAMLAND_EXPRESS) {
            COL_NORMAL(fighter);
        }
        EFFECT_OFF_KIND(fighter, Hash40::new("littlemac_attack_arc_glove_b"), true, true);
    }
}

#[acmd_script( agent = "littlemac", script = "sound_attack13", category = ACMD_SOUND, low_priority)]
unsafe fn littlemac_attack_13_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::littlemac::instance::IS_DREAMLAND_EXPRESS)
        && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) || is_training_mode()) {
            match smash::app::sv_math::rand(smash::hash40("fighter"), 3) {
                0 => PLAY_SE(fighter, Hash40::new("vc_littlemac_special_l02")),
                1 => PLAY_SE(fighter, Hash40::new("vc_littlemac_special_n02")),
                2 => PLAY_SE(fighter, Hash40::new("vc_littlemac_special_n03")),
                _ => PLAY_SE(fighter, Hash40::new("vc_littlemac_special_l02"))
            };
        } else {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_littlemac_rnd_combo03"));
        }
        PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
    }
}

#[acmd_script( agent = "littlemac", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn littlemac_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.81);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0, 55, 70, 0, 80, 4.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 55, 70, 0, 80, 3.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 10.0, 55, 70, 0, 80, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 8.0, 88, 50, 0, 75, 4.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 88, 50, 0, 75, 3.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 8.0, 88, 50, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "littlemac", script = "expression_attackdash", category = ACMD_EXPRESSION, low_priority )]
unsafe fn littlemac_attack_dash_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 10.0);
    if !WorkModule::is_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_ATTACK_HIT) {
        if is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        littlemac_attack_11_game,
        littlemac_attack_12_game,
        littlemac_attack_13_game,
        littlemac_attack_13_effect,
        littlemac_attack_13_sound,
        littlemac_attack_dash_game,
        littlemac_attack_dash_expression,
    );
}