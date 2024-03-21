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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.375);
    }
	frame(lua_state, 9.0); // Effectively F12
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
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_miigunner_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_miigunner_step_right_m"));
    }
    wait(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_miigunner_step_left_m"));
    }
}

unsafe extern "C" fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.125);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 11.0); // Effectively F12
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn miigunner_attackairf_bullet_fly_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, -0.5, 0.5, 0, 180, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

unsafe extern "C" fn miigunner_rapidshot_bullet_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 100, 3, 0, 1.4, 0.0, 0.0, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 100, 3, 0, 1.4, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 1.4, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn miigunner_grenadelauncher_explode_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_whole(boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.4, 160, 9, 0, 29, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.7, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_RAD(fighter, 0, 1, 0.02, 1000, 1, 0, 0, 18);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(boma, 0);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 6.6, 50, 151, 0, 20, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn miigunner_fullthrottle_final_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let pos2dim = Vector3f {x: 0.0, y: 40.0, z: 0.0};
	PostureModule::set_pos(boma, &pos2dim);
	if is_excute(fighter) {
		
	}
    
}

unsafe extern "C" fn miigunner_stealthbomb_tame_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.15, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(fighter, 0.75);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.20, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 5.0);
	}
    frame(lua_state, 100.0);
	if is_excute(fighter) {
		let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
            let gunner = utils::util::get_battle_object_from_id(owner_id);
            let flash_handle = EffectModule::req_follow(boma, Hash40::new("sys_smash_flash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.5, false, 0, 0, 0, 0 ,0, false, false);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.15, 10.0);
            EffectModule::set_rgb(boma, flash_handle as u32, 0.15, 0.15, 10.0);
            EffectModule::set_rate(boma, flash_handle as u32, 0.3);
            VarModule::set_int64(gunner, vars::miigunner::instance::STEALTHBOMB_EFF_HANDLER, flash_handle);
        }
	}
    for h in 101..=120 {
		if is_excute(fighter) {
			let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
        		let gunner = utils::util::get_battle_object_from_id(owner_id);
				let flash_handle = VarModule::get_int64(gunner, vars::miigunner::instance::STEALTHBOMB_EFF_HANDLER);
				let start_color = Vector3f { x: 0.15, y: 0.15, z: 10.0 };
                let end_color = Vector3f { x: 10.0, y: 0.15, z: 0.15 };
                // Smoothly interpolate from starting to ending color
                let blend_vector = Vector3f {
                    x: start_color.x + ((end_color.x - start_color.x) * (((h as f32) - 100.0) / 20.0)),
                    y: start_color.y + ((end_color.y - start_color.y) * (((h as f32) - 100.0) / 20.0)),
                    z: start_color.z + ((end_color.z - start_color.z) * (((h as f32) - 100.0) / 20.0))
                };
                // Apply color blend
                EffectModule::set_rgb(boma, flash_handle as u32, blend_vector.x, blend_vector.y, blend_vector.z);
			}
		}
		wait(lua_state, 1.0);
	}
}

unsafe extern "C" fn miigunner_stealthbomb_s_move_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.0);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 65, 94, 0, 48, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		//AREA_WIND_2ND_RAD(0, 1, 0.02, 1000, 1, 0, 0, 12);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		//AreaModule::erase_wind(boma, 0);
	}
    
}

unsafe extern "C" fn miigunner_stealthbomb_s_move_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
	}
    
}

unsafe extern "C" fn miigunner_supermissile_straight_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
       let gunner = utils::util::get_battle_object_from_id(owner_id);
       VarModule::set_int(gunner, vars::miigunner::instance::MISSILE_OBJECT_ID, fighter.battle_object_id as i32);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 90, 0, 50, 3.0, 0.0, 0.0, 1.2, Some(0.0), Some(0.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
       if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
           let gunner = utils::util::get_battle_object_from_id(owner_id);
           VarModule::on_flag(gunner, vars::miigunner::instance::DETONATE_READY);
       }
    }
}

