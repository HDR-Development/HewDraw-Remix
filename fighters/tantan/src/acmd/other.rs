
use super::*;


unsafe extern "C" fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn damageflylw_sound(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn damageflyn_sound(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn damageflytop_sound(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn dash_game(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_tantan_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}


unsafe extern "C" fn turn_dash_game(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn escape_air_game(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn effect_attacklegsjumpaerial(fighter: &mut L2CAgentBase) {
}


unsafe extern "C" fn catch_game(fighter: &mut L2CAgentBase) {
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



unsafe extern "C" fn catch_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_swing_06"));
    }
    
}


unsafe extern "C" fn catch_expression(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn ramram_game_attackshort(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn ramram_game_attacklong(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn ramram_game_attacklonghold(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn ramram_game_attackfly(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn ramram_game_attacks4fly(fighter: &mut L2CAgentBase) {
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



unsafe extern "C" fn megawatt_game_attackshort(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn megawatt_game_attacklong(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn megawatt_game_attacklonghold(fighter: &mut L2CAgentBase) {
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



unsafe extern "C" fn arm_attack_end(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn dragon_game_attackshort(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn dragon_game_attacklong(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn dragon_game_attackdragonshootlong(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 13.8, 45, 78, 0, 40, 0.7, 5.4, 0.5, 0.3, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 13.8, 45, 78, 0, 40, 3.2, 5.4, 0.5, 0.3, Some(1.0), Some(0.5), Some(0.3), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("have"), 16.75, 45, 78, 0, 40, 3.2, 5.4, 0.5, 0.3, Some(1.0), Some(0.5), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
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
            ATTACK(fighter, 0, 0, Hash40::new("have"), 16.75, angle, kbg, fkb, bkb, 3.2, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
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
            AttackModule::set_power(boma, 0,16.0, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


unsafe extern "C" fn dragon_effect_attackdragonshootlong(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn dragon_special_hi_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let angle = (75.0+WorkModule::get_float(boma,*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L)) as u64;
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 12.75, angle, 73, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(-6.0), Some(0.0), Some(0.3), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 12.75, angle, 73, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(0.0), Some(0.0), Some(0.3), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


unsafe extern "C" fn dragon_sound_attackbeamloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_tantan_attack01_short"));
        STOP_SE(agent, Hash40::new("se_tantan_attack01_long"));
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let sfx = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE) {Hash40::new("se_tantan_attack01_beam_max")} else {Hash40::new("se_tantan_attack01_beam")};
        PLAY_SE(agent, sfx);
    }
}


unsafe extern "C" fn dragonbeam_game_shoot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let mut is_dragonized = false;
        let minmin_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(minmin_id) {
            let minmin = utils::util::get_battle_object_from_id(minmin_id);
            let minmin_boma = &mut *(*minmin).module_accessor;
            is_dragonized = WorkModule::get_int(minmin_boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME) > 0;
        }
        let damage = if is_dragonized {9.0} else {6.0};
        let sfx_level = if is_dragonized {*ATTACK_SOUND_LEVEL_L} else {*ATTACK_SOUND_LEVEL_M};
        let range = if is_dragonized {30.0} else {25.0};
        let size = if is_dragonized {2.8} else {1.3};

        ATTACK(fighter, 0, 0, Hash40::new("top"), damage, 361, 75, 0, 70, size, 0.0, 0.0, 2.0, Some(0.0), Some(0.0), Some(range), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sfx_level, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::disable_tip(boma);
    }
}

unsafe extern "C" fn dragonbeam_effect_beam(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let mut is_dragonized = false;
        let minmin_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(minmin_id) {
            let minmin = utils::util::get_battle_object_from_id(minmin_id);
            let minmin_boma = &mut *(*minmin).module_accessor;
            is_dragonized = WorkModule::get_int(minmin_boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME) > 0;
        }
        let effect = if is_dragonized {Hash40::new("tantan_dragon_beam2_body")} else {Hash40::new("tantan_dragon_beam1_body")};
        let offset = if is_dragonized {-1.0} else {-2.5};
        EFFECT_FOLLOW(agent, effect, Hash40::new("top"), 0, 0, offset, 0, 90, 180, 1, true);
    }
}




pub fn install() {
    smashline::Agent::new("tantan_punch2")
        .acmd("game_attackshort", megawatt_game_attackshort)
        .acmd("game_attacklong", megawatt_game_attacklong)
        .acmd("game_attacklonghold", megawatt_game_attacklonghold)
        .install();
    smashline::Agent::new("tantan_beam")
        .acmd("game_shoot", dragonbeam_game_shoot)
        .acmd("game_bigshoot", dragonbeam_game_shoot)
        .acmd("effect_beam", dragonbeam_effect_beam)
        .acmd("effect_bigbeam", dragonbeam_effect_beam)
        .install();
    smashline::Agent::new("tantan_punch3")
        .acmd("game_attackshort", ramram_game_attackshort)
        .acmd("game_attacklong", ramram_game_attacklong)
        .acmd("game_attacklonghold", ramram_game_attacklonghold)
        .install();
    smashline::Agent::new("tantan_punch1")
        .acmd("game_attackshort", dragon_game_attackshort)
        .acmd("game_attacklong", dragon_game_attacklong)
        .acmd(
            "game_attackdragonshootlong",
            dragon_game_attackdragonshootlong,
        )
        .acmd(
            "effect_attackdragonshootlong",
            dragon_effect_attackdragonshootlong,
        )
        .acmd("game_specialairhiattack", dragon_special_hi_attack_game)
        .acmd(
            "game_specialairhiattackdragon",
            dragon_special_hi_attack_game,
        )
        .acmd("sound_attackbeamloop", dragon_sound_attackbeamloop)
        .install();
    smashline::Agent::new("tantan_ring")
        .acmd("game_attackfly", ramram_game_attackfly)
        .acmd("game_attacks4fly", ramram_game_attacks4fly)
        .install();
    smashline::Agent::new("tantan")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflylw_sound)
        .acmd("sound_damageflyn", damageflyn_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflytop_sound)
        .acmd("game_dash", dash_game)
        .acmd("sound_dash", dash_sound)
        .acmd("game_turndash", turn_dash_game)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .acmd("effect_attacklegsjumpaerialf", effect_attacklegsjumpaerial)
        .acmd("effect_attacklegsjumpaerialb", effect_attacklegsjumpaerial)
        .acmd("game_catch", catch_game)
        .acmd("sound_catch", catch_sound)
        .acmd("expression_catch", catch_expression)
        .acmd("game_attackshortendr1", arm_attack_end)
        .acmd("game_attackshortendrb1", arm_attack_end)
        .acmd("game_attackshortendrb3", arm_attack_end)
        .acmd("game_attacklongendr1", arm_attack_end)
        .acmd("game_attacklongendrb1", arm_attack_end)
        .acmd("game_attacklongendrb3", arm_attack_end)
        .install();
}
