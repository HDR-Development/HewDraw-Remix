use super::*;

// see plizardon/specials.rs for game acmd

unsafe extern "C" fn effect_plizardonspecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let pledge = VarModule::get_int(agent.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.8, 0.6, 0.3);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.2, 0, 0.5);
        FLASH_FRM(agent, 15, 0, 0, 0, 0);
    }
    frame(lua_state, 7.0);
    if pledge == *PLEDGE_STATE_WATER {
        if is_excute(agent) {
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_steam2"), Hash40::new("head"), 0, 5, 0, 0, 0, 0, 0.7, false, 3.0);
            EFFECT_FOLLOW(agent, Hash40::new("sys_drown_out"), Hash40::new("mouth2"), 0, 0, 0, 180, 0, 0, 0.6, false);
        }
    }
    else if pledge == *PLEDGE_STATE_GRASS {
        // Uncomment this to cause a solar eruption
        // let mut handle = 0;
        // for _ in 0..4 {
        //     if is_excute(agent) {
        //         handle = EffectModule::req_follow(boma, Hash40::new("pfushigisou_atk_hi4"), Hash40::new("mouth2"), &Vector3f::zero(), &Vector3f::zero(), 0.0, false, 0, 0, 0, 0, 0, false, false);
        //         EffectModule::set_rate(boma, handle as u32, 9.0);
        //     }
        //     wait(lua_state, 1.0);
        //     if is_excute(agent) {
        //         EffectModule::set_scale(boma, handle as u32, &Vector3f::new(0.45, 0.45, 0.45));
        //         EffectModule::set_rate(boma, handle as u32, 0.5);
        //         EffectModule::detach(boma, handle as u32, 0);
        //     }
        //     wait(lua_state, 3.0);
        // }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("plizardon_flare_blitz_hold"), Hash40::new("head"), -3, 5, 0, 0, 0, 0, 0.3, true);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        let mut flame_color = Vector3f::new(1.0, 1.0, 1.0);
        let mut flame_size = 1.0;
        let mut flame_alpha = 1.0;
        let mut flame_rate = 2.0;
        let mut speedline_alpha_mul = 0.8;
        let mut speedline_color_main = Vector3f::new(0.6, 0.1, 0.0);
        let mut speedline_color_sub = Vector3f::new(0.9, 0.4, 0.0);
        if pledge == *PLEDGE_STATE_WATER {
            EFFECT_OFF_KIND(agent, Hash40::new("sys_drown_out"), false, false);
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_steam"), Hash40::new("sys_steam"), Hash40::new("top"), -1, 5, 20, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 5.0);
            LAST_EFFECT_SET_SCALE_W(agent, 1.8, 1.5, 1.8);
            LAST_EFFECT_SET_COLOR(agent, 0.7, 0.7, 0.74);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 8, 16, 0, 90, 0, 2, false, 1.0);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 3.0);
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_hit_fire"), Hash40::new("top"), 0, 8, 16, 0, 90, 0, 0.5, false, 0.8);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 3.0);

            flame_color = Vector3f::new(0.6, 1.0, 6.0);
            flame_size = 1.1;
            flame_alpha = 0.85;
            speedline_alpha_mul = 0.6;
            speedline_color_main = Vector3f::new(0.25, 0.3, 0.5);
            speedline_color_sub = Vector3f::new(0.6, 0.1, 0.0);
        }
        else if pledge == *PLEDGE_STATE_GRASS {
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_steam"), Hash40::new("sys_steam"), Hash40::new("top"), -1, 5, 19, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 1.2);
            LAST_EFFECT_SET_COLOR(agent, 2.0, 3.0, 0.5);
            LAST_EFFECT_SET_RATE(agent, 0.7);
            EFFECT_FLIP(agent, Hash40::new("sys_grass_landing"), Hash40::new("sys_grass_landing"), Hash40::new("top"), -1, 5, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(agent, 4.0, 3.0, 1.0);
            LAST_EFFECT_SET_RATE(agent, 0.4);
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 8, 16, 0, 90, 0, 2, false, 0.8);
            LAST_EFFECT_SET_COLOR(agent, 0.6, 1.8, 1.0);

            flame_color = Vector3f::new(0.3, 0.9, 0.5);
            flame_size = 1.1;
            flame_alpha = 0.7;
            flame_rate = 1.0;
            speedline_alpha_mul = 0.6;
            speedline_color_main = Vector3f::new(0.7, 0.2, 0.0);
            speedline_color_sub = Vector3f::new(0.4, 0.3, 0.1);
        }

        EFFECT_OFF_KIND(agent, Hash40::new("plizardon_flare_blitz_hold"), false, false);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("plizardon_atk_mouth_fire"), Hash40::new("plizardon_atk_mouth_fire"), Hash40::new("head"), -1, 2, 0, 0, 180, 35, flame_size, true, *EF_FLIP_ROT_X, flame_alpha);
        LAST_EFFECT_SET_SCALE_W(agent, 1.2, 1.9, 1.2);
        LAST_EFFECT_SET_COLOR(agent, flame_color.x, flame_color.y, flame_color.z);
        LAST_EFFECT_SET_RATE(agent, flame_rate);

        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.5 * speedline_alpha_mul);
        LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.3, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.3);
        LAST_EFFECT_SET_COLOR(agent, speedline_color_main.x, speedline_color_main.y, speedline_color_main.z);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.4 * speedline_alpha_mul);
        LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.2, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.3);
        LAST_EFFECT_SET_COLOR(agent, speedline_color_sub.x, speedline_color_sub.y, speedline_color_sub.z);
        if [*PLEDGE_STATE_NONE, *PLEDGE_STATE_FIRE].contains(&pledge) {
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.2 * speedline_alpha_mul);
            LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.1, 1.0);
            LAST_EFFECT_SET_RATE(agent, 0.3);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_hit_fire"), 0);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        if pledge == *PLEDGE_STATE_GRASS {
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("pfushigisou_atk_hi4"), Hash40::new("top"), 0, 8.5, 21.5, 0, 0, 90, 0.6, false, 0.4);
        }
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        // let mut speedline_color_main = Vector3f::new(0.6, 0.1, 0.0);
        // let mut speedline_color_sub = Vector3f::new(0.9, 0.4, 0.0);
        // if pledge == *PLEDGE_STATE_WATER {
        //     speedline_color_main = Vector3f::new(0.25, 0.3, 0.5);
        //     speedline_color_sub = Vector3f::new(0.6, 0.1, 0.0);
        // }
        // else if pledge == *PLEDGE_STATE_GRASS {
        //     speedline_color_main = Vector3f::new(0.7, 0.2, 0.0);
        //     speedline_color_sub = Vector3f::new(0.4, 0.3, 0.1);
        // }
        // EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.5 * 0.6);
        // LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.2, 1.0);
        // LAST_EFFECT_SET_RATE(agent, 0.3);
        // LAST_EFFECT_SET_COLOR(agent, speedline_color_main.x, speedline_color_main.y, speedline_color_main.z);
        // EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.4 * 0.6);
        // LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.1, 1.0);
        // LAST_EFFECT_SET_RATE(agent, 0.3);
        // LAST_EFFECT_SET_COLOR(agent, speedline_color_sub.x, speedline_color_sub.y, speedline_color_sub.z);
        // if [*PLEDGE_STATE_NONE, *PLEDGE_STATE_FIRE].contains(&pledge) {
        //     EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.2 * 0.6);
        //     LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.0, 1.0);
        //     LAST_EFFECT_SET_RATE(agent, 0.3);
        // }
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("plizardon_atk_mouth_fire"), false, false); 
        EFFECT_OFF_KIND(agent, Hash40::new("sys_steam2"), false, false); 
    }
}

