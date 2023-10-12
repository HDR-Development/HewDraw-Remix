
use super::*;
use smash::app::lua_bind::ItemManager::get_num_of_active_item;


#[acmd_script( agent = "snake", script = "game_specialhistart" , category = ACMD_GAME , low_priority)]
unsafe fn snake_special_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_CYPHER, false, 0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 55, 0, 30, 5.5, 0.0, 8.75, -2.0, None, None, None, 0.8, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 361, 55, 0, 30, 5.5, 0.0, 8.75, 2.0, None, None, None, 0.8, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "snake", script = "game_specialairhistart" , category = ACMD_GAME , low_priority)]
unsafe fn snake_special_air_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_CYPHER, false, 0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 55, 0, 30, 5.5, 0.0, 8.75, -2.0, None, None, None, 0.8, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 361, 55, 0, 30, 5.5, 0.0, 8.75, 2.0, None, None, None, 0.8, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "snake", script = "game_speciallwblast" , category = ACMD_GAME , low_priority)]
unsafe fn snake_special_lw_blast_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, 0);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 27.0);
    FT_MOTION_RATE_RANGE(fighter, 27.0, 40.0, 13.0);
    if is_excute(fighter) {
        if !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)) {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
        }
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

#[acmd_script( agent = "snake", script = "game_speciallwsquatblast" , category = ACMD_GAME , low_priority)]
unsafe fn snake_special_lw_squat_blast_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, 0);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 27.0);
    FT_MOTION_RATE_RANGE(fighter, 27.0, 40.0, 13.0);
    if is_excute(fighter) {
        if !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)) {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
        }
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

#[acmd_script( agent = "snake", script = "game_specialairlwblast" , category = ACMD_GAME , low_priority)]
unsafe fn snake_special_air_lw_blast_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, 0);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 27.0);
    FT_MOTION_RATE_RANGE(fighter, 27.0, 40.0, 13.0);
    if is_excute(fighter) {
        if !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)) {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
        }
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

#[acmd_script( agent = "snake", script = "game_specialnstart" , category = ACMD_GAME , low_priority)]
unsafe fn snake_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        /*
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) || (ControlModule::get_stick_y(boma) >= 0.5)){
            if VarModule::get_int(fighter.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER) < 2 {
                if(get_num_of_active_item(*ITEM_KIND_SENSORBOMB) < 1){
                    VarModule::inc_int(fighter.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER);
                    ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_SENSORBOMB), 0, 0, false, false);
                }
            }
        }
        else if(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) || (ControlModule::get_stick_y(boma) <= -0.5)){
            if VarModule::get_int(fighter.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER) < 2 {
                if(get_num_of_active_item(*ITEM_KIND_SMOKESCREEN) < 1){
                    VarModule::inc_int(fighter.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER);
                    ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_SMOKESCREEN), 0, 0, false, false);
                }
            }
        }
        */
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, 0);
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if ArticleModule::is_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.333);
    }
    
}

#[acmd_script( agent = "snake", script = "game_specialairnstart" , category = ACMD_GAME , low_priority)]
unsafe fn snake_special_air_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        /*
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) || (ControlModule::get_stick_y(boma) >= 0.5)){
            if VarModule::get_int(fighter.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER) < 2 {
                if(get_num_of_active_item(*ITEM_KIND_SENSORBOMB) < 1){
                    VarModule::inc_int(fighter.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER);
                    ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_SENSORBOMB), 0, 0, false, false);
                }
            }
        }
        else if(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) || (ControlModule::get_stick_y(boma) <= -0.5)){
            if VarModule::get_int(fighter.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER) < 2 {
                if(get_num_of_active_item(*ITEM_KIND_SMOKESCREEN) < 1){
                    VarModule::inc_int(fighter.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER);
                    ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_SMOKESCREEN), 0, 0, false, false);
                }
            }
        }
        */
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, 0);
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, 0);
        if(ArticleModule::is_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN)){
            ArticleModule::set_visibility_whole(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if ArticleModule::is_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.333);
    }
    
}

#[acmd_script( agent = "snake", script = "game_specialairhihang", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhihang(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        //damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 7);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_SNAKE_STATUS_CYPHER_HANG_TRANS_ID_CUT_STICK);
    }
    WorkModule::unable_transition_term_group_ex(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::unable_transition_term_group_ex(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::unable_transition_term_group_ex(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 89.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_SNAKE_STATUS_CYPHER_HANG_TRANS_ID_CUT_TIME_OUT);
    }
}

