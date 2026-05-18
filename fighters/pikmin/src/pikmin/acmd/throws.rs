use super::*;

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 5.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        ATTACK_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(agent.boma());
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let dmg = if variation == 2 { 7.5 } else { 5.0 };
        ATTACK_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, dmg, 74 + p.angle, 49.mul_f32(p.kbg), 0, 84, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, p.attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        REVERSE_LR(agent);
        let target = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK);
        let target_group = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(agent.boma());
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let dmg = if variation == 2 { 13.0 } else { 8.5 };
        ATTACK_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, dmg, 45 + p.angle, 68.mul_f32(p.kbg), 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, p.attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK);
        let target_group = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(agent.boma());
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let dmg = if variation == 2 { 12.0 } else { 8.0 };
        ATTACK_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, dmg, 86 + p.angle, 90.mul_f32(p.kbg), 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, p.attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        let target = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK);
        let target_group = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(agent.boma());
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 22.0);
    if is_excute(agent) {
        let dmg = 1.0;
        ATTACK(agent, 0, 0, Hash40::new("top"), dmg, 90 + p.angle, 200.mul_f32(p.kbg), 0, 50, 7.0, 0.0, 4.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(boma, true, false);
        let dmg = if variation == 2 { 7.5 } else { 5.0 };
        ATTACK_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, dmg, 74 + p.angle, 49.mul_f32(p.kbg), 0, 84, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, p.attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        let target = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK);
        let target_group = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

pub fn install(agent: &mut Agent) {;
    agent.acmd("game_catchdash", game_catchdash, Priority::Low);
    agent.acmd("game_catchdash_b", game_catchdash, Priority::Low);
    agent.acmd("game_catchdash_v", game_catchdash, Priority::Low);
    agent.acmd("game_catchdash_w", game_catchdash, Priority::Low);
    agent.acmd("game_catchdash_y", game_catchdash, Priority::Low);

    agent.acmd("game_throwb", game_throwb, Priority::Low);
    agent.acmd("game_throwb_b", game_throwb, Priority::Low);
    agent.acmd("game_throwb_v", game_throwb, Priority::Low);
    agent.acmd("game_throwb_w", game_throwb, Priority::Low);
    agent.acmd("game_throwb_y", game_throwb, Priority::Low);

    agent.acmd("game_throwf", game_throwf, Priority::Low);
    agent.acmd("game_throwf_b", game_throwf, Priority::Low);
    agent.acmd("game_throwf_v", game_throwf, Priority::Low);
    agent.acmd("game_throwf_w", game_throwf, Priority::Low);
    agent.acmd("game_throwf_y", game_throwf, Priority::Low);

    agent.acmd("game_throwhi", game_throwhi, Priority::Low);
    agent.acmd("game_throwhi_b", game_throwhi, Priority::Low);
    agent.acmd("game_throwhi_v", game_throwhi, Priority::Low);
    agent.acmd("game_throwhi_w", game_throwhi, Priority::Low);
    agent.acmd("game_throwhi_y", game_throwhi, Priority::Low);
    
    agent.acmd("game_throwlw", game_throwlw, Priority::Low);
    agent.acmd("game_throwlw_b", game_throwlw, Priority::Low);
    agent.acmd("game_throwlw_v", game_throwlw, Priority::Low);
    agent.acmd("game_throwlw_w", game_throwlw, Priority::Low);
    agent.acmd("game_throwlw_y", game_throwlw, Priority::Low);
}
