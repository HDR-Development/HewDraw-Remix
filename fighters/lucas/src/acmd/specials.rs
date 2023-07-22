
use super::*;

#[acmd_script( agent = "lucas", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_s_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "lucas", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_air_s_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "lucas", scripts = ["effect_specialairs", "effect_specials"] , category = ACMD_EFFECT , low_priority)]
unsafe fn lucas_special_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_pkfi_start"), Hash40::new("lucas_pkfi_start"), Hash40::new("havel"), -0.5, 0, 0, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
}

#[acmd_script( agent = "lucas", scripts = ["sound_specials", "sound_specialairs"] , category = ACMD_SOUND , low_priority)]
unsafe fn lucas_special_s_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "lucas", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_LUCAS_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 5.0, 367, 100, 50, 0, 9.0, 3.0, -3.0, 0.0, None, None, None, 0.3, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 5.0, 367, 100, 50, 0, 9.0, -2.0, 0.0, 0.0, None, None, None, 0.3, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    for _ in 0..4{
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("rot"), 1.25, 367, 50, 50, 30, 7.5, 0.0, 2.5, 5.0, Some(0.0), Some(1.75), Some(4.5), 0.5, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            //ATTACK(fighter, 1, 0, Hash40::new("rot"), 2.0, 366, 50, 0, 50, 4.3, 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("rot"), 1.25, 366, 50, 0, 50, 6.0, 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            AttackModule::set_attack_composition_speed(boma, 0, true);
            //AttackModule::set_attack_composition_speed(boma, 1, true);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_FALL_START);
    }
    for _ in 0..4{
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("rot"), 1.0, 367, 50, 50, 30, 7.5, 0.0, 2.5, 5.0, Some(0.0), Some(1.75), Some(4.5), 0.5, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            //ATTACK(fighter, 1, 0, Hash40::new("rot"), 1.5, 366, 100, 130, 0, 4.0, 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("rot"), 1.0, 366, 100, 180, 0, 5.7, 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 50, 74, 0, 90, 12.0, 1.0, -1.0, 0.0, None, None, None, 1.3, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        WorkModule::on_flag(boma, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_CRITICAL);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_NO_LAST_ATTACK);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_END);
        WorkModule::on_flag(boma, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE);
        JostleModule::set_status(boma, true);
        fighter.select_cliff_hangdata_from_name("special_air_hi_end");
    }
}

#[acmd_script( agent = "lucas", script = "game_speciallwstart" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0/(6.0-1.0));
    }
}

#[acmd_script( agent = "lucas", script = "game_specialairlwstart" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_air_lw_start_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "lucas", scripts = ["effect_speciallwstart", "effect_specialairlwstart"] , category = ACMD_EFFECT , low_priority)]
unsafe fn lucas_special_lw_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -1.5, 0, -1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("havel"), -0.3, -4.0, 0.1, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psimagnet_start"), Hash40::new("trans"), 0, 6.5, 10, 0, 0, 0, 0.25, false);
    }
    
}

#[acmd_script( agent = "lucas", scripts = ["game_speciallwhold", "game_specialairlwhold"] , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 80, 80, 30, 0, 4.0, 0.0, 6.3, 3.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 80, 80, 30, 0, 4.25, 0.0, 6.3, 7.25, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 9.0);
    for _ in 0..999 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 80, 0, 0, 0, 4.0, 0.0, 6.3, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 80, 0, 0, 0, 4.25, 0.0, 6.3, 7.25, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 9.0);
    }
    
}

