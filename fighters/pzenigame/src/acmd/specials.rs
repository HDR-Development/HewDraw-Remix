use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 10.0, 6.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 19.0, 2.0);
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialnshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PZENIGAME_STATUS_SPECIAL_N_FLAG_SHOOT_ANGLE_ENABLE);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if agent.kind() == *FIGHTER_KIND_KIRBY {
            if ![*PLEDGE_STATE_NONE, *PLEDGE_STATE_WATER].contains(&VarModule::get_int(agent.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE)) {
                let timer = VarModule::get_int(agent.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
                VarModule::set_int(agent.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer - 180);
            }
        }
        else {
            if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
                let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
                let object = utils::util::get_battle_object_from_id(parent_id);
                if ![*PLEDGE_STATE_NONE, *PLEDGE_STATE_WATER].contains(&VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE)) {
                    let timer = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
                    let pledge_use_cost_frame = ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.pledge_use_cost_frame");
                    VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer - pledge_use_cost_frame);
                }
            }
        }
        ArticleModule::generate_article(boma, *FIGHTER_PZENIGAME_GENERATE_ARTICLE_WATER, false, -1);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_PZENIGAME_STATUS_SPECIAL_N_FLAG_SHOOT_ANGLE_ENABLE);
    }
}

unsafe extern "C" fn effect_specialnshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false); 
        }
        if agent.lr() < 0.0 {
            EFFECT_FLW_POS(agent, Hash40::new("pzenigame_mizuteppo_shoot"), Hash40::new("head"), -0.5, 2.7, 0, 0, 0, -13, 0.8, true);
        }
        else {
            EFFECT_FLW_POS(agent, Hash40::new("pzenigame_mizuteppo_shoot"), Hash40::new("head"), -0.5, 2.7, 0, 0, 0, 13, 0.8, true);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pzenigame_mizuteppo_shoot"), false, false);
    }
}

unsafe extern "C" fn sound_specialnshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pzenigame_special_n03"));
        let rand = sv_math::rand(hash40("fighter"), 2);
        if rand == 1 {
            PLAY_SE(agent, Hash40::new("vc_pzenigame_attack02"));
        }
        else {
            PLAY_SE(agent, Hash40::new("vc_pzenigame_special_n01"));
        }
    }
}

unsafe extern "C" fn expression_specialnshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_waterjets"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 7.0, 1.0);
    frame(lua_state, 7.0);
    FT_MOTION_RATE_RANGE(agent, 7.0, 17.0, 10.0);
    frame(lua_state, 17.0);
    FT_MOTION_RATE_RANGE(agent, 17.0, 21.0, 1.0);
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("bust"), 8.0, 49, 48, 0, 75, 2.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ZENIGAME_SHELLHIT, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn game_specialairshit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 20.0, 15.0);
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 26.0, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(agent, 9.0, 43.0, 24.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 100, 40, 0, 6.0, 0.0, 4.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 43.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 75, 121, 0, 60, 7.0, 0.0, 4.0, 6.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        AttackModule::clear_all(boma);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT(agent, Hash40::new("pzenigame_takinobori_start"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pzenigame_takinobori_splash"), Hash40::new("top"), 0, 0, 2, -55, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("pzenigame_takinobori"), Hash40::new("top"), 0, 12, 12, -50, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_takinobori_end"), Hash40::new("top"), 0, 5, 9, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
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
                VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, *PLEDGE_STATE_WATER);
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
    agent.acmd("game_specialairnstart", game_specialnstart, Priority::Low);

    agent.acmd("game_specialnshot", game_specialnshot, Priority::Low);
    agent.acmd("effect_specialnshot", effect_specialnshot, Priority::Low);
    agent.acmd("sound_specialnshot", sound_specialnshot, Priority::Low);
    agent.acmd("expression_specialnshot", expression_specialnshot, Priority::Low);
    agent.acmd("game_specialairnshot", game_specialnshot, Priority::Low);
    agent.acmd("effect_specialairnshot", effect_specialnshot, Priority::Low);
    agent.acmd("sound_specialairnshot", sound_specialnshot, Priority::Low);
    agent.acmd("expression_specialairnshot", expression_specialnshot, Priority::Low);

    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairs", game_specials, Priority::Low);
    agent.acmd("game_specialairshit", game_specialairshit, Priority::Low);
    agent.acmd("game_specialsend", game_specialsend, Priority::Low);
    agent.acmd("game_specialairsend", game_specialsend, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialhi, Priority::Low);

    agent.acmd("game_speciallwin", game_speciallwin, Priority::Low);
    agent.acmd("game_specialairlwin", game_speciallwin, Priority::Low);
}