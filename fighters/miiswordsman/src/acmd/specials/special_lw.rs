use super::*;

// ===============================================================================================
// ======================================== BLADE COUNTER ========================================
// ===============================================================================================

unsafe extern "C" fn game_speciallw1hit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 21.0, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 80, 60, 0, 85, 8.8, 0.0, 8.0, 15.0, Some(0.0), Some(8.0), Some(3.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 47.0, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairlw1hit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 21.0, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 73, 60, 0, 85, 10.5, 0.0, 7.0, 14.0, Some(0.0), Some(7.0), Some(4.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 47.0, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
}

// Kinesis Blade - Charge Storage
// unsafe fn game_speciallw1hit(agent: &mut L2CAgentBase) {
//     let lua_state = agent.lua_state_agent;
//     let boma = agent.boma();
//     if is_excute(agent) {
//         let current_level = VarModule::get_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL);
//         // Attack transition
//         if VarModule::is_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER) {
//             VarModule::off_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER);
//             if current_level > 0{
//                 if current_level == 1 {
//                     // println!("Changing to level 1");
//                     MotionModule::change_motion(boma, Hash40::new("special_lw1_hit_lv1"), 0.0, 1.0, false, 0.0, false, false);
//                 }
//                 else{
//                     // println!("Changing to level 2");
//                     MotionModule::change_motion(boma, Hash40::new("special_lw1_hit_lv2"), 0.0, 1.0, false, 0.0, false, false);
//                 }
//             }
//         }
//         else{
//             // Increment the level
//             if current_level < 2 {
//                 VarModule::set_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, current_level + 1); // Add a charge level
//                 // println!("Kinesis increment");
//             }
//             // println!("Kinesis Level: {}", VarModule::get_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL));
//         }
//     }
//     frame(lua_state, 23.0);
//     if is_excute(agent) {
//         WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
//         //FT_MOTION_RATE(agent, 0.65);
//     }
// }

// unsafe fn game_specialairlw1hit(agent: &mut L2CAgentBase) {
//     let lua_state = agent.lua_state_agent;
//     let boma = agent.boma();
//     if is_excute(agent) {
//         let current_level = VarModule::get_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL);
//         // Attack transition
//         if VarModule::is_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER) {
//             VarModule::off_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER);
//             if current_level > 0{
//                 if current_level == 1 {
//                     // println!("Changing to level 1");
//                     MotionModule::change_motion(boma, Hash40::new("special_air_lw1_hit_lv1"), 0.0, 1.0, false, 0.0, false, false);
//                 }
//                 else{
//                     // println!("Changing to level 2");
//                     MotionModule::change_motion(boma, Hash40::new("special_air_lw1_hit_lv2"), 0.0, 1.0, false, 0.0, false, false);
//                 }
//             }
//         }
//         else{
//             // Increment the level
//             if current_level < 2 {
//                 VarModule::set_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, current_level + 1); // Add a charge level
//                 // println!("Kinesis increment");
//             }
//             // println!("Kinesis Level: {}", VarModule::get_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL));
//         }
//     }
//     frame(lua_state, 23.0);
//     if is_excute(agent) {
//         WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
//         //FT_MOTION_RATE(agent, 0.65);
//     }
// }

// Kinesis Blade - 1 Charge

unsafe extern "C" fn game_speciallw1hitlv1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, 0); // Reset charge level
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 50, 6.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(-30.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        //WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
}

unsafe extern "C" fn game_specialairlw1hitlv1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, 0); // Reset charge level
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 11.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 50, 6.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(-30.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        //WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
}

// Kinesis Blade - 2 Charges

unsafe extern "C" fn game_speciallw1hitlv2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.1, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 63.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 73.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairlw1hitlv2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.1, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 63.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 73.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

// ================================================================================================================
// ================================================ SHOCK SPELL ===================================================
// ================================================================================================================

unsafe extern "C" fn game_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 12.0, 16.0);
    if is_excute(agent) {
        if boma.is_button_on(Buttons::Special) {
            VarModule::on_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD);
        }
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
        let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD) { 15.0 } else { 0.0 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 55, 0, 40, 5.0, 0.0, 9.0, 15.0 + hold, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 60, 55, 0, 40, 2.0, 0.0, 2.0, 15.0 + hold, Some(0.0), Some(25.0), Some(15.0 + hold), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(agent, 16.0, 23.0, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
    }
}

