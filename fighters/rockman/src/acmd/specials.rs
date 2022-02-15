
use super::*;

#[acmd_script( agent = "rockman", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn rockman_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //VarModule::off_flag(fighter.battle_object, rockman::DETONATE_CRASHBOMB);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, false, 0);
        }
        else{
            //VarModule::on_flag(fighter.battle_object, rockman::DETONATE_CRASHBOMB);
            ArticleModule::change_status(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, *WEAPON_ROCKMAN_CRASHBOMB_STATUS_KIND_EXPLODE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        
    }
    
}

#[acmd_script( agent = "rockman", script = "effect_specials" , category = ACMD_EFFECT , low_priority)]
unsafe fn rockman_special_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_clashbomb_start"), Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 11, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
        }
        else{
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_clashbomb_start"), Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 11, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rockman_clashbomb_light"), Hash40::new("havel"), 0, 0, 1.2, 0, 0, 0, 1, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 5.0, 0, 0, 0, 0, 0, 1.2, 3, 3, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_clashbomb_fire"), Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.3, 9.8, 0, 0, 0, 1, true, *EF_FLIP_YZ);
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_clashbomb_fire"), Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.3, 9.8, 0, 0, 0, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_clashbomb_misfire"), Hash40::new("rockman_clashbomb_misfire"), Hash40::new("top"), 0, 7.5, 10, 0, 0, 0, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
        }
    }
}

#[acmd_script( agent = "rockman", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn rockman_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //VarModule::off_flag(fighter.battle_object, rockman::DETONATE_CRASHBOMB);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, false, 0);
        }
        else{
            //VarModule::on_flag(fighter.battle_object, rockman::DETONATE_CRASHBOMB);
            ArticleModule::change_status(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, *WEAPON_ROCKMAN_CRASHBOMB_STATUS_KIND_EXPLODE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        
    }
    
}

#[acmd_script( agent = "rockman", script = "effect_specialairs" , category = ACMD_EFFECT , low_priority)]
unsafe fn rockman_special_air_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW(fighter,  Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 10, 0, -90, 0, 0.62, true);
        }
        else{
            EFFECT_FOLLOW(fighter,  Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 10, 0, -90, 0, 0.62, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 5.0, 0, 0, 0, 0, 0, 1.2, 3, 3, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW(fighter, Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.5, 11, 0, -0.0, 0, 1, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.5, 11, 0, -0.0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
            EFFECT_FOLLOW(fighter, Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.5, 11, 0, -0.0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
        }
    }
}


pub fn install() {
    install_acmd_scripts!(
        rockman_special_s_game,
        rockman_special_s_effect,
        rockman_special_air_s_game,
        rockman_special_air_s_effect,
    );
}

