use super::*;

unsafe extern "C" fn effect_koopaspecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
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

unsafe extern "C" fn sound_koopaspecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
    wait(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("vc_kirby_copy_koopa_01"));
    }
}

unsafe extern "C" fn game_koopaspecialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent,1.0,31.0,16.0);
}

unsafe extern "C" fn game_koopaspecialnmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,KOOPA_MAX_COOLDOWN);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START);
    }
}

unsafe extern "C" fn effect_koopaspecialnmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_sign"), Hash40::new("jaw"), 0, 1.0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent,1.5);

        if agent.is_motion(Hash40::new("koopa_special_n_max")){
            LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("sys_explosion_sign"),false,false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("koopa_breath_m_fire"),false,false);

        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("jaw"), 0, 0, 0, 180, 0, 50, 0.5, true);
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("koopa_appeal_s"), Hash40::new("mouth2"), 0, -1.3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent,2.0,0.5,0);
    }
}

unsafe extern "C" fn sound_koopaspecialnmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
    wait(lua_state, 19.0);
    if is_excute(agent) {
        if agent.is_motion(Hash40::new("koopa_special_n_max")){
            PLAY_SE_REMAIN(agent, Hash40::new("se_koopa_step_left_m"));
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_fire_m_damage"));
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_attack05"));
    }
}

unsafe extern "C" fn expression_koopaspecialnmax(agent: &mut L2CAgentBase) {
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
    agent.acmd("effect_koopaspecialnstart", effect_koopaspecialnstart);
    agent.acmd("effect_koopaspecialairnstart", effect_koopaspecialnstart);
    agent.acmd("sound_koopaspecialnstart", sound_koopaspecialnstart);
    agent.acmd("game_koopaspecialnend", game_koopaspecialnend);
    agent.acmd("game_koopaspecialairnend", game_koopaspecialnend);
    agent.acmd("game_koopaspecialnmax", game_koopaspecialnmax);
    agent.acmd("game_koopaspecialairnmax", game_koopaspecialnmax);
    agent.acmd("effect_koopaspecialnmax", effect_koopaspecialnmax);
    agent.acmd("effect_koopaspecialairnmax", effect_koopaspecialnmax);
    agent.acmd("sound_koopaspecialnmax", sound_koopaspecialnmax);
    agent.acmd("sound_koopaspecialairnmax", sound_koopaspecialnmax);
    agent.acmd(
        "expression_koopaspecialnmax",
        expression_koopaspecialnmax,
    );
    agent.acmd(
        "expression_koopaspecialairnmax",
        expression_koopaspecialnmax,
    );
}