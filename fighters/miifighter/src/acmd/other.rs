use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_miifighter_damagefly02"));
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
                PLAY_FLY_VOICE(agent, Hash40::new("seq_miifighter_rnd_futtobi01"), Hash40::new("seq_miifighter_rnd_futtobi02"));
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
            PLAY_SE(agent, Hash40::new("vc_miifighter_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_miifighter_rnd_futtobi01"), Hash40::new("seq_miifighter_rnd_futtobi02"));
        }
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_miifighter_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 12.0);
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

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if app::smashball::is_training_mode()
        && agent.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_LW_NO) == 2
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            gimmick_flash(boma);
            let stage = VarModule::get_int(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE);
            match stage {
                0 => {
                    let handle = EffectModule::req_follow(boma, Hash40::new("sys_steam1"), Hash40::new("head"), &Vector3f::new(3.0, 0.0, 0.0), &Vector3f::zero(), 0.8, false, 0, 0, 0, 0, 0, false, false);
                    EffectModule::set_alpha(boma, handle as u32, 3.0);
                    VarModule::set_int(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_1, handle as i32);
                    VarModule::inc_int(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE);
                },
                1 => {
                    //let handle = EffectModule::req_follow(boma, Hash40::new("sys_steam2"), Hash40::new("head"), &Vector3f::new(3.0, 0.0, 0.0), &Vector3f::zero(), 0.8, false, 0, 0, 0, 0, 0, false, false);
                    //EffectModule::set_alpha(boma, handle as u32, 3.0);
                    //VarModule::set_int(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_2, handle as i32);
                    VarModule::inc_int(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE);
                    VarModule::set_int(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_TIMER, 300);
                }
                _ => {
                    VarModule::set_int(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE, 0);
                    let handle = VarModule::get_int(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_1) as u32;
                    EffectModule::kill(boma, handle, false, false);
                    //let handle2 = VarModule::get_int(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_2) as u32;
                    //EffectModule::kill(boma, handle2, false, false);
                    VarModule::set_int(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_1, -1);
                    //VarModule::set_int(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_2, -1);
                    ColorBlendModule::cancel_main_color(boma, 0);
                }
            }
        }
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

    agent.acmd("game_appeallwl", game_appeallw, Priority::Low);
    agent.acmd("game_appeallwr", game_appeallw, Priority::Low);
}
