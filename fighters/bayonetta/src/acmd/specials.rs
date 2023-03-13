
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
unsafe fn bayonetta_special_n_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.333);
}

#[acmd_script( agent = "bayonetta", script = "game_specialsholdend", category = ACMD_GAME, low_priority )]
unsafe fn bayonetta_special_s_hold_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 2, 0, 0, 0, false);
        ATTACK(fighter, 0, 0, Hash40::new("footr"), 3.5, 80, 8, 0, 52, 5.0, 0.0, 0.0, 0.0, Some(-9.5), Some(0.0), Some(0.0), 1.0, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_END_SPECIAL_S);
    }
    frame(lua_state, 34.0);
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 5.0, 98, 65, 0, 64, 5.3, 7.5, 0.0, 0.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 5.0, 98, 65, 0, 64, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("waist"), 5.0, 90, 65, 0, 64, 4.5, 0.0, -0.8, -1.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 44.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_shootinglegl_atkon_specialairsu", category = ACMD_GAME, low_priority )]
unsafe fn bayonetta_shootinglegl_atk_on_special_air_s_u_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 340, 100, 0, 10, 3.5, 0.0, 4.5, -0.5, Some(0.0), Some(4.5), Some(3.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 340, 0, 0, 0, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(-18.0), Some(59.4), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 6, 4.05);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_specialairsu" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_air_s_u_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP) {
            let pos_x = PostureModule::pos_x(boma);
            let pos_y = PostureModule::pos_y(boma);
            let pos = smash::phx::Vector3f { x: pos_x, y: pos_y - 6.7, z: 0.0 };
            PostureModule::set_pos(boma, &pos);
            WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
            PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
            EFFECT(fighter, Hash40::new("bayonetta_change_circle"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
        JostleModule::set_status(boma, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 5, 0, 20, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 5);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) > 2 {
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.000);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 5.0, 65, 30, 0, 60, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);   
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 75, 30, 0, 55, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 35, 30, 0, 60, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.7, 35, 100, 70, 0, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.7, 45, 100, 50, 0, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS);
        }
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            if VarModule::get_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) < 2{
                VarModule::inc_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
            }
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        JostleModule::set_status(boma, true);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            FT_MOTION_RATE(fighter, 18/(43-30));
        }
    }
}

#[acmd_script( agent = "bayonetta", script = "game_specialairsd" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_air_s_d_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP) {
            let pos_x = PostureModule::pos_x(boma);
            let pos_y = PostureModule::pos_y(boma);
            let pos = smash::phx::Vector3f { x: pos_x, y: pos_y - 6.0, z: 0.0 };
            PostureModule::set_pos(boma, &pos);
            WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
            PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
            EFFECT(fighter, Hash40::new("bayonetta_change_circle"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE(fighter, 6.0/(8.0-2.0));
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 7.5, 86, 35, 0, 95, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.1, 0.9, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.5, 86, 35, 0, 95, 5.0, 0.5, 0.0, 0.0, None, None, None, 1.1, 0.9, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 4.5, 86, 45, 0, 105, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.1, 0.9, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.5, 86, 45, 0, 105, 5.0, 0.5, 0.0, 0.0, None, None, None, 1.1, 0.9, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
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
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.540);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 20, 0, 15, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
    }
