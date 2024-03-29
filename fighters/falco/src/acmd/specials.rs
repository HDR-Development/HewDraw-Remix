use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 6.0/(14.0-1.0));
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
}

unsafe extern "C" fn sound_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_special_n02"));
    }
}

unsafe extern "C" fn game_specialnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //DamageModule::add_damage(boma, 1.0, 0);
        FT_MOTION_RATE(agent, 10.0/(4.0-1.0));
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("loop"), false, 0.0);
        }
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 0.25);
    frame(lua_state, 17.0);
    if is_excute(agent) {
    WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
}

unsafe extern "C" fn game_specialairnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 4.0/(5.0-1.0));
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
}

unsafe extern "C" fn game_specialairnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //DamageModule::add_damage(boma, 1.0, 0);
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
        FT_MOTION_RATE(agent, 5.0/(4.0-1.0));
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        //DamageModule::add_damage(boma, 1.0, 0);
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("loop"), false, 0.0);
        }
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
        if WorkModule::is_flag(boma,*FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP){
            //laser loop motion rate(changing this allows for faster or slower sequential lasers from the first)
            FT_MOTION_RATE(agent,0.10);
        }
    }
}

unsafe extern "C" fn game_specialairsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) { }
}
 
 unsafe extern "C" fn game_specialhiholdair(agent: &mut L2CAgentBase) {
        let lua_state = agent.lua_state_agent;
        let boma = agent.boma();
    frame(lua_state, 20.0);
    for _ in 0..7 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 368, 40, 0, 30, 7.0, 0.0, 6.0, 0.0, None, None, None, 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            let target = Vector2f { x: 0.0, y: 5.0 };
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &target, 8, false);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 16.0, 80, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 11.0, 80, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.0, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 92, 6.5, 0.0, 7.0, -2.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ReflectorModule::set_status(boma, *FIGHTER_FALCO_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.7);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.0, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 92, 6.5, 0.0, 7.0, -2.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ReflectorModule::set_status(boma, *FIGHTER_FALCO_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.7);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		//LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		EFFECT_FOLLOW(agent, Hash40::new("falco_ref_loop"), Hash40::new("top"), 0, 7, -2.0, 0, 0, 0, 0.8, true);
		EffectModule::preset_limit_num(boma, 2);
		EFFECT_FOLLOW(agent, Hash40::new("falco_ref_flash"), Hash40::new("reflector"), 1.4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("falco_ref_ref"), Hash40::new("top"), 0, 7.27, -2.0, 0, 0, 0, 0.5, true);
        //FLASH(agent, 1, 1, 1, 0.627);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 5.0, 0, 0, 0, 0, 2.8, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.5);
	}
    // frame(lua_state, 1.0);
	// if is_excute(agent) {
	// 	EFFECT_FOLLOW(agent, Hash40::new("falco_ref_ref"), Hash40::new("top"), 0, 7.27, -2.0, 0, 0, 0, 0.5, true);
    // }
	// frame(lua_state, 32.0);
	// if is_excute(agent) {
	// 	EFFECT_OFF_KIND(agent, Hash40::new("falco_ref_loop"), false, false);
	// 	EFFECT_OFF_KIND(agent, Hash40::new("falco_ref_ref"), false, false);
	// 	EFFECT_OFF_KIND(agent, Hash40::new("falco_ref_flash"), true, false);
	// 	EFFECT_FLW_POS(agent, Hash40::new("sys_flash"), Hash40::new("reflector"), 1.4, -0.6, -0.5, 0, 0, 0, 0.5, true);
	// }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_shield_on"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_item_get"));
    }
}

unsafe extern "C" fn sound_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_item_get"));
    }
}

unsafe extern "C" fn game_speciallwloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairlwloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

}

