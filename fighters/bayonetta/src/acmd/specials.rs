
use super::*;


#[acmd_script( agent = "bayonetta", scripts = ["game_specialnstarth", "game_specialnstartf", "game_specialairnstarth", "game_specialairnstartf"] , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
}

#[acmd_script( agent = "bayonetta", scripts = ["game_specialnchargef", "game_specialnchargeh", "game_specialairnchargef", "game_specialairnchargeh"] , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_n_charge_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    VarModule::off_flag(boma.object(), vars::bayonetta::instance::SHOULD_PRORATE_DAMAGE);
    if VarModule::is_flag(boma.object(), vars::bayonetta::instance::IS_NONSPECIAL_CANCEL) {
        VarModule::on_flag(boma.object(), vars::bayonetta::instance::SHOULD_PRORATE_DAMAGE);
        VarModule::off_flag(boma.object(), vars::bayonetta::instance::IS_NONSPECIAL_CANCEL);
    }
    frame(lua_state, 1.0);
    if VarModule::is_flag(boma.object(), vars::bayonetta::instance::SHOULD_PRORATE_DAMAGE) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    else {
        FT_MOTION_RATE(fighter, 1.65);
    }
}

#[acmd_script( agent = "bayonetta", scripts = ["game_specialnendh", "game_specialnendf", "game_specialairnendh", "game_specialairnendf"] , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_n_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
}

#[acmd_script( agent = "bayonetta", script = "game_specialairsu" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_air_s_u_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.500);
        JostleModule::set_status(boma, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 5, 0, 20, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 5);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 50, 30, 0, 85, 4.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, -7.0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 69, 30, 0, 70, 4.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, -5.0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 85, 30, 0, 60, 4.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, -3.0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 90, 112, 58, 0, 4.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, -1.0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS);
        }
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            if VarModule::get_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) < 2{
                VarModule::inc_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
            }
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
    }

}

