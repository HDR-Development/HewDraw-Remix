
use super::*;
use smash::app::sv_battle_object;

unsafe extern "C" fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflylw_sound(fighter: &mut L2CAgentBase) {
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyn_sound(fighter: &mut L2CAgentBase) {
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));
    }
}

unsafe extern "C" fn damageflytop_sound(fighter: &mut L2CAgentBase) {
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn miiswordsman_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.3, 0.0, 6.6, 0.0, Some(0.0), Some(6.6), Some(7.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

unsafe extern "C" fn miiswordsman_catch_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.3, 0.0, 6.6, 0.0, Some(0.0), Some(6.6), Some(10.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

unsafe extern "C" fn miiswordsman_catch_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.3, 0.0, 6.6, 0.0, Some(0.0), Some(6.6), Some(-12.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

unsafe extern "C" fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_miiswordsman_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_miiswordsman_step_right_m"));
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_miiswordsman_step_left_m"));
    }
}

unsafe extern "C" fn turn_dash_game(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn miiswordsman_tornadoshot_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        AREA_WIND_2ND_RAD_arg9(fighter, 0, 2, 0.05, 200, 1, 3, 3, 25, 30);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 65, 23, 0, 53, 3.3, 0.0, 5.7, 2.6, Some(0.0), Some(2.8), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 68, 61, 0, 48, 3.3, 0.0, 5.7, 2.6, Some(0.0), Some(2.8), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
    }

}

// #[acmd_script( agent = "miiswordsman_tornadoshot", script = "game_fly" , category = ACMD_GAME , low_priority)]
// unsafe fn miiswordsman_tornadoshot_fly_game(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = fighter.boma();
//     if is_excute(fighter) {
//         AREA_WIND_2ND_RAD_arg9(fighter, 0, 2, 0.05, 200, 1, 3, 3, 25, 30);
//         ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 366, 100, 30, 0, 4.5, 0.0, 9.5, 3.2, Some(0.0), Some(9.5), Some(3.2), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
//         ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 90, 100, 50, 0, 4.5, 0.0, 9.5, 3.2, Some(0.0), Some(3.0), Some(2.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
//     }
//     frame(lua_state, 37.0);
//     if is_excute(fighter) {
//         AttackModule::clear_all(boma);
//         ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 80, 40, 0, 100, 4.5, 0.0, 9.5, 3.2, Some(0.0), Some(3.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
//     }
// }

unsafe extern "C" fn miiswordsman_wave_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 55, 60, 0, 38, 9.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
       ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 55, 60, 0, 38, 9.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn miiswordsman_wave_fly_effect(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
        let boma = fighter.boma();
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let mut lead_wave : u32 = std::u32::MAX;
        let mut wave_2 : u32 = std::u32::MAX;
        let mut wave_3 : u32 = std::u32::MAX;
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_hikari_syuriken"), false, true);
            lead_wave = EffectModule::req_follow(boma, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), &Vector3f{x: 0.0, y: 3.0, z: 0.0}, &Vector3f{x: 80.6, y: -69.5, z: 0.0}, 0.9, true, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rate(boma, lead_wave, 0.7);

            wave_2 = EffectModule::req_follow(boma, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), &Vector3f{x: 0.0, y: 3.0, z: -4.0}, &Vector3f{x: 80.6, y: -69.5, z: 0.0}, 0.7, true, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rate(boma, wave_2, 0.7);
            EffectModule::set_alpha(boma, wave_2, 0.4);
            //Ray check here is used for checking if you're on the ground. Unfortunately is_touch and is_wall_touch_line didnt work for this. Sorry!
            if GroundModule::ray_check(
                fighter.module_accessor, 
                &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, 
                &Vector2f{ x: 0.0, y: -7.0}, true
            ) == 1 {
                FOOT_EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(fighter, 0.3);
                FOOT_EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(lua_state, 4.0);
        if is_excute(fighter) {
            EffectModule::set_rate(boma, lead_wave, 0.000001);
            EffectModule::set_rate(boma, wave_2, 0.000001);
            EffectModule::set_alpha(boma, wave_2, 0.4);
            wave_3 = EffectModule::req_follow(boma, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), &Vector3f{x: 0.0, y: 3.0, z: -8.0}, &Vector3f{x: 80.6, y: -69.5, z: 0.0}, 0.5, true, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rate(boma, wave_3, 0.7);
            EffectModule::set_alpha(boma, wave_3, 0.4);
            if GroundModule::ray_check(
                fighter.module_accessor, 
                &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, 
                &Vector2f{ x: 0.0, y: -7.0}, true
            ) == 1 {
                FOOT_EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(fighter, 0.3);
                FOOT_EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(lua_state, 4.0);
        if is_excute(fighter) {
            EffectModule::set_rate(boma, wave_3, 0.000001);
            if GroundModule::ray_check(
                fighter.module_accessor, 
                &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, 
                &Vector2f{ x: 0.0, y: -7.0}, true
            ) == 1 {
                FOOT_EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(fighter, 0.3);
                FOOT_EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            }
        }
        for _ in 0..i32::MAX {
            wait(lua_state, 4.0);
            if is_excute(fighter) {
                    if GroundModule::ray_check(
                        fighter.module_accessor, 
                        &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, 
                        &Vector2f{ x: 0.0, y: -7.0}, true
                    ) == 1 {
                        FOOT_EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
                        LAST_EFFECT_SET_RATE(fighter, 0.3);
                        FOOT_EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
                    }
            }
        }
}

unsafe extern "C" fn miiswordsman_chakram_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    // Tap Input
    if !WorkModule::is_flag(boma, *WEAPON_MIISWORDSMAN_CHAKRAM_INSTANCE_WORK_ID_FLAG_FLICK){
        if is_excute(fighter) {
            VarModule::on_flag(owner_module_accessor.object(), vars::miiswordsman::instance::CHAKRAM_STICK_ATTACK);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 365, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(boma, 0, -3.0, false);
            AttackModule::enable_safe_pos(boma);
        }
        frame(lua_state, 37.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 0, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(boma, 0, -3.0, false);
            AttackModule::enable_safe_pos(boma);
        }
    }
    // Hold Input
    else{
        if is_excute(fighter) {
            VarModule::on_flag(owner_module_accessor.object(), vars::miiswordsman::instance::CHAKRAM_STICK_ATTACK);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 85, 50, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(boma);
        }
    }
}

unsafe extern "C" fn miiswordsman_chakram_fly_normal_sub_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 365, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(boma, 0, -2.0, false);
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 0, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(boma, 0, -2.0, false);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn miiswordsman_chakram_fly_flick_sub_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 85, 50, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn miiswordsman_chakram_hop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        VarModule::off_flag(owner_module_accessor.object(), vars::miiswordsman::instance::CHAKRAM_STICK_ATTACK);
    }
}

unsafe extern "C" fn miiswordsman_chakram_stick_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
    }
        
}

