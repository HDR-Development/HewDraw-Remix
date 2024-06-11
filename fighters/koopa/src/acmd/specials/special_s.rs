use super::*;

unsafe extern "C" fn game_specialscatch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.5, 0.0, 7.0, 17.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 5.5, 0.0, 7.0, 10.0, None, None, None, *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 65, 90, 0, 55, 5.5, 0.0, 7.5, 22.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_THROW);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 65, 90, 0, 55, 6.5, 0.0, 7.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE_RANGE(agent, 25.0, 60.0, 25.0);
    frame(lua_state, 60.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialscatch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 5, 15, -14, 0, 0, 0, 0.8, false, *EF_FLIP_YZ);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), 0, 13.5, 5, 24, -70, 2, 2, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(agent, 0.6, 0.6, 2.0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        FLASH(agent, 1, 1, 0.753, 0.627);
        FLASH_FRM(agent, 5, 0.502, 0, 0, 0);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_specialscatch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_s01"));
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_smash_l02"));
    }
}

unsafe extern "C" fn expression_specialscatch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

unsafe extern "C" fn effect_specialssquat(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_catch"), Hash40::new("haver"), 0, 1, 2, 0, 0, 0, 0.8, false);
    }
}

unsafe extern "C" fn game_specialsthrowf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 65, 49, 0, 88, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    }
}

unsafe extern "C" fn effect_specialsthrowf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 2, 0, -6, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_specialsthrowf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
}

unsafe extern "C" fn expression_specialsthrowf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

unsafe extern "C" fn game_specialsthrowb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 45, 81, 0, 57, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 11.0, 5.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    }
}

unsafe extern "C" fn effect_specialsthrowb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 3, 0, 3, 0, 180, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn sound_specialsthrowb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
}

unsafe extern "C" fn expression_specialsthrowb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

unsafe extern "C" fn effect_specialsfall(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("koopa_drop_air"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.8, true);
    }
}

unsafe extern "C" fn game_specialslanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 18.0, 60, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //CHECK_FINISH_CAMERA(fighter, 3, 9);
        //lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
        //lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: -9.0, z: 0.0});
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 14.0, 8.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE_RANGE(agent, 14.0, 26.0, 16.0);
    frame(lua_state, 26.0);
    FT_MOTION_RATE(agent, 1.0)
}

unsafe extern "C" fn effect_specialslanding(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialslanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_koopa_special_s04"));
        PLAY_STATUS(agent, Hash40::new("se_koopa_special_s05"));
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_koopa_step_left_m"), Hash40::new("se_koopa_step_right_m"));
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_koopa_step_right_m"), Hash40::new("se_koopa_step_left_m"));
    }
}

unsafe extern "C" fn expression_specialslanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

unsafe extern "C" fn game_specialsthrowlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 280, 100, 50, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 3.0, 0, 0, 0, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("head"), 12.0, 361, 50, 0, 80, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        let opponent_boma = agent.get_grabbed_opponent_boma();
        VarModule::on_flag(opponent_boma.object(), vars::common::instance::IS_KNOCKDOWN_THROW);
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    }
}

unsafe extern "C" fn effect_specialsthrowlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 25, 2, 45, 0, 0, 1.7, true);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 2, 0, -8, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("head"), 3, 0, 0, 0, 0, 0, 2.3, 0, 0, 0, 0, 0, 360, true);
        LAST_EFFECT_SET_RATE(agent,0.8);
    }
}

unsafe extern "C" fn sound_specialsthrowlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
}

unsafe extern "C" fn expression_specialsthrowlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialscatch", game_specialscatch, Priority::Low);
    agent.acmd("game_specialsaircatch", game_specialscatch, Priority::Low);
    agent.acmd("effect_specialscatch", effect_specialscatch, Priority::Low);
    agent.acmd("effect_specialsaircatch", effect_specialscatch, Priority::Low);
    agent.acmd("sound_specialscatch", sound_specialscatch, Priority::Low);
    agent.acmd("sound_specialsaircatch", sound_specialscatch, Priority::Low);
    agent.acmd("expression_specialscatch", expression_specialscatch, Priority::Low);
    agent.acmd("expression_specialsaircatch", expression_specialscatch, Priority::Low);

    agent.acmd("effect_specialssquat", effect_specialssquat, Priority::Low);
    agent.acmd("effect_specialairssquat", effect_specialssquat, Priority::Low);

    agent.acmd("game_specialsthrowf", game_specialsthrowf, Priority::Low);
    agent.acmd("game_specialairsthrowf", game_specialsthrowf, Priority::Low);
    agent.acmd("effect_specialsthrowf", effect_specialsthrowf, Priority::Low);
    agent.acmd("effect_specialairsthrowf", effect_specialsthrowf, Priority::Low);
    agent.acmd("sound_specialsthrowf", sound_specialsthrowf, Priority::Low);
    agent.acmd("sound_specialairsthrowf", sound_specialsthrowf, Priority::Low);
    agent.acmd("expression_specialsthrowf", expression_specialsthrowf, Priority::Low);
    agent.acmd("expression_specialairsthrowf", expression_specialsthrowf, Priority::Low);

    agent.acmd("game_specialsthrowb", game_specialsthrowb, Priority::Low);
    agent.acmd("game_specialairsthrowb", game_specialsthrowb, Priority::Low);
    agent.acmd("effect_specialsthrowb", effect_specialsthrowb, Priority::Low);
    agent.acmd("effect_specialairsthrowb", effect_specialsthrowb, Priority::Low);
    agent.acmd("sound_specialsthrowb", sound_specialsthrowb, Priority::Low);
    agent.acmd("sound_specialairsthrowb", sound_specialsthrowb, Priority::Low);
    agent.acmd("expression_specialsthrowb", expression_specialsthrowb, Priority::Low);
    agent.acmd("expression_specialairsthrowb", expression_specialsthrowb, Priority::Low);

    agent.acmd("effect_specialsfall", effect_specialsfall, Priority::Low);

    agent.acmd("game_specialslanding", game_specialslanding, Priority::Low);
    agent.acmd("effect_specialslanding", effect_specialslanding, Priority::Low);
    agent.acmd("sound_specialslanding", sound_specialslanding, Priority::Low);
    agent.acmd("expression_specialslanding", expression_specialslanding, Priority::Low);

    agent.acmd("game_specialsthrowlw", game_specialsthrowlw, Priority::Low);
    agent.acmd("game_specialairsthrowlw", game_specialsthrowlw, Priority::Low);
    agent.acmd("effect_specialsthrowlw", effect_specialsthrowlw, Priority::Low);
    agent.acmd("effect_specialairsthrowlw", effect_specialsthrowlw, Priority::Low);
    agent.acmd("sound_specialsthrowlw", sound_specialsthrowlw, Priority::Low);
    agent.acmd("sound_specialairsthrowlw", sound_specialsthrowlw, Priority::Low);
    agent.acmd("expression_specialsthrowlw", expression_specialsthrowlw, Priority::Low);
    agent.acmd("expression_specialairsthrowlw", expression_specialsthrowlw, Priority::Low);

    // agent.acmd("game_specialsthrowlwlanding", game_specialsthrowlwlanding, Priority::Low);
    // agent.acmd("effect_specialsthrowlwlanding", effect_specialsthrowlwlanding, Priority::Low);
    // agent.acmd("sound_specialsthrowlwlanding", sound_specialsthrowlwlanding, Priority::Low);
    // agent.acmd("expression_specialsthrowlwlanding", expression_specialsthrowlwlanding, Priority::Low);
}