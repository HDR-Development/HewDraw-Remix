
use super::*;

#[acmd_script( agent = "younglink", script = "game_specials1" , category = ACMD_GAME , low_priority)]
unsafe fn younglink_special_s1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.54/(5.0-1.0));
    frame(lua_state, 5.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_BOOMERANG, false, 0);
    }
    frame(lua_state, 27.0);
    FT_MOTION_RATE(fighter, 2.0);
    if is_excute(fighter) {
        ArticleModule::shoot(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_BOOMERANG, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 31.0);
    FT_MOTION_RATE(fighter, 1.0);

}

#[acmd_script( agent = "younglink", script = "game_specialairs1" , category = ACMD_GAME , low_priority)]
unsafe fn younglink_special_air_s1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.54/(5.0-1.0));
    frame(lua_state, 5.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_BOOMERANG) {
            ArticleModule::generate_article(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_BOOMERANG, false, 0);
        }
    }   
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ArticleModule::shoot(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_BOOMERANG, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }

}

#[acmd_script( agent = "younglink", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn younglink_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("sword"), 1.0, 173, 100, 55, 0, 3.5, 2.2, 0.0, 1.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("sword"), 1.0, 173, 100, 55, 0, 3.5, 5.7, 0.0, 1.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 1.0, 173, 100, 55, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 80, 100, 10, 0, 3.0, 0.0, 5.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 70, 85, 0, 90, 4.0, 0.0, 7.0, -11.0, Some(0.0), Some(4.5), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 66.0);
    FT_MOTION_RATE(fighter, 9.0/(70.0-66.0));
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
    frame(lua_state, 70.0);
    FT_MOTION_RATE(fighter, 1.0);

}

#[acmd_script( agent = "younglink", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn younglink_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 100, 0, 20, 4.0, 0.0, 9.0, 12.0, Some(0.0), Some(9.0), Some(-12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 20, 4.0, 0.0, 9.0, 12.0, Some(0.0), Some(9.0), Some(-12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 20, 4.0, 0.0, 9.0, 12.0, Some(0.0), Some(9.0), Some(-12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
     }
    wait(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 66.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
    
}

#[acmd_script( agent = "younglink", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn younglink_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) == 0 {
                VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 1);
                 ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_BOMBCHU), 0, 0, false, false);
            }
             else {
                WorkModule::on_flag(boma, *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
            }
        }
         else {
            WorkModule::on_flag(boma, *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
        }
    }
    
}

#[acmd_script( agent = "younglink", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn younglink_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) == 0 {
                VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 1);
                 ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_BOMBCHU), 0, 0, false, false);
            }
             else {
                WorkModule::on_flag(boma, *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
            }
        }
         else {
            WorkModule::on_flag(boma, *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
        }
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        younglink_special_s1_game,
        younglink_special_air_s1_game,
        younglink_special_hi_game,
        younglink_special_air_hi_game,
        younglink_special_lw_game,
        younglink_special_air_lw_game,
    );
}

