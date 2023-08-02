
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
        if !fighter.is_prev_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR) {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
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
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let gordo_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let char_pos = *PostureModule::pos(owner_module_accessor);
    
    /* Prevents backwards gordos */
    if is_excute(fighter){
        if PostureModule::lr(owner_module_accessor) * gordo_speed_x < 0.0{
            KineticModule::mul_speed(boma, &Vector3f{x: -1.0, y: 1.0,z:  1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
    }
    /* Checks every frame the gordo is active, set equal to the gordo life param */
    for _ in 0..181{
        if is_excute(fighter) {
            if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
                if VarModule::is_flag(owner_module_accessor.object(), vars::dedede::instance::IS_DASH_GORDO){
                    let bounce_dmg_multiplier = ((WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) as f32 + 1.0) * 0.25);
                    ATTACK(fighter, 0, 0, Hash40::new("hip"), 7.0 * bounce_dmg_multiplier, 60, 110, 60, 0, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    
                    //Reduces the max amount of bounces by 1 per recatch on the same gordo
                    if (WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) - VarModule::get_int(owner_module_accessor.object(), vars::dedede::instance::RECATCH_COUNTER)) < 0{
                        StatusModule::change_status_request(fighter.module_accessor, *WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD, true);
                    }
                }
                else{
                    /* Reduces damage on every bounce, by 10% of its last damage in this case */
                    let bounce_dmg_multiplier = ((WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) as f32 + 7.0) * 0.1);
                    ATTACK(fighter, 0, 0, Hash40::new("hip"), 7.0 * bounce_dmg_multiplier, 60, 110, 60, 0, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
                }
            }
        }
        wait(lua_state, 1.0);
    }

}

#[acmd_script( agent = "dedede_gordo", script = "effect_specialsthrow" , category = ACMD_EFFECT , low_priority)]
unsafe fn dedede_gordo_special_s_throw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter){

    }
}

#[acmd_script( agent = "dedede_gordo", script = "game_specialsshot" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_gordo_special_s_shot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    VarModule::inc_int(owner_module_accessor.object(), vars::dedede::instance::INHALE_COUNTER);
    for _ in 0..181{
        if is_excute(fighter) {
            if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
                /* Reduces damage on every bounce, by 12.5% of its last damage in this case */
                let bounce_dmg_multiplier = ((WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) as f32 + 5.0) * 0.125);
                ATTACK(fighter, 0, 0, Hash40::new("hip"), 11.8 * bounce_dmg_multiplier, 60, 50, 0, 60, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            }
        }
        wait(lua_state, 1.0);
    }
}


#[acmd_script( agent = "dedede_gordo", script = "game_specialsattack" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_gordo_special_s_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        StatusModule::change_status_force(boma, *WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD, true);
    }
 
}

#[acmd_script( agent = "dedede_gordo", script = "effect_specialsattack" , category = ACMD_EFFECT , low_priority)]
unsafe fn dedede_gordo_special_s_attack_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "dedede_gordo", script = "game_specialswallstop" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_gordo_special_s_wall_stop_game(fighter: &mut L2CAgentBase){
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    if is_excute(fighter){
        //Diminishing returns on repeated gordo spits
        if VarModule::get_int(owner_module_accessor.object(), vars::dedede::instance::INHALE_COUNTER) > 1{ 
            let wall_stop_life_param = WorkModule::get_param_int(boma, hash40("param_gordo"), hash40("wall_stop_life"));
            let new_gordo_life = wall_stop_life_param - ((VarModule::get_int(owner_module_accessor.object(), vars::dedede::instance::INHALE_COUNTER) - 1)* 60);       
            WorkModule::set_int(boma, new_gordo_life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        }
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.0, 75, 50, 0, 70, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
}

#[acmd_script( agent = "dedede_gordo", script = "effect_specialsdead", category = ACMD_EFFECT, low_priority )]
unsafe fn dedede_gordo_special_s_dead(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("dedede_gordo_smoke"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("dedede_superjump_star"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 3.0);
    }
}

#[acmd_script( agent = "dedede_gordo", script = "game_specialsstart", category = ACMD_GAME, low_priority )]
unsafe fn dedede_gordo_special_s_start_game(fighter: &mut L2CAgentBase){
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter){
        WorkModule::on_flag(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_FLAG_VISIBILITY_ON);
    }
}

#[acmd_script( agent = "dedede_gordo", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn dedede_gordo_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_FLAG_VISIBILITY_ON);
    }
}

#[acmd_script( agent = "dedede", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        VarModule::set_flag(fighter.battle_object, vars::dedede::instance::CAN_WADDLE_DASH_FLAG, false);
    }
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
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        VarModule::set_flag(fighter.battle_object, vars::dedede::instance::CAN_WADDLE_DASH_FLAG, false);
    }
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
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 70, 65, 0, 8, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
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
        //dedede_gordo_special_s_shot_effect,
        dedede_gordo_special_s_attack_game,
        //dedede_gordo_special_s_attack_effect,
        dedede_gordo_special_s_start_game,
        dedede_gordo_special_air_s_start_game,
        //dedede_gordo_special_s_dead,
        dedede_gordo_special_s_wall_stop_game,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound,
        fly_game,
    );
}

