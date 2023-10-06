use super::*;

#[acmd_script( agent = "robot", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn robot_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.8);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 75, 100, 0, 50, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 10.0, 75, 100, 0, 50, 5.0, -1.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 10.0, 75, 100, 0, 50, 5.0, -1.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 3, 0, Hash40::new("hip"), 11.0, 75, 100, 0, 50, 6.0, -6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.5, 55, 100, 0, 40, 5.0, 2.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 8.5, 55, 100, 0, 40, 5.0, -1.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 8.5, 55, 100, 0, 40, 5.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 3, 0, Hash40::new("hip"), 8.5, 55, 100, 0, 40, 5.0, -5.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "robot", script = "effect_attackairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn robot_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_nozzle_flare"), Hash40::new("knee1"), 1.5, 0, 0, 90, -90, 0, 1, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_atk_n_jet"), Hash40::new("knee1"), 1.5, 0, 0, 0, 0, -90, 0.8, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_atk_n_jet"), Hash40::new("knee1"), 1.5, 0, 0, 0, 0, -90, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.75);
        LAST_EFFECT_SET_ALPHA(fighter, 0.45);
        LAST_EFFECT_SET_COLOR(fighter, 0.55, 0.1, 0.1);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_nozzle_flare"), Hash40::new("knee"), 0, 0, 0, 90, -90, 0, 1, true);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("robot_nozzle_flare"), false, false);
    }

}

#[acmd_script( agent = "robot", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn robot_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.0/(6.1-6.0));
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 7.0, 50, 90, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 7.0, 50, 90, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr1"), 9.0, 50, 90, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("arml1"), 9.0, 50, 90, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("handr2"), 9.0, 50, 90, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("handl2"), 9.0, 50, 90, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 6.1);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.0/(7.2-6.1));
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 6.0, 361, 60, 0, 35, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 6.0, 361, 60, 0, 35, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr1"), 6.0, 361, 60, 0, 35, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("arml1"), 6.0, 361, 60, 0, 35, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("handr2"), 6.0, 361, 60, 0, 35, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("handl2"), 6.0, 361, 60, 0, 35, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 7.2);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0/(9.0-7.2));
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 6.0/(13.0-9.0));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 15.0/(35.0-13.0));
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "robot", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn robot_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 3, 9, 8, 180, -160, 75, 1.4, true, *EF_FLIP_YZ);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP){
                LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.01, 0);
            }
            else{
                LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
            }
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
            if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP){
                LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
            }
            else{
                LAST_EFFECT_SET_COLOR(fighter, 0.22, 0.059, 0.039);
            }
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
            LAST_EFFECT_SET_COLOR(fighter, 0.176, 0.137, 0.059);
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
            LAST_EFFECT_SET_COLOR(fighter, 0.235, 0.196, 0.255);
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
            LAST_EFFECT_SET_COLOR(fighter, 0.098, 0.157, 0.196);
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
            LAST_EFFECT_SET_COLOR(fighter, 0.098, 0.059, 0);
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
            LAST_EFFECT_SET_COLOR(fighter, 0.098, 0.098, 0.157);
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
            LAST_EFFECT_SET_COLOR(fighter, 0.118, 0.039, 0.051);
        }
        LAST_EFFECT_SET_RATE(fighter, 2.0);


        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -6, 9, 8, 180, -180, 120, 1.4, true, *EF_FLIP_YZ);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP){
                LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.01, 0);
            }
            else{
                LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
            }
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
            if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP){
                LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
            }
            else{
                LAST_EFFECT_SET_COLOR(fighter, 0.22, 0.059, 0.039);
            }
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
            LAST_EFFECT_SET_COLOR(fighter, 0.176, 0.137, 0.059);
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
            LAST_EFFECT_SET_COLOR(fighter, 0.235, 0.196, 0.255);
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
            LAST_EFFECT_SET_COLOR(fighter, 0.098, 0.157, 0.196);
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
            LAST_EFFECT_SET_COLOR(fighter, 0.098, 0.059, 0);
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
            LAST_EFFECT_SET_COLOR(fighter, 0.098, 0.098, 0.157);
        }
        else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
            LAST_EFFECT_SET_COLOR(fighter, 0.118, 0.039, 0.051);
        }
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("handr2"), 1.0, 0, 2.0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(lua_state, 6.4);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, false);
    }
}

