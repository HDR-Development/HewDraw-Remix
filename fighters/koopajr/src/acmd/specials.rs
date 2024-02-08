
use super::*;

/* #[acmd_script( agent = "koopajr_remainclown", script = "game_specialhiclownfall" , category = ACMD_GAME , low_priority)]
unsafe fn koopajr_remainclown_special_hi_clownfall_game(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.boma();
    frame(lua_state, 5.0);
    if is_excute(weapon) {
        ATTACK(weapon,0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 6.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE)
    }
}

#[acmd_script( agent = "koopajr_remainclown", script = "game_specialairhiclownfall" , category = ACMD_GAME , low_priority)]
unsafe fn koopajr_remainclown_special_air_hi_clownfall_game(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.boma();
    frame(lua_state, 5.0);
    if is_excute(weapon) {
        ATTACK(weapon,0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 6.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE)
    }
    for _ in 0..30 {
        wait(lua_state, 1.0);
        if is_excute(weapon) {
        }
    }
} */

// ATTACK(weapon,0, 0, Hash40::new("top"), 13.0, 55, 70, 0, 85, 14.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB)

#[acmd_script( agent = "koopajr", scripts = ["game_specialnshoot", "game_specialairnshoot"], category = ACMD_GAME, low_priority )]
unsafe fn koopajr_special_n_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if ArticleModule::is_exist(boma, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_CANNONBALL) {
           WorkModule::on_flag(boma, *FIGHTER_KOOPAJR_STATUS_SPECIAL_N_FLAG_FAIL);
        }
        else {
            WorkModule::on_flag(boma, *FIGHTER_KOOPAJR_STATUS_SPECIAL_N_FLAG_SHOOT);
        }
    }
}

#[acmd_script( agent = "koopajr", script = "effect_specialnshoot", category = ACMD_EFFECT, low_priority )]
unsafe fn koopajr_special_n_shoot_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_KOOPAJR_STATUS_SPECIAL_N_FLAG_FAIL) {
            EFFECT(fighter, Hash40::new("koopajr_cannon_miss"), Hash40::new("clowntongue2"), 3, 0, 0, 0, 0, -90, 0.5, 0, 0, 0, 0, 0, 0, true);
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        else {
            let offset = if fighter.is_situation(*SITUATION_KIND_GROUND) { 0 } else { 2 };
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
    }
}

#[acmd_script( agent = "koopajr", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME , low_priority)]
unsafe fn koopajr_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::koopajr::instance::DISABLE_MECHAKOOPA) {
            ArticleModule::generate_article(boma, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA, false, 0);
            VarModule::on_flag(fighter.object(), vars::koopajr::instance::DISABLE_MECHAKOOPA);
        }
    }
}

#[acmd_script( agent = "koopajr", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn koopajr_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("clownhip"), 4.0, 80, 95, 0, 60, 3.6, 0.0, 1.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("clownhip"), 4.0, 80, 95, 0, 60, 1.5, 0.0, 3.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "koopajr", script = "game_specialsspin" , category = ACMD_GAME , low_priority)]
unsafe fn koopajr_special_s_spin_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 10.0, 361, 65, 0, 60, 5.2, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
    }
}

#[acmd_script( agent = "koopajr", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn koopajr_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL);
        JostleModule::set_status(boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("clownhip"), 2.0, 55, 100, 0, 65, 3.6, 0.0, 1.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("clownhip"), 2.0, 55, 100, 0, 65, 1.5, 0.0, 3.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "koopajr", script = "game_specialairsspin" , category = ACMD_GAME , low_priority)]
unsafe fn koopajr_special_air_s_spin_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.0, 361, 70, 0, 60, 5.2, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_NORMAL_GRAVITY);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
            JostleModule::set_status(boma, true);
    }
}

#[acmd_script( agent = "koopajr_cannonball", script = "game_hop" , category = ACMD_GAME , low_priority)]
unsafe fn koopajr_cannonball_hop_game(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.boma();
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

#[acmd_script( agent = "koopajr", script = "game_specialhijrrise", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhijrrise(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPAJR_STATUS_SPECIAL_HI_SHOOT_FLAG_ACTION);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "koopajr", script = "game_specialhijrfall", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhijrfall(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        koopajr_special_n_shoot_game,
        koopajr_special_n_shoot_effect,
        koopajr_special_lw_game,
        koopajr_special_s_game,
        koopajr_special_s_spin_game,
        koopajr_special_air_s_game,
        koopajr_special_air_s_spin_game,
        koopajr_cannonball_hop_game,
        game_specialhijrrise,
        game_specialhijrfall
    );
}

