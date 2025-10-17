use super::*;

unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 366, 100, 20, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn effect_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if VarModule::get_int(agent.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE) == 1 {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("pfushigisou_tanemg_tama"), Hash40::new("top"), 0, -6, 0, -90, 0, 0, 1.3, true);
            LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.0, 2.0);
            LAST_EFFECT_SET_COLOR(agent, 0.8, 0.8, 5.0);
        }
        for i in 1..=30 {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("sys_drown_out"), Hash40::new("top"), 0, 0, 0, 180, 0, 0, 0.4, false);
            }
            wait(lua_state, 15.0);
        }
    }
    else if VarModule::get_int(agent.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE) == 3 {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("pfushigisou_tanemg_tama"), Hash40::new("top"), 0, -6, 0, -90, 0, 0, 1.3, true);
            LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.0, 2.0);
            LAST_EFFECT_SET_COLOR(agent, 5.0, 0.8, 0.8);
        }
        for i in 1..=30 {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, false);
            }
            wait(lua_state, 10.0);
        }
    }
    else {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("pfushigisou_tanemg_tama"), Hash40::new("top"), 0, -6, 0, -90, 0, 0, 1.3, true);
            LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.0, 2.0);
        }
    }
}

unsafe extern "C" fn sound_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pfushigisou_special_n03"));
    }
}

unsafe extern "C" fn game_clash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 80, 147, 0, 22, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_clash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pfushigisou_tanemg_hit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn game_clashpledgew(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 80, 109, 0, 39, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("sys_hit_normal"));
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_water_hit_m"));
        AttackModule::set_poison_param(boma, 0, 121, 30, 2.0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_clashpledgew(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pfushigisou_tanemg_hit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("pzenigame_takinobori_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("pzenigame_takinobori_end"), -1);
    }
}

unsafe extern "C" fn game_clashpledgef(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 60, 156, 0, 30, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_clashpledgef(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pfushigisou_tanemg_hit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn game_clashground(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_move", game_move, Priority::Low);
    agent.acmd("effect_move", effect_move, Priority::Low);

    agent.acmd("game_clash", game_clash, Priority::Low);
    agent.acmd("effect_clash", effect_clash, Priority::Low);
    agent.acmd("sound_clash", acmd_stub, Priority::Low);

    agent.acmd("game_clashpledgew", game_clashpledgew, Priority::Low);
    agent.acmd("effect_clashpledgew", effect_clashpledgew, Priority::Low);
    agent.acmd("sound_clashpledgew", acmd_stub, Priority::Low);

    agent.acmd("game_clashpledgef", game_clashpledgef, Priority::Low);
    agent.acmd("effect_clashpledgef", effect_clashpledgef, Priority::Low);
    agent.acmd("sound_clashpledgef", acmd_stub, Priority::Low);

    agent.acmd("game_clashground", game_clashground, Priority::Low);
}