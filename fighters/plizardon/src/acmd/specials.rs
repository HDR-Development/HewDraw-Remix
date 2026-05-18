use super::*;


unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let pledge = if agent.kind() == *FIGHTER_KIND_KIRBY {
        VarModule::get_int(agent.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE)
    }
    else if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
        let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE)
    } else { *PLEDGE_STATE_NONE };
    frame(lua_state, 1.0);
    if pledge == *PLEDGE_STATE_WATER {
        FT_MOTION_RATE_RANGE(agent, 1.0, 20.0, 12.0);
    } else {
        FT_MOTION_RATE_RANGE(agent, 1.0, 20.0, 17.0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        let speed_y = if agent.is_situation(*SITUATION_KIND_GROUND) { 0.0 } else { 0.25 };
        SET_SPEED_EX(agent, -1.0, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if agent.kind() == *FIGHTER_KIND_KIRBY {
            if ![*PLEDGE_STATE_NONE, *PLEDGE_STATE_FIRE].contains(&VarModule::get_int(agent.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE)) {
                let timer = VarModule::get_int(agent.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
                VarModule::set_int(agent.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer - 180);
            }
        }
        else {
            if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
                let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
                let object = utils::util::get_battle_object_from_id(parent_id);
                if ![*PLEDGE_STATE_NONE, *PLEDGE_STATE_FIRE].contains(&VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE)) {
                    let timer = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
                    let pledge_use_cost_frame = ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.pledge_use_cost_frame");
                    VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer - pledge_use_cost_frame);
                }
            }
        }
    }
    if pledge == *PLEDGE_STATE_WATER {
        // Water Pledge
        if is_excute(agent) {
            ATTACK(agent, 0, 1, Hash40::new("top"), 13.0, 361, 100, 0, 40, 7.0, 0.0, 8.5, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"),  2.0,   0,   0, 0,  0, 8.0, 0.0, 8.5, 11.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 23.0);
        if is_excute(agent) {
            AttackModule::clear(boma, 1, false);
            ATTACK(agent, 0, 1, Hash40::new("top"), 13.0, 361, 100, 0, 40,  9.0, 0.0, 8.5, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"),  2.0,   0,   0, 0,  0, 10.0, 0.0, 8.5, 14.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            AttackModule::clear(boma, 1, false);
            ATTACK(agent, 0, 1, Hash40::new("top"), 11.0, 361, 100, 0, 40, 11.0, 0.0, 8.5, 17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"),  1.0,   0,   0, 0,  0, 12.0, 0.0, 8.5, 17.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 29.0);
        if is_excute(agent) {
            AttackModule::clear(boma, 1, false);
            ATTACK(agent, 0, 1, Hash40::new("top"), 11.0, 361, 100, 0, 40, 13.0, 0.0, 8.5, 20.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"),  1.0,   0,   0, 0,  0, 14.0, 0.0, 8.5, 20.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
    }
    else if pledge == *PLEDGE_STATE_GRASS {
        // Grass Pledge
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 7.0, 0.0, 8.5, 11.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 23.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 8.5, 0.0, 8.5, 14.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 10.0, 0.0, 8.5, 18.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 29.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 100, 0, 60, 11.5, 0.0, 8.5, 21.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 1, Hash40::new("top"), 11.0, 361, 100, 0, 40, 7.0, 0.0, 8.5, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"),  2.0,   0,   0, 0,  0, 8.0, 0.0, 8.5, 11.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 23.0);
        if is_excute(agent) {
            AttackModule::clear(boma, 1, false);
            ATTACK(agent, 0, 1, Hash40::new("top"), 11.0, 361, 100, 0, 40, 7.5, 0.0, 8.5, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"),  2.0,   0,   0, 0,  0, 8.5, 0.0, 8.5, 13.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            AttackModule::clear(boma, 1, false);
            ATTACK(agent, 0, 1, Hash40::new("top"), 7.0, 361, 100, 0, 40, 8.0, 0.0, 8.5, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.0,   0,   0, 0,  0, 9.0, 0.0, 8.5, 15.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 29.0);
        if is_excute(agent) {
            AttackModule::clear(boma, 1, false);
            ATTACK(agent, 0, 1, Hash40::new("top"), 7.0, 361, 100, 0, 40, 8.5, 0.0, 8.5, 17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.0,   0,   0, 0,  0, 9.5, 0.0, 8.5, 17.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 32.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn effect_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let pledge = if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
        let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE)
    } else { *PLEDGE_STATE_NONE };
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.8, 0.6, 0.3);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.2, 0, 0.5);
        FLASH_FRM(agent, 15, 0, 0, 0, 0);
    }
    frame(lua_state, 7.0);
    if pledge == *PLEDGE_STATE_WATER {
        if is_excute(agent) {
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_steam2"), Hash40::new("head"), 0, 5, 0, 0, 0, 0, 0.7, false, 3.0);
            EFFECT_FOLLOW(agent, Hash40::new("sys_drown_out"), Hash40::new("mouth2"), 0, 0, 0, 180, 0, 0, 0.6, false);
        }
    }
    else if pledge == *PLEDGE_STATE_GRASS {
        let mut handle = 0;
        for _ in 0..4 {
            if is_excute(agent) {
                handle = EffectModule::req_follow(boma, Hash40::new("pfushigisou_atk_hi4"), Hash40::new("mouth2"), &Vector3f::zero(), &Vector3f::zero(), 0.0, false, 0, 0, 0, 0, 0, false, false);
                EffectModule::set_rate(boma, handle as u32, 9.0);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EffectModule::set_scale(boma, handle as u32, &Vector3f::new(0.45, 0.45, 0.45));
                EffectModule::set_rate(boma, handle as u32, 0.5);
                EffectModule::detach(boma, handle as u32, 0);
            }
            wait(lua_state, 3.0);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("plizardon_flare_blitz_hold"), Hash40::new("head"), -3, 5, 0, 0, 0, 0, 0.3, true);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        let mut flame_color = Vector3f::new(1.0, 1.0, 1.0);
        let mut flame_size = 1.0;
        let mut flame_alpha = 1.0;
        let mut flame_rate = 2.0;
        let mut speedline_alpha_mul = 0.8;
        let mut speedline_color_main = Vector3f::new(0.6, 0.1, 0.0);
        let mut speedline_color_sub = Vector3f::new(0.9, 0.4, 0.0);
        if pledge == *PLEDGE_STATE_WATER {
            EFFECT_OFF_KIND(agent, Hash40::new("sys_drown_out"), false, false);
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_steam"), Hash40::new("sys_steam"), Hash40::new("top"), -1, 5, 20, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 5.0);
            LAST_EFFECT_SET_SCALE_W(agent, 1.8, 1.5, 1.8);
            LAST_EFFECT_SET_COLOR(agent, 0.7, 0.7, 0.74);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 8, 16, 0, 90, 0, 2, false, 1.0);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 3.0);
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_hit_fire"), Hash40::new("top"), 0, 8, 16, 0, 90, 0, 0.5, false, 0.8);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 3.0);

            flame_color = Vector3f::new(0.6, 1.0, 6.0);
            flame_size = 1.1;
            flame_alpha = 0.85;
            speedline_alpha_mul = 0.6;
            speedline_color_main = Vector3f::new(0.25, 0.3, 0.5);
            speedline_color_sub = Vector3f::new(0.6, 0.1, 0.0);
        }
        else if pledge == *PLEDGE_STATE_GRASS {
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_steam"), Hash40::new("sys_steam"), Hash40::new("top"), -1, 5, 19, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 1.2);
            LAST_EFFECT_SET_COLOR(agent, 2.0, 3.0, 0.5);
            LAST_EFFECT_SET_RATE(agent, 0.7);
            EFFECT_FLIP(agent, Hash40::new("sys_grass_landing"), Hash40::new("sys_grass_landing"), Hash40::new("top"), -1, 5, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(agent, 4.0, 3.0, 1.0);
            LAST_EFFECT_SET_RATE(agent, 0.4);
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 8, 16, 0, 90, 0, 2, false, 0.8);
            LAST_EFFECT_SET_COLOR(agent, 0.6, 1.8, 1.0);

            flame_color = Vector3f::new(0.3, 0.9, 0.5);
            flame_size = 1.1;
            flame_alpha = 0.7;
            flame_rate = 1.0;
            speedline_alpha_mul = 0.6;
            speedline_color_main = Vector3f::new(0.7, 0.2, 0.0);
            speedline_color_sub = Vector3f::new(0.4, 0.3, 0.1);
        }

        EFFECT_OFF_KIND(agent, Hash40::new("plizardon_flare_blitz_hold"), false, false);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("plizardon_atk_mouth_fire"), Hash40::new("plizardon_atk_mouth_fire"), Hash40::new("head"), -1, 2, 0, 0, 180, 35, flame_size, true, *EF_FLIP_ROT_X, flame_alpha);
        LAST_EFFECT_SET_SCALE_W(agent, 1.2, 1.9, 1.2);
        LAST_EFFECT_SET_COLOR(agent, flame_color.x, flame_color.y, flame_color.z);
        LAST_EFFECT_SET_RATE(agent, flame_rate);

        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.5 * speedline_alpha_mul);
        LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.3, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.3);
        LAST_EFFECT_SET_COLOR(agent, speedline_color_main.x, speedline_color_main.y, speedline_color_main.z);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.4 * speedline_alpha_mul);
        LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.2, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.3);
        LAST_EFFECT_SET_COLOR(agent, speedline_color_sub.x, speedline_color_sub.y, speedline_color_sub.z);
        if [*PLEDGE_STATE_NONE, *PLEDGE_STATE_FIRE].contains(&pledge) {
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.2 * speedline_alpha_mul);
            LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.1, 1.0);
            LAST_EFFECT_SET_RATE(agent, 0.3);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_hit_fire"), 0);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        if pledge == *PLEDGE_STATE_GRASS {
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("pfushigisou_atk_hi4"), Hash40::new("top"), 0, 8.5, 21.5, 0, 0, 90, 0.6, false, 0.4);
        }
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        let mut speedline_color_main = Vector3f::new(0.6, 0.1, 0.0);
        let mut speedline_color_sub = Vector3f::new(0.9, 0.4, 0.0);
        if pledge == *PLEDGE_STATE_WATER {
            speedline_color_main = Vector3f::new(0.25, 0.3, 0.5);
            speedline_color_sub = Vector3f::new(0.6, 0.1, 0.0);
        }
        else if pledge == *PLEDGE_STATE_GRASS {
            speedline_color_main = Vector3f::new(0.7, 0.2, 0.0);
            speedline_color_sub = Vector3f::new(0.4, 0.3, 0.1);
        }
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.5 * 0.6);
        LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.2, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.3);
        LAST_EFFECT_SET_COLOR(agent, speedline_color_main.x, speedline_color_main.y, speedline_color_main.z);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.4 * 0.6);
        LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.1, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.3);
        LAST_EFFECT_SET_COLOR(agent, speedline_color_sub.x, speedline_color_sub.y, speedline_color_sub.z);
        if [*PLEDGE_STATE_NONE, *PLEDGE_STATE_FIRE].contains(&pledge) {
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.2 * 0.6);
            LAST_EFFECT_SET_SCALE_W(agent, 2.0, 1.0, 1.0);
            LAST_EFFECT_SET_RATE(agent, 0.3);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("plizardon_atk_mouth_fire"), false, false); 
        EFFECT_OFF_KIND(agent, Hash40::new("sys_steam2"), false, false); 
    }
}

