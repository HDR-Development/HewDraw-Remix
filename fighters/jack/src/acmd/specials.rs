
use super::*;

#[acmd_script( agent = "jack", script = "effect_specialn1" , category = ACMD_EFFECT , low_priority)]
unsafe fn jack_special_n1_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_gun_muzzle"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_staff_shot"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 10.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 20.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 30.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 40.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 50.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 60.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
    }
}

#[acmd_script( agent = "jack", script = "effect_specialairn1" , category = ACMD_EFFECT , low_priority)]
unsafe fn jack_special_air_n1_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_gun_muzzle"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_staff_shot"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 10.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 20.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 30.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 40.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 50.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 60.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
    }
}

#[acmd_script( agent = "jack", script = "effect_specialn2" , category = ACMD_EFFECT , low_priority)]
unsafe fn jack_special_n2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_gun_muzzle"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_staff_shot"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 10.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 20.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 30.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 40.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 50.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 60.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
    }
}

#[acmd_script( agent = "jack", script = "effect_specialairn2" , category = ACMD_EFFECT , low_priority)]
unsafe fn jack_special_air_n2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_gun_muzzle"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_staff_shot"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 10.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 20.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 30.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 40.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 50.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 60.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
    }
}

#[acmd_script( agent = "jack", script = "effect_specialn3" , category = ACMD_EFFECT , low_priority)]
unsafe fn jack_special_n3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_gun_muzzle"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_staff_shot"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 10.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 20.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 30.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 40.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 50.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 60.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
    }
}

#[acmd_script( agent = "jack", script = "effect_specialairn3" , category = ACMD_EFFECT , low_priority)]
unsafe fn jack_special_air_n3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("jack_gun_muzzle"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_staff_shot"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 10.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 20.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 30.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 40.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 50.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("gunl"), 0, 0.8, 60.0, 0, 0, 0, 0.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 20.0);
    }
}

#[acmd_script( agent = "jack", script = "effect_specialnlanding" , category = ACMD_EFFECT , low_priority)]
unsafe fn jack_special_n_landing_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("jack_gun_muzzle"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("jack_gunspecial_muzzle"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_genesis_beam"), true, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "jack", script = "game_specialhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn jack_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("special_hi"), false, 0.0);
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
        }
        frame(lua_state, 19.0);
        if is_excute(fighter) {
            ArticleModule::change_status(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            AreaModule::reset_area(boma, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
            ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
            AreaModule::reset_area(boma, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
            ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            SEARCH(fighter, 0, 0, Hash40::new("throw"), 3.0, 1.5, 1.5, 0.7, Some(1.5), Some(-15.0), Some(-6.4), *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
            SEARCH(fighter, 1, 0, Hash40::new("throw"), 5.5, 1.5, 1.5, 0.7, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
            SEARCH(fighter, 2, 0, Hash40::new("throw"), 5.5, 1.2, 1.5, 0.7, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
        }
        frame(lua_state, 21.0);
        if is_excute(fighter) {
            AttackModule::clear(boma, 1, false);
            search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
        }
        frame(lua_state, 22.0);
        if is_excute(fighter) {
        SET_SEARCH_SIZE_EXIST(fighter, 2, 8);
        }
        frame(lua_state, 23.0);
        if is_excute(fighter) {
            UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
        }
        frame(lua_state, 24.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 27.0);
        if is_excute(fighter) {
            ArticleModule::change_status(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
            UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        }
}

#[acmd_script( agent = "jack", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn jack_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_JACK_CLIFF_HANG_DATA_AIR_LASSO as u32);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ArticleModule::change_status(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        AreaModule::reset_area(boma, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        AreaModule::reset_area(boma, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
        ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 10.0, 70, 60, 0, 60, 6.5, 1.5, 2.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70,60, 0, 60, 6.5, 0.0, 22.0, 3.2, Some(0.0), Some(38.0), Some(10.6), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 10.0, 255, 15, 0, 70, 6.5, 1.5, 2.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        SET_SEARCH_SIZE_EXIST(fighter, 2, 8);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ArticleModule::change_status(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
    }
    
}

#[acmd_script( agent = "jack", script = "game_specialairhif" , category = ACMD_GAME , low_priority)]
unsafe fn jack_special_air_hi_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        DamageModule::add_damage(boma, 1.0, 0);
        KineticModule::mul_speed(boma, &Vector3f::new(0.8, 1.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_STOP);
        ArticleModule::change_motion(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WING, Hash40::new("special_hi2_f"), false, 0.0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 34.0);
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_HI2_FLAG_APPEAR_DOYLE);
    }
}

pub fn install() {
    install_acmd_scripts!(
        //jack_special_n1_effect,
        //jack_special_air_n1_effect,
        //jack_special_n2_effect,
        //jack_special_air_n2_effect,
        //jack_special_n3_effect,
        //jack_special_air_n3_effect,
        //jack_special_n_landing_effect,
        jack_special_air_hi_game,
        jack_special_air_hi_f_game,
        jack_special_hi_game,
    );
}

