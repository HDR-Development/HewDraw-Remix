
use super::*;

unsafe extern "C" fn lucas_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 25.0/(21.0-1.0));
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        KineticModule::add_speed(boma, &Vector3f::new(-0.5, 0.1, 0.0));
        ArticleModule::generate_article(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FIRE, false, 0);
    }
}

unsafe extern "C" fn lucas_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 14.0/(21.0-1.0));
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ArticleModule::generate_article(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FIRE, false, 0);
    }
}

unsafe extern "C" fn lucas_special_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_pkfi_start"), Hash40::new("lucas_pkfi_start"), Hash40::new("havel"), -0.5, 0, 0, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn lucas_special_s_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_h01"));
        PLAY_SE(fighter, Hash40::new("se_lucas_special_s03"));
        let rand = sv_math::rand(hash40("fighter"), 7) as i32;
        if rand == 0 {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucas_attack01"));
        }
        else if rand == 1 {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucas_attack02"));
        }
        else if rand == 2 {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucas_attack03"));
        }
        else if rand == 3 {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucas_cliffcatch"));
        }
    }
}

unsafe extern "C" fn lucas_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_LUCAS_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 5.0, 367, 130, 50, 0, 8.2, 3.5, -3.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 5.0, 367, 130, 50, 0, 8.2, -2.0, 0.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 2.0, 366, 130, 50, 0, 7.0, 0.0, 2.5, 5.5, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 2.0, 363, 130, 50, 0, 5.7, 0.0, 1.0, -2.2, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        AttackModule::set_attack_composition_speed(boma, 0, true);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_FALL_START);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 10.0, 50, 74, 0, 90, 8.0, 0.0, 2.5, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 10.0, 50, 74, 0, 90, 6.5, 0.0, 1.0, -2.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        fighter.on_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_CRITICAL);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        fighter.on_flag(*FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_NO_LAST_ATTACK);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        fighter.on_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_END);
        fighter.on_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE);
        JostleModule::set_status(boma, true);
        fighter.select_cliff_hangdata_from_name("special_air_hi_end");
    }
}

unsafe extern "C" fn lucas_special_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_pkt_attack"), Hash40::new("rot"), 0, 1, 8, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT(fighter, Hash40::new("lucas_pkt_bomb"), Hash40::new("top"), 0, 2.2, 2.9, 0, 0, 0, 0.64, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..15 {
        if is_excute(fighter) {
            BURN_COLOR(fighter, 0.5, 0.2, 1, 0.9);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            BURN_COLOR_FRAME(fighter, 1, 0.5, 0.2, 1, 0);
            BURN_COLOR_NORMAL(fighter);
            FLASH(fighter, 0, 0, 0.1, 0.8);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            FLASH_FRM(fighter, 1, 0, 0, 0.1, 0);
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkt_attack"), false, false);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_pkt_hold"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 0.9, true);
        BURN_COLOR(fighter, 0.7, 0.2, 1, 0.6);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        BURN_COLOR_FRAME(fighter, 1, 0.7, 0.2, 1, 0);
        BURN_COLOR_NORMAL(fighter);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.8, 0.7, 1, 0.5);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 1, 0.8, 0.7, 1, 0);
        COL_NORMAL(fighter);
    }
}

unsafe extern "C" fn lucas_special_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0/(6.0-1.0));
    }
}

unsafe extern "C" fn lucas_special_air_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if StatusModule::prev_status_kind(boma, 0) == FIGHTER_STATUS_KIND_SPECIAL_S && fighter.get_num_used_jumps() == 1 {
            let mag_pull_mult = ParamModule::get_float(fighter.object(), ParamType::Agent, "mag_pull_x_mult");
            KineticModule::mul_speed(boma, &Vector3f{x: mag_pull_mult, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        FT_MOTION_RATE(fighter, 3.0/(6.0-1.0));
    }
}

unsafe extern "C" fn lucas_special_lw_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -1.5, 0, -1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("havel"), -0.3, -4.0, 0.1, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psimagnet_start"), Hash40::new("trans"), 0, 6.5, 10, 0, 0, 0, 0.25, false);
    }

}

