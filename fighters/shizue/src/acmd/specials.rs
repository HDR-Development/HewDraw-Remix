
use super::*;

#[acmd_script( agent = "shizue", script = "game_specialnfailure" , category = ACMD_GAME , low_priority)]
unsafe fn shizue_special_n_failure_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 69, 90, 0, 70, 5.0, 0.0, 6.0, 12.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 69, 90, 0, 70, 3.0, 0.0, 6.0, 6.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            FT_MOTION_RATE(fighter, 0.8);
        }
    }
    
}

#[acmd_script( agent = "shizue", script = "effect_specialnfailure" , category = ACMD_EFFECT , low_priority)]
unsafe fn shizue_special_n_failure_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("shizue_cracker"), Hash40::new("top"), 0.0, 6.0, 8.0, 50, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "shizue", script = "game_specialairnfailure" , category = ACMD_GAME , low_priority)]
unsafe fn shizue_special_air_n_failure_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 69, 90, 0, 70, 5.0, 0.0, 6.0, 12.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 69, 90, 0, 70, 3.0, 0.0, 6.0, 6.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            FT_MOTION_RATE(fighter, 0.8);
        }
    }
    
}

#[acmd_script( agent = "shizue", script = "effect_specialairnfailure" , category = ACMD_EFFECT , low_priority)]
unsafe fn shizue_special_air_n_failure_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("shizue_cracker"), Hash40::new("top"), 0.0, 6.0, 8.0, 50, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    
}

#[acmd_script( agent = "shizue", script = "game_specialairhidetach", category = ACMD_GAME, low_priority)]
unsafe fn shizue_special_air_hi_detach_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.object(), vars::common::UP_SPECIAL_CANCEL);
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        if VarModule::is_flag(fighter.object(), vars::shizue::status::IS_NOT_QUICK_RELEASE) {
            VarModule::off_flag(fighter.object(), vars::shizue::status::IS_NOT_QUICK_RELEASE);
            WorkModule::set_float(fighter.module_accessor, VarModule::get_float(fighter.object(), vars::shizue::instance::STORED_BALLOON_POWER), *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FRAME);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        shizue_special_n_failure_game,
        shizue_special_n_failure_effect,
        shizue_special_air_n_failure_game,
        shizue_special_air_n_failure_effect,
        shizue_special_air_hi_detach_game,
    );
}

