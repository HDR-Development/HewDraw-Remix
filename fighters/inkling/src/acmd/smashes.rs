
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
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 95, 0, 36, 3.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 361, 95, 0, 36, 6.0, 0.0, 7.0, 17.0, Some(0.0), Some(7.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            AttackModule::set_ink_value(boma, 0, 100.0);
            AttackModule::set_ink_value(boma, 1, 120.0);
        }
        if !WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 93, 0, 35, 3.0, 0.0, 6.0, 11.0, Some(0.0), Some(6.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 93, 0, 35, 5.0, 0.0, 6.0, 17.0, Some(0.0), Some(6.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            AttackModule::clear_all(boma);
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
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 125, 100, 100, 0, 3.5, 0.0, 4.0, 9.5, Some(0.0), Some(4.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 88, 82, 0, 55, 10.0, 0.0, 20.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 88, 100, 0, 55, 14.0, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(boma, 0, 120.0);
            AttackModule::set_ink_value(boma, 1, 100.0);
        }
        if !WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 88, 66, 0, 55, 10.0, 0.0, 20.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 88, 66, 0, 55, 12.0, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_DESIRED_RATE(fighter, 63.0-22.0, 31.0);
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