unsafe extern "C" fn lucas_special_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 80, 80, 30, 0, 4.0, 0.0, 6.3, 3.5, None, None, None, 0.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 80, 80, 30, 0, 4.25, 0.0, 6.3, 7.25, None, None, None, 0.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 9.0);
    for _ in 0..999 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 80, 0, 0, 0, 4.0, 0.0, 6.3, 3.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 80, 0, 0, 0, 4.25, 0.0, 6.3, 7.25, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 9.0);
    }

}

unsafe extern "C" fn lucas_special_lw_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 127, 80, 0, 77, 4.5, 0.0, 6.3, 3.75, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 127, 80, 0, 77, 5.0, 0.0, 6.3, 8.25, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.800);
    }

}

unsafe extern "C" fn lucas_special_lw_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psimagnet_end"), Hash40::new("trans"), 0, 6.5, 9.25, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psimagnet_hit"), Hash40::new("trans"), 0, 6.5, 9.25, 0, 0, 0, 0.25, true);
        FLASH(fighter, 0.5, 1, 1, 0.4);
        EFFECT_DETACH_KIND(fighter, Hash40::new("lucas_psimagnet_end"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("lucas_psimagnet_hit"), -1);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 10, 0, 1, 1, 0.1);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }

}

unsafe extern "C" fn lucas_special_lw_hit_effect (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("lucas_psimagnet_hit"), Hash40::new("trans"), 0, 6.5, 9.5, 0, 0, 0, 0.4, false);
		FLASH(fighter, 0.5, 1, 1, 0.4);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		FLASH_FRM(fighter, 4, 0, 1, 1, 0.1);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		FLASH_FRM(fighter, 6, 0, 0, 1, 0);
	}
	wait(lua_state, 6.0);
	if is_excute(fighter) {
		COL_NORMAL(fighter);
	}
}

// SPECIAL N START //

unsafe extern "C" fn lucas_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 10, 0, 0, 55, 14.0, 0.0, 10.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

unsafe extern "C" fn lucas_special_n_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_smash_l03"));
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucas_attack06"));
    }

}

// SPECIAL N HOLD //

unsafe extern "C" fn lucas_special_n_hold_game(fighter: &mut L2CAgentBase) {
    // INTENTIONALLY LEFT BLANK
    /* if fighter.kind() == *FIGHTER_KIND_KIRBY {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    } */
}

unsafe extern "C" fn lucas_special_n_hold_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 9, 0, 9, 0, 0, 0, false);
        FLASH(fighter, 0.01, 0.5, 1, 0.4);
    }
    for i in 1..=50 {
        if is_excute(fighter) {
            if i%2==0 {
                EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkfr_hold"), false, false);
                EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkfr_hold"), Hash40::new("top"), 0, sv_math::rand(hash40("fighter"), 4) as i32 + 12, sv_math::rand(hash40("fighter"), 4) as i32 - 2, 0, 0, 0, 0.5, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_defense_up"), false, false);
                EFFECT_FLW_POS(fighter, Hash40::new("sys_status_defense_up"), Hash40::new("top"), 0, sv_math::rand(hash40("fighter"), 4) as i32 + 12, sv_math::rand(hash40("fighter"), 4) as i32 - 2, 0, 0, 0, 0.2, true);
            }
            if i%4==0 {
                EFFECT_FLW_POS(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, false);
            }
            FLASH(fighter, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter){
            COL_NORMAL(fighter);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            FLASH(fighter, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter){
            COL_NORMAL(fighter);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn lucas_special_n_hold_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_special_h02"));
        PLAY_STATUS(fighter, Hash40::new("se_lucas_pk_charge"));
    }
}

// SPECIAL N FIRE //

