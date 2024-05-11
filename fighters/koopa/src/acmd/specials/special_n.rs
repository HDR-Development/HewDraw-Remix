use super::*;

unsafe extern "C" fn effect_specialairnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    wait(lua_state, 20.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn game_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 31.0, 16.0);
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 50, 90, 0, 55, 6.0, 0.0, 9.0, 7.0, Some(0.0), Some(9.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_piyo"), Hash40::new("mouth2"), 0, 1, 1, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn game_specialnmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,MAX_COOLDOWN);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 42.0, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START);
    }
    frame(lua_state, 42.0);
    FT_MOTION_RATE_RANGE(agent, 42.0, 65.0, 21.0);
    frame(lua_state, 65.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialnmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_sign"), Hash40::new("jaw"), 0, 0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        if agent.is_motion(Hash40::new("special_n_max")){
            LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("koopa_appeal_s"), Hash40::new("mouth2"), 0, -1.3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent,2.0,0.5,0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_sign"), false, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("jaw"), 3, 1, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(agent, 1.25);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("koopa_breath_m_fire"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("jaw"), 0, 0, 0, 180, 0, 50, 0.5, true);
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("koopa_wait_breath"), Hash40::new("head"), -1.5, 3, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.8, 0.8, 0.8);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("koopa_wait_breath"), false, false);
    }
}

unsafe extern "C" fn sound_specialairnmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
        PLAY_SE(agent, Hash40::new("vc_koopa_furasleep"));
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        if agent.is_motion(Hash40::new("special_n_max")) {
            PLAY_SE_REMAIN(agent, Hash40::new("se_koopa_step_left_m"));
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_fire_m_damage"));
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_koopa_attack06"));
    }
}

unsafe extern "C" fn expression_specialnmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_specialnstart", effect_specialairnstart, Priority::Low);
    agent.acmd("effect_specialairnstart", effect_specialairnstart, Priority::Low);

    agent.acmd("game_specialnend", game_specialnend, Priority::Low);
    agent.acmd("game_specialairnend", game_specialnend, Priority::Low);
    agent.acmd("effect_specialnend", effect_specialnend, Priority::Low);
    agent.acmd("effect_specialairnend", effect_specialnend, Priority::Low);
    
    agent.acmd("game_specialnmax", game_specialnmax, Priority::Low);
    agent.acmd("game_specialairnmax", game_specialnmax, Priority::Low);
    agent.acmd("effect_specialnmax", effect_specialnmax, Priority::Low);
    agent.acmd("effect_specialairnmax", effect_specialnmax, Priority::Low);
    agent.acmd("sound_specialnmax", sound_specialairnmax, Priority::Low);
    agent.acmd("sound_specialairnmax", sound_specialairnmax, Priority::Low);
    agent.acmd("expression_specialnmax", expression_specialnmax, Priority::Low);
    agent.acmd("expression_specialairnmax", expression_specialnmax, Priority::Low);
}