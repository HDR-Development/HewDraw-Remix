
use super::*;

unsafe extern "C" fn damageflyhi_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyroll_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));
    }
}

unsafe extern "C" fn dash_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_lucas_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_lucas_step_right_l"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_lucas_step_left_l"));
    }
}

unsafe extern "C" fn lucas_turn_dash_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn lucas_pkfire_shoot_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 10, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn lucas_pkfire_shoot_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    for i in 1..=50 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet_ed"), true, true);
            EFFECT_FOLLOW(agent, Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(agent, 0.25);
        }
        wait(lua_state, 8.0);
    }
}

unsafe extern "C" fn lucas_pkfire_pillar_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    //if is_excute(agent) {
    //    ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 45, 10, 0, 20, 7.0, 0.0, 2.0, 2.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
    //    ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 45, 10, 0, 20, 5.0, 0.0, 7.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
    //}
    //wait(lua_state, 10.0);
    //if is_excute(agent) {
    //    AttackModule::clear_all(boma);
    //    AREA_WIND_2ND_RAD_arg9(agent, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
    //}
}

unsafe extern "C" fn lucas_pkfire_pillar_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_pkfi_start"), Hash40::new("lucas_pkfi_start"), Hash40::new("top"), -0.5, 0, 0, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        EFFECT(agent, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, -4.5, -2.7, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    /*
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("lucas_pkfi_bomb"), Hash40::new("top"), 0, -4.5, -2.7, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    */
}

unsafe extern "C" fn escape_air_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn escape_air_slide_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn lucas_pkfire_pillar_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) { 
        PLAY_SE_REMAIN(agent, Hash40::new("se_lucas_special_n04_s"));
    }
}

unsafe extern "C" fn lucas_appeal_hi_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 40.0);
    if is_excute(agent) {
        if agent.is_button_on(Buttons::AppealHi) {
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SLIP_WAIT, true);
        }
    }
}

unsafe extern "C" fn lucas_appeal_lw_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if agent.is_button_on(Buttons::Guard) && is_training_mode() {
            let charge_time = ParamModule::get_int(agent.object(), ParamType::Agent, "attack_up_charge_time");
            VarModule::set_int(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, charge_time);
            VarModule::on_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
        }
    }
}

unsafe extern "C" fn lucas_pkthunder_game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 361, 50, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.object(), vars::lucas::status::THUNDER_LOOSE) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 361, 50, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }
    }
}

unsafe extern "C" fn lucas_pkthunder_game_move_child(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 1, 1, Hash40::new("top"), 0.75, 80, 40, 0, 4, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        AttackModule::set_attack_composition_speed(boma, 1, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.object(), vars::lucas::status::THUNDER_LOOSE) {
            ATTACK(agent, 1, 1, Hash40::new("top"), 0.75, 80, 40, 0, 4, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }
    }
}

pub fn install() {
    smashline::Agent::new("lucas_pkthunder")
        .acmd("game_move", lucas_pkthunder_game_move)
        .acmd("game_movechild", lucas_pkthunder_game_move_child)
        .install();
    smashline::Agent::new("lucas_pkfire")
        .acmd("game_shoot", lucas_pkfire_shoot_game)
        .acmd("effect_shoot", lucas_pkfire_shoot_effect)
        .acmd("game_pillar", lucas_pkfire_pillar_game)
        .acmd("effect_pillar", lucas_pkfire_pillar_effect)
        .acmd("sound_pillar", lucas_pkfire_pillar_sound)
        .install();
    smashline::Agent::new("lucas")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflyhi_sound)
        .acmd("sound_damageflyn", damageflyhi_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflyhi_sound)
        .acmd("sound_dash", dash_sound)
        .acmd("game_turndash", lucas_turn_dash_game)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .acmd("game_appealhir", lucas_appeal_hi_game)
        .acmd("game_appealhil", lucas_appeal_hi_game)
        .acmd("game_appeallwr", lucas_appeal_lw_game)
        .acmd("game_appeallwl", lucas_appeal_lw_game)
        .install();
}
