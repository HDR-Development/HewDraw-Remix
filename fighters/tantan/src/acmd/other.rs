
use super::*;

#[acmd_script( agent = "tantan", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(boma) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_tantan_rnd_futtobi01"), Hash40::new("seq_tantan_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_tantan_rnd_futtobi01"), Hash40::new("seq_tantan_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "tantan", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(boma) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_tantan_rnd_futtobi01"), Hash40::new("seq_tantan_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_tantan_rnd_futtobi01"), Hash40::new("seq_tantan_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "tantan", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(boma) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_tantan_rnd_futtobi01"), Hash40::new("seq_tantan_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_tantan_rnd_futtobi01"), Hash40::new("seq_tantan_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "tantan", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(boma) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_tantan_rnd_futtobi01"), Hash40::new("seq_tantan_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_tantan_rnd_futtobi01"), Hash40::new("seq_tantan_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "tantan", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(boma) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_tantan_rnd_futtobi01"), Hash40::new("seq_tantan_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_tantan_rnd_futtobi01"), Hash40::new("seq_tantan_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "tantan", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.5);
    }
	frame(lua_state, 11.0); // Effectively F16
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "tantan", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_tantan_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

#[acmd_script( agent = "tantan", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.3);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0); // Effectively F16
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "tantan", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "tantan", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "tantan", scripts = ["effect_attacklegsjumpaerialf","effect_attacklegsjumpaerialb"], category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacklegsjumpaerial(fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "tantan", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 6.0/(15.0-1.0));
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 3.5);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 1.0);

    frame(lua_state, 16.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 7.5, 8.0, Some(0.0), Some(7.5), Some(10.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.250);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}


#[acmd_script( agent = "tantan", script = "sound_catch" , category = ACMD_SOUND , low_priority)]
unsafe fn catch_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x1281c86397));
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new_raw(0x1281c86397));
    }
    
}

#[acmd_script( agent = "tantan", script = "expression_catch" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn catch_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
	    ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, 0 as u32);
    }
}


//Ram Ram attacks//
#[acmd_script( agent = "tantan_punch3", script = "game_attackshort", category = ACMD_GAME, low_priority )]
unsafe fn ramram_game_attackshort(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 5.0, 70, 95, 0, 40, 0.7, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 5.0, 70, 95, 0, 40, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
    }
    let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {8.0} else {6.0};
    let reboundFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {5.0} else {4.0};
    frame(lua_state, reboundFrame);
    //Rebound hitbox
    if is_excute(fighter) {
        if (!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 20, false, *BATTLE_OBJECT_ID_INVALID as u32);

            ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 6.0, 90, 0, 10, 60, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
            //AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        }
    }
    frame(lua_state, clearFrame);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "tantan_punch3", script = "game_attacklong", category = ACMD_GAME, low_priority )]
unsafe fn ramram_game_attacklong(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 10.0, 70, 69, 0, 30, 0.7, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 10.0, 70, 69, 0, 30, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            //Rebound hitbox
            ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 20, false, *BATTLE_OBJECT_ID_INVALID as u32);

            ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 12.0, 90, 0, 10, 60, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
            //AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        }
    }
    let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {10.0} else {9.0};
    frame(lua_state, clearFrame);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "tantan_punch3", script = "game_attacklonghold", category = ACMD_GAME, low_priority )]
unsafe fn ramram_game_attacklonghold(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 12.0, 368, 0, 10, 90, 0.7, 0.0, 2.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("cakram1"), &Vector2f{x: 14.0, y: 1.0}, 6, false);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 7.0, false);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 12.0, 368, 0, 10, 90, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("cakram1"), &Vector2f{x: 14.0, y: 1.0}, 6, false);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 7.0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if (WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 13.8, 368, 0, 10, 60, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("cakram1"), &Vector2f{x: 14.0, y: 1.0}, 6, false);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if (WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            //Rebound hitbox
            ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 20, false, *BATTLE_OBJECT_ID_INVALID as u32);
            ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 13.8, 90, 0, 10, 60, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
            //AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        }
    }
    let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {10.0} else {9.0};
    frame(lua_state, clearFrame);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