unsafe extern "C" fn sound_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_common_c_fire_l_short_02"));
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("vc_plizardon_appeal02"));
    }
    let pledge = if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
        let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE)
    } else { *PLEDGE_STATE_NONE };
    if pledge == *PLEDGE_STATE_WATER {
        frame(lua_state, 20.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_common_bomb_s"));
        }
    }
    else if pledge == *PLEDGE_STATE_GRASS {
        frame(lua_state, 23.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_common_bomb_s"));
        }
    }
}

unsafe extern "C" fn expression_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    let pledge = if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
        let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE)
    } else { *PLEDGE_STATE_NONE };
    if pledge != *PLEDGE_STATE_GRASS {
        frame(lua_state, 20.0);
        if is_excute(agent) {
            if pledge == *PLEDGE_STATE_WATER {
                QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            }
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
        frame(lua_state, 23.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
        frame(lua_state, 29.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        }
    }
    else {
        frame(lua_state, 20.0);
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        }
        frame(lua_state, 23.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        }
        frame(lua_state, 29.0);
        if is_excute(agent) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 21.0, 17.0);
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 21.0, 361, 70, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"),  21.0, 361, 70, 0, 50, 4.5, 0.0, 6.5, 4.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_kick_hit_l"));
        AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_kick_hit_l"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 14.0, 361, 82, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"),  14.0, 361, 82, 0, 50, 4.5, 0.0, 6.5, 4.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BODY);
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_kick_hit_l"));
        AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_kick_hit_l"));
    }
}

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        KineticModule::mul_speed(boma, &Vector3f::new(0.8, 1.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_STOP);
        DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("neck"), 21.0, 46, 70, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"),  21.0, 46, 70, 0, 50, 4.5, 0.0, 6.5, 4.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 14.0, 361, 82, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"),  14.0, 361, 82, 0, 50, 4.5, 0.0, 6.5, 4.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn sound_specialsblown(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if !SoundModule::is_playing(boma, Hash40::new("se_plizardon_special_s02")) {
            PLAY_SE(agent, Hash40::new("vc_plizardon_special_s01"));
            PLAY_SE(agent, Hash40::new("se_plizardon_special_s02"));
            SET_PLAY_INHIVIT(agent, Hash40::new("se_plizardon_special_s02"), 20);
        }
    }
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 15.0, 1.0);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 20.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn game_specialairsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    FT_MOTION_RATE_RANGE(agent, 7.0, 47.0, 27.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("plizardon_flare_blitz"), false, false);
        EFFECT_FLW_POS(agent, Hash40::new("plizardon_flare_blitz_smoke"), Hash40::new("rot"), 0, 2, 15, -90, 0, 0, 1.3, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("plizardon_flare_blitz_smoke"), -1);
    }
    for _ in 0..7 {
        if is_excute(agent) {
            BURN_COLOR(agent, 2, 0.1, 0, 0.5);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            BURN_COLOR_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, 1);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 100, 100, 120, 0, 5.0, 0.0, 5.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 70, 100, 80, 0, 4.5, 0.0, 14.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 65, 100, 115, 0, 6.0, 0.0, 5.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 75, 83, 0, 45, 4.8, 0.0, 11.0, -0.5, Some(0.0), Some(7.0),  Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 75, 83, 0, 45, 2.8, 0.0, 10.0,  5.5, Some(0.0), Some(6.0),  Some(4.0),  1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 14.0, 75, 83, 0, 45, 2.8, 0.0, 16.0,  2.0, Some(0.0), Some(14.0), Some(1.0),  1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 14.0, 75, 83, 0, 45, 3.0, 0.0, 12.5, -4.5, Some(0.0), Some(8.5),  Some(-6.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        sv_kinetic_energy!(reset_energy, agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);
        let fall_x_mul = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("fall_x_mul"));
        sv_kinetic_energy!(set_stable_speed, agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * fall_x_mul, 0.0);
    }
}

