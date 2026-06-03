use super::*;

unsafe extern "C" fn game_specials4hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.0, 9.0);
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 1.2, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 84, 34, 0, 91, 4.0, 0.0,  6.0, 11.0, Some(0.0), Some(24.0), Some(11.0), 1.2, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 84, 34, 0, 91, 4.0, 0.0, 10.0, 16.0, Some(0.0), Some(24.0), Some(16.0), 1.2, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 11.0, 84, 34, 0, 91, 4.0, 0.0, 24.0, 11.0, Some(0.0), Some(28.0), Some(4.5),  1.2, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 11.0, 84, 34, 0, 91, 4.0, 0.0, 24.0, 16.0, Some(0.0), Some(30.5), Some(7.0),  1.2, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    for _ in 0..5 {
        wait(lua_state, 1.0);
        if is_excute(agent) {
            KineticModule::mul_speed(boma, &Vector3f::new(0.85, 1.0, 1.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
    }
    frame(lua_state, 12.0);
    // FT_MOTION_RATE(agent, 0.75);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specials4hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_hi"), Hash40::new("clavicler"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_hi"), Hash40::new("claviclel"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_hi"), Hash40::new("hip"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_hi"), Hash40::new("kneer"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_hi"), Hash40::new("kneel"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_hi"), Hash40::new("footr"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_hi"), Hash40::new("footl"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_hi"), Hash40::new("armr"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_hi"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword"), Hash40::new("sword1"), -0.0, 0, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_4hi_hdr"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword"), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_mc_aura_hi"), false, true);
    }
}

unsafe extern "C" fn sound_specials4hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_special_s04h"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_chrom_attack05"));
    }
}

unsafe extern "C" fn expression_specials4hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y_MINUS));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 2);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

unsafe extern "C" fn game_specials4s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 5.0, 10.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.3, 0);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 1.5, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.75, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 3.0, 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 55, 74, 0, 60, 5.0, 0.0, 9.0,  5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 55, 74, 0, 60, 7.0, 0.0, 9.0,  7.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 16.0, 55, 74, 0, 60, 8.0, 0.0, 9.0, 11.7, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 16.0, 55, 74, 0, 60, 7.0, 0.0, 9.0, 15.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    for _ in 0..4 {
        wait(lua_state, 1.0);
        if is_excute(agent) {
            KineticModule::mul_speed(boma, &Vector3f::new(0.9, 1.0, 1.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
    }
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    for _ in 0..6 {
        wait(lua_state, 1.0);
        if is_excute(agent) {
            KineticModule::mul_speed(boma, &Vector3f::new(0.8, 1.0, 1.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
    }
}

unsafe extern "C" fn effect_specials4s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_s"), Hash40::new("clavicler"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_s"), Hash40::new("claviclel"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_s"), Hash40::new("hip"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_s"), Hash40::new("kneer"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_s"), Hash40::new("kneel"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_s"), Hash40::new("footr"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_s"), Hash40::new("footl"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_s"), Hash40::new("armr"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_s"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_red"), Hash40::new("sword1"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_4s_hdr"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_red"), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_mc_aura_s"), false, true);
    }
}

unsafe extern "C" fn sound_specials4s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_jump01"));
        PLAY_SE(agent, Hash40::new("vc_chrom_attack06"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_special_s04s"));
    }
}

unsafe extern "C" fn game_specials4lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let dash_speed = Vector3f::new(1.5 / 4.0, 0.0, 0.0);
    if is_excute(agent) {
        // enables tilt cancels
        agent.on_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        ControlModule::clear_command(boma, false);
    }
    frame(lua_state, 3.0);
    FT_MOTION_RATE_RANGE(agent, 3.0, 14.0, 8.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &dash_speed);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        // disables tilt cancels
        agent.off_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        KineticModule::add_speed(boma, &dash_speed);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &dash_speed);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &dash_speed);
    }
    wait(lua_state, 1.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    for _ in 0..6 {
        if is_excute(agent) {
            KineticModule::mul_speed(boma, &Vector3f::new(0.9, 1.0, 1.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn effect_specials4lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_lw"), Hash40::new("clavicler"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_lw"), Hash40::new("claviclel"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_lw"), Hash40::new("hip"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_lw"), Hash40::new("kneer"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_lw"), Hash40::new("kneel"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_lw"), Hash40::new("footr"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_lw"), Hash40::new("footl"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_lw"), Hash40::new("armr"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura_lw"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 14, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_mc_aura_lw"), false, true);
    }
}

unsafe extern "C" fn sound_specials4lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_chrom_special_s01"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials4hi", game_specials4hi, Priority::Low);
    agent.acmd("game_specialairs4hi", game_specials4hi, Priority::Low);
    agent.acmd("effect_specials4hi", effect_specials4hi, Priority::Low);
    agent.acmd("effect_specialairs4hi", effect_specials4hi, Priority::Low);
    agent.acmd("sound_specials4hi", sound_specials4hi, Priority::Low);
    agent.acmd("sound_specialairs4hi", sound_specials4hi, Priority::Low);
    agent.acmd("expression_specials4hi", expression_specials4hi, Priority::Low);
    agent.acmd("expression_specialairs4hi", expression_specials4hi, Priority::Low);

    agent.acmd("game_specials4s", game_specials4s, Priority::Low);
    agent.acmd("game_specialairs4s", game_specials4s, Priority::Low);
    agent.acmd("effect_specials4s", effect_specials4s, Priority::Low);
    agent.acmd("effect_specialairs4s", effect_specials4s, Priority::Low);
    agent.acmd("sound_specials4s", sound_specials4s, Priority::Low);
    agent.acmd("sound_specialairs4s", sound_specials4s, Priority::Low);

    agent.acmd("game_specials4lw", game_specials4lw, Priority::Low);
    agent.acmd("game_specialairs4lw", game_specials4lw, Priority::Low);
    agent.acmd("effect_specials4lw", effect_specials4lw, Priority::Low);
    agent.acmd("effect_specialairs4lw", effect_specials4lw, Priority::Low);
    agent.acmd("sound_specials4lw", game_specials4lw, Priority::Low);
    agent.acmd("sound_specialairs4lw", game_specials4lw, Priority::Low);
    agent.acmd("expression_specials4lw", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairs4lw", acmd_stub, Priority::Low);
}