unsafe extern "C" fn miigunner_supermissile_burst_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
       let gunner = utils::util::get_battle_object_from_id(owner_id);
       VarModule::off_flag(gunner, vars::miigunner::instance::DETONATE_READY);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        let gunner = utils::util::get_battle_object_from_id(owner_id);
        if VarModule::is_flag(gunner, vars::miigunner::status::MISSILE_DETONATE) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 50, 75, 0, 70, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BOMB);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn miigunner_supermissile_burst_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if is_excute(fighter) {
       if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
           let gunner = utils::util::get_battle_object_from_id(owner_id);
           if VarModule::is_flag(gunner, vars::miigunner::status::MISSILE_DETONATE) {
               EFFECT(fighter, Hash40::new("miigunner_atk_shot5"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 1.45, 0, 0, 0, 0, 0, 0, false);
               LAST_EFFECT_SET_COLOR(fighter, 0.5, 10.0, 25.0);
           }
           else {
               EFFECT(fighter, Hash40::new("sys_misfire"), Hash40::new("top"), 0, -1, 2, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, false);
               EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
               LAST_EFFECT_SET_RATE(fighter, 1.5);
           }
       }
    }
}

unsafe extern "C" fn miigunner_supermissile_burst_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if is_excute(fighter) {
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
           let gunner = utils::util::get_battle_object_from_id(owner_id);
           if VarModule::is_flag(gunner, vars::miigunner::status::MISSILE_DETONATE) {
               PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_s03"));
               PLAY_SE_REMAIN(fighter, Hash40::new("se_common_bomb_l"));
           }
           else {
                PLAY_SE_REMAIN(fighter, Hash40::new("se_common_bomb_s"));
           }
        }
    }
}

unsafe extern "C" fn miigunner_bottomshoot_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if is_excute(fighter) {
        if VarModule::get_float(owner_module_accessor.object(), vars::miigunner::status::CURRENT_CHARGE) <= 10.0 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 30, 0, 110, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 100, 0, 60, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
        }
	}
}

unsafe extern "C" fn miigunner_gunnercharge_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 42, 0, 14, 1.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 21.0, 50, 80, 0, 27, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
		AttackModule::enable_safe_pos(boma);
	}
	for _ in 0..1000 {
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			let motion_vec = Vector3f{x: 0.5, y: 1.0, z: 1.0};
			KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
		}
	}
    
}

unsafe extern "C" fn miigunner_gunnercharge_shoot_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if(WorkModule::get_float(boma, *WEAPON_MIIGUNNER_GUNNERCHARGE_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.25) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_miigunner_special_n02"));
        }
        else if(WorkModule::get_float(boma, *WEAPON_MIIGUNNER_GUNNERCHARGE_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.625) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_miigunner_special_n03"));
        }
        else if(WorkModule::get_float(boma, *WEAPON_MIIGUNNER_GUNNERCHARGE_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.875) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_miigunner_special_n04"));
        }
        else {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_miigunner_special_n05"));
        }
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
    smashline::Agent::new("miigunner")
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
        .install();
    smashline::Agent::new("miigunner_stealthbomb_s")
        .acmd("game_move", miigunner_stealthbomb_s_move_game)
        .acmd("effect_move", miigunner_stealthbomb_s_move_effect)
        .install();
    smashline::Agent::new("miigunner_grenadelauncher")
        .acmd("game_explode", miigunner_grenadelauncher_explode_game)
        .install();
    smashline::Agent::new("miigunner_gunnercharge")
        .acmd("game_shoot", miigunner_gunnercharge_shoot_game)
        .acmd("sound_shoot", miigunner_gunnercharge_shoot_sound)
        .install();
    smashline::Agent::new("miigunner_stealthbomb")
        .acmd("effect_tame", miigunner_stealthbomb_tame_effect)
        .install();
    smashline::Agent::new("miigunner_attackairf_bullet")
        .acmd("effect_fly", miigunner_attackairf_bullet_fly_effect)
        .install();
    smashline::Agent::new("miigunner_supermissile")
        .acmd("game_straight", miigunner_supermissile_straight_game)
        .acmd("game_sburst", miigunner_supermissile_burst_game)
        .acmd("effect_sburst", miigunner_supermissile_burst_effect)
        .acmd("sound_sburst", miigunner_supermissile_burst_sound)
        .install();
    smashline::Agent::new("miigunner_fullthrottle")
        .acmd("game_final", miigunner_fullthrottle_final_game)
        .install();
    smashline::Agent::new("miigunner_rapidshot_bullet")
        .acmd("game_fly", miigunner_rapidshot_bullet_fly_game)
        .install();
    smashline::Agent::new("miigunner_bottomshoot")
        .acmd("game_shoot", miigunner_bottomshoot_shoot_game)
        .install();
}
