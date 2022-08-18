
use super::*;

use smash::app::BattleObjectModuleAccessor;

#[acmd_script( agent = "gaogaen", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) < 50.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_gaogaen_rnd_futtobi01"), Hash40::new("seq_gaogaen_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) < 50.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_gaogaen_rnd_futtobi01"), Hash40::new("seq_gaogaen_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "gaogaen", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) < 50.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_gaogaen_rnd_futtobi01"), Hash40::new("seq_gaogaen_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) < 50.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_gaogaen_rnd_futtobi01"), Hash40::new("seq_gaogaen_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "gaogaen", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) < 50.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_gaogaen_rnd_futtobi01"), Hash40::new("seq_gaogaen_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) < 50.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_gaogaen_rnd_futtobi01"), Hash40::new("seq_gaogaen_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "gaogaen", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_gaogaen_rnd_futtobi01"), Hash40::new("seq_gaogaen_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_gaogaen_rnd_futtobi01"), Hash40::new("seq_gaogaen_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "gaogaen", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) < 50.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_gaogaen_rnd_futtobi01"), Hash40::new("seq_gaogaen_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) < 50.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_gaogaen_rnd_futtobi01"), Hash40::new("seq_gaogaen_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "gaogaen", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
        ControlModule::clear_command(boma, true);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        let grab_y = VarModule::get_float(fighter.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let mut z_mod = -1.0 * grab_y;
        if grab_y > 0.0 {
            z_mod = 3.0 * grab_y;
        }
        CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, (grab_y * 8.0) + 11.0, 13.0 - z_mod, Some(0.0), Some(10.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

#[acmd_script( agent = "gaogaen", script = "effect_catch" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_catch_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        let grab_y = VarModule::get_float(fighter.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let rot_right = 0 - ((grab_y * 50.0) as i32);
        let rot_left = 180 + ((grab_y * 50.0) as i32);

        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 0, 11.0, 5, rot_right, -45, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 7, 10.5, 4, rot_left, -225, 0, 1.2, true, *EF_FLIP_YZ);
    }
}


#[acmd_script( agent = "gaogaen", script = "game_catchturn" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_catchturn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
        ControlModule::clear_command(boma, true);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        let grab_y = VarModule::get_float(fighter.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let mut z_mod = 0.0;
        if grab_y > 0.0 {
            z_mod = 3.0 * grab_y;
        }
        CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, (grab_y * 8.0) + 11.0, -17.0 + z_mod, Some(0.0), Some(10.0), Some(-2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

#[acmd_script( agent = "gaogaen", script = "effect_catchturn" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_catchturn_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        let grab_y = VarModule::get_float(fighter.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let rot_right = 0 - ((grab_y * 50.0) as i32);
        let rot_left = 180 + ((grab_y * 50.0) as i32);

        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 0, 11.0, -7, rot_right, 135, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 7, 10.5, -8, rot_left, -45, 0, 1.2, true, *EF_FLIP_YZ);
    }
}


#[acmd_script( agent = "gaogaen", script = "game_catchdash" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_catchdash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
        ControlModule::clear_command(boma, true);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        let grab_y = VarModule::get_float(fighter.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let mut z_mod = 0.0;
        if grab_y > 0.0 {
            z_mod = 4.0 * grab_y;
        }
        CATCH(fighter, 0, Hash40::new("top"), 3.5, 0.0, (grab_y * 8.0) + 9.0, 12.0 - z_mod, Some(0.0), Some(9.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

#[acmd_script( agent = "gaogaen", script = "effect_catchdash" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_catchdash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        let grab_y = VarModule::get_float(fighter.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let rot_right = 0 - ((grab_y * 50.0) as i32);
        let rot_left = 180 + ((grab_y * 50.0) as i32);

        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 0, 9.0, 3.5, rot_right, -45, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 7, 8.5, 2.5, rot_left, -225, 0, 1.2, true, *EF_FLIP_YZ);
    }
}

#[acmd_script( agent = "gaogaen", script = "game_dash" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "gaogaen", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gaogaen_dash_start"));
    }
    wait(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_STEP_FLIPPABLE(fighter, Hash40::new("se_gaogaen_step_left_ll"), Hash40::new("se_gaogaen_step_right_ll"));
    }
}

#[acmd_script( agent = "gaogaen", script = "game_turndash" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "gaogaen", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "gaogaen", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_slide_hit_xlu_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_slide_hit_xlu_frame"));
    let escape_air_slide_hit_normal_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_slide_hit_normal_frame"));
    let ledgegrab_frame = (escape_air_slide_hit_xlu_frame + escape_air_slide_hit_normal_frame) + 4.0;

    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, ledgegrab_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
        gaogaen_catch_game,
        gaogaen_catch_effect,
        dash_game,
        dash_sound,
        turn_dash_game,
        gaogaen_catchturn_game,
        gaogaen_catchturn_effect,
        gaogaen_catchdash_game,
        gaogaen_catchdash_effect,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}

