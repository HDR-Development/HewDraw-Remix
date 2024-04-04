use super::*;
use smash::app::lua_bind::ItemManager::get_num_of_active_item;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        /*
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) || (ControlModule::get_stick_y(boma) >= 0.5)){
            if VarModule::get_int(agent.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER) < 2 {
                if(get_num_of_active_item(*ITEM_KIND_SENSORBOMB) < 1){
                    VarModule::inc_int(agent.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER);
                    ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_SENSORBOMB), 0, 0, false, false);
                }
            }
        }
        else if(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) || (ControlModule::get_stick_y(boma) <= -0.5)){
            if VarModule::get_int(agent.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER) < 2 {
                if(get_num_of_active_item(*ITEM_KIND_SMOKESCREEN) < 1){
                    VarModule::inc_int(agent.battle_object, vars::snake::instance::SNAKE_GRENADE_COUNTER);
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
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.333);
    }
}

unsafe extern "C" fn game_specialsstart(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        if VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_RELOAD_VULNERABLE) {
            VarModule::off_flag(agent.battle_object, vars::snake::instance::TRANQ_RELOAD_VULNERABLE);
            VarModule::on_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD);
        }
    }
    frame(lua_state, 1.0);
    if VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
        FT_MOTION_RATE_RANGE(agent, 1.0, 16.0, 12.0);
    }
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, false, 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if boma.is_button_on(Buttons::Guard) {
            VarModule::on_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD);
        }
    }
    frame(lua_state, 16.0);
    if VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
        FT_MOTION_RATE_RANGE(agent, 16.0, 38.0, 1.0);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            VarModule::inc_int(agent.battle_object, vars::snake::instance::TRANQ_AMMO_COUNT);
            ArticleModule::set_flag(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, true, *WEAPON_SNAKE_NIKITA_INSTANCE_WORK_ID_FLAG_SHOOT);
            if VarModule::get_int(agent.battle_object, vars::snake::instance::TRANQ_AMMO_COUNT) == 3 {
                VarModule::on_flag(agent.battle_object, vars::snake::instance::TRANQ_RELOAD_VULNERABLE);
            }
        }
    }
    frame(lua_state, 38.0);
    if VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
        FT_MOTION_RATE(agent, 1.0);
    }
    else {
        FT_MOTION_RATE_RANGE(agent, 38.0, 64.0, 1.0);
    }
    frame(lua_state, 64.0);
    if !VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 79.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            VarModule::set_int(agent.battle_object, vars::snake::instance::TRANQ_AMMO_COUNT, 0);
            VarModule::off_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD);
        }
    }
    frame(lua_state, 83.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn effect_specialsstart(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -3, 10, 0, 0, 0, 0, 0.4, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -1, 10, 10, 0, 0, 0, 0.6, true);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_bananagun_shot"), Hash40::new("haver"), 0, 0.5, 3, 0, 0, 0, 0.4, true);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 0, 1, 4.5, 0, 0, 0, 0.2, true);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_RELOAD_VULNERABLE) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_piyo"), Hash40::new("head"), 2.5, 0, 2, 0, 80, 0, 1.0, true);
        }
    }
}

unsafe extern "C" fn sound_specialsstart(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_snake_special_l05"))
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            PLAY_SE(agent, Hash40::new("se_snake_special_s01"));
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_RELOAD_VULNERABLE) {
            let sfx_handle = SoundModule::play_se(boma, smash::phx::Hash40::new("vc_snake_heavyget"), true, false, false, false, app::enSEType(0));
            SoundModule::set_se_vol(boma, sfx_handle as i32, 3.0, 0);
        }
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_snake_special_s02"));
    }
    wait(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_snake_special_s03"));
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_snake_special_s07"))
    }
}

unsafe extern "C" fn expression_specialsstart(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::snake::instance::TRANQ_NEED_RELEOAD) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
        }
    }
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_CYPHER, false, 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 55, 0, 30, 5.5, 0.0, 8.75, -2.0, None, None, None, 0.8, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 361, 55, 0, 30, 5.5, 0.0, 8.75, 2.0, None, None, None, 0.8, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_speciallwblast(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, 0);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 27.0);
    FT_MOTION_RATE_RANGE(agent, 27.0, 40.0, 13.0);
    if is_excute(agent) {
        if !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)) {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
        }
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_speciallwsquatblast(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, 0);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 27.0);
    FT_MOTION_RATE_RANGE(agent, 27.0, 40.0, 13.0);
    if is_excute(agent) {
        if !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)) {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
        }
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairhihang(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 7);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_SNAKE_STATUS_CYPHER_HANG_TRANS_ID_CUT_STICK);
    }
    WorkModule::unable_transition_term_group_ex(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::unable_transition_term_group_ex(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::unable_transition_term_group_ex(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 89.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_SNAKE_STATUS_CYPHER_HANG_TRANS_ID_CUT_TIME_OUT);
    }
}

unsafe extern "C" fn game_speciallwselfstick(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_STICK_TARGET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_speciallwsquatselfstick(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_STICK_TARGET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart);
    agent.acmd("game_specialairnstart", game_specialnstart);

    agent.acmd("game_specialsstart", game_specialsstart);
    agent.acmd("game_specialairsstart", game_specialsstart);
    agent.acmd("effect_specialsstart", effect_specialsstart);
    agent.acmd("effect_specialairsstart", effect_specialsstart);
    agent.acmd("sound_specialsstart", sound_specialsstart);
    agent.acmd("sound_specialairsstart", sound_specialsstart);
    agent.acmd("expression_specialsstart", expression_specialsstart);
    agent.acmd("expression_specialairsstart", expression_specialsstart);

    agent.acmd("game_specialhistart", game_specialhistart);
    agent.acmd("game_specialairhistart", game_specialhistart);
    agent.acmd("game_specialairhihang", game_specialairhihang);

    agent.acmd("game_speciallwblast", game_speciallwblast);
    agent.acmd("game_specialairlwblast", game_speciallwblast);
    agent.acmd("game_speciallwsquatblast", game_speciallwsquatblast);

    agent.acmd("game_speciallwselfstick", game_speciallwselfstick);
    agent.acmd("game_specialairlwselfstick", game_speciallwselfstick);
    agent.acmd("game_speciallwsquatselfstick", game_speciallwsquatselfstick);
}