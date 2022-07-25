 
use super::*;


#[acmd_script( agent = "inkling", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn inkling_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, smash::phx::Hash40::new("attack_s4_s"), true, 0.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, smash::phx::Hash40::new("attack_s4_s"), true, 6.0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 80, 0, 35, 3.0, 0.0, 6.0, 11.7, Some(0.0), Some(6.0), Some(6.8), 0.95, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 361, 100, 0, 40, 4.5, 0.0, 6.0, 18.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            AttackModule::set_ink_value(boma, 0, 80.0);
            AttackModule::set_ink_value(boma, 1, 120.0);
        }
        if !WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 70, 0, 35, 3.0, 0.0, 6.0, 11.7, Some(0.0), Some(6.0), Some(6.8), 0.95, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 70, 0, 35, 4.3, 0.0, 6.0, 18.0, None, None, None, 0.95, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

#[acmd_script( agent = "inkling", script = "game_attacks4charge" , category = ACMD_GAME , low_priority)]
unsafe fn inkling_attack_s4_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, smash::phx::Hash40::new_raw(0x104aaf9b16), false, 0.0);
    }

}

#[acmd_script( agent = "inkling", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn inkling_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BLASTER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("attack_hi4"), true, 0.0);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        if WorkModule::is_flag(boma,  *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
            ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("attack_hi4"), true, WorkModule::get_float(boma, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME));
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 105, 50, 110, 0, 3.3, 0.0, 3.0, 8.2, None, None, None, 0.95, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 105, 50, 110, 0, 3.3, 0.0, 7.0, 8.3, None, None, None, 0.95, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 90, 100, 0, 50, 8.3, 0.0, 20.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 90, 85, 0, 50, 14.0, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(boma, 0, 120.0);
            AttackModule::set_ink_value(boma, 1, 100.0);
        }
        if !WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 88, 66, 0, 55, 8.0, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 88, 66, 0, 55, 12.0, 0.0, 20.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

#[acmd_script( agent = "inkling", script = "game_attackhi4charge" , category = ACMD_GAME , low_priority)]
unsafe fn inkling_attack_hi4_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new_raw(0x11a8f5a6bf), false, 0.0);
    }

}

pub fn install() {
    install_acmd_scripts!(
        inkling_attack_s4_s_game,
        inkling_attack_s4_hold_game,
        inkling_attack_hi4_game,
        inkling_attack_hi4_hold_game,
    );
}
