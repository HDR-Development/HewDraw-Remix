use super::*;

unsafe extern "C" fn game_final(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(agent, 80.0, 50.0);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(agent.lua_state_agent, 10.0);
        if is_excute(agent) {
            FT_SET_FINAL_FEAR_FACE(agent, 40);
            REQ_FINAL_START_CAMERA(agent, Hash40::new("d04final.nuanmb"), true);
            FT_START_CUTIN(agent);
        }
    } else {
        if is_excute(agent) {
            CAM_ZOOM_IN_arg5(agent, 3.0, 0.0, 1.8, 0.0, 0.0);
            FT_START_CUTIN(agent);
        }
        frame(agent.lua_state_agent, 28.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 60, 90, 0, 50, 8.0, 0.0, 5.0, 8.0, Some(0.0), Some(9.5), Some(8.0), 1.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_damage_shake_scale(agent.module_accessor, 0.18);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        SlowModule::set_whole(agent.module_accessor, 2, 0);
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_LOCK_ATTACK);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 80, 95, 0, 50, 8.0, 0.0, 5.0, 10.0, Some(0.0), Some(9.5), Some(10.0), 2.6, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_FINAL01, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_BRANCH_HIT);
        SlowModule::clear_whole(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 52.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 2.0, 367, 100, 120, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ATTACK(agent, 0, 0, Hash40::new("handr"), 5.0, 80, 120, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_finalhit(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        SlowModule::set_whole(agent.module_accessor, 2, 0);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 80, 95, 0, 50, 8.0, 0.0, 12.0, 10.0, None, None, None, 2.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_FINAL02, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_damage_shake_scale(agent.module_accessor, 0.18);
        CAM_ZOOM_OUT(agent);
        CAM_ZOOM_IN_arg5(agent, 3.0, 0.0, 1.9, 0.0, 0.0);
        WorkModule::set_int(agent.module_accessor, *FIGHTER_RYU_FINAL_CAMERA_OFFSET_2, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_CAMERA_OFFSET_TYPE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_CAMERA_OFFSET);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_REMOVE_FINAL_AURA);
        CAM_ZOOM_OUT(agent);
        CAM_ZOOM_IN_arg5(agent, 5.0, 0.0, 2.1, 0.0, 0.0);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END_SET_PARAM);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END);
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.1, 80, 126, 0, 82, 10.0, 0.0, 20.0, 8.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_FINAL03, *ATTACK_REGION_PUNCH);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_damage_shake_scale(agent.module_accessor, 0.18);
        WorkModule::set_int(agent.module_accessor, *FIGHTER_RYU_FINAL_CAMERA_OFFSET_3, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_CAMERA_OFFSET_TYPE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_CAMERA_OFFSET);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        SlowModule::clear_whole(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        CAM_ZOOM_OUT(agent);
        WorkModule::set_int(agent.module_accessor, *FIGHTER_RYU_FINAL_CAMERA_OFFSET_RETURN, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_CAMERA_OFFSET_TYPE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_CAMERA_OFFSET);
    }
}

unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 32, 100, 90, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -0.6, 0.0, 5, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.6, 366, 100, 75, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -0.6, 0.0, 5, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        // ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 366, 100, 80, 0, 35.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 90.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ATTACK(agent, 0, 1, Hash40::new("top"), 7.0, 83, 130, 0, 50, 16.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_final", game_final);
    agent.acmd("game_finalair", game_final);
    
    agent.acmd("game_finalhit", game_finalhit);
    agent.acmd("game_finalairhit", game_finalhit);
}
