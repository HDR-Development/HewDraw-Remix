
use super::*;

use hdr_modules::consts::{*, globals::*};
use hdr_modules::*;

#[acmd_script( agent = "palutena", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.8, 6.8);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
           && !ArticleModule::is_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD) {
            VarModule::set_float(boma, palutena::SPECIAL_LW_LR, PostureModule::lr(fighter.battle_object));
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT, true);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_SHIELD);
            shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
            SEARCH(fighter, 0, 0, Hash40::new("top"), 12.3, 0.0, 10.0, 1.5, None, None, None, *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.9);
        WorkModule::off_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_SHIELD);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 3.2);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }
    
}

#[acmd_script( agent = "palutena", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.8, 6.8);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
           && !ArticleModule::is_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD) {
            VarModule::set_float(boma, palutena::SPECIAL_LW_LR, PostureModule::lr(fighter.battle_object));
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT, true);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_SHIELD);
            shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
            SEARCH(fighter, 0, 0, Hash40::new("top"), 12.3, 0.0, 10.0, 1.5, None, None, None, *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.9);
        WorkModule::off_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_SHIELD);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 3.2);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }
    
}

#[acmd_script( agent = "palutena", script = "game_speciallwreflect" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_lw_reflect_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        PostureModule::set_lr(boma, VarModule::get_float(fighter.battle_object, palutena::SPECIAL_LW_LR));
        PostureModule::update_rot_y_lr(boma);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, 0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.6);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }

}

#[acmd_script( agent = "palutena", script = "game_specialairlwreflect" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_air_lw_reflect_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        PostureModule::set_lr(boma, VarModule::get_float(fighter.battle_object, palutena::SPECIAL_LW_LR));
        PostureModule::update_rot_y_lr(boma);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, 0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.6);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }

}


pub fn install() {
    install_acmd_scripts!(
        palutena_special_lw_game,
        palutena_special_air_lw_game,
        palutena_special_lw_reflect_game,
        palutena_special_air_lw_reflect_game,
    );
}

