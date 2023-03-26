
use super::*;

#[acmd_script( agent = "tantan", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
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
        if !StopModule::is_stop(fighter.module_accessor) {
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
        if !StopModule::is_stop(fighter.module_accessor) {
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
        if !StopModule::is_stop(fighter.module_accessor) {
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
        if !StopModule::is_stop(fighter.module_accessor) {
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
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_tantan_dash_start"), true, false, false, false, app::enSEType(0));
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

//Ram Ram attacks//
#[acmd_script( agent = "tantan_punch3", script = "game_attackshort", category = ACMD_GAME, low_priority )]
unsafe fn ramram_game_attackshort(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 5.0, 55, 95, 0, 30, 0.7, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 5.0, 55, 95, 0, 30, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
    }
    let clearFrame = if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {5.0} else {8.0};
    let reboundFrame = if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {3.0} else {4.0};
    frame(lua_state, reboundFrame);
    //Rebounce hitbox
    if is_excute(fighter) {
        if (!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 20, false, *BATTLE_OBJECT_ID_INVALID as u32);

            ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 5.0, 50, 25, 0, 60, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, -2.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
            //AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);

            ATTACK(fighter, 1, 0, Hash40::new("cakram1"), 5.0, 92, 15, 0, 80, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
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
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 10.0, 55, 69, 0, 30, 0.7, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 10.0, 55, 69, 0, 30, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            //Rebounce hitbox
            ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 20, false, *BATTLE_OBJECT_ID_INVALID as u32);

            ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 12.0, 50, 27, 0, 45, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
            //AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);

            ATTACK(fighter, 1, 0, Hash40::new("cakram1"), 12.0, 92, 15, 0, 86, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "tantan_punch3", script = "game_attacklonghold", category = ACMD_GAME, low_priority )]
unsafe fn ramram_game_attacklonghold(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 11.5, 55, 67, 0, 30, 0.7, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 11.5, 55, 67, 0, 30, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if (WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            //Rebounce hitbox
            ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 20, false, *BATTLE_OBJECT_ID_INVALID as u32);

            ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 13.8, 50, 0, 10, 90, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
            //AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);

            ATTACK(fighter, 1, 0, Hash40::new("cakram1"), 13.8, 92, 15, 0, 86, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 8.0);
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
        //Scoup up hitbox
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 2.5, 80, 85, 0, 35, 3.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_OBJECT);
        //Rebound hitbox
        ATTACK(fighter, 1, 0, Hash40::new("cakram1"), 2.5, 50, 40, 0, 60, 3.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_OBJECT);
        //AttackModule::set_add_reaction_frame_revised(boma, 1, 2.0, false);
        AttackModule::enable_safe_pos(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 2.5, 35, 40, 0, 60, 3.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_OBJECT);
        //AttackModule::set_add_reaction_frame_revised(boma, 0, 2.0, false);
        AttackModule::enable_safe_pos(boma);
        AttackModule::clear(boma,1,false);
    }
    let clearFrame = if !WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_AIR)
    && WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_LONG)
    {8.0} else {7.0};
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
        //Scoup up hitbox
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 4.0, 80, 90, 0, 40, 4.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        //Rebound hitbox
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 4.0, 50, 0, 10, 90, 4.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        //AttackModule::set_add_reaction_frame_revised(boma, 1, 4.0, false);
        AttackModule::enable_safe_pos(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 4.0, 35, 0, 10, 90, 4.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        //AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        AttackModule::clear(boma,1,false);
    }
    let clearFrame = if !WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_AIR)
    && WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_LONG)
    {8.0} else {7.0};
    frame(lua_state, clearFrame);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


#[acmd_script( agent = "tantan_punch2", script = "game_attacklong", category = ACMD_GAME, low_priority )]
unsafe fn megawatt_game_attacklong(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("center"), 16.0, 45, 70, 0, 55, 0.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("center"), 16.0, 45, 70, 0, 55, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("center"), 18.0, 45, 70, 0, 55, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("center"), 16.0, 45, 70, 0, 55, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 23.0);
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
    let armType =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);

    frame(lua_state, 8.0);
    if (armType==2)
    && !WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)
    {
        VarModule::on_flag(fighter.battle_object, vars::tantan::status::RAMRAM_CAN_CANCEL);
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

        ramram_game_attackshort,
        ramram_game_attacklong,
        ramram_game_attacklonghold,
        ramram_game_attackfly,
        ramram_game_attacks4fly,

        megawatt_game_attacklong,

        arm_attack_end
    );
}

