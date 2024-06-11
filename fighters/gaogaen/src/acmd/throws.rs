use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
        ControlModule::clear_command(boma, true);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        let grab_y = VarModule::get_float(agent.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let mut z_mod = -1.0 * grab_y;
        if grab_y > 0.0 {
            z_mod = 3.0 * grab_y;
        }
        else if grab_y < 0.0 {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 20, 10, 0, 4.2, 0.0, (grab_y * 8.0) + 11.0, 13.4 - z_mod, Some(0.0), Some(10.0), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        CATCH(agent, 0, Hash40::new("top"), 4.2, 0.0, (grab_y * 8.0) + 11.0, 13.4 - z_mod, Some(0.0), Some(10.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(lua_state, 11.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn effect_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        let grab_y = VarModule::get_float(agent.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let rot_right = 0 - ((grab_y * 50.0) as i32);
        let rot_left = 180 + ((grab_y * 50.0) as i32);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 0, 11.0, 5, rot_right, -45, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 7, 10.5, 4, rot_left, -225, 0, 1.2, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
        ControlModule::clear_command(boma, true);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        let grab_y = VarModule::get_float(agent.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let mut z_mod = 0.0;
        if grab_y > 0.0 {
            z_mod = 4.0 * grab_y;
        }
        else if grab_y < 0.0 {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 20, 10, 0, 4.2, 0.0, (grab_y * 8.0) + 9.0, 11.6 - z_mod, Some(0.0), Some(9.0), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        CATCH(agent, 0, Hash40::new("top"), 4.2, 0.0, (grab_y * 8.0) + 9.0, 11.6 - z_mod, Some(0.0), Some(9.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn effect_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        let grab_y = VarModule::get_float(agent.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let rot_right = 0 - ((grab_y * 50.0) as i32);
        let rot_left = 180 + ((grab_y * 50.0) as i32);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 0, 9.0, 3.5, rot_right, -45, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 7, 8.5, 2.5, rot_left, -225, 0, 1.2, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
        ControlModule::clear_command(boma, true);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        let grab_y = VarModule::get_float(agent.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let mut z_mod = 0.0;
        if grab_y > 0.0 {
            z_mod = 3.0 * grab_y;
        }
        else if grab_y < 0.0 {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 20, 10, 0, 4.2, 0.0, (grab_y * 8.0) + 11.0, -16.9 - z_mod, Some(0.0), Some(10.0), Some(-2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        CATCH(agent, 0, Hash40::new("top"), 4.2, 0.0, (grab_y * 8.0) + 11.0, -16.9 + z_mod, Some(0.0), Some(10.0), Some(-2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn effect_catchturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        let grab_y = VarModule::get_float(agent.battle_object, vars::gaogaen::status::ANGLE_GRAB_STICK_Y);
        let rot_right = 0 - ((grab_y * 50.0) as i32);
        let rot_left = 180 + ((grab_y * 50.0) as i32);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 0, 11.0, -7, rot_right, 135, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 7, 10.5, -8, rot_left, -45, 0, 1.2, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
        VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    }
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 40, 64, 0, 82, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("hip"), 4.0, 30, 63, 0, 100, 8.0, -3.0, 7.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("hip"), 6.0, 30, 62, 0, 100, 8.0, -4.5, 14.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("hip"), 8.0, 30, 61, 0, 100, 8.0, -6.0, 22.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("hip"), 10.0, 30, 60, 0, 100, 8.0, -6.0, 22.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        CHECK_FINISH_CAMERA(agent, 16, 15);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn effect_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, 13.0, 20.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.8);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);

            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);

            EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 1.25);

            EFFECT(agent, Hash40::new("sys_hit_fire"), Hash40::new("top"), 0.0, 8.0, 10.0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.25);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -3, 0, 180, -90, 0, 1.6, true, *EF_FLIP_YZ, 1.0);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 1, 0, 180, -90, 0, 1.8, true, *EF_FLIP_YZ, 1.0);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -2, 0, 180, 0, 0, 1.7, true, *EF_FLIP_YZ, 1.0);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 0, 0, 180, 0, 0, 2, true, *EF_FLIP_YZ, 1.0);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 1.5, 0, 180, 0, 0, 2.3, true, *EF_FLIP_YZ, 1.0);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 59.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 20, 13, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 360, true, 0.7);
        LAST_EFFECT_SET_RATE(agent, 0.6);
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        }
        // Kill throw
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 40, 490, 0, 20, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        // Techchase throw
        else{
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 275, 100, 30, 0, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 15.0/(16.0-11.0));
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 1.0);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        // Kill throw
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 32, 80, 0, 60, 7.0, 0.0, 2.7, 2.6, Some(0.0), Some(2.7), Some(-3.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        // Techchase throw
        else{
            ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 32, 80, 0, 60, 7.0, 0.0, 2.7, 2.6, Some(0.0), Some(2.7), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(agent, -4, 5);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            ModelModule::set_joint_translate(boma, Hash40::new("throw"), &Vector3f{x: 2.439, y: -6.660, z: -5.0}, false, false);
            let opponent_boma = agent.get_grabbed_opponent_boma();
            VarModule::on_flag(opponent_boma.object(), vars::common::instance::IS_KNOCKDOWN_THROW);
        }
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {1.0} else {26.0/(52.0 - 15.0)});
        REVERSE_LR(agent);
    }
}

unsafe extern "C" fn effect_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, 13.0, 20.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.8);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);

            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);

            EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 1.25);

            EFFECT(agent, Hash40::new("sys_hit_fire"), Hash40::new("top"), 0.0, 8.0, 10.0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.25);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 27.0);
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT(agent, Hash40::new("sys_bomb_b"), Hash40::new("top"), 10, 0, -5.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
        VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    }
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            // Incin Buster
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 14.0, 45, 20, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        else {
            // Normal throw
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 88, 79, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 33.0/(22.0-10.0));
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 1.0);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        //FT_CATCH_STOP(fighter, 14, 1);
        CHECK_FINISH_CAMERA(agent, 1, 20);
        let hitlag = if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {4.5} else {3.0};
        let sound = if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {*COLLISION_SOUND_ATTR_HEAVY} else {*COLLISION_SOUND_ATTR_PUNCH};
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 88, 70, 0, 65, 7.0, 0.0, 10.0, 0.0, Some(0.0), Some(10.0), Some(0.0), hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);

            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);

            EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 1.25);

            EFFECT(agent, Hash40::new("sys_hit_fire"), Hash40::new("top"), 0.0, 8.0, 10.0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.25);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT(agent, Hash40::new("sys_hit_fire"), Hash40::new("waist"), 0.0, 0.0, 10.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.75);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 32, 0, 90, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.5);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("gaogaen_throw_hi"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT(agent, Hash40::new("gaogaen_revenge_start"), Hash40::new("top"), -2, 10, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            PLAY_SE(agent, Hash40::new("vc_gaogaen_appeal_l01"));
        }
        else {
            PLAY_SEQUENCE(agent, Hash40::new("seq_gaogaen_rnd_attack"));
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_kick_hit_m"));
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            PLAY_SEQUENCE(agent, Hash40::new("seq_gaogaen_rnd_attackappeal01"));
        }
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
        VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    }
    if is_excute(agent) {
        let bkb = if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) { 50 } else { 70 };
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 68, 47, 0, bkb, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        FT_CATCH_STOP(agent, 12, 1);
        CHECK_FINISH_CAMERA(agent, 8, 0);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn effect_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);

            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);

            EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 1.25);

            EFFECT(agent, Hash40::new("sys_hit_fire"), Hash40::new("top"), 0.0, 8.0, 10.0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.25);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -10, 24, 0, 90, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.5);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -10, 21, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.5);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch, Priority::Low);
    agent.acmd("effect_catch", effect_catch, Priority::Low);
    agent.acmd("game_catchturn", game_catchturn, Priority::Low);
    agent.acmd("effect_catchturn", effect_catchturn, Priority::Low);
    agent.acmd("game_catchdash", game_catchdash, Priority::Low);
    agent.acmd("effect_catchdash", effect_catchdash, Priority::Low);

    agent.acmd("game_throwf", game_throwf, Priority::Low);
    agent.acmd("effect_throwf", effect_throwf, Priority::Low);

    agent.acmd("game_throwb", game_throwb, Priority::Low);
    agent.acmd("effect_throwb", effect_throwb, Priority::Low);

    agent.acmd("game_throwhi", game_throwhi, Priority::Low);
    agent.acmd("effect_throwhi", effect_throwhi, Priority::Low);
    agent.acmd("sound_throwhi", sound_throwhi, Priority::Low);
    
    agent.acmd("game_throwlw", game_throwlw, Priority::Low);
    agent.acmd("effect_throwlw", effect_throwlw, Priority::Low);
}
