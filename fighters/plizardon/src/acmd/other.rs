use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_plizardon_damagefly02"));
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
                PLAY_FLY_VOICE(agent, Hash40::new("seq_plizardon_rnd_futtobi01"), Hash40::new("seq_plizardon_rnd_futtobi02"));
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
            PLAY_SE(agent, Hash40::new("vc_plizardon_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_plizardon_rnd_futtobi01"), Hash40::new("seq_plizardon_rnd_futtobi02"));
        }
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.3);
    }
    frame(lua_state, 11.0); // Effectively F14
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_plizardon_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_plizardon_step_right_m"));
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_plizardon_step_left_m"));
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.1);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0); // Effectively F14
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
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

unsafe extern "C" fn expression_landingheavy(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl"), 0, false, 0x50000000 /* default value */);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        if !agent.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_AIR]) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
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
                1 /* WATER */ => (*PLEDGE_STATE_NONE, 0),
                _ => (*PLEDGE_STATE_WATER, ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.pledge_duration_frame_training"))
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
            if pledge == *PLEDGE_STATE_WATER {
                VarModule::set_int(boma.object(), vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
            } else {
                let handle = EffectModule::req_follow(boma, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
                VarModule::set_int(agent.battle_object, vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, handle as i32);
                boma.play_pledge_effect(*PLEDGE_STATE_WATER);
            }
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("plizardon_atk_mouth_fire"), Hash40::new("plizardon_atk_mouth_fire"), Hash40::new("head"), -1, 2.5, 0, 130, 0, 190, 0.85, true, *EF_FLIP_ROT_X);
        LAST_EFFECT_SET_SCALE_W(agent, 0.9, 2.0, 0.9);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), -3, 13, 5, 0, -30, -90, 0.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.9);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), -3, 13, 1, 0, -30, -90, 0.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.9);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), -3, 13, 5, 0, -30, -90, 0.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.9);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), -3, 13, 1, 0, -30, -90, 0.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.9);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("plizardon_atk_mouth_fire"), false, false);
    }
}

unsafe extern "C" fn sound_appealhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_plizardon_attack06"));
    }
    for f in [18.0, 27.0, 37.0, 48.0] {
        frame(lua_state, f);
        if is_excute(agent) {
            let sfx = if [18.0, 37.0].contains(&f) {
                "se_plizardon_step_left_m"
            } else {
                "se_plizardon_step_right_m"
            };
            PLAY_SE(agent, Hash40::new(sfx));
        }
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
                2 /* GRASS */ => (*PLEDGE_STATE_NONE, 0),
                _ => (*PLEDGE_STATE_GRASS, ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.pledge_duration_frame_training"))
            };
            VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, state);
            VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 80, 70, 0, 10.0, 0.0, 17.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
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
            if pledge == *PLEDGE_STATE_GRASS {
                VarModule::set_int(boma.object(), vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
            } else {
                let handle = EffectModule::req_follow(boma, Hash40::new("sys_status_speed_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
                VarModule::set_int(agent.battle_object, vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, handle as i32);
                boma.play_pledge_effect(*PLEDGE_STATE_GRASS);
            }
        }
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("plizardon_roar"), Hash40::new("head"), -3, 4, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 96.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 5.2 * -boma.lr(), 0, -1, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
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

    agent.acmd("expression_landingheavy", expression_landingheavy, Priority::Low);

    agent.acmd("game_appealhil", game_appealhi, Priority::Low);
    agent.acmd("game_appealhir", game_appealhi, Priority::Low);
    agent.acmd("effect_appealhil", effect_appealhi, Priority::Low);
    agent.acmd("effect_appealhir", effect_appealhi, Priority::Low);
    agent.acmd("sound_appealhil", sound_appealhi, Priority::Low);
    agent.acmd("sound_appealhir", sound_appealhi, Priority::Low);

    agent.acmd("game_appeallwl", game_appeallw, Priority::Low);
    agent.acmd("game_appeallwr", game_appeallw, Priority::Low);
    agent.acmd("effect_appeallwl", effect_appeallw, Priority::Low);
    agent.acmd("effect_appeallwr", effect_appeallw, Priority::Low);
}