frame(lua_state, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.700);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 368, 100, 120, 0, 2.5, 0.0, 4.0, 0.0, Some(0.0), Some(12.5), Some(0.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 368, 100, 130, 0, 3.0, 0.0, 5.5, 5.0, Some(0.0), Some(10.0), Some(5.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: -1.0, y: 43.0}, 13, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: -1.0, y: 43.0}, 13, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) { 
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 368, 100, 120, 0, 2.4, 0.0, 8.5, 0.5, Some(0.0), Some(17.5), Some(0.5), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: -1.0, y: 40.0}, 12, false);
        AttackModule::clear(boma, 1, false);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 367, 100, 38, 0, 3.8, 0.0, 22.5, 0.0, Some(0.0), Some(18.0), Some(0.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 367, 100, 68, 0, 3.0, 0.0, 24.5, 0.0, Some(0.0), Some(22.0), Some(0.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 50, 60, 0, 65, 3.8, 0.0, 24.5, 0.0, Some(0.0), Some(19.0), Some(0.0), 1.1, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            let motion_vec = Vector3f {x: 0.0, y: 7.0, z: 0.0};
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2bfb02b69a), true);
    }
    frame(lua_state, 39.0);
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
    let mut strike = false;
    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP) {
            let pos_x = PostureModule::pos_x(boma);
            let pos_y = PostureModule::pos_y(boma);
            let pos = smash::phx::Vector3f { x: pos_x, y: pos_y - 7.0, z: 0.0 };
            PostureModule::set_pos(boma, &pos);
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
            PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
            EFFECT(fighter, Hash40::new("bayonetta_change_circle"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
        FT_MOTION_RATE(fighter, 0.450);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 20, 0, 15, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) { 
        FT_MOTION_RATE(fighter, 0.700);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 90, 62, 0, 78, 2.5, 0.0, 4.5, 0.0, Some(0.0), Some(10.0), Some(0.0), 1.1, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 94, 62, 0, 78, 3.0, 0.0, 6.0, 5.0, Some(0.0), Some(8.5), Some(5.0), 1.1, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);    
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) { 
    AttackModule::clear(boma, 1, false);
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        let strike = true;
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 368, 100, 120, 0, 2.4, 0.0, 8.5, 0.5, Some(0.0), Some(17.5), Some(0.5), 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: -1.0, y: 40.0}, 12, false);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 367, 100, 38, 0, 3.8, 0.0, 22.5, 0.0, Some(0.0), Some(18.0), Some(0.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 367, 100, 68, 0, 3.0, 0.0, 24.5, 0.0, Some(0.0), Some(22.0), Some(0.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 367, 100, 38, 0, 3.8, 0.0, 22.5, 0.0, Some(0.0), Some(18.0), Some(0.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 367, 100, 68, 0, 3.0, 0.0, 24.5, 0.0, Some(0.0), Some(22.0), Some(0.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 20, 40, 0, 40, 3.8, 0.0, 24.5, 0.0, Some(0.0), Some(18.8), Some(0.0), 1.1, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma,  0,  6.0,  false);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 40, 60, 0, 45, 3.8, 0.0, 24.5, 0.0, Some(0.0), Some(18.5), Some(0.0), 1.1, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma,  0,  6.0,  false);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && strike == false {
            if VarModule::get_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) < 2 {
                VarModule::inc_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
            }
            FT_MOTION_RATE(fighter, 12/(42-35));
        }
        else {
            let motion_vec = Vector3f {x: 0.0, y: 7.0, z: 0.0};
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2bfb02b69a), true);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_NO_SHOOTING_ENABLE_CANCEL);
    }
    frame(lua_state, 42.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_lw (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
	}
    frame(lua_state, 5.0);
	if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
	frame(lua_state, 9.0);
	if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), true);
    }
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 21.0);
	if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), false);
    }
	frame(lua_state, 33.0);
	if is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
	}
}

#[acmd_script( agent = "bayonetta", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_air_lw (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
	}
    frame(lua_state, 5.0);
	if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), true);
    }
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 21.0);
	if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), false);
    }
	frame(lua_state, 33.0);
	if is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
	}
}

pub fn install() {
    install_acmd_scripts!(
        bayonetta_special_n_charge_game,
        bayonetta_special_n_end_game,
        bayonetta_special_n_start_game,
        bayonetta_special_s_hold_end_game,
        bayonetta_shootinglegl_atk_on_special_air_s_u_game,
        bayonetta_special_air_s_u_game,
        bayonetta_special_air_s_d_game,
        bayonetta_special_air_s_d_landing_game,
        bayonetta_special_hi_game,
        bayonetta_special_air_hi_game,
        bayonetta_special_lw,
        bayonetta_special_air_lw,
    );
}
