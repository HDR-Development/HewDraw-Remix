
use super::*;


#[acmd_script( agent = "falco", script = "game_specialnstart" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 6.0/(14.0-1.0));
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
}

#[acmd_script( agent = "falco", script = "game_specialnloop" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_n_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        //DamageModule::add_damage(boma, 1.0, 0);
        FT_MOTION_RATE(fighter, 10.0/(4.0-1.0));
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("loop"), false, 0.0);
        }
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 0.25);
    frame(lua_state, 17.0);
    if is_excute(fighter) {
    WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
}

#[acmd_script( agent = "falco", script = "game_specialairnstart" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_air_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 4.0/(5.0-1.0));
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
}

#[acmd_script( agent = "falco", script = "game_specialairnloop" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_air_n_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        //DamageModule::add_damage(boma, 1.0, 0);
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
        FT_MOTION_RATE(fighter, 5.0/(4.0-1.0));
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        //DamageModule::add_damage(boma, 1.0, 0);
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("loop"), false, 0.0);
        }
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
        if WorkModule::is_flag(boma,*FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP){
            //laser loop motion rate(changing this allows for faster or slower sequential lasers from the first)
            FT_MOTION_RATE(fighter,0.10);
        }
    }
}

#[acmd_script( agent = "falco", script = "game_specialairsend" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_air_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) { }
}
 #[acmd_script( agent = "falco", script = "game_specialhiholdair" , category = ACMD_GAME , low_priority)]
 unsafe fn falco_special_hi_hold(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
        let boma = fighter.boma();
    frame(lua_state, 20.0);
    for _ in 0..7 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 368, 40, 0, 30, 7.0, 0.0, 6.0, 0.0, None, None, None, 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            let target = smash::phx::Vector2f { x: 0.0, y: 5.0 };
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &target, 8, false);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
}

#[acmd_script( agent = "falco", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 16.0, 80, 60, 0, 80, 5.0, 2.5, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 11.0, 80, 60, 0, 80, 5.0, 2.5, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
}

#[acmd_script( agent = "falco", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 4.0, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 92, 6.5, 0.0, 7.0, -2.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ReflectorModule::set_status(boma, *FIGHTER_FALCO_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.7);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "falco", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 4.0, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 92, 6.5, 0.0, 7.0, -2.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ReflectorModule::set_status(boma, *FIGHTER_FALCO_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.7);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "falco", scripts = ["effect_speciallw", "effect_specialairlw"] , category = ACMD_EFFECT , low_priority)]
unsafe fn falco_special_lw_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	if is_excute(fighter) {
		//LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		EFFECT_FOLLOW(fighter, Hash40::new("falco_ref_loop"), Hash40::new("top"), 0, 7, -2.0, 0, 0, 0, 0.8, true);
		EffectModule::preset_limit_num(fighter.module_accessor, 2);
		EFFECT_FOLLOW(fighter, Hash40::new("falco_ref_flash"), Hash40::new("reflector"), 1.4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("falco_ref_ref"), Hash40::new("top"), 0, 7.27, -2.0, 0, 0, 0, 0.5, true);
        //FLASH(fighter, 1, 1, 1, 0.627);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 5.0, 0, 0, 0, 0, 2.8, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
	}
    // frame(lua_state, 1.0);
	// if is_excute(fighter) {
	// 	EFFECT_FOLLOW(fighter, Hash40::new("falco_ref_ref"), Hash40::new("top"), 0, 7.27, -2.0, 0, 0, 0, 0.5, true);
    // }
	// frame(lua_state, 32.0);
	// if is_excute(fighter) {
	// 	EFFECT_OFF_KIND(fighter, Hash40::new("falco_ref_loop"), false, false);
	// 	EFFECT_OFF_KIND(fighter, Hash40::new("falco_ref_ref"), false, false);
	// 	EFFECT_OFF_KIND(fighter, Hash40::new("falco_ref_flash"), true, false);
	// 	EFFECT_FLW_POS(fighter, Hash40::new("sys_flash"), Hash40::new("reflector"), 1.4, -0.6, -0.5, 0, 0, 0, 0.5, true);
	// }
}


#[acmd_script( agent = "falco", scripts = ["expression_speciallw", "expression_specialairlw"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn falco_special_lw_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_shield_on"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
}

#[acmd_script( agent = "falco", script = "sound_speciallw" , category = ACMD_SOUND , low_priority)]
unsafe fn falco_special_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
}

