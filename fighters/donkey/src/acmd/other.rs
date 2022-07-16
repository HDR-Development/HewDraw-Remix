
use super::*;

#[acmd_script( agent = "donkey", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::damage(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::damage(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "donkey", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::damage(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::damage(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "donkey", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::damage(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::damage(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "donkey", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "donkey", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::damage(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::damage(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "donkey", script = "expression_landingheavy" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn expression_landingheavy(fighter: &mut L2CAgentBase) {
  let lua_state = fighter.lua_state_agent;
  let boma = fighter.boma();
  if is_excute(fighter) {
      ControlModule::set_rumble(boma, Hash40::new("rbkind_landl"), 0, false, 0x50000000 /* default value */);
      slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
  } 
}

#[acmd_script( agent = "donkey", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.93, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "donkey", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn dk_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		  WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "donkey", script = "game_itemheavythrowf" , category = ACMD_GAME , low_priority)]
unsafe fn heavy_item_throw_f(fighter: &mut L2CAgentBase) {
  let lua_state = fighter.lua_state_agent;
  let boma = fighter.boma();
  if is_excute(fighter) {
    FT_MOTION_RATE(fighter, 1.0);
  }
  frame(lua_state, 16.0);
  if is_excute(fighter) {
    ItemModule::throw_item(boma, 75.0, 4.0, 1.0, 0, true, 0.0);
  }
}

#[acmd_script( agent = "donkey", script = "game_itemheavythrowb" , category = ACMD_GAME , low_priority)]
unsafe fn heavy_item_throw_b(fighter: &mut L2CAgentBase) {
  let lua_state = fighter.lua_state_agent;
  let boma = fighter.boma();
  if is_excute(fighter) {
    FT_MOTION_RATE(fighter, 1.0);
  }
  frame(lua_state, 18.0);
  if is_excute(fighter) {
    ItemModule::throw_item(boma, 125.0, 4.0, 1.0, 0, true, 0.0);
  }
}

#[acmd_script( agent = "donkey", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "donkey", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
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
    //dash_effect,
		dk_turn_dash_game,
    heavy_item_throw_f,
    heavy_item_throw_b,
    expression_landingheavy,
	    damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}

