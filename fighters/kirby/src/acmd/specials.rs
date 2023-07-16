use super::*;

#[acmd_script( agent = "kirby", scripts = ["game_specialneat", "game_specialairneat"], category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_n_eat_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 52.0, 24.0);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_ITEM_REMOVE);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_ITEM_USE);
    }
}

#[acmd_script( agent = "kirby", scripts = ["game_specialndrink", "game_specialairndrink"], category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_n_drink_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 361, 0, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_SPIT);
    }
}

#[acmd_script( agent = "kirby", scripts = ["game_specialnlarge", "game_specialairnlarge"], category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_n_large_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 86.0, 54.0);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_ITEM_REMOVE);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_ITEM_USE);
    }
}

// Copy Ability Effects
#[acmd_script( agent = "kirby", scripts = ["effect_shizuespecialnfailure", "effect_shizuespecialairnfailure"] , category = ACMD_EFFECT , low_priority)]
unsafe fn shizue_special_n_failure_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("shizue_cracker"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "kirby", script = "game_mariospecialn" , category = ACMD_GAME , low_priority)]
unsafe fn mario_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 16.0/(14.0-1.0));
        VarModule::off_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            VarModule::on_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
            FT_MOTION_RATE(fighter, 3.0/(14.0-12.0));
        } 
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 10.0, 45, 110, 0, 50, 3.0, 0.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 50, 115, 0, 50, 5.5, 5.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            FT_MOTION_RATE(fighter, 1.0);
        }
        else {
            ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 8.0, 40, 100, 0, 50, 3.0, 0.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 40, 100, 0, 50, 5.5, 5.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            AttackModule::clear_all(boma);
            FT_MOTION_RATE(fighter, 32.0/(49.0 - 21.0));
        }
        else {
            FT_MOTION_RATE(fighter, 23.0/(49.0 - 21.0));
        }
    }
    
}

#[acmd_script( agent = "kirby", script = "effect_mariospecialn" , category = ACMD_EFFECT , low_priority)]
unsafe fn mario_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 0.2);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1.0, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
        
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flame"), Hash40::new("handl"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_bomb_a"), Hash40::new("havel"), 1.0, 0, 0, 0, 0, 0, 0.23, true);
            LAST_EFFECT_SET_RATE(fighter, 1.2);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.35);
        }
        else{
            FLASH(fighter, 1, 0, 0, 0.353);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.75);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_bomb_a"), false, false);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.35);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.35);
        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.75);  
        }
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.35);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_flame"), false, false);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.35);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
}

#[acmd_script( agent = "kirby", script = "sound_mariospecialn" , category = ACMD_SOUND , low_priority)]
unsafe fn mario_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            PLAY_SE(fighter, Hash40::new("se_mario_smash_s01"));
            PLAY_SE(fighter, Hash40::new("vc_kirby_attack02"));
        }
    }
}

#[acmd_script( agent = "kirby", script = "game_mariospecialairn" , category = ACMD_GAME , low_priority)]
unsafe fn mario_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 16.0/(14.0-1.0));
        VarModule::off_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){ 
            VarModule::on_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
            FT_MOTION_RATE(fighter, 3.0/(14.0-12.0));
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 10.0, 45, 110, 0, 50, 3.0, 0.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 50, 115, 0, 50, 5.5, 5.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            FT_MOTION_RATE(fighter, 1.0);
        }
        else {
            ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 8.0, 40, 100, 0, 50, 3.0, 0.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 40, 100, 0, 50, 5.5, 5.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            AttackModule::clear_all(boma);
            FT_MOTION_RATE(fighter, 32.0/(49.0 - 21.0));
        }
        else {
            FT_MOTION_RATE(fighter, 23.0/(49.0 - 21.0));
        }
    }
}

#[acmd_script( agent = "kirby", script = "effect_mariospecialairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn mario_special_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 0.2);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1.0, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flame"), Hash40::new("havel"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_bomb_a"), Hash40::new("handl"), 1.0, 0, 0, 0, 0, 0, 0.23, true);
            LAST_EFFECT_SET_RATE(fighter, 1.2);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.5);
        }
        else{
            FLASH(fighter, 1, 0, 0, 0.35);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 1.0);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_bomb_a"), false, false);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.5);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.5);
        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.0);
        }
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.5);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_flame"), false, false);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.5);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
}

