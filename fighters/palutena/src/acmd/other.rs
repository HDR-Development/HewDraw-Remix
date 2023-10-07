
use super::*;

#[acmd_script( agent = "palutena", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "palutena", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "palutena", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "palutena", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "palutena", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "palutena", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.6, 0.0, 8.0, 0.0, Some(0.0), Some(8.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "palutena", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.4);
    }
	frame(lua_state, 11.0); // Effectively F15
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "palutena", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_palutena_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

#[acmd_script( agent = "palutena", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.2);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0); // Effectively F15
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "palutena_explosiveflame_reserve", script = "effect_wait" , category = ACMD_EFFECT , low_priority)]
unsafe fn palutena_explosiveflame_reserve_wait_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
		FT_MOTION_RATE(fighter, 10.0);
    }
    
}

#[acmd_script( agent = "palutena_reflectionboard", script = "game_shoot" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_reflectionboard_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 125, 40, 0, 75, 5.0, 0.0, 8.5, 0.0, Some(0.0), Some(-4.5), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 40, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.1);
    }
    frame(lua_state, 210.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "palutena_autoaimbullet", script = "game_shot", category = ACMD_GAME, low_priority )]
unsafe fn palutena_autoaimbullet_shot_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let palutena = owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA;
    let damage = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {10.0} else {6.0};
    let paralyze = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {0.6} else {0.4};
    if !palutena {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 361, 41, 0, 40, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1.7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_palutena_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beamss"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), damage, 65, 40, 0, 75, 2.3, 0.0, 0.0, 0.0, None, None, None, paralyze, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beamss"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

#[acmd_script( agent = "palutena_autoaimbullet", script = "effect_shot", category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_autoaimbullet_shot_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let palutena = owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA;
    let power = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {Hash40::new("sys_hit_elec")} else {Hash40::new("sys_hit_elec_s")};
    let size = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {2.0} else {1.0};
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_bullet_shot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        if palutena {
            LAST_EFFECT_SET_COLOR(agent, 0.85, 0.40, 0.001);
        }
    }
    if palutena {
        for _ in 0..99 {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 2.2, 1.2, 0, 0, 0, 0.23 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 0.2, -1.4, 0, 0, 0, 0.17 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 1.7, 0.1, 0, 0, 0, 0.32 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
                LAST_EFFECT_SET_RATE(agent, 1);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 1.4, 1.0, 0, 0, 0, 0.2 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 2.3, -1.4, 0, 0, 0, 0.15 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
        }
    }
}

#[acmd_script( agent = "palutena", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "palutena", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
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
        palutena_catch_game,
        dash_game,
        dash_sound,
        turn_dash_game,
		//palutena_explosiveflame_reserve_effect,
        palutena_reflectionboard_shoot_game,
        palutena_autoaimbullet_shot_game,
        palutena_autoaimbullet_shot_effect,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}

