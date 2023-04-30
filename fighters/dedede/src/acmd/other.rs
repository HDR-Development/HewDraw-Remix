
use super::*;

#[acmd_script( agent = "dedede", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "dedede", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "dedede", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "dedede", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "dedede", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "dedede", script = "expression_landingheavy" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn expression_landingheavy(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl"), 0, false, 0x50000000 /* default value */);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    } 
}

#[acmd_script( agent = "dedede", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 5.0/(7.0-1.0));
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 5.5, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(11.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        //FT_MOTION_RATE(fighter, 1.120);
    }
    
}

#[acmd_script( agent = "dedede", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.2);
    }
	frame(lua_state, 11.0); // Effectively F13
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "dedede", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_dedede_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_dedede_step_right_l"));
    }
}

#[acmd_script( agent = "dedede", script = "game_turndash" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "dedede_gordo", script = "game_specialsthrow" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_gordo_special_s_throw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 14.0, 60, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 14.0, 60, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 14.0, 60, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.5, 60, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 12.5, 60, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("hip"), 12.5, 60, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 11.0, 60, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 11.0, 60, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("hip"), 11.0, 60, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 90.0);
    if is_excute(fighter) {
        if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 9.5, 60, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4.7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 9.5, 60, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4.7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("hip"), 9.5, 60, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4.7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd_script( agent = "dedede_gordo", script = "effect_specialsthrow" , category = ACMD_EFFECT , low_priority)]
unsafe fn dedede_gordo_special_s_throw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("dedede_gordo_impact"), Hash40::new("top"), -5, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.8, 0.1);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_fire_fly"), false, false);
    }
}

#[acmd_script( agent = "dedede_gordo", script = "game_specialsshot" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_gordo_special_s_shot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 16.8, 40, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8.4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 16.8, 40, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8.4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 16.8, 40, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8.4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 15.0, 40, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 15.0, 40, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("hip"), 15.0, 40, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 13.2, 40, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 13.2, 40, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("hip"), 13.2, 40, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 11.4, 40, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 11.4, 40, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("hip"), 11.4, 40, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        }
    }

}

#[acmd_script( agent = "dedede_gordo", script = "effect_specialsshot" , category = ACMD_EFFECT , low_priority)]
unsafe fn dedede_gordo_special_s_shot_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.8, 0.1);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_fire_fly"), false, false);
    }
}

#[acmd_script( agent = "dedede_gordo", script = "game_specialsattack" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_gordo_special_s_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 14.0, 60, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 14.0, 60, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 14.0, 60, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.5, 60, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 12.5, 60, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("hip"), 12.5, 60, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 11.0, 60, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 11.0, 60, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("hip"), 11.0, 60, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 90.0);
    if is_excute(fighter) {
        if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 9.5, 60, 66, 0, 70, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4.7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 9.5, 60, 66, 0, 70, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4.7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("hip"), 9.5, 60, 66, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4.7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd_script( agent = "dedede_gordo", script = "effect_specialsattack" , category = ACMD_EFFECT , low_priority)]
unsafe fn dedede_gordo_special_s_attack_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.8, 0.1);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_fire_fly"), false, false);
    }
}

#[acmd_script( agent = "dedede", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "dedede", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "dedede_star", script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter){
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 70, 75, 0, 6, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }

}
pub fn install() {
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
        expression_landingheavy,
        dedede_catch_game,
        dash_game,
        dash_sound,
        turn_dash_game,
        dedede_gordo_special_s_throw_game,
        dedede_gordo_special_s_throw_effect,
        dedede_gordo_special_s_shot_game,
        dedede_gordo_special_s_shot_effect,
        dedede_gordo_special_s_attack_game,
        dedede_gordo_special_s_attack_effect,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound,
        fly_game
    );
}

