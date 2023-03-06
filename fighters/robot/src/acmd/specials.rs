
use super::*;

#[acmd_script( agent = "robot", script = "game_specialsstart", category = ACMD_GAME, low_priority )]
unsafe fn robot_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 2.0/(12.0-1.0));
}

#[acmd_script( agent = "robot", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn robot_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 2.0/(12.0-1.0));
}

#[acmd_script( agent = "robot", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn robot_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 65, 40, 1, 4.5, 0.0, 13.0, 10.0, Some(0.0), Some(6.0), Some(10.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_ROBOT_REFLECTOR_KIND_ARMSPIN, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 367, 70, 50, 1, 4.0, 0.0, 13.0, -8.0, Some(0.0), Some(6.0), Some(-8.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_ROBOT_REFLECTOR_KIND_ARMSPIN, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
}

#[acmd_script( agent = "robot", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn robot_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 65, 40, 0, 4.5, 0.0, 13.0, 10.0, Some(0.0), Some(6.0), Some(10.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_ROBOT_REFLECTOR_KIND_ARMSPIN, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 367, 70, 50, 0, 4.0, 0.0, 13.0, -8.0, Some(0.0), Some(6.0), Some(-8.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }

    frame(lua_state, 12.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_ROBOT_REFLECTOR_KIND_ARMSPIN, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }

}

#[acmd_script( agent = "robot", script = "game_specialsend" , category = ACMD_GAME , low_priority)]
unsafe fn robot_special_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 361, 70, 0, 65, 4.0, 0.0, 13.0, 10.0, Some(0.0), Some(6.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 70, 0, 65, 4.0, 0.0, 13.0, -8.0, Some(0.0), Some(6.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }

    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "robot", script = "game_specialairsend" , category = ACMD_GAME , low_priority)]
unsafe fn robot_special_air_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 361, 70, 0, 100, 4.0, 0.0, 13.0, 10.0, Some(0.0), Some(6.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 70, 0, 100, 4.0, 0.0, 13.0, -8.0, Some(0.0), Some(6.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "robot", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn robot_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_ROBOT_STATUS_GYRO_FLAG_SHOOT);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_ROBOT_GENERATE_ARTICLE_GYRO, false,  app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 21.0/(37.0-10.0));
    }
}

#[acmd_script( agent = "robot", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn robot_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_ROBOT_STATUS_GYRO_FLAG_SHOOT);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_ROBOT_GENERATE_ARTICLE_GYRO, false,  app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 21.0/(37.0-10.0));
    }
}

#[acmd_script( agent = "robot", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn robot_special_hi_game (fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        let vec = Vector3f{x: 0.3, y: 0.0, z: 0.0};
        KineticModule::add_speed(boma, &vec);
        WorkModule::on_flag(boma, *FIGHTER_ROBOT_STATUS_BURNER_FLAG_TRANSFORM_COMP);
    }
}


pub fn install() {
    install_acmd_scripts!(
		robot_special_s_start_game,
		robot_special_air_s_start_game,
        robot_special_s_game,
		robot_special_air_s_game,
        robot_special_s_end_game,
		robot_special_air_s_end_game,
        robot_special_lw_game,
        robot_special_air_lw_game,
        robot_special_hi_game,
    );
}

