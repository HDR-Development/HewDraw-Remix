
use super::*;

#[acmd_script( agent = "toonlink", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.53, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1.5, 0, 1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }  
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "toonlink", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn toonlink_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "toonlink", script = "effect_jumpaerialfront" , category = ACMD_EFFECT , low_priority)]
unsafe fn toonlink_jump_aerial_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new_raw(0x0f97480fb2), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_chicken_feather"), Hash40::new("top"), 0.0, 5.0, -2.0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 3.0);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_chicken_feather"), Hash40::new("top"), 0.0, 4.0, 1.5, 0.0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.7, 0.7, 3.0);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    
}

#[acmd_script( agent = "toonlink", script = "effect_jumpaerialback" , category = ACMD_EFFECT , low_priority)]
unsafe fn toonlink_jump_aerial_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new_raw(0x0f97480fb2), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_chicken_feather"), Hash40::new("top"), 0.0, 5.0, -2.0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.7, 0.7, 3.0);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_chicken_feather"), Hash40::new("top"), 0.0, 4.0, 1.5, 0.0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.7, 0.7, 3.0);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    
}

#[acmd_script( agent = "toonlink", script = "game_aircatch" , category = ACMD_GAME , low_priority)]
unsafe fn toonlink_air_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, false, 0);
        ArticleModule::generate_article(boma, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, 0);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 6.0, 110, 40, 0, 85, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ArticleModule::change_status(boma, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(boma, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, smash::phx::Hash40::new("shoot"), false, 0.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        ArticleModule::change_status_exist(boma, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(boma, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, smash::phx::Hash40::new("back"), false, 0.0);
        WorkModule::on_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 68.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(boma, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }       
    
}

#[acmd_script( agent = "toonlink", script = "effect_aircatch" , category = ACMD_EFFECT , low_priority)]
unsafe fn toonlink_air_catch_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 5.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "toonlink", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "toonlink", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_slide_hit_xlu_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_slide_hit_xlu_frame"));
    let escape_air_slide_hit_normal_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_slide_hit_normal_frame"));
    let ledgegrab_frame = (escape_air_slide_hit_xlu_frame + escape_air_slide_hit_normal_frame) + 4.0;

    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, ledgegrab_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
        //dash_effect,
        toonlink_turn_dash_game,
		toonlink_jump_aerial_f_effect,
        toonlink_jump_aerial_b_effect,
        toonlink_air_catch_game,
        toonlink_air_catch_effect,
    );
}

