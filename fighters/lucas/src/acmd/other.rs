
use super::*;

#[acmd_script( agent = "lucas", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "lucas", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "lucas", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "lucas", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "lucas", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "lucas", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_lucas_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_lucas_step_right_l"));
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_lucas_step_left_l"));
    }
}

#[acmd_script( agent = "lucas", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "lucas_pkfire", script = "game_shoot" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_pkfire_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 10, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        AttackModule::enable_safe_pos(boma);
    }
}

#[acmd_script( agent = "lucas_pkfire", script = "effect_shoot" , category = ACMD_EFFECT , low_priority)]
unsafe fn lucas_pkfire_shoot_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    for i in 1..=50 {
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkfr_bullet_ed"), true, true);
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
        }
        wait(lua_state, 8.0);
    }
}

#[acmd_script( agent = "lucas_pkfire", script = "game_pillar" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_pkfire_pillar_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    //if is_excute(fighter) {
    //    ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 45, 10, 0, 20, 7.0, 0.0, 2.0, 2.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
    //    ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 45, 10, 0, 20, 5.0, 0.0, 7.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
    //}
    //wait(lua_state, 10.0);
    //if is_excute(fighter) {
    //    AttackModule::clear_all(boma);
    //    AREA_WIND_2ND_RAD_arg9(fighter, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
    //}
}

#[acmd_script( agent = "lucas_pkfire", script = "effect_pillar" , category = ACMD_EFFECT , low_priority)]
unsafe fn lucas_pkfire_pillar_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_pkfi_start"), Hash40::new("lucas_pkfi_start"), Hash40::new("top"), -0.5, 0, 0, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        EFFECT(fighter, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, -4.5, -2.7, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    /*
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("lucas_pkfi_bomb"), Hash40::new("top"), 0, -4.5, -2.7, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    */
}

#[acmd_script( agent = "lucas", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, 29.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "lucas", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "lucas_pkfire", script = "sound_pillar" , category = ACMD_SOUND , low_priority)]
unsafe fn lucas_pkfire_pillar_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) { 
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_n04_s"));
    }
}

#[acmd_script( agent = "lucas", scripts = ["game_appealhir", "game_appealhil"] , category = ACMD_GAME, low_priority)]
unsafe fn lucas_appeal_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        if fighter.is_button_on(Buttons::AppealHi) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SLIP_WAIT, true);
        }
    }
}

#[acmd_script( agent = "lucas", scripts = ["game_appeallwr", "game_appeallwl"] , category = ACMD_GAME, low_priority)]
unsafe fn lucas_appeal_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if fighter.is_button_on(Buttons::Guard) && is_training_mode() {
            let charge_time = ParamModule::get_int(fighter.object(), ParamType::Agent, "attack_up_charge_time");
            VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, charge_time);
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
        }
    }
}

#[acmd_script( agent = "lucas_pkthunder", script = "game_move", category = ACMD_GAME, low_priority )]
unsafe fn lucas_pkthunder_game_move(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 50, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 4.0);
    for _ in 0..12 {
        wait(lua_state, 1.0);
        if VarModule::is_flag(fighter.object(), vars::lucas::status::THUNDER_LOOSE) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 50, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 4.0);
    }
}

#[acmd_script( agent = "lucas_pkthunder", script = "game_movechild", category = ACMD_GAME, low_priority )]
unsafe fn lucas_pkthunder_game_move_child(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 1, 1, Hash40::new("top"), 0.75, 80, 40, 0, 4, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        AttackModule::set_attack_composition_speed(boma, 1, true);
    }
    wait(lua_state, 4.0);
    for _ in 0..12 {
        wait(lua_state, 1.0);
        if VarModule::is_flag(fighter.object(), vars::lucas::status::THUNDER_LOOSE) {
            ATTACK(fighter, 1, 1, Hash40::new("top"), 0.75, 80, 40, 0, 4, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 4.0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucas_pkthunder_game_move,
        lucas_pkthunder_game_move_child,
        escape_air_game,
        escape_air_slide_game,
        dash_sound,
        lucas_pkfire_shoot_game,
        lucas_pkfire_shoot_effect,
        lucas_pkfire_pillar_game,
        lucas_pkfire_pillar_effect,
        lucas_pkfire_pillar_sound,
        lucas_appeal_hi_game,
        lucas_appeal_lw_game,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}
