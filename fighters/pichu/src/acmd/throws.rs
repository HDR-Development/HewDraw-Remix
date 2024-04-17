use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.3, 0.0, 6.6, 0.0, Some(0.0), Some(6.6), Some(9.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(9.75), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.3, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-14.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0 * damage_mul, 45, 115, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0 * damage_mul, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        FT_ADD_DAMAGE(agent, 0.5 * recoil_mul);
    }
    for _ in 0..4{
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.5 * damage_mul, 361, 100, 0, 0, 5.5, 0.0, 8.5, 4.7, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            AttackModule::set_catch_only_all(boma, true, false);
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 6, 26);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(agent) {
        if charged {
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0 * damage_mul, 85, 30, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        else {
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 145, 50, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if charged {
            FT_DESIRED_RATE(agent, 14.0, 9.0);
            FT_ADD_DAMAGE(agent, 1.0);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, -6, 4);
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn effect_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(agent) {
        if charged {
            EFFECT(agent, Hash40::new("sys_level_up"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.35, 0.95);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 1, 0, 0, 90, 0.55, true);
        LAST_EFFECT_SET_RATE(agent, 1.4);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 1, 0, 30, 90, 0.55, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -0.5, 0, 0, 90, 0.55, true);
        LAST_EFFECT_SET_RATE(agent, 1.4);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -0.5, 0, 30, 90, 0.55, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        if charged {
            EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            //EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec_shock_finish"), Hash40::new("top"), 0, 5.5, 13, 0, 0, 0, 0.85, true);
            EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.01, 0.2, 0.95);
        }
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(agent) {
        if charged {
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 90, 55, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        else {
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 90, 50, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hat"), 5.0 * damage_mul, 80, 60, 0, 70, 4.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(boma, true, false);
        FT_ADD_DAMAGE(agent, 1.0 * recoil_mul);
        CHECK_FINISH_CAMERA(agent, 2, 17);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    let charge_state_time = ParamModule::get_int(boma.object(), ParamType::Agent, "charge_state_time");
    if is_excute(agent) {
        if charged{
            VarModule::sub_int(agent.battle_object, vars::common::instance::GIMMICK_TIMER, 180);
            MeterModule::drain_direct(boma.object(), (50.0/(charge_state_time as f32)) * 180.0);
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 60, 5, 0, 135, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_THROW);
        }
        else{ 
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 80, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 7.0);
    if charged {
        FT_DESIRED_RATE(agent, 4.0, 10.0)
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if charged{
            ATTACK(agent, 0, 0, Hash40::new("trans"), 15.0, 361, 60, 0, 60, 11.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_HEAD);
            FT_ADD_DAMAGE(agent, 5.0);
        }
        else{ 
            ATTACK(agent, 0, 0, Hash40::new("trans"), 4.0, 361, 60, 0, 60, 5.9, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        }
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(agent, 1, 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
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
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if charged {
            EFFECT(agent, Hash40::new("sys_level_up"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.35, 0.95);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if charged {
            EFFECT(agent, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_COLOR(agent, 0.01, 0.2, 0.95);
        }
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch);
    agent.acmd("game_catchdash", game_catchdash);
    agent.acmd("game_catchturn", game_catchturn);

    agent.acmd("game_throwf", game_throwf);

    agent.acmd("game_throwb", game_throwb);
    agent.acmd("effect_throwb", effect_throwb);

    agent.acmd("game_throwhi", game_throwhi);

    agent.acmd("game_throwlw", game_throwlw);
    agent.acmd("effect_throwlw", effect_throwlw);
}
