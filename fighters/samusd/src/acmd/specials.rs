use super::*;

// #[acmd_script( agent = "samusd", script = "game_specials" , category = ACMD_GAME , low_priority)]
// unsafe fn game_specials(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = fighter.boma();
//     frame(lua_state, 21.0);
//     if is_excute(fighter) {
//         FT_MOTION_RATE(fighter, 3.0);
//         WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
//     }
//     frame(lua_state, 22.0);
//     if is_excute(fighter) {
//         FT_MOTION_RATE(fighter, 1.0);
//     }
// }

// #[acmd_script( agent = "samusd", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
// unsafe fn game_specialairs(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = fighter.boma();
//     frame(lua_state, 21.0);
//     if is_excute(fighter) {
//         WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
//     }
//     frame(lua_state, 30.0);
//     if is_excute(fighter) {
//         FT_MOTION_RATE(fighter, 2.0);
//     }
//     frame(lua_state, 35.0);
//     if is_excute(fighter) {
//         FT_MOTION_RATE(fighter, 1.0);
//         WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
//     }
// }


unsafe extern "C" fn samusd_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_ACC_X);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 92, 100, 200, 0, 3.2, 0.0, 0.0, 5.0, Some(0.0), Some(0.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 105, 100, 200, 0, 3.2, 0.0, 0.0, -5.0, Some(0.0), Some(0.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 92, 100, 200, 0, 3.2, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 105, 100, 200, 0, 3.2, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 92, 100, 180, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 98, 100, 180, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 92, 100, 80, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 98, 100, 80, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 84, 100, 100, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 90, 100, 100, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 84, 100, 40, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 90, 100, 40, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 82, 100, 40, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 90, 100, 40, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 82, 100, 20, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 90, 100, 20, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 189, 0, 56, 10.0, 0.0, 6.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUSD_SCREW_FINISH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}


unsafe extern "C" fn samusd_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 93, 100, 115, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 96, 100, 115, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 93, 100, 60, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 96, 100, 60, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 96, 100, 70, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 98, 100, 70, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 96, 100, 40, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 98, 100, 40, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 189, 0, 50, 10.0, 0.0, 6.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUSD_SCREW_FINISH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}


unsafe extern "C" fn samusd_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON);
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_sphere") as i64);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_normal") as i64);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
    }
    FT_MOTION_RATE(fighter, 0.6);
}


unsafe extern "C" fn samusd_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON);
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_sphere") as i64);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_normal") as i64);
    }
    
}




pub fn install() {
    smashline::Agent::new("samusd")
        .acmd("game_specialhi", samusd_special_hi_game)
        .acmd("game_specialairhi", samusd_special_air_hi_game)
        .acmd("game_speciallw", samusd_special_lw_game)
        .acmd("game_specialairlw", samusd_special_air_lw_game)
        .install();
}