unsafe extern "C" fn sound_plizardonspecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_common_c_fire_l_short_02"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("vc_kirby_002"));
    }
    let pledge = VarModule::get_int(agent.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE);
    if pledge == *PLEDGE_STATE_WATER {
        frame(lua_state, 20.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_common_bomb_s"));
        }
    }
    else if pledge == *PLEDGE_STATE_GRASS {
        frame(lua_state, 23.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_common_bomb_s"));
        }
    }
}

unsafe extern "C" fn expression_plizardonspecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    let pledge = VarModule::get_int(agent.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE);
    if pledge != *PLEDGE_STATE_GRASS {
        frame(lua_state, 20.0);
        if is_excute(agent) {
            if pledge == *PLEDGE_STATE_WATER {
                QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            }
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
        frame(lua_state, 23.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
        frame(lua_state, 29.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        }
    }
    else {
        frame(lua_state, 20.0);
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        }
        frame(lua_state, 23.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        }
        frame(lua_state, 29.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_plizardonspecialnstart", effect_plizardonspecialnstart, Priority::Low);
    agent.acmd("effect_plizardonspecialairnstart", effect_plizardonspecialnstart, Priority::Low);
    agent.acmd("sound_plizardonspecialnstart", sound_plizardonspecialnstart, Priority::Low);
    agent.acmd("sound_plizardonspecialairnstart", sound_plizardonspecialnstart, Priority::Low);
    agent.acmd("expression_plizardonspecialnstart", expression_plizardonspecialnstart, Priority::Low);
    agent.acmd("expression_plizardonspecialairnstart", expression_plizardonspecialnstart, Priority::Low);
}