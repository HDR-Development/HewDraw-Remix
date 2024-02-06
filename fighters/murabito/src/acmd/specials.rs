
use super::*;


unsafe extern "C" fn murabito_special_lw1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MURABITO_STATUS_SPECIAL_LW_PLANT_FLAG_CHECK_PLANT);
        FT_MOTION_RATE(fighter, 0.661);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SEED, false, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 72, 20, 0, 80, 4.0, 0.0, 3.0, 12.0, Some(0.0), Some(3.0), Some(3.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(boma, *FIGHTER_MURABITO_STATUS_SPECIAL_LW_PLANT_FLAG_PLANT);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


unsafe extern "C" fn murabito_special_lw3_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !ItemModule::is_have_item(boma, 0){
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_WOOD), 0, 0, false, false);
        }
    }
    
}


unsafe extern "C" fn murabito_special_air_lw3_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !ItemModule::is_have_item(boma, 0){
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_WOOD), 0, 0, false, false);
        }
    }
    
}





pub fn install() {
    smashline::Agent::new("murabito")
        .acmd("game_speciallw1", murabito_special_lw1_game)
        .acmd("game_speciallw3hit", murabito_special_lw3_hit_game)
        .acmd("game_specialairlw3hit", murabito_special_air_lw3_hit_game)
        .install();
}
