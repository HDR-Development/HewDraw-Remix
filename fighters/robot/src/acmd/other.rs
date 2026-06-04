use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_robot_damagefly02"));
        } else {
            let damage_speed_x = agent.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            let damage_speed_y = agent.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);

            let speed_vector = sv_math::vec2_length(damage_speed_x, damage_speed_y);

            let play_vc = if speed_vector < 3.8 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {
                PLAY_FLY_VOICE(agent, Hash40::new("seq_robot_rnd_futtobi01"), Hash40::new("seq_robot_rnd_futtobi02"));
            }
        }
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_robot_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_robot_rnd_futtobi01"), Hash40::new("seq_robot_rnd_futtobi02"));
        }
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_robot_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_robot_landing01"));
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }

}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }

}

unsafe extern "C" fn game_itemlightthrowdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 6.0);
    if is_excute(agent) {
        agent.clear_lua_stack(); 
        lua_args!(agent, 19, 10, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_ANGLE, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_SPEED, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER);
        sv_animcmd::THROW_ITEM_OFFSET(agent.lua_state_agent);
    }
}

unsafe extern "C" fn effect_squatf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    loop {
        frame(lua_state, 2.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("robot_footmark"), Hash40::new("knee"), 1.5, 0, 0, 0, -2, 90, 1, true);
            FOOT_EFFECT(agent, Hash40::new("robot_run_smoke"), Hash40::new("top"), -3.8, 0, 0, 0, 0, 0, 0.9, 1, 0, 0, 0, 0, 0, false);
        }
        wait_loop_clear(agent);
    }
}

unsafe extern "C" fn sound_squatf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        PLAY_STATUS(agent, Hash40::new("se_robot_step_s"));
    }
}

unsafe extern "C" fn expression_squatf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    loop {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TPART, 5);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 8.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait_loop_clear(agent);
    }
}

unsafe extern "C" fn effect_squatb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    loop { 
        frame(lua_state, 2.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("robot_footmark"), Hash40::new("knee"), 1.5, 0, 0, 0, -2, 90, 1, true);
            FOOT_EFFECT(agent, Hash40::new("robot_run_smoke"), Hash40::new("top"), 3.8, 0, 0, 180, 0, 0, 0.9, 1, 0, 0, 0, 0, 0, false);
        }
        wait_loop_clear(agent);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffescape", acmd_stub, Priority::Low);

    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damageflyroll, Priority::Low);

    agent.acmd("game_dash", game_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("game_itemlightthrowdash", game_itemlightthrowdash, Priority::Low);

    agent.acmd("game_squatf", acmd_stub, Priority::Low);
    agent.acmd("effect_squatf", effect_squatf, Priority::Low);
    agent.acmd("sound_squatf", sound_squatf, Priority::Low);
    agent.acmd("expression_squatf", expression_squatf, Priority::Low);

    agent.acmd("game_squatb", acmd_stub, Priority::Low);
    agent.acmd("effect_squatb", effect_squatb, Priority::Low);
    agent.acmd("sound_squatb", sound_squatf, Priority::Low);
    agent.acmd("expression_squatb", expression_squatf, Priority::Low);
}