#[acmd_script( agent = "lucas", scripts = ["game_speciallwend", "game_specialairlwend"] , category = ACMD_GAME , low_priority)]
unsafe fn lucas_special_lw_end_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "lucas", scripts = ["effect_speciallwend", "effect_specialairlwend"] , category = ACMD_EFFECT , low_priority)]
unsafe fn lucas_special_lw_end_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "lucas", scripts = ["effect_speciallwhit", "effect_specialairlwhit"] , category = ACMD_EFFECT , low_priority)]
unsafe fn lucas_special_lw_hit_effect (fighter: &mut L2CAgentBase) {
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

#[acmd_script ( agent = "lucas", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME, low_priority)]
unsafe fn lucas_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if fighter.kind() == *FIGHTER_KIND_KIRBY {
        // INTENTIONALLY LEFT BLANK
    } else {
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 10, 0, 0, 55, 14.0, 0.0, 10.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    
}

#[acmd_script (agent = "lucas", scripts = ["sound_specialnstart", "sound_specialairnstart"], category = ACMD_SOUND, low_priority)]
unsafe fn lucas_special_n_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_smash_l03"));
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucas_attack06"));
    }
    
}

// SPECIAL N HOLD //

#[acmd_script ( agent = "lucas", scripts = ["game_specialnhold", "game_specialairnhold"], category = ACMD_GAME, low_priority)]
unsafe fn lucas_special_n_hold_game(fighter: &mut L2CAgentBase) {
    // INTENTIONALLY LEFT BLANK
    if fighter.kind() == *FIGHTER_KIND_KIRBY {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
}

#[acmd_script (agent = "lucas", scripts = ["effect_specialnhold", "effect_specialairnhold"], category = ACMD_EFFECT, low_priority)]
unsafe fn lucas_special_n_hold_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script (agent = "lucas", scripts = ["sound_specialairnhold", "sound_specialnhold"], category = ACMD_SOUND, low_priority)]
unsafe fn lucas_special_n_hold_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_special_h02"));
        PLAY_STATUS(fighter, Hash40::new("se_lucas_pk_charge"));
    }
}

// SPECIAL N FIRE //

#[acmd_script ( agent = "lucas", scripts = ["game_specialnfire", "game_specialairnfire"], category = ACMD_GAME, low_priority)]
unsafe fn lucas_special_n_fire_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if fighter.kind() == *FIGHTER_KIND_KIRBY {
        // INTENTIONALLY LEFT BLANK
    } else {
        if !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT) {
            frame(lua_state, 2.0);
            if is_excute(fighter) {
                VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 115, 0, 50, 3.0, 0.0, 10.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
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
}

#[acmd_script ( agent = "lucas", scripts = ["effect_specialnfire", "effect_specialairnfire"], category = ACMD_EFFECT, low_priority)]
unsafe fn lucas_special_n_fire_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script (agent = "lucas", scripts = ["sound_specialairnfire", "sound_specialnfire"], category = ACMD_SOUND, low_priority)]
unsafe fn lucas_special_n_fire_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucas_attack05"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_n04_l"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_electric_hit_m"));    
    }
}

#[acmd_script( agent = "lucas", scripts = ["game_specialairhiend", "game_specialhiend"], category = ACMD_GAME, low_priority )]
unsafe fn game_specialhiend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        fighter.select_cliff_hangdata_from_name("special_air_hi_end");
    }
}

#[acmd_script( agent = "lucas", script = "game_fallspecial", category = ACMD_GAME, low_priority )]
unsafe fn game_fallspecial(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        fighter.select_cliff_hangdata_from_name("special_air_hi_end");
    }
}


pub fn install() {
    install_acmd_scripts!(
        lucas_special_n_start_game,
        lucas_special_n_hold_game,
        lucas_special_n_hold_sound,
        lucas_special_n_fire_sound,
        lucas_special_n_start_sound,
        lucas_special_n_hold_effect,
        lucas_special_n_fire_game,
        lucas_special_n_fire_effect,
        lucas_special_s_game,
        lucas_special_s_effect,
        lucas_special_air_s_game,
        lucas_special_s_sound,
        lucas_special_air_lw_start_game,
        lucas_special_lw_start_game,
        lucas_special_lw_start_effect,
        lucas_special_lw_hold_game,
        lucas_special_lw_end_game,
        lucas_special_lw_end_effect,
        lucas_special_lw_hit_effect,
        lucas_special_air_hi_game,
        game_specialhiend,
        game_fallspecial,
    );
}