#[acmd_script( agent = "falco", script = "sound_specialairlw" , category = ACMD_SOUND , low_priority)]
unsafe fn falco_special_air_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
}

#[acmd_script( agent = "falco", script = "game_speciallwloop", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwloop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "falco", script = "game_specialairlwloop", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlwloop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "falco", scripts = ["effect_speciallwloop", "effect_specialairlwloop"], category = ACMD_EFFECT, low_priority )]
unsafe fn effect_speciallwloop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if !VarModule::is_flag(fighter.battle_object, vars::falco::status::SET_EFFECT) {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("falco_ref_loop"), Hash40::new("top"), 0, 7, -2, 0, 0, 0, 0.8, true);
            EFFECT_FOLLOW(fighter, Hash40::new("falco_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1, true);
            VarModule::on_flag(fighter.battle_object, vars::falco::status::SET_EFFECT);
        }
    }
    if is_excute(fighter) {
        FLASH(fighter, 0, 0.5, 1, 0.25);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    wait(lua_state, 1.0);
}

#[acmd_script( agent = "falco", scripts = ["sound_speciallwloop", "sound_specialairlwloop"], category = ACMD_SOUND, low_priority )]
unsafe fn sound_speciallwloop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //STOP_SE(fighter, Hash40::new("se_falco_special_l02"));
        //PLAY_STATUS(fighter, Hash40::new("se_falco_special_l02"));
    }
}

#[acmd_script( agent = "falco", scripts = ["expression_speciallwloop", "expression_specialairlwloop"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_speciallwloop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    sv_animcmd::wait_loop_sync_mot(fighter.lua_state_agent);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 15.0);
    ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
}

#[acmd_script( agent = "falco", scripts = ["game_speciallwend", "game_specialairlwend"], category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "falco", scripts = ["effect_speciallwend", "effect_specialairlwend"], category = ACMD_EFFECT, low_priority )]
unsafe fn effect_speciallwend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "falco", scripts = ["sound_speciallwend", "sound_specialairlwend"], category = ACMD_SOUND, low_priority )]
unsafe fn sound_speciallwend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "falco", scripts = ["expression_speciallwend", "expression_specialairlwend"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_speciallwend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_shield_off"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "falco", script = "sound_specialnstart" , category = ACMD_SOUND , low_priority)]
unsafe fn falco_special_n_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_n02"));
    }

}

#[acmd_script( agent = "falco", script = "sound_specialairnstart" , category = ACMD_SOUND , low_priority)]
unsafe fn falco_special_air_n_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_n02"));
    }

}

// #[acmd_script( agent = "falco", script = "sound_specialairhi" , category = ACMD_SOUND , low_priority)]
// unsafe fn falco_special_air_hi_sound(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = sv_system::battle_object_module_accessor(lua_state);
//     frame(lua_state, 1.0);
//     if is_excute(fighter) {
//         PLAY_SEQUENCE(fighter, Hash40::new("seq_falco_rnd_firebird"));
//         PLAY_SE(fighter, Hash40::new("se_falco_special_h02"));
//     }

// }

// #[acmd_script( agent = "falco", script = "sound_specialhi" , category = ACMD_SOUND , low_priority)]
// unsafe fn falco_special_hi_sound(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = sv_system::battle_object_module_accessor(lua_state);
//     frame(lua_state, 1.0);
//     if is_excute(fighter) {
//         PLAY_SEQUENCE(fighter, Hash40::new("seq_falco_rnd_firebird"));
//         PLAY_SE(fighter, Hash40::new("se_falco_special_h02"));
//     }

// }

pub fn install() {
    install_acmd_scripts!(
        falco_special_n_start_game,
        falco_special_n_loop_game,
        falco_special_air_n_start_game,
        falco_special_air_n_loop_game,
        falco_special_air_s_end_game,
        falco_special_hi_hold,
        falco_special_hi_game,
        falco_special_air_lw_game,
        falco_special_lw_game,
        falco_special_lw_effect,
        falco_special_lw_expression,
        falco_special_lw_sound,
        falco_special_air_lw_sound,
        game_speciallwloop,
        game_specialairlwloop,
        effect_speciallwloop,
        sound_speciallwloop,
        expression_speciallwloop,
        game_speciallwend,
        effect_speciallwend,
        sound_speciallwend,
        expression_speciallwend,
        falco_special_n_start_sound,
        falco_special_air_n_start_sound,
        // falco_special_air_hi_sound,
        // falco_special_hi_sound,
    );
}