unsafe extern "C" fn effect_speciallwloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if !VarModule::is_flag(agent.battle_object, vars::falco::status::SET_EFFECT) {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("falco_ref_loop"), Hash40::new("top"), 0, 7, -2, 0, 0, 0, 0.8, true);
            EFFECT_FOLLOW(agent, Hash40::new("falco_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1, true);
            VarModule::on_flag(agent.battle_object, vars::falco::status::SET_EFFECT);
        }
    }
    if is_excute(agent) {
        FLASH(agent, 0, 0.5, 1, 0.25);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    wait(lua_state, 1.0);
}

unsafe extern "C" fn sound_speciallwloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //STOP_SE(agent, Hash40::new("se_falco_special_l02"));
        //PLAY_STATUS(agent, Hash40::new("se_falco_special_l02"));
    }
}

unsafe extern "C" fn expression_speciallwloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_animcmd::wait_loop_sync_mot(lua_state);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 15.0);
    ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
}

unsafe extern "C" fn expression_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_shield_off"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// unsafe fn sound_specialairhi(agent: &mut L2CAgentBase) {
//     let lua_state = agent.lua_state_agent;
//     let boma = sv_system::battle_object_module_accessor(lua_state);
//     frame(lua_state, 1.0);
//     if is_excute(agent) {
//         PLAY_SEQUENCE(agent, Hash40::new("seq_falco_rnd_firebird"));
//         PLAY_SE(agent, Hash40::new("se_falco_special_h02"));
//     }
// }

// unsafe fn sound_specialhi(agent: &mut L2CAgentBase) {
//     let lua_state = agent.lua_state_agent;
//     let boma = sv_system::battle_object_module_accessor(lua_state);
//     frame(lua_state, 1.0);
//     if is_excute(agent) {
//         PLAY_SEQUENCE(agent, Hash40::new("seq_falco_rnd_firebird"));
//         PLAY_SE(agent, Hash40::new("se_falco_special_h02"));
//     }
// }

unsafe extern "C" fn stub(agent: &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart);
    agent.acmd("game_specialnloop", game_specialnloop);
    agent.acmd("game_specialairnstart", game_specialairnstart);
    agent.acmd("game_specialairnloop", game_specialairnloop);

    agent.acmd("game_specialairsend", game_specialairsend);

    agent.acmd("game_specialhiholdair", game_specialhiholdair);
    agent.acmd("game_specialhi", game_specialhi);

    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("game_specialairlw", game_specialairlw);
    agent.acmd("effect_speciallw", effect_speciallw);
    agent.acmd("effect_specialairlw", effect_speciallw);
    agent.acmd("expression_speciallw", expression_speciallw);
    agent.acmd("expression_specialairlw", expression_speciallw);
    agent.acmd("sound_speciallw", sound_speciallw);
    agent.acmd("sound_specialairlw", sound_specialairlw);
    agent.acmd("game_speciallwloop", game_speciallwloop);
    agent.acmd("game_specialairlwloop", game_specialairlwloop);
    agent.acmd("effect_speciallwloop", effect_speciallwloop);
    agent.acmd("effect_specialairlwloop", effect_speciallwloop);
    agent.acmd("sound_speciallwloop", sound_speciallwloop);
    agent.acmd("sound_specialairlwloop", sound_speciallwloop);
    agent.acmd("expression_speciallwloop", expression_speciallwloop);
    agent.acmd("expression_specialairlwloop", expression_speciallwloop);
    agent.acmd("game_speciallwend", stub);
    agent.acmd("game_specialairlwend", stub);
    agent.acmd("effect_speciallwend", stub);
    agent.acmd("effect_specialairlwend", stub);
    agent.acmd("sound_speciallwend", stub);
    agent.acmd("sound_specialairlwend", stub);
    agent.acmd("expression_speciallwend", expression_speciallwend);
    agent.acmd("expression_specialairlwend", expression_speciallwend);
    agent.acmd("sound_specialnstart", sound_specialnstart);
    agent.acmd("sound_specialairnstart", sound_specialnstart);
}
