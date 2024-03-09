
use super::*;

unsafe extern "C" fn nana_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 9.0, 80, 60, 0, 80, 4.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
        ArticleModule::shoot_exist(boma, *FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_ENABLE_COUPLE);
    }
}

unsafe extern "C" fn nana_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 9.0, 80, 60, 0, 80, 4.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
        ArticleModule::shoot_exist(boma, *FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_ENABLE_COUPLE);
    }
}

pub fn install() {
    smashline::Agent::new("nana")
        .acmd("game_specialn_nana", nana_special_n_game)
        .acmd("game_specialairn_nana", nana_special_air_n_game)
        .install();
}
