
use super::*;

#[acmd_script( agent = "pit", script = "game_specialairnfires" , category = ACMD_GAME , low_priority)]
unsafe fn pit_special_n_fire_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        let stop_rise = Vector3f{x: 1.0, y: 0.0, z: 1.0};
        KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let hop_vec = Vector3f{ x: 0.0, y: 1.0, z: 0.0 };
        KineticModule::add_speed(boma, &hop_vec);
        ArticleModule::shoot(boma, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

#[acmd_script( agent = "pit", script = "game_specialairnfirehi" , category = ACMD_GAME , low_priority)]
unsafe fn pit_special_n_fire_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        let stop_rise = Vector3f{x: 1.0, y: 0.0, z: 1.0};
        KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let hop_vec = Vector3f{ x: 0.0, y: 1.0, z: 0.0 };
        KineticModule::add_speed(boma, &hop_vec);
        ArticleModule::shoot(boma, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

#[acmd_script( agent = "pit", script = "game_specialsstart", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT);
        //damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.0, 0.0, 12.0, 9.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        //damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MTRANS_AIR_UNABLE);
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        WorkModule::off_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF);
    }
}

#[acmd_script( agent = "pit", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairsstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_s");
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        //damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.0, 0.0, 14.0, 9.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        //damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        pit_special_n_fire_s_game,
        pit_special_n_fire_hi_game,
        game_specialsstart,
        game_specialairsstart
    );
}

