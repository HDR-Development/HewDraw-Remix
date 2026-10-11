use super::*;

unsafe extern "C" fn game_specials3s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        agent.off_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 3.0, 6.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.1, 0);
    }
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 1.2, 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 53, 31, 0, 50, 5.0, 0.0, 7.5,  6.0, Some(0.0), Some(9.5),  Some(6.0),  1.0, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 58, 31, 0, 50, 5.5, 0.0, 8.0, 11.0, Some(0.0), Some(11.5), Some(11.0), 1.0, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 63, 31, 0, 50, 5.5, 0.0, 9.0, 16.0, Some(0.0), Some(11.5), Some(16.0), 1.0, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    for _ in 0..3 {
        wait(lua_state, 1.0);
        if is_excute(agent) {
            KineticModule::mul_speed(boma, &Vector3f::new(0.9, 1.0, 1.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
    }
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    for _ in 0..3 {
        wait(lua_state, 1.0);
        if is_excute(agent) {
            KineticModule::mul_speed(boma, &Vector3f::new(0.8, 1.0, 1.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            agent.on_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        }
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

unsafe extern "C" fn effect_specials3s(agent: &mut L2CAgentBase) {
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
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_red"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_3s_hdr"), Hash40::new("top"), 0, 0, 4, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_red"), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_mc_aura_s"), false, true);
    }
}

unsafe extern "C" fn game_specials3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        agent.off_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.0, 7.0);
    frame(lua_state, 4.0);
    FT_MOTION_RATE_RANGE(agent, 4.0, 5.0, 4.0);
    frame(lua_state, 4.5);
    if is_excute(agent) {
        // knockdown is forced in CheckAttack status script
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 10, 100, 50, 0, 5.0, 0.0, 7.5, 7.5, Some(0.0), Some(7.5), Some(17.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            agent.on_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        }
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 44.0, 8.0);
    frame(lua_state, 32.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(lua_state, 44.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specials3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
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
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_green"), Hash40::new("sword1"), -0.0, 0, 0, 0, 0, 0, 1, true);    
    }
    frame(lua_state, 4.5);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_3lw_hdr"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_green"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_mc_aura_lw"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials3s", game_specials3s, Priority::Low);
    agent.acmd("game_specialairs3s", game_specials3s, Priority::Low);
    agent.acmd("effect_specials3s", effect_specials3s, Priority::Low);
    agent.acmd("effect_specialairs3s", effect_specials3s, Priority::Low);

    agent.acmd("game_specials3lw", game_specials3lw, Priority::Low);
    agent.acmd("game_specialairs3lw", game_specials3lw, Priority::Low);
    agent.acmd("effect_specials3lw", effect_specials3lw, Priority::Low);
    agent.acmd("effect_specialairs3lw", effect_specials3lw, Priority::Low);
}