#[acmd_script( agent = "robot", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn robot_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        VarModule::off_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_STARTED);
		VarModule::off_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_FINISHED);
        VarModule::off_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_MAX);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
		VarModule::set_float(fighter.battle_object, vars::robot::status::CHARGE_ATTACK_LEVEL, 0.0);
        FT_MOTION_RATE(fighter, 2.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
		else {
			FT_MOTION_RATE(fighter, 0.5);
		}

        for _ in 0..5 {
            wait(lua_state, 1.0);
            if is_excute(fighter) {
                if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) && !VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_FINISHED){
                    // If holding down the button, increment the charge and continue the slowed animation
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                        VarModule::on_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_STARTED);
                        VarModule::add_float(fighter.battle_object, vars::robot::status::CHARGE_ATTACK_LEVEL, 1.0);
                        let current_fuel = WorkModule::get_float(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
                        let current_fuel_depletion = (VarModule::get_float(fighter.battle_object, vars::robot::status::CHARGE_ATTACK_LEVEL) * 15.0);
                        if (current_fuel_depletion > current_fuel) {
                            VarModule::on_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_FINISHED);
                            WorkModule::set_float(boma, 0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
                            FT_MOTION_RATE(fighter, 1.0);
                        } else {
                            FT_MOTION_RATE(fighter, 2.0);
                        }
                    }
                    // If no longer holding the button, play out the rest of the animation as normal
                    else {
                        VarModule::on_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_FINISHED);
                        FT_MOTION_RATE(fighter, 1.0);
                    }
                } 
            } 
        }
    }

    if !VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_FINISHED) {
        WorkModule::set_float(boma, WorkModule::get_float(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE) - (VarModule::get_float(fighter.battle_object, vars::robot::status::CHARGE_ATTACK_LEVEL) * 6.0), *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
		FT_MOTION_RATE(fighter, 1.0);

        if VarModule::get_float(fighter.battle_object, vars::robot::status::CHARGE_ATTACK_LEVEL) >= 5.0 {
            VarModule::on_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_MAX);
        }
    }

    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        if WorkModule::is_flag(boma, vars::robot::instance::AIRTIME_BAIR) {
            if VarModule::get_float(fighter.battle_object, vars::robot::status::CHARGE_ATTACK_LEVEL) >= 5.0 {
                SET_SPEED_EX(fighter, 1.75, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            } else {
                let boost_speed_x = 1.1 + (0.13 * VarModule::get_float(fighter.battle_object, vars::robot::status::CHARGE_ATTACK_LEVEL));
                let boost_speed_y = 0.0 + (0.04 * VarModule::get_float(fighter.battle_object, vars::robot::status::CHARGE_ATTACK_LEVEL));
                SET_SPEED_EX(fighter, boost_speed_x, boost_speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }
	}

    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            let charge_attack_damage_mul = 1.0 + (VarModule::get_float(fighter.battle_object, vars::robot::status::CHARGE_ATTACK_LEVEL) * 0.07);
            ATTACK(fighter, 0, 0, Hash40::new("knee"), 12.0 * charge_attack_damage_mul, 35, 85, 0, 30, 7.5, 3.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 12.0 * charge_attack_damage_mul, 35, 85, 0, 30, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 2, 0, Hash40::new("knee"), 12.0 * charge_attack_damage_mul, 35, 85, 0, 30, 4.0, 0.0, 0.0, 0.0, Some(16.0), Some(0.0), Some(0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("knee"), 12.0, 35, 70, 0, 35, 6.0, 0.0, 0.0, 0.0, Some(4.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 12.0, 35, 70, 0, 35, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 2, 0, Hash40::new("knee"), 12.0, 35, 70, 0, 35, 4.0, 0.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
        FT_MOTION_RATE(fighter, 0.7);
        WorkModule::off_flag(boma, vars::robot::instance::AIRTIME_BAIR);
    }

    frame(lua_state, 24.0);
    if is_excute(fighter) {
        let charge_attack_damage_mul = 1.0 + (VarModule::get_float(fighter.battle_object, vars::robot::status::CHARGE_ATTACK_LEVEL) * 0.07);
        ATTACK(fighter, 0, 0, Hash40::new("knee"), 8.0 * charge_attack_damage_mul, 361, 80, 0, 35, 6.0, 0.0, 0.0, 0.0, Some(4.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 8.0 * charge_attack_damage_mul, 361, 80, 0, 35, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 2, 0, Hash40::new("knee"), 8.0 * charge_attack_damage_mul, 361, 80, 0, 35, 4.0, 0.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "robot", script = "effect_attackairb" , category = ACMD_EFFECT , low_priority)]
unsafe fn robot_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_nozzle_flare"), Hash40::new("knee"), 0, 0, 0, 90, -90, 0, 1, true);
        if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_MAX) {
            LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 3.0);
        } else if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_STARTED) {
            LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 1.0);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_MAX) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 2.0, 0, 0, 0, 0, 0, 1.1, true);
            LAST_EFFECT_SET_RATE(fighter, 0.75);
            LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 1.0);

            EFFECT_FOLLOW(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("hip"), -5.0, 0, 0, 0, 0, 0, 4.0, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
        } else if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_STARTED) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 2.0, 0, 0, 0, 0, 0, 1.1, true);
            LAST_EFFECT_SET_RATE(fighter, 0.9);
            LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 1.0);

            EFFECT_FOLLOW(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("hip"), -5.0, 0, 0, 0, 0, 0, 4.0, true);
            LAST_EFFECT_SET_RATE(fighter, 0.75);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_MAX) {
            EFFECT_FOLLOW(fighter, Hash40::new("robot_atk_ab_jet"), Hash40::new("knee1"), 1.5, 0, 0, 0, 0, -90, 0.9, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.8, 1.0));
        } else if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_STARTED) {
            EFFECT_FOLLOW(fighter, Hash40::new("robot_atk_ab_jet"), Hash40::new("knee1"), 1.5, 0, 0, 0, 0, -90, 0.9, true);
            LAST_EFFECT_SET_RATE(fighter, 0.75);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 2.5);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.8, 1.0));
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("robot_atk_ab_jet"), Hash40::new("knee1"), 1.5, 0, 0, 0, 0, -90, 0.9, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EffectModule::set_scale_last(boma, &Vector3f::new(0.9, 0.55, 1.0));
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("robot_nozzle_flare"), Hash40::new("knee"), 0, 0, 0, 90, -90, 0, 1, true);
        if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_MAX) {
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
        } else if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_STARTED) {
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 2.5);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("robot_nozzle_flare"), false, false);
    
        if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_MAX) {   
            EFFECT_FOLLOW(fighter, Hash40::new("robot_atk_ab_jet"), Hash40::new("knee1"), 1.5, 0, 0, 0, 0, -90, 0.9, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.8, 1.0));
        } else if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_STARTED) {
            EFFECT_FOLLOW(fighter, Hash40::new("robot_atk_ab_jet"), Hash40::new("knee1"), 1.5, 0, 0, 0, 0, -90, 0.9, true);
            LAST_EFFECT_SET_RATE(fighter, 0.75);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 2.5);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.8, 1.0));
        } else {
            EFFECT_FOLLOW(fighter, Hash40::new("robot_atk_ab_jet"), Hash40::new("knee1"), 1.5, 0, 0, 0, 0, -90, 0.9, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            LAST_EFFECT_SET_ALPHA(fighter, 0.35);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 1.0);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.8, 1.0));
        }
    }
    
}

