
use super::*;

#[acmd_script( agent = "murabito", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "murabito", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_murabito_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

#[acmd_script( agent = "murabito", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "murabito_slingshot", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_slingshot_attackairb_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
    
    }    
    
}

#[acmd_script( agent = "murabito_bullet", script = "game_shootf" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_bullet_shoot_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 80, 0, 55, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 25, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 100, 0, 10, 1.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    
}

#[acmd_script( agent = "murabito_clayrocket", script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_clayrocket_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 110, 55, 0, 75, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    
}

#[acmd_script( agent = "murabito_flowerpot", script = "game_throwed" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_flowerpot_throwed_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 10.0, 70, 60, 0, 70, 4.2, 0.0, 2.5, 0.0, Some(0.0), Some(-2.0), Some(3.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    wait(lua_state, 2.0);
	if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 10.0, 70, 60, 0, 70, 4.2, 0.0, 2.5, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 6.0, 70, 60, 0, 70, 3.7, 0.0, 2.5, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    
}

#[acmd_script( agent = "murabito_flowerpot", script = "effect_bound" , category = ACMD_EFFECT , low_priority)]
unsafe fn murabito_flowerpot_bound_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !WorkModule::is_flag(boma, *WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_ASASE) {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("murabito_pot_boundsmoke"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_water_walk"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 4.0, 361, 60, 0, 40, 3.7, 0.0, 2.5, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}



#[acmd_script( agent = "murabito", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "murabito", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_slide_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_slide_cancel_frame"));
    
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, escape_air_slide_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "murabito", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.5);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter){
        CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 5.5, 4.0, Some(0.0), Some(5.5), Some(14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 2.5, 0.0, 5.5, 1.5, Some(0.0), Some(5.5), Some(16.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        ENABLE_AREA(fighter, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        UNABLE_AREA(fighter, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "murabito", script = "game_catchdash" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_catchdash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.5);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter){
        CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 5.5, 4.0, Some(0.0), Some(5.5), Some(13.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 2.0, 0.0, 5.5, 2.0, Some(0.0), Some(5.5), Some(15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        ENABLE_AREA(fighter, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        UNABLE_AREA(fighter, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH);
    }
}

#[acmd_script( agent = "murabito", script = "game_catchturn" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_catchturn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.5);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter){
        CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 5.5, -5.0, Some(0.0), Some(5.5), Some(-14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 2.5, 0.0, 5.5, -2.5, Some(0.0), Some(5.5), Some(-16.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        ENABLE_AREA(fighter, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        UNABLE_AREA(fighter, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH);
    }
}

pub fn install() {
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
        dash_game,
        dash_sound,
        turn_dash_game,
        murabito_slingshot_attackairb_game,
        murabito_bullet_shoot_f_game,
        murabito_flowerpot_throwed_game,
        murabito_flowerpot_bound_effect,
        murabito_clayrocket_fly_game,
        murabito_catch_game,
        murabito_catchdash_game,
        murabito_catchturn_game,
    );
}

