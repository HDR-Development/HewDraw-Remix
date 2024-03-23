use super::*;

unsafe extern "C" fn pit_special_n_fire_s_game(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn pit_special_n_fire_hi_game(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn pit_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MTRANS_AIR_UNABLE);
    }
}

unsafe extern "C" fn pit_special_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 80, 63, 0, 102, 6.0, 0.0, 6.0, 9.0, Some(0.0), Some(10.0), Some(9.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 80, 63, 0, 102, 6.0, 0.0, 13.0, 9.0, Some(0.0), Some(19.0), Some(5.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
}

unsafe extern "C" fn pit_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_s");
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
}

unsafe extern "C" fn pit_special_air_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        fighter.set_back_cliff_hangdata(10.0, 8.0);
        fighter.set_front_cliff_hangdata(15.0, 8.0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 53, 0, 94, 6.0, 0.0, 6.0, 9.0, Some(0.0), Some(10.0), Some(9.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f{x: -0.5, y: 1.75, z: 0.0});
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 53, 0, 94, 6.0, 0.0, 13.0, 9.0, Some(0.0), Some(19.0), Some(5.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        AttackModule::clear_all(boma);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            WorkModule::off_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT);
        }
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(fighter, 0.83);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn pit_special_lw_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        let facing = PostureModule::lr(boma);
        let rot1 = if facing > 0.0 { 240 } else { 300 };
        let rot2 = if facing > 0.0 { 120 } else { 60 };
        EFFECT_FOLLOW(fighter, Hash40::new("pit_guardian_shield"), Hash40::new("virtualguardianf"), 2.0 * facing, 1, 0, 0, rot1, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_guardian_shield"), Hash40::new("virtualguardianb"), 2.0 * facing, 1, 0, 0, rot2, 0, 1.5, true);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn pit_special_lw_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let facing = PostureModule::lr(boma);
        let rot1 = if facing > 0.0 { 240 } else { 300 };
        let rot2 = if facing > 0.0 { 120 } else { 60 };
        EFFECT(fighter, Hash40::new("pit_guardian_shield_end"), Hash40::new("virtualguardianf"), 2.0 * facing, 1, 0, 0, rot1, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pit_guardian_shield_end"), Hash40::new("virtualguardianb"), 2.0 * facing, 1, 0, 0, rot2, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_guardian_shield"), false, false);
    }
}

pub fn install() {
    smashline::Agent::new("pit")
        .acmd("game_specialairnfires", pit_special_n_fire_s_game)
        .acmd("game_specialairnfirehi", pit_special_n_fire_hi_game)
        .acmd("game_specialsstart", pit_special_s_start_game)
        .acmd("game_specialsend", pit_special_s_end_game)
        .acmd("game_specialairsstart", pit_special_air_s_start_game)
        .acmd("game_specialairsend", pit_special_air_s_end_game)
        .acmd("effect_speciallwstartl", pit_special_lw_start_effect)
        .acmd("effect_speciallwstartr", pit_special_lw_start_effect)
        .acmd("effect_specialairlwstartl", pit_special_lw_start_effect)
        .acmd("effect_specialairlwstartr", pit_special_lw_start_effect)
        .acmd("effect_speciallwendl", pit_special_lw_end_effect)
        .acmd("effect_speciallwendr", pit_special_lw_end_effect)
        .acmd("effect_specialairlwendl", pit_special_lw_end_effect)
        .acmd("effect_specialairlwendr", pit_special_lw_end_effect)
        .install();
}