#[acmd_script( agent = "bayonetta", script = "game_specialairsd" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_air_s_d_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 5.0/(6.0-2.0));
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        // Air-only
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 7.5, 86, 35, 0, 95, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.45, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.5, 86, 35, 0, 95, 5.0, 0.5, 0.0, 0.0, None, None, None, 1.45, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // Ground-only
        ATTACK(fighter, 2, 0, Hash40::new("legr"), 7.5, 86, 35, 0, 90, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.45, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneer"), 7.5, 86, 35, 0, 90, 5.0, 0.5, 0.0, 0.0, None, None, None, 1.45, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "bayonetta", script = "game_specialairsdlanding" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_air_s_d_landing_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 20, 0, 15, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma,  *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_GROUND_START) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 96, 100, 120, 0, 3.5, 0.0, 4.5, 0.5, Some(0.0), Some(9.5), Some(0.5), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 113, 100, 130, 0, 5.0, 0.0, 6.0, 5.0, Some(0.0), Some(8.0), Some(5.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 90, 100, 140, 0, 4.0, 0.0, 4.0, 1.0, Some(0.0), Some(11.0), Some(1.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 110, 100, 135, 0, 6.3, 0.0, 6.0, 7.5, Some(0.0), Some(9.5), Some(7.5), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS);
        }
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.2, 93, 100, 80, 0, 6.0, 0.0, 21.0, 0.0, Some(0.0), Some(19.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.2, 367, 100, 40, 0, 5.5, 0.0, 26.0, 0.0, Some(0.0), Some(19.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 0.2, 93, 100, 100, 0, 4.0, 0.0, 18.0, 0.0, Some(0.0), Some(8.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 0.2, 93, 100, 100, 0, 4.0, 0.0, 18.0, 0.0, Some(0.0), Some(9.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.2, 105, 100, 50, 0, 6.0, 0.0, 23.0, 0.0, Some(0.0), Some(20.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.2, 105, 100, 30, 0, 5.0, 0.0, 25.0, 0.0, Some(0.0), Some(23.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.2, 367, 100, 38, 0, 5.0, 0.0, 23.0, 0.0, Some(0.0), Some(20.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.2, 367, 100, 68, 0, 4.0, 0.0, 25.0, 0.0, Some(0.0), Some(23.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 0.2, 93, 100, 100, 0, 4.0, 0.0, 18.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 20, 40, 0, 40, 7.5, 0.0, 26.5, 0.0, Some(0.0), Some(19.0), Some(0.0), 1.05, 1.05, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 6.0, false);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 40, 60, 0, 45, 7.5, 0.0, 26.5, 0.0, Some(0.0), Some(19.0), Some(0.0), 1.05, 1.05, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 6.0, false);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            let motion_vec = Vector3f {x: 0.0, y: 7.0, z: 0.0};
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
        else {
            FT_MOTION_RATE(fighter, 2.0);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2bfb02b69a), true);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_NO_SHOOTING_ENABLE_CANCEL);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }

}

#[acmd_script( agent = "bayonetta", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.700);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 20, 0, 15, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma,  *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_GROUND_START) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 96, 100, 120, 0, 3.5, 0.0, 4.5, 0.5, Some(0.0), Some(9.5), Some(0.5), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 113, 100, 130, 0, 5.0, 0.0, 6.0, 5.0, Some(0.0), Some(8.0), Some(5.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 90, 100, 140, 0, 4.0, 0.0, 4.0, 1.0, Some(0.0), Some(11.0), Some(1.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 110, 100, 135, 0, 6.3, 0.0, 6.0, 7.5, Some(0.0), Some(9.5), Some(7.5), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS);
        }
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.2, 93, 100, 80, 0, 6.0, 0.0, 21.0, 0.0, Some(0.0), Some(19.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.2, 367, 100, 40, 0, 5.5, 0.0, 26.0, 0.0, Some(0.0), Some(19.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 0.2, 93, 100, 100, 0, 4.0, 0.0, 18.0, 0.0, Some(0.0), Some(8.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 0.2, 93, 100, 100, 0, 4.0, 0.0, 18.0, 0.0, Some(0.0), Some(9.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.2, 105, 100, 50, 0, 6.0, 0.0, 23.0, 0.0, Some(0.0), Some(20.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.2, 105, 100, 30, 0, 5.0, 0.0, 25.0, 0.0, Some(0.0), Some(23.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.2, 367, 100, 38, 0, 5.0, 0.0, 23.0, 0.0, Some(0.0), Some(20.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.2, 367, 100, 68, 0, 4.0, 0.0, 25.0, 0.0, Some(0.0), Some(23.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 0.2, 93, 100, 100, 0, 4.0, 0.0, 18.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 20, 40, 0, 40, 7.5, 0.0, 26.5, 0.0, Some(0.0), Some(19.0), Some(0.0), 1.05, 1.05, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma,  0,  6.0,  false);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 40, 60, 0, 45, 7.5, 0.0, 26.5, 0.0, Some(0.0), Some(19.0), Some(0.0), 1.05, 1.05, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma,  0,  6.0,  false);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            let motion_vec = Vector3f {x: 0.0, y: 7.0, z: 0.0};
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
        else {
            FT_MOTION_RATE(fighter, 2.0);
        }
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && !WorkModule::is_flag(boma,  *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_GROUND_START){
            if VarModule::get_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) < 2 {
                VarModule::inc_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
            }
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2bfb02b69a), true);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_NO_SHOOTING_ENABLE_CANCEL);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }

}

pub fn install() {
    install_acmd_scripts!(
        bayonetta_special_n_charge_game,
        bayonetta_special_n_loop_game,
        bayonetta_special_n_start_game,
        bayonetta_special_air_s_u_game,
        bayonetta_special_air_s_d_game,
        bayonetta_special_air_s_d_landing_game,
        bayonetta_special_hi_game,
        bayonetta_special_air_hi_game,
    );
}