#[acmd_script( agent = "snake_cypher", script = "game_detach", category = ACMD_GAME, low_priority )]
unsafe fn game_cypher_detach(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 42, 17, 0, 63, 2.5, 0.0, 1.5, -3.2, Some(0.0), Some(1.5), Some(3.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        //ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 42, 17, 0, 63, 3.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

//new self-stick scripts
#[acmd_script( agent = "snake", script = "game_speciallwselfstick", category = ACMD_GAME, low_priority )]
unsafe fn snake_down_special_floor(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_STICK_TARGET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "snake", script = "game_speciallwsquatselfstick", category = ACMD_GAME, low_priority )]
unsafe fn snake_down_special_crouch_floor(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_STICK_TARGET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "snake", script = "game_specialairlwselfstick", category = ACMD_GAME, low_priority )]
unsafe fn snake_down_special_air_floor(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_STICK_TARGET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "snake", scripts = ["game_specialsstart", "game_specialairsstart"], category = ACMD_GAME, low_priority )]
unsafe fn snake_side_special_game(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        if VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_RELOAD_VULNERABLE) {
            VarModule::off_flag(fighter.battle_object, vars::snake::instance::TRANQ_RELOAD_VULNERABLE);
            VarModule::on_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD);
        }
    }
    frame(lua_state, 1.0);
    if VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
        FT_MOTION_RATE_RANGE(fighter, 1.0, 16.0, 12.0);
    }
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, false, 0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if boma.is_button_on(Buttons::Guard) {
            VarModule::on_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD);
        }
    }
    frame(lua_state, 16.0);
    if VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
        FT_MOTION_RATE_RANGE(fighter, 16.0, 38.0, 1.0);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            VarModule::inc_int(fighter.battle_object, vars::snake::instance::TRANQ_AMMO_COUNT);
            ArticleModule::set_flag(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, true, *WEAPON_SNAKE_NIKITA_INSTANCE_WORK_ID_FLAG_SHOOT);
            if VarModule::get_int(fighter.battle_object, vars::snake::instance::TRANQ_AMMO_COUNT) == 3 {
                VarModule::on_flag(fighter.battle_object, vars::snake::instance::TRANQ_RELOAD_VULNERABLE);
            }
        }
    }
    frame(lua_state, 38.0);
    if VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    else {
        FT_MOTION_RATE_RANGE(fighter, 38.0, 64.0, 1.0);
    }
    frame(lua_state, 64.0);
    if !VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 79.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            VarModule::set_int(fighter.battle_object, vars::snake::instance::TRANQ_AMMO_COUNT, 0);
            VarModule::off_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD);
        }
    }
    frame(lua_state, 83.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, ArticleOperationTarget(0));
    }
}

#[acmd_script( agent = "snake", scripts = ["expression_specialsstart", "expression_specialairsstart"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn snake_side_special_expr(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
        }
    }
}

#[acmd_script( agent = "snake", scripts = ["sound_specialsstart", "sound_specialairsstart"], category = ACMD_SOUND, low_priority )]
unsafe fn snake_side_special_snd(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_l05"))
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            PLAY_SE(fighter, Hash40::new("se_snake_special_s01"));
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_RELOAD_VULNERABLE) {
            let sfx_handle = SoundModule::play_se(fighter.module_accessor, smash::phx::Hash40::new("vc_snake_heavyget"), true, false, false, false, app::enSEType(0));
            SoundModule::set_se_vol(fighter.module_accessor, sfx_handle as i32, 3.0, 0);
        }
    }
    frame(lua_state, 41.0);
    if macros::is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s02"));
    }
    wait(lua_state, 11.0);
    if macros::is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s03"));
    }
    frame(lua_state, 80.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s07"))
    }
}

#[acmd_script( agent = "snake", scripts = ["effect_specialsstart", "effect_specialairsstart"], category = ACMD_EFFECT, low_priority )]
unsafe fn snake_side_special_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -3, 10, 0, 0, 0, 0, 0.4, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -1, 10, 10, 0, 0, 0, 0.6, true);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_bananagun_shot"), Hash40::new("haver"), 0, 0.5, 3, 0, 0, 0, 0.4, true);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 0, 1, 4.5, 0, 0, 0, 0.2, true);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::snake::instance::TRANQ_RELOAD_VULNERABLE) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_piyo"), Hash40::new("head"), 2.5, 0, 2, 0, 80, 0, 1.0, true);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_cypher_detach,
        game_specialairhihang,
        snake_special_hi_start_game,
        snake_special_air_hi_start_game,
        snake_special_lw_blast_game,
        snake_special_lw_squat_blast_game,
        snake_special_air_lw_blast_game,
        snake_special_n_start_game,
        snake_special_air_n_start_game,
        snake_down_special_floor,
        snake_down_special_crouch_floor,
        snake_down_special_air_floor,
        snake_side_special_game,
        snake_side_special_expr,
        snake_side_special_snd,
        snake_side_special_eff,
    );
}