unsafe extern "C" fn game_speciallwin(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_OFF), 0);
        if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
            let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
            let object = utils::util::get_battle_object_from_id(parent_id);
            VarModule::on_flag(object, vars::ptrainer::instance::SPECIAL_LW_BACKWARDS_SWITCH); // we will turn this off in opff
            //if VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE) == *PLEDGE_STATE_NONE {
                VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, *PLEDGE_STATE_FIRE);
                let pledge_duration_frame = ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.pledge_duration_frame");
                VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, pledge_duration_frame);
            //}
            let swap_lockout_frame = ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.swap_lockout_frame");
            VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER, swap_lockout_frame);
            VarModule::on_flag(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialairnstart", game_specialnstart, Priority::Low);
    agent.acmd("effect_specialnstart", effect_specialnstart, Priority::Low);
    agent.acmd("effect_specialairnstart", effect_specialnstart, Priority::Low);
    agent.acmd("sound_specialnstart", sound_specialnstart, Priority::Low);
    agent.acmd("sound_specialairnstart", sound_specialnstart, Priority::Low);
    agent.acmd("expression_specialnstart", expression_specialnstart, Priority::Low);
    agent.acmd("expression_specialairnstart", expression_specialnstart, Priority::Low);

    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart, Priority::Low);

    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairs", game_specials, Priority::Low);
    agent.acmd("sound_specialsblown", sound_specialsblown, Priority::Low);
    agent.acmd("sound_specialairsblown", sound_specialsblown, Priority::Low);
    agent.acmd("game_specialsend", game_specialsend, Priority::Low);
    agent.acmd("game_specialairsend", game_specialairsend, Priority::Low);
    agent.acmd("effect_specialsend", effect_specialsend, Priority::Low);
    agent.acmd("effect_specialairsend", effect_specialsend, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);

    agent.acmd("game_speciallwin", game_speciallwin, Priority::Low);
    agent.acmd("game_specialairlwin", game_speciallwin, Priority::Low);
}