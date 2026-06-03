use super::*;

// Specials
unsafe extern "C" fn game_buddyspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL) {
            WorkModule::on_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_GENERATE_BULLET);
            WorkModule::on_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_ENABLE_SHOOT);
        }
    }
}

unsafe extern "C" fn effect_buddyspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 
    && !VarModule::is_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL) {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("buddy_missile_shot_l"), Hash40::new("top"), 0, 10, 9, 0, 0, 0, 1, false);
        }
        else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("buddy_missile_shot_r"), Hash40::new("top"), 0, 10, 9, 0, 0, 0, 1, false);
        }
    }
}
if is_excute(agent) {
    EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_all"), 0, -6, 0, 0, 0, 0, 0.8, true);
}
frame(lua_state, 16.0);
if is_excute(agent) {
    EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
}
}

unsafe extern "C" fn sound_buddyspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n01"));
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL) {
            PLAY_SE(agent, Hash40::new("se_buddy_special_n04_01"));
        }
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_step_left_m"));
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_step_right_m"));
    }
    frame(lua_state, 73.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n04_02"));
    }
}

unsafe extern "C" fn expression_buddyspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

const KIRBY_OFFSET_Y: f32 = -2.0;
const KIRBY_OFFSET_Z: f32 = -2.0;

unsafe extern "C" fn game_buddyspecialnattack(agent: &mut L2CAgentBase) {
    let mut is_hi = false;
    let mut is_lw = false;
    let mut is_s = false;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let motion = MotionModule::motion_kind(boma);
        is_hi = motion == hash40("buddy_special_n_attack_hi");
        is_lw = motion == hash40("buddy_special_n_attack_lw");
        is_s = !is_hi && !is_lw;
        //ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if is_hi {
            let offset_y = KIRBY_OFFSET_Y-3.0;
            let offset_z = KIRBY_OFFSET_Z;
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 50, 67, 0, 46, 3.2, 0.0, 12.4+offset_y, 6.8+offset_z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 50, 71, 0, 46, 2.6, 0.0, 16.2+offset_y, 17.4+offset_z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 50, 67, 0, 46, 3.6, 0.0, 14.2+offset_y, 12.2+offset_z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        }
        else if is_s {
            let offset_y = KIRBY_OFFSET_Y;
            let offset_z = KIRBY_OFFSET_Z;
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 45, 67, 0, 46, 3.2, 0.0, 7.2+offset_y, 7.4+offset_z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 45, 71, 0, 46, 2.6, 0.0, 7.2+offset_y, 18.4+offset_z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 45, 67, 0, 46, 3.6, 0.0, 7.2+offset_y, 13.2+offset_z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        }
        else {
            let offset_y = KIRBY_OFFSET_Y+2.0;
            let offset_z = KIRBY_OFFSET_Z;
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 40, 67, 0, 46, 3.2, 0.0, 4.2+offset_y, 6.8+offset_z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 40, 71, 0, 46, 2.6, 0.0, 1.2+offset_y, 17.4+offset_z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 40, 67, 0, 46, 3.6, 0.0, 2.8+offset_y, 12.2+offset_z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_END);
    }
}
unsafe extern "C" fn effect_buddyspecialnattack(agent: &mut L2CAgentBase) {
    let mut is_hi = false;
    let mut is_lw = false;
    let mut is_s = false;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let motion = MotionModule::motion_kind(boma);
        is_hi = motion == hash40("buddy_special_n_attack_hi");
        is_lw = motion == hash40("buddy_special_n_attack_lw");
        is_s = !is_hi && !is_lw;
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if is_hi {
            let offset_y = KIRBY_OFFSET_Y-2.0;
            let offset_z = KIRBY_OFFSET_Z;
            EFFECT_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.0+offset_y, 9.5+offset_z, -26, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, 0.5);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("buddy_attack_line"), Hash40::new("buddy_attack_line"), Hash40::new("top"), 0, 10.0+offset_y, 6.5+offset_z, -26, 0, 0, 0.75, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 1.6);
            EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -0.5, 15.0+offset_y, 18.0+offset_z, 0, 0, 0, 0.9, true);
        }
        else if is_s {
            let offset_y = KIRBY_OFFSET_Y;
            let offset_z = KIRBY_OFFSET_Z;
            EFFECT_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6.5+offset_y, 9.5+offset_z, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, 0.5);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("buddy_attack_line"), Hash40::new("buddy_attack_line"), Hash40::new("top"), 0, 6.5+offset_y, 7.0+offset_z, 0, 0, 0, 0.75, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 1.6);
            EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -0.5, 7, 19, 0, 0, 0, 0.9, true);
        }
        else {
            let offset_y = KIRBY_OFFSET_Y+2.0;
            let offset_z = KIRBY_OFFSET_Z;
            EFFECT_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3.5+offset_y, 9.5+offset_z, 15, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, 0.5);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("buddy_attack_line"), Hash40::new("buddy_attack_line"), Hash40::new("top"), 0, 3.5+offset_y, 6.0+offset_z, 15, 0, 0, 0.75, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 1.6);
            EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 0.8+offset_y, 17.5+offset_z, 0, 0, 0, 0.9, true);
        }
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bag_bottom"), -3, 4, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
    }
}

unsafe extern "C" fn sound_buddyspecialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_s01"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_s02"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_buddy_attackhard_s03"));
    }
}

unsafe extern "C" fn expression_buddyspecialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}
unsafe extern "C" fn game_buddyspecialnattackend(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 0.75);
    wait(agent.lua_state_agent, 36.0);
    FT_MOTION_RATE(agent, 0.25);
    frame(agent.lua_state_agent, 40.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    // Specials
    agent.acmd("game_buddyspecialnfire", game_buddyspecialnfire, Priority::Low);
    agent.acmd("effect_buddyspecialnfire", effect_buddyspecialnfire, Priority::Low);
    agent.acmd("sound_buddyspecialnfire", sound_buddyspecialnfire, Priority::Low);
    agent.acmd("expression_buddyspecialnfire", expression_buddyspecialnfire, Priority::Low);

    agent.acmd("game_buddyspecialnattack", game_buddyspecialnattack, Priority::Low);
    agent.acmd("effect_buddyspecialnattack", effect_buddyspecialnattack, Priority::Low);
    agent.acmd("sound_buddyspecialnattack", sound_buddyspecialnattack, Priority::Low);
    agent.acmd("expression_buddyspecialnattack", expression_buddyspecialnattack, Priority::Low);

    agent.acmd("game_buddyspecialnattackend", game_buddyspecialnattackend, Priority::Low);
}
