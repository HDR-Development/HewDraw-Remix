
use super::*;

unsafe extern "C" fn game_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(8.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

unsafe extern "C" fn game_catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.9);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

unsafe extern "C" fn game_catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.9);
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(-15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

unsafe extern "C" fn game_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 56, 85, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(fighter, 8, 6);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.3);
        //FighterCutInManager::set_throw_finish_offset(boma, 3, 1, 0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if ArticleModule::is_exist(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if ArticleModule::is_exist(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("close"), false, 0.0);
        }
        FT_MOTION_RATE(fighter, 16.0/(50.0-24.0));
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

unsafe extern "C" fn game_throwhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 8.0/(5.0-1.0));
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 90, 110, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 2, 20);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.6);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, 4, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if ArticleModule::is_exist(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        if ArticleModule::is_exist(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("close"), false, 0.0);
        }
        FT_MOTION_RATE(fighter, 7.0/(31.0-27.0));
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.8);
    }
}

pub fn install() {
    smashline::Agent::new("fox")
        .acmd("game_catch", game_catch)
        .acmd("game_catchdash", game_catchdash)
        .acmd("game_catchturn", game_catchturn)
        .acmd("game_throwb", game_throwb)
        .acmd("game_throwhi", game_throwhi)
        .install();
}