#[acmd_script( agent = "kirby", script = "sound_mariospecialairn" , category = ACMD_SOUND , low_priority)]
unsafe fn mario_special_air_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            PLAY_SE(fighter, Hash40::new("se_mario_smash_s01"));
            PLAY_SE(fighter, Hash40::new("vc_kirby_attack02"));
        }
    }
}

#[acmd_script( agent = "kirby", script = "game_mariodspecialn" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 16.0/(14.0-1.0));
        VarModule::off_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){ 
            VarModule::on_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL);
        }

        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL){
            FT_MOTION_RATE(fighter, 3.0/(14.0-10.0));
        } 
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL){
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 10.0, 69, 90, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 69, 90, 0, 50, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("arml"), 10.0, 69, 90, 0, 50, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 1.0);
        }
        else {
            ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, 0);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL){
            AttackModule::clear_all(boma);
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
            FT_MOTION_RATE(fighter, 1.5);
        }
    }
}

#[acmd_script( agent = "kirby", script = "effect_mariodspecialn" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.75, 0.6, 1.0);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_ice"), Hash40::new("arml"), 7.5, 0, 0, 0, 0, 0, 0.35, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice_landing"), Hash40::new("arml"), 7.5, 0, 0, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_RATE(fighter, 0.75);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.35);
        }
        else{
            FLASH(fighter, 1, 1, 0, 0.353);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {

        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.75);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.35);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.75);
        }
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_ice"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice_landing"), false, true);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
        }
    }
}

#[acmd_script( agent = "kirby", script = "game_mariodspecialairn" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 16.0/(14.0-1.0));
        VarModule::off_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){ 
            VarModule::on_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL);
        }

        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL){
            FT_MOTION_RATE(fighter, 3.0/(14.0-10.0));
        } 
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL){
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 10.0, 69, 90, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 69, 90, 0, 50, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("arml"), 10.0, 69, 90, 0, 50, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 1.0);
        }
        else {
            ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, 0);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL){
            AttackModule::clear_all(boma);
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
            FT_MOTION_RATE(fighter, 1.5);
        }
    }
    
}

#[acmd_script( agent = "kirby", script = "effect_mariodspecialairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.75, 0.6, 1.0);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_ice"), Hash40::new("arml"), 7.5, 0, 0, 0, 0, 0, 0.35, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice_landing"), Hash40::new("arml"), 7.5, 0, 0, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_RATE(fighter, 0.75);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.35);
        }
        else{
            FLASH(fighter, 1, 1, 0, 0.353);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {

        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.75);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.35);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.75);
        }
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_ice"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice_landing"), false, true);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
        }
    }
}

#[acmd_script( agent = "kirby", script = "game_luigispecialn" , category = ACMD_GAME , low_priority)]
unsafe fn luigi_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
        }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            VarModule::on_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 9.0, 69, 55, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            //HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 2.0);
        }
        else{
            ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            FT_MOTION_RATE(fighter, 0.5);
        }
    }
    
}

#[acmd_script( agent = "kirby", script = "effect_luigispecialn" , category = ACMD_EFFECT , low_priority)]
unsafe fn luigi_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("havel"), 3.0, 0.0, 0.0, 0, 90, 90, 0.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 90, 90, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 1.25);
            FLASH(fighter, 0, 0.25, 1.0, 0.7);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
            FLASH(fighter, 0, 1, 0, 0.353);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_thunder"), false, false);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), true, true);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("luigi_fb_shoot"), false, false);
    }
}

#[acmd_script( agent = "kirby", script = "sound_luigispecialn" , category = ACMD_SOUND , low_priority)]
unsafe fn luigi_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("se_common_elec_s_damage"));
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("vc_kirby_attack02"));
            PLAY_SE(fighter, Hash40::new("se_common_elec_m_damage"));
        }
        else{
            PLAY_SE(fighter, Hash40::new("se_luigi_special_n01"));
        }
    }
}

#[acmd_script( agent = "kirby", script = "game_luigispecialairn" , category = ACMD_GAME , low_priority)]
unsafe fn luigi_special_air_n_game(fighter: &mut L2CAgentBase)  {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
        }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            VarModule::on_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 9.0, 69, 55, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            //HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 2.0);
        }
        else{
            ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            FT_MOTION_RATE(fighter, 0.5);
        }
    }
}

#[acmd_script( agent = "kirby", script = "effect_luigispecialairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn luigi_special_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("havel"), 3.0, 0.0, 0.0, 0, 90, 90, 0.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 90, 90, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 1.25);
            FLASH(fighter, 0, 0.25, 1.0, 0.7);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
            FLASH(fighter, 0, 1, 0, 0.353);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_thunder"), false, false);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), true, true);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("luigi_fb_shoot"), false, false);
    }
}