unsafe extern "C" fn lucas_special_n_fire_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT) {
        frame(lua_state, 2.0);
        if is_excute(fighter) {
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 60, 0, 60, 3.0, 0.0, 10.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 100, 0, 50, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            AttackModule::clear_all(boma);
        }
    } else {
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkfr_hold"), false, false);
        }
        frame(lua_state, 2.0);
        if is_excute(fighter) {
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
        }

    }
}

unsafe extern "C" fn lucas_special_n_fire_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkt_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.9, true);
        EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.5, true);
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 1..=5 {
        if is_excute(fighter) {
            FLASH(fighter, 0.01, 0.5, 1, 0.4);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            FLASH(fighter, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(lua_state, 3.0)
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkt_hold"), false, false);
    }
}

unsafe extern "C" fn lucas_special_n_fire_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucas_attack05"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_n04_l"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_electric_hit_m"));
    }
}

unsafe extern "C" fn game_specialhiend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        fighter.select_cliff_hangdata_from_name("special_air_hi_end");
    }
}

unsafe extern "C" fn game_fallspecial(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        fighter.select_cliff_hangdata_from_name("special_air_hi_end");
    }
}

pub fn install() {
    smashline::Agent::new("lucas")
        .acmd("game_specials", lucas_special_s_game)
        .acmd("game_specialairs", lucas_special_air_s_game)
        .acmd("effect_specialairs", lucas_special_s_effect)
        .acmd("effect_specials", lucas_special_s_effect)
        .acmd("sound_specials", lucas_special_s_sound)
        .acmd("sound_specialairs", lucas_special_s_sound)
        .acmd("game_specialairhi", lucas_special_air_hi_game)
        .acmd("effect_specialairhi", lucas_special_air_hi_effect)
        .acmd("game_speciallwstart", lucas_special_lw_start_game)
        .acmd("game_specialairlwstart", lucas_special_air_lw_start_game)
        .acmd("effect_speciallwstart", lucas_special_lw_start_effect)
        .acmd("effect_specialairlwstart", lucas_special_lw_start_effect)
        .acmd("game_speciallwhold", lucas_special_lw_hold_game)
        .acmd("game_specialairlwhold", lucas_special_lw_hold_game)
        .acmd("game_speciallwend", lucas_special_lw_end_game)
        .acmd("game_specialairlwend", lucas_special_lw_end_game)
        .acmd("effect_speciallwend", lucas_special_lw_end_effect)
        .acmd("effect_specialairlwend", lucas_special_lw_end_effect)
        .acmd("effect_speciallwhit", lucas_special_lw_hit_effect)
        .acmd("effect_specialairlwhit", lucas_special_lw_hit_effect)
        .acmd("game_specialnstart", lucas_special_n_start_game)
        .acmd("game_specialairnstart", lucas_special_n_start_game)
        .acmd("sound_specialnstart", lucas_special_n_start_sound)
        .acmd("sound_specialairnstart", lucas_special_n_start_sound)
        .acmd("game_specialnhold", lucas_special_n_hold_game)
        .acmd("game_specialairnhold", lucas_special_n_hold_game)
        .acmd("effect_specialnhold", lucas_special_n_hold_effect)
        .acmd("effect_specialairnhold", lucas_special_n_hold_effect)
        .acmd("sound_specialairnhold", lucas_special_n_hold_sound)
        .acmd("sound_specialnhold", lucas_special_n_hold_sound)
        .acmd("game_specialnfire", lucas_special_n_fire_game)
        .acmd("game_specialairnfire", lucas_special_n_fire_game)
        .acmd("effect_specialnfire", lucas_special_n_fire_effect)
        .acmd("effect_specialairnfire", lucas_special_n_fire_effect)
        .acmd("sound_specialairnfire", lucas_special_n_fire_sound)
        .acmd("sound_specialnfire", lucas_special_n_fire_sound)
        .acmd("game_specialairhiend", game_specialhiend)
        .acmd("game_specialhiend", game_specialhiend)
        .acmd("game_fallspecial", game_fallspecial)
        .install();
}
