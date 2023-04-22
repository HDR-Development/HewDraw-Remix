use super::*;

#[acmd_script( agent = "brave", scripts = ["game_specialn1", "game_specialairn1"] , category = ACMD_GAME , low_priority)]
unsafe fn brave_special_n1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
            ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE(fighter, 14.0/(44.0 - 20.0));
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_REVERT_FALL_SPEED);
    }
    frame(lua_state, 44.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "brave", script = "game_specialn2" , category = ACMD_GAME , low_priority)]
unsafe fn brave_special_n2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 8.0, 6.0);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
            ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE(fighter, 16.0/(52.0 - 26.0));
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
        
    }
    frame(lua_state, 52.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "brave", script = "game_specialairn2" , category = ACMD_GAME , low_priority)]
unsafe fn brave_special_air_n2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 8.0, 6.0);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_HOP);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
            ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE(fighter, 16.0/(52.0 - 26.0));
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(lua_state, 52.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "brave", script = "game_specialn3" , category = ACMD_GAME , low_priority)]
unsafe fn brave_special_n3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 19.0/(16.0 - 1.0));
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 9.0, 6.0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
            ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_FIREBALL, false, 0);
        }    
    }
    frame(lua_state, 38.0);
    FT_MOTION_RATE(fighter, 20.0/(68.0 - 38.0));
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 4.0);
    }
    frame(lua_state, 68.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "brave", script = "game_specialairn3" , category = ACMD_GAME , low_priority)]
unsafe fn brave_special_air_n3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 19.0/(16.0 - 1.0));
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 9.0, 6.0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
            ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_FIREBALL, false, 0);
            KineticModule::add_speed(boma, &Vector3f::new(-0.1, 0.5, 0.0));
        }    
    }
    frame(lua_state, 38.0);
    FT_MOTION_RATE(fighter, 20.0/(68.0 - 38.0));
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 4.0);
    }
    frame(lua_state, 68.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "brave", scripts = ["game_specials1", "game_specialairs1"] , category = ACMD_GAME , low_priority)]
unsafe fn brave_special_s1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 9.0/(9.0 - 1.0));
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 11.0, 6.0);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_ENABLE_SPARK);
    }
    if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_SUCCESS_SP) {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 45, 100, 0, 35, 8.0, 0.0, 8.0, 11.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 100, 0, 35, 6.0, 0.0, 10.5, 17.0, Some(0.0), Some(10.5), Some(17.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 100, 0, 35, 6.0, 0.0, 10.5, 22.0, Some(0.0), Some(10.5), Some(22.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 100, 0, 35, 6.0, 0.0, 10.5, 26.5, Some(0.0), Some(10.5), Some(26.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 100, 0, 35, 5.5, 0.0, 10.5, 29.0, Some(0.0), Some(10.5), Some(29.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else{
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 45, 100, 0, 30, 7.0, 0.0, 9.0, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 6.0);
    }
}

#[acmd_script( agent = "brave", scripts = ["game_specials2", "game_specialairs2"] , category = ACMD_GAME , low_priority)]
unsafe fn brave_special_s2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 13.0/(10.0 - 1.0));
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_ENABLE_SPARK);
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_SUCCESS_SP) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 90, 0, 25, 4.5, 0.0, 11.5, 12.0, Some(0.0), Some(11.5), Some(50.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 10, 4);
        }   
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "brave", scripts = ["game_specialhi1", "game_specialairhi1"] , category = ACMD_GAME , low_priority)]
unsafe fn brave_special_hi1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 7.0/(4.0 - 1.0));
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, 0);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE(fighter, 11.0/(41.0 - 20.0));
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
    frame(lua_state, 41.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

#[acmd_script( agent = "brave", scripts = ["game_specialhi2", "game_specialairhi2"] , category = ACMD_GAME , low_priority)]
unsafe fn brave_special_hi2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 11.0/(6.0 - 1.0));
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, 0);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
    
}

#[acmd_script( agent = "brave", scripts = ["game_specialhi3", "game_specialairhi3"] , category = ACMD_GAME , low_priority)]
unsafe fn brave_special_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 14.0/(8.0 - 1.0));
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, 0);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
    
}

#[acmd_script( agent = "brave", script = "game_speciallwstart", category = ACMD_GAME, low_priority )]
unsafe fn brave_special_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    //FT_MOTION_RATE(fighter, 17.0/(20.0 - 1.0));
}

#[acmd_script( agent = "brave", script = "game_specialairlw10", category = ACMD_GAME, low_priority )]
unsafe fn brave_special_air_lw10_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 70, 65, 0, 80, 7.5, 0.0, 2.5, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 15.0);
}

#[acmd_script( agent = "brave", scripts = ["game_speciallw20", "game_specialairlw20"], category = ACMD_GAME, low_priority )]
unsafe fn brave_special_lw20_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
        ATTACK(fighter, 0, 0, Hash40::new("sword1"), 35.0, 55, 46, 0, 110, 3.3, 8.8, 0.0, -2.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("sword1"), 35.0, 55, 46, 0, 110, 3.8, 3.0, 0.0, -1.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 30.0, 55, 46, 0, 110, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        brave_special_n1_game,
        brave_special_n2_game,
        brave_special_air_n2_game,
        brave_special_n3_game,
        brave_special_air_n3_game,
        brave_special_s1_game,
        brave_special_s2_game,
        brave_special_hi1_game,
        brave_special_hi2_game,
        brave_special_hi3_game,
        brave_special_lw_start_game,
        brave_special_air_lw10_game,
        brave_special_lw20_game,
    );
}