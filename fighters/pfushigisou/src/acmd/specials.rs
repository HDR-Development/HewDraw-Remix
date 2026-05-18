use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    FT_MOTION_RATE_RANGE(agent, 3.0, 15.0, 6.0);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if agent.kind() == *FIGHTER_KIND_KIRBY {
            if ![*PLEDGE_STATE_NONE, *PLEDGE_STATE_GRASS].contains(&VarModule::get_int(agent.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE)) {
                let timer = VarModule::get_int(agent.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
                VarModule::set_int(agent.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer - 90);
            }
        }
        else {
            if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
                let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
                let object = utils::util::get_battle_object_from_id(parent_id);
                if ![*PLEDGE_STATE_NONE, *PLEDGE_STATE_GRASS].contains(&VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE)) {
                    let timer = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
                    let pledge_use_cost_frame = ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.pledge_use_cost_frame");
                    VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer - (pledge_use_cost_frame / 2));
                }
            }  
        }
        ArticleModule::generate_article(boma, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_SEED, false, -1);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 5.0, 5.0);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if agent.kind() == *FIGHTER_KIND_KIRBY {
            if ![*PLEDGE_STATE_NONE, *PLEDGE_STATE_GRASS].contains(&VarModule::get_int(agent.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE)) {
                let timer = VarModule::get_int(agent.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
                VarModule::set_int(agent.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer - 90);
            }
        }
        else {
            if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
                let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
                let object = utils::util::get_battle_object_from_id(parent_id);
                if ![*PLEDGE_STATE_NONE, *PLEDGE_STATE_GRASS].contains(&VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE)) {
                    let timer = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
                    let pledge_use_cost_frame = ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.pledge_use_cost_frame");
                    VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer - (pledge_use_cost_frame / 2));
                }
            }  
        }
        ArticleModule::generate_article(boma, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_SEED, false, -1);
    }
}

unsafe extern "C" fn effect_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("pfushigisou_tanemg"), Hash40::new("flower"), 5.7, 0, 0, 0, 0, 0, 1.3, true);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_ALPHA(agent, 0.8);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pfushigisou_leaf"), Hash40::new("flower"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 360, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("pfushigisou_tanemg"), Hash40::new("flower"), 5.7, 0, 0, 0, 0, 0, 1.3, true);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_ALPHA(agent, 0.8);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pfushigisou_leaf"), Hash40::new("flower"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 360, 0, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_ALPHA(agent, 0.5);
        }
    }
}

unsafe extern "C" fn sound_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pfushigisou_special_n03"));
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pfushigisou_special_n03"));
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pfushigisou_landing01"));
    }
}

unsafe extern "C" fn expression_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 22.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            agent.on_flag(*FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_S_FLAG_SMASH);
        }
        ArticleModule::generate_article(boma, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_LEAFCUTTER, false, 0);
    }
    frame(lua_state, 29.0);
    FT_MOTION_RATE(agent, 0.9);
    frame(lua_state, 49.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.off_flag(*FIGHTER_PFUSHIGISOU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_HOP);
        ArticleModule::generate_article(boma, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_VINE, false, 0);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_VINE, false, app::ArticleOperationTarget(0));
        WorkModule::on_flag(boma, *FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_HI_SET_MAP_COLL_OFFSET);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 12.0, 16.0);
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::set_float(boma, 35.0 * agent.lr(), *FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_HI_FLOAT_ANGLE);
        ATTACK(agent, 0, 0, Hash40::new("viner2"), 12.0, 60, 68, 0, 52, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("viner3"), 12.0, 60, 68, 0, 52, 4.2, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("viner4"), 12.0, 60, 68, 0, 52, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("viner5"), 12.0, 60, 68, 0, 52, 2.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 4, 0, Hash40::new("viner5"), 15.0, 65, 85, 0, 52, 4.5, 8.3, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::set_optional_hit_sound(boma, 4, Hash40::new("se_common_slap_hit_l"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                CancelModule::enable_cancel(boma);
            }
            else {
                VarModule::on_flag(agent.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            }
        } 
    }
}

unsafe extern "C" fn game_speciallwin(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_OFF), 0);
        if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
            let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
            let object = utils::util::get_battle_object_from_id(parent_id);
            VarModule::on_flag(object, vars::ptrainer::instance::SPECIAL_LW_BACKWARDS_SWITCH); // we will turn this off in opff
            //if VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE) == *PLEDGE_STATE_NONE {
                VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, *PLEDGE_STATE_GRASS);
                let pledge_duration_frame = ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.pledge_duration_frame");
                VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, pledge_duration_frame);
            //}
            let swap_lockout_frame = ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.swap_lockout_frame");
            VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER, swap_lockout_frame);
            VarModule::on_flag(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("effect_specialnstart", acmd_stub, Priority::Low);
    agent.acmd("game_specialairnstart", game_specialnstart, Priority::Low);
    agent.acmd("effect_specialairnstart", acmd_stub, Priority::Low);

    agent.acmd("game_specialn", acmd_stub, Priority::Low);
    agent.acmd("game_specialairn", acmd_stub, Priority::Low);

    agent.acmd("game_specialnend", game_specialnend, Priority::Low);
    agent.acmd("effect_specialnend", effect_specialnend, Priority::Low);
    agent.acmd("sound_specialnend", sound_specialnend, Priority::Low);
    agent.acmd("expression_specialnend", expression_specialnend, Priority::Low);
    agent.acmd("game_specialairnend", game_specialnend, Priority::Low);
    agent.acmd("effect_specialairnend", effect_specialnend, Priority::Low);
    agent.acmd("sound_specialairnend", sound_specialnend, Priority::Low);
    agent.acmd("expression_specialairnend", expression_specialnend, Priority::Low);

    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairs", game_specials, Priority::Low);
    
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);

    agent.acmd("game_speciallwin", game_speciallwin, Priority::Low);
    agent.acmd("game_specialairlwin", game_speciallwin, Priority::Low);
}