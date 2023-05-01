
use super::*;

#[acmd_script( agent = "snake", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "snake", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "snake", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "snake", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "snake", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "snake", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn snake_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.875);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.875);
        CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 8.2, 0.0, Some(0.0), Some(8.2), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "snake", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "snake", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_snake_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_snake_step_left_m"));
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_snake_step_right_m"));
    }
}

#[acmd_script( agent = "snake", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}
#[acmd_script( agent = "snake_trenchmortar", script = "game_impact" , category = ACMD_GAME , low_priority)]
unsafe fn snake_trenchmortar_bullet_impact_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
        KineticModule::unable_energy(boma, *WEAPON_SNAKE_TRENCHMORTAR_BULLET_KINETIC_ENERGY_ID_GRAVITY);
        VisibilityModule::set_int64(boma, hash40("main") as i64, hash40("impact") as i64);
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 77, 80, 0, 45, 12.0, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *WEAPON_SNAKE_TRENCHMORTAR_BULLET_STATUS_FLAG_ENABLE_ADVANCE_STATUS);
    }
}

#[acmd_script( agent = "snake_c4", script = "effect_stickother" , category = ACMD_EFFECT , low_priority)]
unsafe fn snake_c4_stick_other_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.65, true);
    }
    for _ in 0..5 {
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
            //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.5, true);
        }
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
    }
    for _ in 0..10 {
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        }
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 10.0, true);
    }
}

#[acmd_script( agent = "snake_c4", script = "game_sticktarget" , category = ACMD_GAME , low_priority)]
unsafe fn snake_c4_stick_target_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        
    }
}

#[acmd_script( agent = "snake_c4", script = "effect_sticktarget" , category = ACMD_EFFECT , low_priority)]
unsafe fn snake_c4_stick_target_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_final_lockon"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_final_lockon2"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_final_lockon_ready"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_final_lockon_ready2"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.65, true);
    }
    wait(lua_state, 60.0);
    if is_excute(fighter) {
        //EFFECT_OFF_KIND(fighter, Hash40::new("snake_final_lockon_ready"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("snake_final_lockon_ready2"), false, false);
    }
    wait(lua_state, 90.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.5, true);
    }
    for _ in 0..4 {
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.5, true);
        }
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
    }
    for _ in 0..10 {
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        }
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 10.0, true);
    }
}

#[acmd_script( agent = "snake_c4", script = "sound_sticktarget" , category = ACMD_SOUND , low_priority)]
unsafe fn snake_c4_stick_target_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_C3){
            PLAY_SE(fighter, Hash40::new("se_snake_special_l08"));        
        }
        else{
            PLAY_SE(fighter, Hash40::new("se_snake_special_l03"));
        }
        PLAY_SE(fighter, Hash40::new("se_snake_final02"));
    }
}

#[acmd_script( agent = "snake_c4", script = "game_explosion" , category = ACMD_GAME , low_priority)]
unsafe fn snake_c4_explosion_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        // Hitbox for opponents
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 16.0, 86, 90, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        // Snake-only hitbox
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 16.0, 84, 90, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        VisibilityModule::set_whole(boma, false);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::set_size(boma, 0, 17.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_GROUND){
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_GROUND){
            AttackModule::clear_all(boma);
        }
    }
}

#[acmd_script( agent = "snake", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "snake", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
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

pub fn install() {
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
        snake_catch_game,
        dash_game,
        dash_sound,
        turn_dash_game,
        snake_trenchmortar_bullet_impact_game,
        snake_c4_stick_target_game,
        snake_c4_stick_target_effect,
        snake_c4_stick_target_sound,
        snake_c4_stick_other_effect,
        snake_c4_explosion_game,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}