unsafe extern "C" fn miiswordsman_chakram_stick_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 142.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
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

pub fn install() {
    smashline::Agent::new("miiswordsman")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflylw_sound)
        .acmd("sound_damageflyn", damageflyn_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflytop_sound)
        .acmd("game_catch", miiswordsman_catch_game)
        .acmd("game_catchdash", miiswordsman_catch_dash_game)
        .acmd("game_catchturn", miiswordsman_catch_turn_game)
        .acmd("game_dash", dash_game)
        .acmd("sound_dash", dash_sound)
        .acmd("game_turndash", turn_dash_game)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .install();
    smashline::Agent::new("miiswordsman_chakram")
        .acmd("game_fly", miiswordsman_chakram_fly_game)
        .acmd("game_flynormalsub", miiswordsman_chakram_fly_normal_sub_game)
        .acmd("game_flyflicksub", miiswordsman_chakram_fly_flick_sub_game)
        //.acmd("game_hop", miiswordsman_chakram_hop_game)
        .acmd("game_stick", miiswordsman_chakram_stick_game)
        .acmd("effect_stick", miiswordsman_chakram_stick_effect)
        .install();
    smashline::Agent::new("miiswordsman_tornadoshot")
        .acmd("game_fly", miiswordsman_tornadoshot_fly_game)
        .install();
    smashline::Agent::new("miiswordsman_lightshuriken")
        .acmd("game_fly", miiswordsman_wave_fly_game)
        .acmd("effect_fly", miiswordsman_wave_fly_effect)
        .install();
}