//Fly controls when RamRam gets sent outwards
#[acmd_script( agent = "tantan_ring", script = "game_attackfly", category = ACMD_GAME, low_priority )]
unsafe fn ramram_game_attackfly(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //Rebound hitbox
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 2.5, 50, 40, 0, 60, 1.75, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 2.5, 35, 40, 0, 60, 3.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
        AttackModule::clear(boma,1,false);
    }
    let clearFrame = if !WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_AIR)
    && WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_LONG)
    {7.0} else {6.0};
    frame(lua_state, clearFrame);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}
//Fire ramram
#[acmd_script( agent = "tantan_ring", script = "game_attacks4fly", category = ACMD_GAME, low_priority )]
unsafe fn ramram_game_attacks4fly(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    //Scoop for the first few frames, then send back and down
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //Rebound hitbox
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 4.0, 50, 0, 10, 90, 2.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 4.0, 35, 0, 10, 90, 4.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::clear(boma,1,false);
    }
    let clearFrame = if !WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_AIR)
    && WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_LONG)
    {7.0} else {6.0};
    frame(lua_state, clearFrame);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


#[acmd_script( agent = "tantan_punch2", script = "game_attackshort", category = ACMD_GAME, low_priority )]
unsafe fn megawatt_game_attackshort(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("center"), 13.0, 55, 92, 0, 50, 0.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("center"), 13.0, 55, 92, 0, 50, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {19.0} else {17.0};
    frame(lua_state, clearFrame-2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "tantan_punch2", script = "game_attacklong", category = ACMD_GAME, low_priority )]
unsafe fn megawatt_game_attacklong(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("center"), 17.0, 55, 75, 0, 60, 0.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("center"), 17.0, 55, 75, 0, 60, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("center"), 19.0, 55, 75, 0, 60, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("center"), 17.0, 55, 75, 0, 60, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        }
    }
    let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {23.0} else {21.0};
    frame(lua_state, clearFrame-2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}
#[acmd_script( agent = "tantan_punch2", script = "game_attacklonghold", category = ACMD_GAME, low_priority )]
unsafe fn megawatt_game_attacklonghold(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("center"), 20.0, 55, 75, 0, 60, 0.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("center"), 20.0, 55, 75, 0, 60, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("center"), 22.0, 55, 75, 0, 60, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("center"), 20.0, 55, 75, 0, 60, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        }
    }
    let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {23.0} else {20.0};
    frame(lua_state, clearFrame-2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


#[acmd_script( agent = "tantan", scripts = [
    "game_attackshortendr1","game_attackshortendrb1","game_attackshortendrb3",
    "game_attacklongendr1","game_attacklongendrb1","game_attacklongendrb3",
], category = ACMD_GAME, low_priority )]
unsafe fn arm_attack_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);

    frame(lua_state, 8.0);
    if (armType==2)
    && !WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)
    {
        VarModule::on_flag(fighter.battle_object, vars::tantan::status::ARMS_ATTACK_CANCEL);
    }
}

#[acmd_script( agent = "tantan_punch1", script = "game_attackshort", category = ACMD_GAME, low_priority )]
unsafe fn dragon_game_attackshort(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 8.0, 45, 87, 0, 50, 0.7, 3.1, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 8.0, 45, 87, 0, 50, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {12.0} else {9.0};
    frame(lua_state, clearFrame);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "tantan_punch1", script = "game_attacklong", category = ACMD_GAME, low_priority )]
