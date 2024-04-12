use super::*;
use globals::*;
use std::ops::Index;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_SYNC);
    }
    FT_MOTION_RATE_RANGE(agent, 2.0, 7.0, 3.0);
    frame(lua_state, 7.0);
    FT_MOTION_RATE_RANGE(agent, 7.0, 23.0, 20.0);
    if is_excute(agent) {
        let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        if (pikmin_count == 0) {
            ATTACK(agent, 0, 0, Hash40::new("handr"),     5.0, 75, 100, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 5.0, 75, 100, 0, 40, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 2, 0, Hash40::new("head"),      5.0, 75, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        }
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        if (pikmin_count == 0) {
            ATTACK(agent, 0, 0, Hash40::new("handr"),     4.5, 62, 100, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 4.5, 62, 100, 0, 40, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 2, 0, Hash40::new("head"),      4.5, 62, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        if (pikmin_count == 0) {
            ATTACK(agent, 0, 0, Hash40::new("handr"),     4.0, 50, 100, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 4.0, 50, 100, 0, 40, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 2, 0, Hash40::new("head"),      4.0, 50, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        }
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_DETACH);
    }
}

unsafe extern "C" fn sound_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pikmin_attackair_n01"));
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pikmin_attackair_n02"));
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pikmin_attackair_n03"));
    }
}

unsafe extern "C" fn expression_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 3.0, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_SYNC);
    }
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        if (pikmin_count == 0) {
            ATTACK(agent, 0, 0, Hash40::new("handr"), 5.0, 55, 70, 0, 62, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 5.0, 55, 70, 0, 62, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        }
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_DETACH);
    }
}

unsafe extern "C" fn expression_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if (pikmin_count != 0) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_SYNC);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        if (pikmin_count == 0) {
            ATTACK(agent, 0, 0, Hash40::new("handr"), 6.0, 35, 120, 0, 50, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 6.0, 35, 120, 0, 50, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_DETACH);
    }
}

unsafe extern "C" fn expression_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    //With Pikmin
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if (pikmin_count != 0) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
    }
    //Without Pikmin
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if (pikmin_count == 0) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 3.0, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_SYNC);
    }
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        if (pikmin_count == 0) {
            ATTACK(agent, 0, 0, Hash40::new("handr"), 5.0, 95, 90, 0, 52, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 5.0, 95, 90, 0, 52, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_DETACH);
    }
}

unsafe extern "C" fn expression_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    //With Pikmin
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if (pikmin_count != 0) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_SYNC);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if (pikmin_count == 0) {
        frame(lua_state, 9.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("handr"), 5.0, 270, 90, 0, 42, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 5.0, 270, 90, 0, 42, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        }
        frame(lua_state, 14.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("handr"), 4.0, 270, 90, 0, 42, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 4.0, 270, 90, 0, 42, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_DETACH);
    }
}

unsafe extern "C" fn stub(agent: &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn);
    agent.acmd("effect_attackairn", stub);
    agent.acmd("sound_attackairn", sound_attackairn);
    agent.acmd("expression_attackairn", expression_attackairn);

    agent.acmd("game_attackairf", game_attackairf);
    agent.acmd("expression_attackairf", expression_attackairf);

    agent.acmd("game_attackairb", game_attackairb);
    agent.acmd("expression_attackairb", expression_attackairb);

    agent.acmd("game_attackairhi", game_attackairhi);
    agent.acmd("expression_attackairhi", expression_attackairhi);

    agent.acmd("game_attackairlw", game_attackairlw);
}
