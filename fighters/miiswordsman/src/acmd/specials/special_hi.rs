use super::*;

// =============================================================================================
// ======================================== STONE SCABBARD =====================================
// =============================================================================================

unsafe extern "C" fn game_specialhi1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.7);
    }
}

unsafe extern "C" fn game_specialairhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let addSpeed = Vector3f { x: 0.0, y: 0.5, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 46, 95, 0, 60, 4.0, 0.0, 7.0, 6.0, Some(0.0), Some(4.0), Some(6.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 46, 95, 0, 60, 3.5, 0.0, 7.0, 10.0, Some(0.0), Some(4.0), Some(10.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 46, 95, 0, 60, 3.5, 0.0, 7.0, 14.0, Some(0.0), Some(4.0), Some(14.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 92, 100, 60, 0, 4.0, 0.0, 7.0, 6.0, Some(0.0), Some(4.0), Some(6.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 92, 100, 60, 0, 3.5, 0.0, 7.0, 10.0, Some(0.0), Some(4.0), Some(10.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 92, 100, 60, 0, 3.5, 0.0, 7.0, 14.0, Some(0.0), Some(4.0), Some(14.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 7.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 7.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 7.0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        // Tip
        ATTACK(agent, 0, 0, Hash40::new("haver"), 3.0, 268, 180, 30, 0, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 3.0, 361, 100, 20, 0, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // Middle
        ATTACK(agent, 2, 0, Hash40::new("haver"), 3.0, 268, 180, 30, 0, 5.0, 0.0, 7.25, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 3.0, 361, 100, 20, 0, 5.0, 0.0, 7.25, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // Hilt
        ATTACK(agent, 4, 0, Hash40::new("haver"), 3.0, 268, 180, 30, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("haver"), 3.0, 361, 100, 20, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_ROKET_UNDER_DISABLE_CONTROL_X);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialhi1end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 55, 116, 0, 103, 8.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(4.5), 1.55, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.8);
    }
}

unsafe extern "C" fn game_specialairhi1end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 55, 116, 0, 103, 8.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(4.5), 1.55, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.8);
    } 
}

// ====================================================================================================
// ======================================== SKYWARD SLASH DASH ========================================
// ====================================================================================================

unsafe extern "C" fn game_specialhi2hold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        FT_MOTION_RATE(agent, 0.6);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            //VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            FT_MOTION_RATE(agent, 2.5);
        }
        else{
            FT_MOTION_RATE(agent, 0.8);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.2, 91, 100, 140, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.2, 100, 100, 80, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
}

unsafe extern "C" fn game_specialhi2holdair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        FT_MOTION_RATE(agent, 0.6);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            //VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            FT_MOTION_RATE(agent, 2.5);
        }
        else{
            FT_MOTION_RATE(agent, 0.8);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("rot"), 2.2, 91, 100, 140, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("rot"), 2.2, 100, 100, 80, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    for _ in 0..5 {
        frame(lua_state, 2.0);
        if is_excute(agent) {
            // Charged
            //if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
                ATTACK(agent, 0, 0, Hash40::new("rot"), 3.0, 367, 40, 0, 40, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("rot"), 3.0, 55, 40, 0, 40, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 2, 0, Hash40::new("rot"), 3.0, 367, 100, 100, 0, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 3, 0, Hash40::new("rot"), 3.0, 363, 100, 120, 0, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
            // Uncharged
            else{
                ATTACK(agent, 0, 0, Hash40::new("rot"), 1.0, 367, 40, 0, 40, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("rot"), 1.0, 55, 40, 0, 40, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 2, 0, Hash40::new("rot"), 1.0, 367, 100, 100, 0, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 3, 0, Hash40::new("rot"), 1.0, 363, 100, 120, 0, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(agent) {
        
        let h_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) as f32;
        let v_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) as f32;
        //let angle_calc = v_speed / h_speed as f32;
        //println!("Angle: {}", angle_calc);
        
        let mut launch_angle = if v_speed > 0.0 {(v_speed / h_speed.abs()).atan().to_degrees() as u64} else {361 as u64};

        //println!("H speed: {}", h_speed);
        //println!("V speed: {}", v_speed);

        //println!("Launch Angle (pre-calc): {}", launch_angle);

        if h_speed == 0.0 {
            launch_angle = 90 as u64;
        }

        //println!("Launch Angle: {}", launch_angle);
        
        // Charged
        //if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            ATTACK(agent, 0, 0, Hash40::new("rot"), 6.0, launch_angle, 110, 0, 70, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("rot"), 6.0, launch_angle, 110, 0, 70, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        // Uncharged
        else{
            ATTACK(agent, 0, 0, Hash40::new("rot"), 4.0, 79, 40, 0, 90, 6.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("rot"), 4.0, 79, 40, 0, 90, 3.0, 0.0, 6.0, 6.0, Some(0.0), Some(-6.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.3, 1.23, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash"), Hash40::new("rot"), -2, -4, 0, 0, 0, 0, 0.9, true);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0 ,0, 0, false);
        }
        else if agent.is_situation(*SITUATION_KIND_AIR) {
            EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0 ,0, 0, false);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash"), Hash40::new("rot"), 0, 0, 9, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash2"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash3"), Hash40::new("rot"), 0, -5, 0, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash4"), Hash40::new("rot"), 0, -7.5, 0, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash5"), Hash40::new("rot"), -1, -10, 3.5, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_sdash_slash6"), Hash40::new("rot"), 0, 0, -1.5, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_DETACH_KIND(agent, Hash40::new("miiswordsman_sdash_hit"), -1);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_sdash"), false, true);
    }
}

unsafe extern "C" fn game_specialhi2landing(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 0.8);
        }
    }
}

unsafe extern "C" fn game_specialhi2fall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 0.8);
        }
        if VarModule::is_flag(agent.battle_object, vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT) {
            FT_MOTION_RATE(agent, 0.5);
        }
    }
}