#[acmd_script( agent = "robot", script = "sound_attackairb" , category = ACMD_SOUND , low_priority)]
unsafe fn robot_attack_air_b_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_STARTED) {
            PLAY_SE(fighter, Hash40::new("vc_robot_attack05"));
		} 
	}

    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_MAX) {
            PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
            wait(lua_state, 3.0);
            PLAY_SEQUENCE(fighter, Hash40::new("seq_robot_rnd_attack"));
        } else if VarModule::is_flag(fighter.battle_object, vars::robot::status::IS_CHARGE_STARTED) {
            wait(lua_state, 3.0);
            PLAY_SEQUENCE(fighter, Hash40::new("seq_robot_rnd_attack"));
            PLAY_SE(fighter, Hash40::new("se_robot_attackair_b01"));
        } else {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_robot_rnd_attack"));
            PLAY_SE(fighter, Hash40::new("se_robot_attackair_b01"));
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_robot_jet"));
    }
}

#[acmd_script( agent = "robot", script = "game_landingairb" , category = ACMD_GAME , low_priority)]
unsafe fn robot_landing_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let landing_frame_normal = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_lw"), 0);
        let landing_frame_boost = landing_frame_normal + 3.0;
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, landing_frame_boost/landing_frame_normal);
        } 
    }
}

