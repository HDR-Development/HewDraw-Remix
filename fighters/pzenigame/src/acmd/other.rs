use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_pzenigame_damagefly02"));
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
                PLAY_FLY_VOICE(agent, Hash40::new("seq_pzenigame_rnd_futtobi01"), Hash40::new("seq_pzenigame_rnd_futtobi02"));
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
            PLAY_SE(agent, Hash40::new("vc_pzenigame_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_pzenigame_rnd_futtobi01"), Hash40::new("seq_pzenigame_rnd_futtobi02"));
        }
    }
}

unsafe extern "C" fn effect_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_dash"), Hash40::new("top"), 2.5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.56, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_pzenigame_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn effect_runbrake(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.56, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_runbrakel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.56, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_runbraker(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.56, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_turnrun(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_run"), Hash40::new("top"), 0, 0, 1.5, 0, 180, 0, 0.84, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
}

unsafe extern "C" fn effect_turnrunbrake(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 0.56, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
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

unsafe extern "C" fn game_appealhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if is_training_mode() && LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
            let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
            let object = utils::util::get_battle_object_from_id(parent_id);
            let pledge = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
            let (state, timer) = match pledge {
                2 /* GRASS */ => (*PLEDGE_STATE_NONE, 0),
                _ => (*PLEDGE_STATE_GRASS, ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.pledge_duration_frame_training"))
            };
            VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, state);
            VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer);
        }
    }
}

unsafe extern "C" fn effect_appealhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if is_training_mode() && LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
            EffectModule::kill_kind(boma, Hash40::new("sys_status_attack_up"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("sys_status_defense_up"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("sys_status_speed_up"), false, false);
            let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
            let object = utils::util::get_battle_object_from_id(parent_id);
            let pledge = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE) as i32;
            if pledge == *PLEDGE_STATE_GRASS {
                VarModule::set_int(boma.object(), vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
            } else {
                let handle = EffectModule::req_follow(boma, Hash40::new("sys_status_speed_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
                VarModule::set_int(agent.battle_object, vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, handle as i32);
                boma.play_pledge_effect(*PLEDGE_STATE_GRASS);
            }
        }
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if is_training_mode() && LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
            let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
            let object = utils::util::get_battle_object_from_id(parent_id);
            let pledge = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
            let (state, timer) = match pledge {
                3 /* FIRE */ => (*PLEDGE_STATE_NONE, 0),
                _ => (*PLEDGE_STATE_FIRE, ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.pledge_duration_frame_training"))
            };
            VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, state);
            VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer);
        }
    }
}

unsafe extern "C" fn effect_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if is_training_mode() && LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
            EffectModule::kill_kind(boma, Hash40::new("sys_status_attack_up"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("sys_status_defense_up"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("sys_status_speed_up"), false, false);
            let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
            let object = utils::util::get_battle_object_from_id(parent_id);
            let pledge = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE) as i32;
            if pledge == *PLEDGE_STATE_FIRE {
                VarModule::set_int(boma.object(), vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
            } else {
                let handle = EffectModule::req_follow(boma, Hash40::new("sys_status_attack_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
                VarModule::set_int(agent.battle_object, vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, handle as i32);
                boma.play_pledge_effect(*PLEDGE_STATE_FIRE);
            }
        }
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -1.5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), -1.2, 0, 0, 0, 0, 0, 0.55, false, *EF_FLIP_NONE);
    }
}

unsafe extern "C" fn game_win3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ModelModule::set_scale(boma, 0.97);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffescape", acmd_stub, Priority::Low);

    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damageflyroll, Priority::Low);

    agent.acmd("effect_dash", effect_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);

    agent.acmd("effect_runbrake", effect_runbrake, Priority::Low);
    agent.acmd("effect_runbrakel", effect_runbrakel, Priority::Low);
    agent.acmd("effect_runbraker", effect_runbraker, Priority::Low);
    agent.acmd("effect_turnrun", effect_turnrun, Priority::Low);
    agent.acmd("effect_turnrunbrake", effect_turnrunbrake, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("game_appealhil", game_appealhi, Priority::Low);
    agent.acmd("game_appealhir", game_appealhi, Priority::Low);
    agent.acmd("effect_appealhil", effect_appealhi, Priority::Low);
    agent.acmd("effect_appealhir", effect_appealhi, Priority::Low);

    agent.acmd("game_appeallwl", game_appeallw, Priority::Low);
    agent.acmd("game_appeallwr", game_appeallw, Priority::Low);
    agent.acmd("effect_appeallwl", effect_appeallw, Priority::Low);
    agent.acmd("effect_appeallwr", effect_appeallw, Priority::Low);

    agent.acmd("game_win3", game_win3, Priority::Low);
}
