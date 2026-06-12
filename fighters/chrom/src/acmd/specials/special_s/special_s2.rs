use super::*;

unsafe extern "C" fn game_specials2lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    let speed_mul = if agent.is_situation(*SITUATION_KIND_GROUND) { 1.0 } else { 0.75 };
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.4 * speed_mul, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.7 * speed_mul, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.2 * speed_mul, 0);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(agent, 8.0, 20.0, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 5.0, 361, 100, 30, 0, 3.5,  0.0, 0.0, 0.0, None, None, None, 1.55, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("armr"),      5.0, 361, 100, 30, 0, 3.5,  0.0, 0.0, 0.0, None, None, None, 1.55, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("sword2"),    5.0, 361, 100, 30, 0, 4.0, -1.5, 0.0, 0.0, None, None, None, 1.55, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("sword2"),    5.0, 361, 100, 30, 0, 4.0, -1.5, 0.0, 5.0, None, None, None, 1.55, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        } else {
            // the animation is a bit different
            ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 5.0, 361, 100, 30, 0, 3.5,  0.0, 0.0, 0.0, None, None, None, 1.55, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("armr"),      5.0, 361, 100, 30, 0, 3.5,  0.0, 0.0, 0.0, None, None, None, 1.55, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("sword2"),    5.0, 361, 100, 30, 0, 4.0,  1.5, 0.0, 0.0, None, None, None, 1.55, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("sword2"),    5.0, 361, 100, 30, 0, 4.0,  1.5, 0.0, 5.0, None, None, None, 1.55, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        }
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn effect_specials2lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura"), Hash40::new("clavicler"), -0.0, 0, 0, 0, 0, 0, 2, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura"), Hash40::new("claviclel"), -0.0, 0, 0, 0, 0, 0, 2, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura"), Hash40::new("hip"), -0.0, 0, 0, 0, 0, 0, 2.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura"), Hash40::new("kneer"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura"), Hash40::new("kneel"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura"), Hash40::new("footr"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura"), Hash40::new("footl"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura"), Hash40::new("armr"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_aura"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_stormsword1"), Hash40::new("tex_chrom_stormsword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), 0.0, 0.0, 12.4, true, Hash40::new("chrom_sword_purple"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -3.5, 0, -90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        //LAST_PARTICLE_SET_COLOR(fighter, 0.4, 1, 0.3);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        //EFFECT_FOLLOW(fighter, Hash40::new_raw(0x0a4c6301c0), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        //EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new_raw(0x0fc6763e95), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_mc_aura"), false, true);
    }
}

unsafe extern "C" fn sound_specials2lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_chrom_attack03"));
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_smash_l01"));
    }
}

unsafe extern "C" fn expression_specials2lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials2lw", game_specials2lw, Priority::Low);
    agent.acmd("game_specialairs2lw", game_specials2lw, Priority::Low);
    agent.acmd("effect_specials2lw", effect_specials2lw, Priority::Low);
    agent.acmd("effect_specialairs2lw", effect_specials2lw, Priority::Low);
    agent.acmd("sound_specials2lw", sound_specials2lw, Priority::Low);
    agent.acmd("sound_specialairs2lw", sound_specials2lw, Priority::Low);
    agent.acmd("expression_specials2lw", expression_specials2lw, Priority::Low);
    agent.acmd("expression_specialairs2lw", expression_specials2lw, Priority::Low);
}