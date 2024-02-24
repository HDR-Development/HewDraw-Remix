use super::*;

#[acmd_script( agent = "pikmin", scripts = ["game_specials", "game_specialairs"] , category = ACMD_GAME , low_priority)]
unsafe fn pikmin_special_s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_SPECIAL_S_FLAG_THROW);
    }
}

#[acmd_script( agent = "pikmin", script = "game_specialnstart" , category = ACMD_GAME , low_priority)]
unsafe fn pikmin_special_n(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, WorkModule::get_float(boma, *FIGHTER_PIKMIN_STATUS_PULL_OUT_WORK_FLOAT_MOT_RATE));
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
}

#[acmd_script( agent = "pikmin", scripts = ["game_specialnfailure", "game_specialairnfailure"] , category = ACMD_GAME , low_priority)]
unsafe fn pikmin_special_n_failure(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
}

#[acmd_script( agent = "pikmin", scripts = ["game_speciallw", "game_specialairlw"] , category = ACMD_GAME , low_priority)]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_SPECIAL_LW_FLAG_SORT);
        // damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 7.5, 0.0, 7.0, -8.5, 0.0, 7.0, 8.5, 1.0, 0.8, 50, false, 0.8, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        // damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
}

#[acmd_script( agent = "pikmin", scripts = ["effect_speciallw", "effect_specialairlw"] , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 0.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pikmin_order"), Hash40::new("s_antenna4"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("pikmin_seiretsu"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 0.55, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        pikmin_special_s,
        pikmin_special_n,
        pikmin_special_n_failure,
        game_speciallw,
        effect_speciallw
    );
}

