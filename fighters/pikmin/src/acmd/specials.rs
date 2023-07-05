
use super::*;

use super::PikminInfo;

// TODO need to check if the "air" names are necessary
#[acmd_script( agent = "pikmin_pikmin", scripts = ["game_splwrespond", "game_splwrespond_b","game_splwrespond_v","game_splwrespond_w","game_splwrespond_y",   "game_splwairrespond", "game_splwairrespond_b","game_splwairrespond_v","game_splwairrespond_w","game_splwairrespond_y"] , category = ACMD_GAME , low_priority)]
unsafe fn pikmin_whistle_damage(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let pikmin = PikminInfo::from(variation);
    if is_excute(fighter) && StatusModule::prev_status_kind(boma, 0) == *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING {
        AttackModule::clear_all(boma);
        let damage = 6.0;
        ATTACK(fighter, 0, 0, Hash40::new("head1"), damage * pikmin.damage, 90, 105, 0, 65, 5.0, 0.0, 0.0, 0.0, None, None, None, pikmin.hitlag * 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * pikmin.shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(pikmin.attr), *ATTACK_SOUND_LEVEL_L, pikmin.sound, *ATTACK_REGION_PIKMIN);
        
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MotionModule::set_rate(boma, 0.5);
    }
    
}


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
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, 0);
        FT_MOTION_RATE(fighter, 10.0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.5, 80, 85, 0, 45, 4.0, 0.0, 2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "pikmin_pikmin", scripts = ["game_spntakenoutstart", "game_spntakenoutstart_y", "game_spntakenoutstart_b", "game_spntakenoutstart_w", "game_spntakenoutstart_v"] , category = ACMD_GAME , low_priority)]
unsafe fn pikmin_special_n_pikmin(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 20.0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "pikmin", scripts = ["game_specialnfailure", "game_specialairnfailure"] , category = ACMD_GAME , low_priority)]
unsafe fn pikmin_special_n_failure(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.5, 80, 85, 0, 45, 4.0, 0.0, 2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        pikmin_whistle_damage,
        pikmin_special_s,
        pikmin_special_n,
        pikmin_special_n_failure,
        pikmin_special_n_pikmin
    );
}