unsafe extern "C" fn effect_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -1.5, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_reflect_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0.0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.65);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let mut offset = 0;
        EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("arml"), 4, 0, 0, 0, 0, 0, 0.4, true);
        if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD) {
            offset = 13;
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15.0, 8.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        }
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_smokescreen"), Hash40::new("top"), 0, 25, 13 + offset, 0, 0.0, 0, 0.4, true);
            LAST_EFFECT_SET_SCALE_W(agent, 0.4, 0.25, 0.4);
            LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.1);
            LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD) { 13 } else { 0 };
            EFFECT_FOLLOW(agent, Hash40::new("sys_smokescreen"), Hash40::new("top"), 0, 25, 13 + hold, 0, 0.0, 0, 0.4, true);
            LAST_EFFECT_SET_SCALE_W(agent, 0.4, 0.25, 0.4);
            LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.1);
            LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    frame(lua_state, 10.5);
    if is_excute(agent) {
        let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD) { 13 } else { 0 };
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0.5, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        }
        EFFECT_FOLLOW(agent, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 15, 13 + hold, 0, 0, 180, 0.15, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SHOCK_SPELL_HOLD) { 13 } else { 0 };
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_paralysis"), Hash40::new("arml"), 4, 0, 0, 0, 0, 0, 0.35, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_smokescreen"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 8, 13 + hold, 0, 0, 0, 0.4, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.84, 0.17);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_paralysis"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder_flash"), true, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_hit_elec"), -1);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_reflect_sword"), false, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("miiswordsman_reflect1"), -1);
    }
}

unsafe extern "C" fn sound_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miiswordsman_special_c2_l01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_special_c2_l01"));
    }
    frame(lua_state, 10.5);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_m"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_s"));
    }
}

// ================================================================================================================
// =========================================== POWER THRUST =======================================================
// ================================================================================================================

unsafe extern "C" fn game_speciallw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 12.0, /*Angle*/ 35, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 77, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.8, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 35, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 77, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(19.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_WALL_CHECK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 42, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 42, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(19.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 50, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 50, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(19.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_speciallw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {

    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //ATTACK(agent, 0, 1, Hash40::new("haver"), 1.0, 100, 10, 0, 10, 4.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 4.0, y: 5.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 0.0, y: 17.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 4, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        //REVERSE_LR(agent);
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 17.0, 0.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -6.0, y: 15.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 3, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        //REVERSE_LR(agent);
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 15.0, -6.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -12.0, y: 0.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 2, false);
    }
    /*
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 0.1, 365, 30, 0, 0, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        //let hit1 = Vector2f {x: 12.0, y: 0.0};
        //AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    */
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 2, Hash40::new("haver"), 5.0, 80, 100, 0, 60, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(11.5), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairlw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    sv_kinetic_energy!(set_speed_mul_2nd, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.4, 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_WALL_CHECK);
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 275, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 275, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0);
    sv_kinetic_energy!(set_speed_mul_2nd, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.0, 1.0);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(agent, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairlw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.790);
        //WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_SP_BRAKE);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        //WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_SP_BRAKE);
    }
}

unsafe extern "C" fn game_specialairlw3endair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //ATTACK(agent, 0, 1, Hash40::new("haver"), 1.0, 100, 10, 0, 10, 4.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 4.0, y: 5.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 0.0, y: 17.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 4, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        //REVERSE_LR(agent);
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 17.0, 0.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -6.0, y: 15.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 3, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        //REVERSE_LR(agent);
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 15.0, -6.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -12.0, y: 0.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 2, false);
    }
    /*
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 0.1, 365, 30, 0, 0, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        //let hit1 = Vector2f {x: 12.0, y: 0.0};
        //AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    */
    frame(lua_state, 21.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        ATTACK(agent, 0, 2, Hash40::new("haver"), 5.0, 80, 100, 0, 60, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(11.5), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw1hit", game_speciallw1hit);
    agent.acmd("game_specialairlw1hit", game_specialairlw1hit);
    //agent.acmd("game_speciallw1hitlv1", game_speciallw1hitlv1);
    //agent.acmd("game_specialairlw1hitlv1", game_specialairlw1hitlv1);
    //agent.acmd("game_speciallw1hitlv2", game_speciallw1hitlv2);
    //agent.acmd("game_specialairlw1hitlv2", game_specialairlw1hitlv2);

    agent.acmd("game_speciallw2", game_speciallw2);
    agent.acmd("game_specialairlw2", game_speciallw2);
    agent.acmd("effect_speciallw2", effect_speciallw2);
    agent.acmd("effect_specialairlw2", effect_speciallw2);
    agent.acmd("sound_speciallw2", sound_speciallw2);
    agent.acmd("sound_specialairlw2", sound_speciallw2);
    
    agent.acmd("game_speciallw3", game_speciallw3);
    //agent.acmd("game_speciallw3end", game_speciallw3end);
    agent.acmd("game_specialairlw3", game_specialairlw3);
    agent.acmd("game_specialairlw3end", game_specialairlw3end);
    //agent.acmd("game_specialairlw3endair", game_specialairlw3endair);
}