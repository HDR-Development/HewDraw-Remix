
use super::*;


#[acmd_script( agent = "palutena", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn palutena_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("bust"), 80.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_TARGET_EXIST){
            FT_MOTION_RATE(fighter, 0.75);
        }
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

#[acmd_script( agent = "palutena", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn palutena_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("bust"), 80.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_TARGET_EXIST){
            FT_MOTION_RATE(fighter, 0.75);
        }
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}


#[acmd_script( agent = "palutena", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.8, 6.8);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
           && !ArticleModule::is_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD)
           && !(VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) > 0) {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 1); // Start counting the cooldown timer
            VarModule::set_float(fighter.battle_object, vars::palutena::status::SPECIAL_LW_LR, PostureModule::lr(fighter.module_accessor));
            VarModule::on_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR);
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
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.8, 6.8);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
           && !ArticleModule::is_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD)
           && !(VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) > 0) {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 1); // Start counting the cooldown timer
            VarModule::set_float(fighter.battle_object, vars::palutena::status::SPECIAL_LW_LR, PostureModule::lr(fighter.module_accessor));
            VarModule::on_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR);
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
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            FT_MOTION_RATE(fighter, 1.5);
            PostureModule::set_lr(boma, VarModule::get_float(fighter.battle_object, vars::palutena::status::SPECIAL_LW_LR));
            PostureModule::update_rot_y_lr(boma);
        }
        else {
            shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, 0);
            FT_MOTION_RATE(fighter, 0.75);
        }
        else{
            FT_MOTION_RATE(fighter, 0.6);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }

}

#[acmd_script( agent = "palutena", script = "effect_speciallwreflect" , category = ACMD_EFFECT , low_priority)]
unsafe fn palutena_special_lw_reflect_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
            EFFECT(fighter, Hash40::new("palutena_throw_twinkle"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT(fighter, Hash40::new("palutena_mirror_break"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.225, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT(fighter, Hash40::new("palutena_mirror"), Hash40::new("top"), 12.0, 12.0, 0, 0, 22.5, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            EFFECT_DETACH_KIND(fighter, Hash40::new("palutena_backlight"), -1);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), -1, 21, 1, 0, 90, 0, 1, true, 0.7);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT_OFF_KIND(fighter, Hash40::new("palutena_wand_light_trace"), false, false);
            EFFECT_OFF_KIND(fighter, Hash40::new("palutena_wand_light2"), false, false);
        }
    }

}

#[acmd_script( agent = "palutena", script = "game_specialairlwreflect" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_air_lw_reflect_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            FT_MOTION_RATE(fighter, 1.5);
            PostureModule::set_lr(boma, VarModule::get_float(fighter.battle_object, vars::palutena::status::SPECIAL_LW_LR));
            PostureModule::update_rot_y_lr(boma);
        }
        else {
            shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, 0);
            FT_MOTION_RATE(fighter, 0.75);
        }
        else{
            FT_MOTION_RATE(fighter, 0.6);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }

}

#[acmd_script( agent = "palutena", script = "effect_specialairlwreflect" , category = ACMD_EFFECT , low_priority)]
unsafe fn palutena_special_air_lw_reflect_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
            EFFECT(fighter, Hash40::new("palutena_throw_twinkle"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT(fighter, Hash40::new("palutena_mirror_break"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.225, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT(fighter, Hash40::new("palutena_mirror"), Hash40::new("top"), 12.0, 12.0, 0, 0, 22.5, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            EFFECT_DETACH_KIND(fighter, Hash40::new("palutena_backlight"), -1);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), -1, 21, 1, 0, 90, 0, 1, true, 0.7);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT_OFF_KIND(fighter, Hash40::new("palutena_wand_light_trace"), false, false);
            EFFECT_OFF_KIND(fighter, Hash40::new("palutena_wand_light2"), false, false);
        }
    }

}


pub fn install() {
    install_acmd_scripts!(
        palutena_special_n_game,
        palutena_special_air_n_game,
        palutena_special_lw_game,
        palutena_special_air_lw_game,
        palutena_special_lw_reflect_game,
        palutena_special_lw_reflect_effect,
        palutena_special_air_lw_reflect_game,
        palutena_special_air_lw_reflect_effect,
    );
}

