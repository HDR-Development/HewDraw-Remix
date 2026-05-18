use super::*;

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.1);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15.0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 22.0);
    let rate = 10.0 / 16.0;
    FT_MOTION_RATE(agent, rate);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, rate * 0.9);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 5.0, 5.0, Some(0.0), Some(5.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 38.0);
    FT_MOTION_RATE(agent, 1.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.9);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::on_flag(agent.battle_object, vars::chrom::status::SPECIAL_LW_LEDGE_CANCEL);
    }
    frame(lua_state, 44.0);
    let rate = 26.0 / 32.0;
    FT_MOTION_RATE(agent, rate);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, rate * 0.9);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.1);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 22.0);
    let rate = 10.0 / 16.0;
    FT_MOTION_RATE(agent, rate);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, rate * 0.9);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 4.0, 5.0, Some(0.0), Some(4.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 38.0);
    FT_MOTION_RATE(agent, 1.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.9);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("chrom_counter_flash"), Hash40::new("sword1"), 0, 2, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        FLASH(agent, 1, 1, 1, 0.75);
    }

    // flashing effect
    frame(lua_state, 7.0);
    for _ in 0..3 {
        if is_excute(agent) {
            FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }

    // dash smoke
    frame(lua_state, 22.0);
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 10, -15, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.4);
        }
        for _ in 0..4 {
            if is_excute(agent) {
                FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
            }
            wait(lua_state, 4.0);
        }
    } else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 15, -12, 30, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.4);
        }
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_special_l01"));
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_escape"));
        PLAY_SE(agent, Hash40::new("se_chrom_swing_l"));
        PLAY_SE(agent, Hash40::new("se_chrom_final06"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_chrom_rnd_attack"));
    }
}

unsafe extern "C" fn game_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.1);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.5, 2.0);
    frame(lua_state, 4.5);
    FT_MOTION_RATE_RANGE(agent, 4.5, 5.0, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 14.0, 40, 80, 0, 75, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 18, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"),    14.0, 40, 80, 0, 75, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 18, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword2"),    14.0, 40, 80, 0, 75, 5.0, 0.0, 0.0, 5.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 18, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        // big fallback hitbox
        ATTACK(agent, 3, 0, Hash40::new("top"), 14.0, 40, 80, 0, 75, 8.5, 0.0, 8.0, 12.5, Some(0.0), Some(8.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 24, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.1);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
        if agent.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
}

unsafe extern "C" fn effect_speciallwhit(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_guard_mark"), true, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("chrom_counter_success"), Hash40::new("top"), 0, 14.8, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 14.8, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_stormsword1"), Hash40::new("tex_chrom_stormsword2"), 9, Hash40::new("sword1"), 0, 0, 1.7, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("chrom_sword_purple"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn effect_specialairlwhit(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_guard_mark"), true, true);
        EFFECT(agent, Hash40::new("chrom_counter_success"), Hash40::new("top"), 0, 14.8, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 14.8, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_stormsword1"), Hash40::new("tex_chrom_stormsword2"), 9, Hash40::new("sword1"), 0, 0, 1.7, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("chrom_sword_purple"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);
    agent.acmd("sound_speciallw", sound_speciallw, Priority::Low);
    agent.acmd("sound_specialairlw", sound_speciallw, Priority::Low);
    
    agent.acmd("game_speciallwhit", game_speciallwhit, Priority::Low);
    agent.acmd("game_specialairlwhit", game_speciallwhit, Priority::Low);
    agent.acmd("effect_speciallwhit", effect_speciallwhit, Priority::Low);
    agent.acmd("effect_specialairlwhit", effect_specialairlwhit, Priority::Low);
}