
use super::*;

#[acmd_script( agent = "diddy", script = "game_specialairhistart" , category = ACMD_GAME , low_priority)]
unsafe fn diddy_special_air_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        KineticModule::clear_speed_all(boma);
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        
    }
}

#[acmd_script( agent = "diddy", script = "game_specialsstickattack", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsstickattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 1, 7.0, 361, 100, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 1, 5.0, 361, 50, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 3.0, 361, 150, 0, 60, 6.0, 2.5, 2.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 150, 0, 60, 3.0, 0.0, 1.5, -7.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        CHECK_FINISH_CAMERA(agent, 0, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 5.0, y: -4.0, z: 0.0});
        AttackModule::set_catch_only_all(boma, true, false);
    }
}

#[acmd_script( agent = "diddy", script = "game_specialsstickattack2", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsstickattack2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 1, 7.0, 95, 100, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 1, 4.0, 95, 50, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        let target = WorkModule::get_int64(boma, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_TARGET_TASK);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
        SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::enable_transition_term(boma, *FIGHTER_DIDDY_STATUS_SPECIAL_S_TRANSITION_TERM_ID_GROUND);
        SET_SPEED_EX(agent, -1.5, 2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

#[acmd_script( agent = "diddy", script = "game_specialsstick", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsstick(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    //wait_loop_clear();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 1, 2.0, 361, 50, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 4.0, 0.0, 2.0, -4.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DIDDY_SCRATCH, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 4.0, 0.0, 2.0, -4.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DIDDY_SCRATCH, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    AttackModule::clear_all(boma);
}

#[acmd_script( agent = "diddy", script = "game_specialairsjump", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairsjump(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 5.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 0);
    }
}

#[acmd_script( agent = "diddy", script = "game_specialairskick", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairskick(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 14.0, 361, 50, 0, 80, 4.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 12.0, 361, 50, 0, 80, 4.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "diddy", script = "game_specialncancel" , category = ACMD_GAME , low_priority)]
unsafe fn diddy_special_n_cancel_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, -1);
        FT_MOTION_RATE(fighter, 8.0/(31.0 - 1.0));
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "diddy", script = "effect_specialncancel" , category = ACMD_EFFECT , low_priority)]
unsafe fn diddy_special_n_cancel_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "diddy", script = "sound_specialncancel" , category = ACMD_SOUND , low_priority)]
unsafe fn diddy_special_n_cancel_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "diddy", script = "expression_specialncancel" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn diddy_special_n_cancel_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "diddy", script = "game_specialairncancel" , category = ACMD_GAME , low_priority)]
unsafe fn diddy_special_air_n_cancel_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, -1);
        FT_MOTION_RATE(fighter, 8.0/(35.0 - 1.0));
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "diddy", script = "effect_specialairncancel" , category = ACMD_EFFECT , low_priority)]
unsafe fn diddy_special_air_n_cancel_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "diddy", script = "sound_specialairncancel" , category = ACMD_SOUND , low_priority)]
unsafe fn diddy_special_air_n_cancel_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "diddy", script = "expression_specialairncancel" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn diddy_special_air_n_cancel_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

pub fn install() {
    install_acmd_scripts!(
        diddy_special_air_hi_start_game,
        game_specialsstick,
        game_specialairsjump,
        game_specialairskick,
        diddy_special_n_cancel_game,
        diddy_special_n_cancel_effect,
        diddy_special_n_cancel_sound,
        diddy_special_n_cancel_expression,
        diddy_special_air_n_cancel_game,
        diddy_special_air_n_cancel_effect,
        diddy_special_air_n_cancel_sound,
        diddy_special_air_n_cancel_expression,
        //game_specialsstickattack,
        game_specialsstickattack2,
        
    );
}