#[acmd_script( agent = "robot", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn robot_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 85, 80, 0, 35, 7.5, 0.0, 15.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 85, 80, 0, 35, 9.5, 0.0, 24.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 367, 80, 0, 45, 7.5, 0.0, 15.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 367, 80, 0, 45, 9.5, 0.0, 24.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 85, 80, 0, 35, 7.5, 0.0, 15.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 85, 80, 0, 35, 9.5, 0.0, 24.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 367, 80, 0, 45, 7.5, 0.0, 15.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 367, 80, 0, 45, 9.5, 0.0, 24.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 85, 80, 0, 35, 7.5, 0.0, 15.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 85, 80, 0, 35, 9.5, 0.0, 24.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 367, 80, 0, 45, 7.5, 0.0, 15.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 367, 80, 0, 45, 9.5, 0.0, 24.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 85, 80, 0, 35, 7.5, 0.0, 15.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 85, 80, 0, 35, 9.5, 0.0, 24.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 367, 80, 0, 45, 7.5, 0.0, 15.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 367, 80, 0, 45, 9.5, 0.0, 24.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 70, 105, 0, 65, 8.0, 0.0, 16.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 70, 105, 0, 65, 10.0, 0.0, 24.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "robot", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn robot_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(7.0-1.0));

    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 1.0);
        for _ in 0..6 {
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 1.2, 365, 100, 40, 0, 3.0, 0.0, 6.0, -3.0, Some(0.0), Some(6.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 365, 100, 40, 0, 3.0, 0.0, 2.0, -3.0, Some(0.0), Some(2.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 365, 100, 40, 0, 2.0, 0.0, -2.0, -3.0, Some(0.0), Some(-2.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
            wait(lua_state, 1.0);
            if is_excute(fighter) {
                AttackModule::clear_all(boma);
            }
            wait(lua_state, 2.0);
        }
        
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 85, 85, 0, 45, 8.0, 0.0, 1.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }

        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }

        frame(lua_state, 28.0);
        if is_excute(fighter) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "robot", script = "effect_attackairlw" , category = ACMD_EFFECT , low_priority)]
unsafe fn robot_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 6.0);
        for _ in 0..5 {
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 7, 0, 80, 0, 0, 1.0, true, *EF_FLIP_NONE);
                LAST_EFFECT_SET_RATE(fighter, 3.0);
            }

            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
                if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP){
                    LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.01, 0);
                }
                else{
                    LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
                }
            }
            else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
                if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP){
                    LAST_EFFECT_SET_COLOR(fighter, 0.196, 0.196, 0.216);
                }
                else{
                    LAST_EFFECT_SET_COLOR(fighter, 0.22, 0.059, 0.039);
                }
            }
            else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
                LAST_EFFECT_SET_COLOR(fighter, 0.176, 0.137, 0.059);
            }
            else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
                LAST_EFFECT_SET_COLOR(fighter, 0.235, 0.196, 0.255);
            }
            else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
                LAST_EFFECT_SET_COLOR(fighter, 0.098, 0.157, 0.196);
            }
            else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                LAST_EFFECT_SET_COLOR(fighter, 0.098, 0.059, 0);
            }
            else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
                LAST_EFFECT_SET_COLOR(fighter, 0.098, 0.098, 0.157);
            }
            else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                LAST_EFFECT_SET_COLOR(fighter, 0.118, 0.039, 0.051);
            }

            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_RATE(fighter, 3.0);
            wait(lua_state, 3.0);
        }
}

#[acmd_script( agent = "robot", script = "sound_attackairlw", category = ACMD_SOUND, low_priority )]
unsafe fn robot_attack_air_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_robot_attack07"));
        PLAY_SE(fighter, Hash40::new("se_common_swing_07"));
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_robot_swing_m"));
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_robot_swing_m"));
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_robot_swing_m"));
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_robot_swing_m"));
    }
}

#[acmd_script( agent = "robot", script = "expression_attackairlw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn robot_attack_air_lw_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 20.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        robot_attack_air_n_game,
        robot_attack_air_n_effect,
        robot_attack_air_f_game,
        robot_attack_air_f_effect,
        robot_attack_air_b_game,
        robot_attack_air_b_effect,
        robot_attack_air_b_sound,
        robot_landing_air_b_game,
        robot_attack_air_hi_game,
        robot_attack_air_lw_game,
        robot_attack_air_lw_effect,
        robot_attack_air_lw_sound,
        robot_attack_air_lw_expression
    );
}