
use super::*;


#[acmd_script( agent = "kamui", script = "game_throwlw" , category = ACMD_GAME , low_priority)]
unsafe fn game_throwlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, 0);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, smash::phx::Hash40::new("throw_lw"), false, 0.0);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 69, 100, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(boma, false);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 361, 150, 0, 40, 7.0, 0.0, 4.0, 7.0, Some(0.0), Some(4.0), Some(-6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, 3.0, 0.0);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_whole(boma, true);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        game_throwlw,
    );
}

