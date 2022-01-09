
use super::*;


#[acmd_script( agent = "koopajr", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn koopajr_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA, false, 0);
    }
}

#[acmd_script( agent = "koopajr", script = "game_specialirlw" , category = ACMD_GAME , low_priority)]
unsafe fn koopajr_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA, false, 0);
    }
}

#[acmd_script( agent = "koopajr_cannonball", script = "game_hop" , category = ACMD_GAME , low_priority)]
unsafe fn koopajr_cannonball_hop_game(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = fighter.boma();
    let mut keep_hitbox = false;

    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL) {
        keep_hitbox = true;
        PostureModule::reverse_lr(boma);
    }
    if is_excute(weapon) {
        ATTACK(weapon,0,0,Hash40::new("top"),1.0,48,52,0,90,3.8,0.0,0.0,0.0,None,None,None,1.0,0.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,false,8,0.0,0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 1.0);
    if is_excute(weapon) {
        if !keep_hitbox {
            AttackModule::clear_all(boma);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        koopajr_special_lw_game,
        koopajr_special_air_lw_game,
        koopajr_cannonball_hop_game,
    );
}