#[acmd_script( agent = "kirby", script = "sound_luigispecialairn" , category = ACMD_SOUND , low_priority)]
unsafe fn luigi_special_air_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("se_common_elec_s_damage"));
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("vc_kirby_attack02"));
            PLAY_SE(fighter, Hash40::new("se_common_elec_m_damage"));
        }
        else{
            PLAY_SE(fighter, Hash40::new("se_luigi_special_n01"));
        }
    }
}

// End of Copy Ability Effects
#[acmd_script( agent = "kirby", script = "game_specialsstart", category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE_RANGE(fighter, 0.0, 15.0, 6.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, -1);
    }
}

#[acmd_script( agent = "kirby", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 19.0, 48, 78, 0, 60, 5.4, 0.0, 4.5, 11.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 19.0, 48, 78, 0, 60, 3.5, 0.0, 4.5, 5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE_RANGE(fighter, 13.0, 54.0, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 54.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "kirby", script = "game_specialss", category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_ss_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 19.0, 48, 78, 0, 60, 5.4, 0.0, 4.5, 11.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 19.0, 48, 78, 0, 60, 3.5, 0.0, 4.5, 5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE_RANGE(fighter, 13.0, 54.0, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 54.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "kirby", script = "game_specialsmax", category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_s_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 35.0, 361, 78, 0, 60, 5.8, 0.0, 4.2, 11.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 35.0, 361, 78, 0, 60, 3.9, 0.0, 4.2, 5.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HAMMER);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE_RANGE(fighter, 13.0, 54.0, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 54.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 56.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "kirby", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE_RANGE(fighter, 0.0, 18.0, 11.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, -1);
    }

}

#[acmd_script( agent = "kirby", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 11.0, 5.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 106, 0, 67, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 50, 106, 0, 67, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.05);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 35, 93, 0, 67, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 35, 93, 0, 67, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.05);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "kirby", script = "game_specialairss" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_special_air_ss_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 11.0, 5.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 106, 0, 67, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 50, 106, 0, 67, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.05);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 35, 93, 0, 67, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 35, 93, 0, 67, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.05);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "kirby", script = "game_specialairhi2", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi2"), false, -1.0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 85, 100, 117, 0, 3.5, 0.0, 3.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 93, 100, 117, 0, 3.5, 0.0, 3.5, 18.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 90, 100, 102, 0, 3.5, 0.0, 13.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 93, 100, 102, 0, 3.5, 0.0, 13.5, 18.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 85, 100, 60, 0, 3.5, 0.0, 3.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 93, 100, 60, 0, 3.5, 0.0, 3.5, 18.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 90, 100, 50, 0, 3.5, 0.0, 13.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 93, 100, 50, 0, 3.5, 0.0, 13.5, 18.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 275, 100, 96, 0, 6.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 361, 180, 0, 30, 5.5, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

#[acmd_script( agent = "kirby", script = "game_specialhih", category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_hi_h_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE_RANGE(fighter, 0.0, 23.0, 7.0);  // startup
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi1"), false, -1.0);
    }
    frame(lua_state, 22.0); // rush
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.0, 72, 48, 0, 60, 5.5, 0.0, 2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 10.0, 72, 48, 0, 60, 5.5, 0.0, 10.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 72, 48, 0, 50, 5.5, 0.0, 7.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 72, 48, 0, 50, 5.5, 0.0, 10.0, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    FT_MOTION_RATE_RANGE(fighter, 42.0, 74.0, 15.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "kirby", script = "effect_specialhih", category = ACMD_EFFECT, low_priority )]
unsafe fn kirby_special_hi_h_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 1, 6, 85, 120, -90, 1, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.66);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        effect!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0, 2, 0.25, Hash40::new("haver"), 0, 14, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 4, 6, -65, -100, -90, 1, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.66);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 0);
        effect!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0, 2, 0.25, Hash40::new("haver"), 0, 14, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 0);
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 6, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 6, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }

}

#[acmd_script( agent = "kirby", script = "sound_specialhih", category = ACMD_SOUND, low_priority )]
unsafe fn kirby_special_hi_h_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h02"));
        match smash::app::sv_math::rand(smash::hash40("fighter"), 2) {
            0 => PLAY_SE(fighter, Hash40::new("vc_kirby_002")),
            1 => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
            _ => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
        };
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h04"));
    }
}

