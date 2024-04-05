use super::*;
use globals::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, smash::phx::Hash40::new("attack_s4_s"), true, 0.0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, smash::phx::Hash40::new("attack_s4_s"), true, 6.0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 95, 0, 36, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 361, 95, 0, 36, 4.2, 0.0, 6.0, 17.200001, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            AttackModule::set_ink_value(boma, 0, 100.0);
            AttackModule::set_ink_value(boma, 1, 120.0);
        }
        if !WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 93, 0, 35, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 361, 93, 0, 35, 4.2, 0.0, 6.0, 17.200001, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}
/* unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 14.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(agent, 0, 0, Hash40::new("brush2"), 14.0, 361, 95, 0, 36, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("brush2"), 16.0, 361, 95, 0, 36, 4.2, 0.0, 6.0, 17.200001, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
        if !WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(agent, 0, 0, Hash40::new("brush2"), 12.0, 361, 93, 0, 35, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("brush2"), 12.0, 361, 93, 0, 35, 4.2, 0.0, 6.0, 17.200001, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }

    frame(lua_state, 53.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}*/ 

unsafe extern "C" fn game_attacks4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, smash::phx::Hash40::new("attack_s4_charge"), false, 0.0);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BLASTER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("attack_hi4"), true, 0.0);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        if WorkModule::is_flag(boma,  *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
            ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("attack_hi4"), true, WorkModule::get_float(boma, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME));
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 125, 100, 100, 0, 3.5, 0.0, 4.0, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 88, 82, 0, 55, 12.5, 0.0, 20.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 88, 100, 0, 55, 14.0, 0.0, 20.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(boma, 0, 120.0);
            AttackModule::set_ink_value(boma, 1, 100.0);
        }
        if !WorkModule::is_flag(boma,  *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 88, 66, 0, 55, 10.0, 0.0, 20.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 88, 66, 0, 55, 12.0, 0.0, 20.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackhi4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("attack_hi4_charge"), false, 0.0);
    }
}
unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SLOSHER, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SLOSHER, Hash40::new("attack_lw4"), false, -1.0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);

        if is_excute(agent) {
            let restart_frame = WorkModule::get_float(boma, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
            ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SLOSHER, Hash40::new("attack_lw4"), true, restart_frame);
        }
    
    frame(lua_state, 11.0);
    WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS);
    
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 32, 90, 0, 40, 3.8, 0.0, 3.5, 12.0, Some(0.0), Some(3.5), Some(18.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(boma, 0, 150.0);
        }
        else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 32, 85, 0, 35, 3.8, 0.0, 3.5, 12.0, Some(0.0), Some(3.5), Some(18.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        }
    }

wait(lua_state, 2.0);
WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS);

    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 32, 85, 0, 40, 4.5, 0.0, 4.0, 12.0, Some(0.0), Some(4.0), Some(22.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        AttackModule::set_ink_value(boma, 0, 120.0);
    }
    else {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 32, 75, 0, 35, 4.5, 0.0, 4.0, 12.0, Some(0.0), Some(4.0), Some(22.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
    }
}

wait(lua_state, 3.0);
if is_excute(agent) {
AttackModule::clear_all(boma);
}
frame(lua_state, 20.0);
WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS);

if is_excute(agent) {
    ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 30, 82, 0, 40, 3.8, 0.0, 3.5, -12.0, Some(0.0), Some(3.5), Some(-16.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
    AttackModule::set_ink_value(boma, 0, 130.0);
}
else {
if is_excute(agent) {
    ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 30, 75, 0, 35, 3.8, 0.0, 3.5, -12.0, Some(0.0), Some(3.5), Some(-16.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
}
}

wait(lua_state, 2.0);
WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS);

if is_excute(agent) {
ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 30, 85, 0, 40, 4.5, 0.0, 4.0, -12.0, Some(0.0), Some(4.0), Some(-20.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
AttackModule::set_ink_value(boma, 0, 100.0);
}
else {
if is_excute(agent) {
ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 30, 75, 0, 35, 4.5, 0.0, 4.0, -12.0, Some(0.0), Some(4.0), Some(-20.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
}
}

wait(lua_state, 3.0);
if is_excute(agent) {
AttackModule::clear_all(boma);
}
FT_MOTION_RATE(agent, 0.806);
}


pub fn install() {
    smashline::Agent::new("inkling")
        .acmd("game_attacks4", game_attacks4)
        .acmd("game_attacks4charge", game_attacks4charge)
        .acmd("game_attackhi4", game_attackhi4)
        .acmd("game_attackhi4charge", game_attackhi4charge)
        .acmd("game_attacklw4", game_attacklw4)
        .install();
}