unsafe fn dragon_game_attacklong(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 12.0, 45, 78, 0, 40, 0.7, 3.1, 0.5, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 12.0, 45, 78, 0, 40, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("have"), 16.0, 45, 78, 0, 40, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else 
        {
            ATTACK(fighter, 0, 0, Hash40::new("have"), 13.0, 45, 78, 0, 40, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            WorkModule::off_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
        }
    }
    let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {15.0} else {11.0};
    frame(lua_state, clearFrame);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "tantan_punch1", script = "game_attackdragonshootlong", category = ACMD_GAME, low_priority )]
unsafe fn dragon_game_attackdragonshootlong(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    let damageBonus = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE) {1.2} else {0.0};
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 13.8+damageBonus, 45, 78, 0, 40, 0.7, 5.4, 0.5, 0.3, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 13.8+damageBonus, 45, 78, 0, 40, 3.2, 5.4, 0.5, 0.3, Some(1.0), Some(0.5), Some(0.3), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("have"), 16.75+damageBonus, 45, 78, 0, 40, 3.2, 5.4, 0.5, 0.3, Some(1.0), Some(0.5), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
        }
    }
    //Dragon Beam linker
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            let angle: u64 = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {45} else {361};
            let kbg = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {70} else {0};
            let fkb = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {0} else {10};
            let bkb = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {40} else {60};
            ATTACK(fighter, 0, 0, Hash40::new("have"), 16.75+damageBonus, angle, kbg, fkb, bkb, 3.2, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            WorkModule::off_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 5.0, false);
                                
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else
        {
            AttackModule::set_power(boma, 0,16.0+damageBonus, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "tantan_punch1", script = "effect_attackdragonshootlong", category = ACMD_EFFECT, low_priority )]
unsafe fn dragon_effect_attackdragonshootlong(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("tantan_wepon1_wind_big"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_COLOR(fighter,1.0,0.8,0.25);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("tantan_wepon_ringwind"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_COLOR(fighter,1.0,0.8,0.25);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(fighter, Hash40::new("tantan_dragon_attack_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, true);
        LAST_EFFECT_SET_COLOR(fighter,1.0,0.8,0.25);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("tantan_dragon_eye_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
}
#[acmd_script( agent = "tantan_punch1", scripts = ["game_specialairhiattack","game_specialairhiattackdragon"], category = ACMD_GAME, low_priority )]
unsafe fn dragon_special_hi_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let angle = (80.0+WorkModule::get_float(boma,*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L)) as u64;
    let damageBonus = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE) {1.25} else {0.0};
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 12.75+damageBonus, angle, 79, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(-6.0), Some(0.0), Some(0.3), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 12.75+damageBonus, angle, 79, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(0.0), Some(0.0), Some(0.3), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "tantan_beam", script = "game_shoot", category = ACMD_GAME, low_priority )]
unsafe fn dragonbeam_game_shoot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 75, 0, 70, 1.3, 0.0, 0.0, 2.0, Some(0.0), Some(0.0), Some(25.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::disable_tip(boma);
    }
}
#[acmd_script( agent = "tantan_beam", script = "game_bigshoot", category = ACMD_GAME, low_priority )]
unsafe fn dragonbeam_game_bigshoot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 75, 0, 65, 2.8, 0.0, 0.0, 4.0, Some(0.0), Some(0.0), Some(30.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::disable_tip(boma);
    }
}
pub fn install() {
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
        dash_game,
        dash_sound,
        turn_dash_game,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound,
        catch_game,
        catch_sound,
        catch_expression,
        effect_attacklegsjumpaerial,

        ramram_game_attackshort,
        ramram_game_attacklong,
        ramram_game_attacklonghold,
        ramram_game_attackfly,
        ramram_game_attacks4fly,

        megawatt_game_attackshort,
        megawatt_game_attacklong,
        megawatt_game_attacklonghold,

        dragon_game_attackshort,
        dragon_game_attacklong,
        dragon_game_attackdragonshootlong,
        dragon_effect_attackdragonshootlong,
        dragon_special_hi_attack_game,

        dragonbeam_game_shoot,
        dragonbeam_game_bigshoot,

        arm_attack_end,
    );
}