#[acmd_script( agent = "kirby", script = "game_specialairhih", category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_air_hi_h_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE_RANGE(fighter, 0.0, 18.0, 6.0);  // startup
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi1"), false, -1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.0, 78, 48, 0, 80, 5.0, 0.0, 2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 10.0, 78, 48, 0, 80, 5.0, 0.0, 10.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0); // rush
    FT_MOTION_RATE_RANGE(fighter, 18.0, 29.0, 7.0);
    frame(lua_state, 29.0); // flip
    FT_MOTION_RATE_RANGE(fighter, 29.0, 48.0, 12.0);
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 48.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, vars::kirby::instance::DISABLE_SPECIAL_HI);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
    frame(lua_state, 56.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "kirby", script = "effect_specialairhih", category = ACMD_EFFECT, low_priority )]
unsafe fn kirby_special_air_hi_h_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 2, 0, 0, 90, -90, 1, false, *EF_FLIP_XY);
        LAST_EFFECT_SET_SCALE_W(fighter, 0.65, 1.0, 0.55);
        LAST_EFFECT_SET_RATE(fighter, 0.66);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        effect!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0, 2, 0.25, Hash40::new("haver"), 0, 14, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 0);
    }

}

#[acmd_script( agent = "kirby", script = "sound_specialairhih", category = ACMD_SOUND, low_priority )]
unsafe fn kirby_special_air_hi_h_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h02"));
        match smash::app::sv_math::rand(smash::hash40("fighter"), 2) {
            0 => PLAY_SE(fighter, Hash40::new("vc_kirby_002")),
            1 => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
            _ => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
        };
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h04"));
    }
}

#[acmd_script( agent = "kirby", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw"), false, -1.0);
    }
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_BLINK_ONOFF);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw_to_ground"), false, -1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 25, 30, 0, 86, 6.0, 0.0, 4.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 25, 30, 0, 86, 6.0, 0.0, 4.0, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::init_attack_pos(boma, 0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "kirby", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn kirby_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 30.0, 24.0);
    if is_excute(fighter) {
        //KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_air_lw"), false, -1.0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 100);
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_BLINK_ONOFF);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15.0);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        //KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw_to_air"), false, -1.0);
        AttackModule::clear(boma, 1, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 70, 76, 0, 69, 6.5, 0.0, 2.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::init_attack_pos(boma, 0);
    }
}

#[acmd_script( agent = "kirby", script = "effect_specialairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn kirby_special_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 3, 2, 7, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_XY);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("kirby_stone_s"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        FLASH(fighter, 0.706, 0.502, 0.392, 0.157);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.314, 0.235, 0.157, 0.235);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.706, 0.502, 0.392, 0.314);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.314, 0.235, 0.157, 0.392);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.706, 0.502, 0.392, 0.471);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.314, 0.235, 0.157, 0.549);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.706, 0.502, 0.392, 0.627);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.314, 0.235, 0.157, 0.706);
    }
}

pub fn install() {
    install_acmd_scripts!(
        kirby_special_n_eat_game,
        kirby_special_n_drink_game,
        kirby_special_n_large_game,
        kirby_special_s_start_game,
        kirby_special_s_game,
        kirby_special_ss_game,
        kirby_special_s_max_game,
        kirby_special_air_s_start_game,
        kirby_special_air_s_game,
        kirby_special_air_ss_game,
        game_specialairhi2,
        kirby_special_hi_h_game,
        kirby_special_hi_h_effect,
        kirby_special_hi_h_sound,
        kirby_special_air_hi_h_game,
        kirby_special_air_hi_h_effect,
        kirby_special_air_hi_h_sound,
        kirby_special_air_lw_game,
        kirby_special_air_lw_effect,
    
        // Copy Ability
        shizue_special_n_failure_effect,
        mario_special_n_game,
        mario_special_n_effect,
        mario_special_n_sound,
        mario_special_air_n_game,
        mario_special_air_n_effect,
        mario_special_air_n_sound,
        mariod_special_n_game,
        mariod_special_n_effect,
        mariod_special_air_n_effect,
        mariod_special_air_n_effect,
        luigi_special_n_game,
        luigi_special_n_effect,
        luigi_special_n_sound,
        luigi_special_air_n_game,
        luigi_special_air_n_effect,
        luigi_special_air_n_sound

    );
}