// =============================================================================================
// ======================================== HERO SPIN ==========================================
// =============================================================================================

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let start_x_mul = ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_hi3.start_x_mul");
        KineticModule::mul_speed(boma, &Vector3f{x: start_x_mul, y: 1.0, z: 1.0}, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 14.0, 40, 64, 0, 90, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 13.0, 40, 64, 0, 90, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 10.0, 40, 64, 0, 90, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 361, 64, 0, 80, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 11.0, 361, 64, 0, 80, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 361, 64, 0, 80, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 9.0, 361, 64, 0, 75, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 9.0, 361, 64, 0, 75, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 7.0, 361, 64, 0, 75, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        //AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 7.0, 361, 64, 0, 70, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 7.0, 361, 64, 0, 70, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 5.0, 361, 64, 0, 70, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let start_speed = agent.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let start_x_mul = ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_hi3.start_x_mul");
    agent.clear_lua_stack();
    lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_speed * start_x_mul);
    app::sv_kinetic_energy::set_speed(lua_state);
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 100, 115, 0, 6.0, 0.0, 9.0, 7.0, Some(0.0), Some(6.0), Some(14.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 60, 100, 80, 0, 6.0, 0.0, 10.0, -14.0, Some(0.0), Some(10.0), Some(-9.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 90, 0, 6.5, 0.0, 8.0, 9.0, Some(0.0), Some(7.0), Some(14.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 60, 100, 90, 0, 6.5, 0.0, 5.0, -15.0, Some(0.0), Some(6.0), Some(-11.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 75, 0, 6.0, 0.0, 8.0, 9.0, Some(0.0), Some(6.0), Some(14.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 60, 100, 90, 0, 6.5, 0.0, 8.0, -13.0, Some(0.0), Some(8.0), Some(-10.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 80, 100, 60, 0, 6.3, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(15.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 60, 100, 100, 0, 6.0, 0.0, 9.0, -14.0, Some(0.0), Some(9.0), Some(-10.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 39.0, 47.0, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 208, 0, 30, 7.5, 0.0, 14.0, 4.0, Some(0.0), Some(18.0), Some(11.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 208, 0, 30, 7.5, 0.0, 15.0, 3.0, Some(0.0), Some(19.0), Some(7.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi1start", game_specialhi1start);
    agent.acmd("game_specialairhi1", game_specialairhi1);
    agent.acmd("game_specialhi1end", game_specialhi1end);
    agent.acmd("game_specialairhi1end", game_specialairhi1end);

    agent.acmd("game_specialhi2hold", game_specialhi2hold);
    agent.acmd("game_specialhi2holdair", game_specialhi2holdair);
    agent.acmd("game_specialhi2", game_specialhi2);
    agent.acmd("effect_specialhi2", effect_specialhi2);
    agent.acmd("game_specialhi2landing", game_specialhi2landing);
    agent.acmd("game_specialhi2fall", game_specialhi2fall);

    agent.acmd("game_specialhi3", game_specialhi3);
    agent.acmd("game_specialairhi3", game_specialairhi3);
}