
use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut shatter_strike_speed = -0.5;
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.499);
        VarModule::off_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE);
        VarModule::off_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            VarModule::on_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
            VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.000);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            if MeterModule::drain(agent.battle_object, 2) {
                VarModule::on_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE);
            }
        }
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE){
            VarModule::off_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
            FT_MOTION_RATE(agent, 1.000);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }

    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE){
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 20.0);
            KineticModule::add_speed(boma, &Vector3f::new(shatter_strike_speed, 0.0, 0.0));
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE) {
            // Air-only
            ATTACK(agent, 0, 0, Hash40::new("legl"), 11.0, 361, 10, 0, 10, 4.5, 0.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneel"), 11.0, 361, 10, 0, 10, 4.5, 0.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("kneel"), 11.0, 361, 10, 0, 10, 4.5, 5.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("kneer"), 11.0, 361, 10, 0, 10, 3.5, 5.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Ground-only
            ATTACK(agent, 4, 0, Hash40::new("legl"), 11.0, 361, 10, 0, 10, 4.5, 0.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 0, Hash40::new("kneel"), 11.0, 361, 10, 0, 10, 4.5, 0.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 6, 0, Hash40::new("kneel"), 11.0, 361, 10, 0, 10, 4.5, 5.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 7, 0, Hash40::new("kneer"), 11.0, 361, 10, 0, 10, 3.5, 5.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_level(boma, 0, 3 as u8);
            AttackModule::set_attack_level(boma, 1, 3 as u8);
            AttackModule::set_attack_level(boma, 2, 3 as u8);
            AttackModule::set_attack_level(boma, 3, 3 as u8);
            AttackModule::set_attack_level(boma, 4, 3 as u8);
            AttackModule::set_attack_level(boma, 5, 3 as u8);
            AttackModule::set_attack_level(boma, 6, 3 as u8);
            AttackModule::set_attack_level(boma, 7, 3 as u8);
            AttackModule::set_add_reaction_frame(boma, 0, 60.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 60.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 60.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, 60.0, false);
            AttackModule::set_add_reaction_frame(boma, 4, 60.0, false);
            AttackModule::set_add_reaction_frame(boma, 5, 60.0, false);
            AttackModule::set_add_reaction_frame(boma, 6, 60.0, false);
            AttackModule::set_add_reaction_frame(boma, 7, 60.0, false);
            FT_MOTION_RATE(agent, 2.0/(18.0-13.0));
            // Add more negative speed to counteract the faster animation speed
            KineticModule::add_speed(boma, &Vector3f::new(2.5*shatter_strike_speed, 0.0, 0.0));
        }
        else{
            if VarModule::is_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {
                ATTACK(agent, 0, 0, Hash40::new("legl"), 8.0, 361, 40, 0, 70, 4.5, 0.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
                ATTACK(agent, 1, 0, Hash40::new("kneel"), 8.0, 361, 40, 0, 70, 4.5, 0.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
                ATTACK(agent, 2, 0, Hash40::new("kneel"), 8.0, 361, 40, 0, 70, 4.5, 5.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
                ATTACK(agent, 3, 0, Hash40::new("kneer"), 8.0, 361, 40, 0, 70, 4.5, 5.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("legl"), 16.0, 361, 85, 0, 50, 4.5, 0.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
                ATTACK(agent, 1, 0, Hash40::new("kneel"), 16.0, 361, 85, 0, 50, 4.5, 0.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
                ATTACK(agent, 2, 0, Hash40::new("kneel"), 16.0, 361, 85, 0, 50, 4.5, 5.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
                ATTACK(agent, 3, 0, Hash40::new("kneer"), 16.0, 361, 85, 0, 50, 3.5, 5.0, -3.0, 0.0, None, None, None, 1.25, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            }
        }

        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE){
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
            FT_MOTION_RATE(agent, 10.0/(20.0-18.0));
            // Counteract the added speed we had during the faster part of the animation
            KineticModule::add_speed(boma, &Vector3f::new(-2.5*shatter_strike_speed, 0.0, 0.0));
            // Add positive speed to keep the speed consistent through the slower part of the animation [anim is 80% slower <10/(20-18) = 0.2; 1-0.2 = 0.8> so counteract 80% of the originally added speed]
            KineticModule::add_speed(boma, &Vector3f::new(-0.8*shatter_strike_speed, 0.0, 0.0));
            // Add a tiny bit of forward speed
            KineticModule::add_speed(boma, &Vector3f::new(-0.2*shatter_strike_speed, 0.0, 0.0));
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE){
            // Counteract the added speed we had during the slower part of the animation, and the additional forward speed we added
            KineticModule::add_speed(boma, &Vector3f::new(0.8*shatter_strike_speed, 0.0, 0.0));
            KineticModule::add_speed(boma, &Vector3f::new(0.2*shatter_strike_speed, 0.0, 0.0));
            // Add positive speed to counteract the original added speed on frame 9 by 40%
            KineticModule::add_speed(boma, &Vector3f::new(-0.4*shatter_strike_speed, 0.0, 0.0));
            FT_MOTION_RATE(agent, 0.75);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE){

        }
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE){
            // Counteract 20% more of the remaining speed
            KineticModule::add_speed(boma, &Vector3f::new(-0.2*shatter_strike_speed, 0.0, 0.0));
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE){
            // Counteract the last portion of the originally added speed
            KineticModule::add_speed(boma, &Vector3f::new(-0.4*shatter_strike_speed, 0.0, 0.0));
            FT_MOTION_RATE(agent, 1.0);
        }
    }
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -5, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.45, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("dolly_drive_start0"), Hash40::new("top"), 0, 14.0, 4.0, 90, 0, 0, 0.95, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.05, 2.5);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE) {
            EFFECT_FOLLOW(agent, Hash40::new("dolly_drive_start0"), Hash40::new("top"), 0, 9.0, 6.0, 100, 0, 0, 0.75, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.05, 2.5);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE) {
            EFFECT_FOLLOW(agent, Hash40::new("dolly_drive_start0"), Hash40::new("top"), 0, 4.0, 6.0, 80, 0, 0, 1.25, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.05, 2.5);
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        else{
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_attack_arc6"), Hash40::new("dolly_attack_arc6"), Hash40::new("top"), 2, 12, 5, -9, -46, 32, 0.8, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 6.0);
    app::sv_animcmd::execute(lua_state, 6.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 4);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_SHATTER_STRIKE) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
        } else {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        VarModule::off_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
            VarModule::on_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {
            FT_MOTION_RATE(agent, 1.0);
        }
        else{
            FT_MOTION_RATE(agent, 2.0);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.000);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        FT_MOTION_RATE(agent, 1.0/(7.5-7.0));
    }
    frame(lua_state, 7.5);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0/(8.0-7.5));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.000);
        MeterModule::watch_damage(agent.battle_object, true);
         if VarModule::is_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {
            ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 11.0, 60, 40, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("armr"), 11.0, 60, 40, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 11.0, 60, 40, 0, 50, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.2, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 60, 40, 0, 50, 5.0, 0.0, 6.0, 9.5, None, None, None, 1.2, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
         }
         else {
            ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 16.0, 83, 90, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("armr"), 16.0, 83, 90, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 16.0, 83, 90, 0, 40, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.2, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 3, 0, Hash40::new("top"), 16.0, 83, 90, 0, 40, 5.0, 0.0, 7.0, 9.5, None, None, None, 1.2, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
         }
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {

         }
         else {
            ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 18.0, 85, 82, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("armr"), 18.0, 85, 82, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 18.0, 85, 82, 0, 40, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.2, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
         }
        AttackModule::clear(boma, 3, false);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut fire_kick_speed = 0.85;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::dolly::status::IS_USE_FIRE_KICK);
        VarModule::off_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
     }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 2.000);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            VarModule::on_flag(agent.battle_object, vars::dolly::status::IS_USE_FIRE_KICK);
            KineticModule::add_speed(boma, &Vector3f::new(fire_kick_speed, 0.0, 0.0));
         }

    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        FT_MOTION_RATE(agent, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_USE_FIRE_KICK) {
            ATTACK(agent, 0, 0, Hash40::new("legr"), 8.0, 70, 100, 65, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneer"), 8.0, 80, 100, 63, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("kneer"), 8.0, 91, 100, 61, 0, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        }
        else{
            ATTACK(agent, 0, 0, Hash40::new("legr"), 10.0, 35, 50, 0, 50, 4.0, 0.0, 0.0, 2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("legr"), 10.0, 35, 50, 0, 50, 4.0, 1.5, 0.0, 2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("kneer"), 12.0, 38, 82, 0, 35, 5.0, 0.0, 0.0, 2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("kneer"), 12.0, 38, 82, 0, 35, 5.0, 4.0, 0.0, 2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_USE_FIRE_KICK) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_13"), 2.0, 1.0, 1.0);
            }
        }
    }
    frame(lua_state, 13.0);
    for _ in 0..7 {
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_USE_FIRE_KICK) {
                KineticModule::add_speed(boma, &Vector3f::new(-0.125*fire_kick_speed, 0.0, 0.0));
            }
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -5, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::dolly::status::IS_USE_FIRE_KICK) {
            EFFECT_FLIP_ALPHA(agent, Hash40::new("dolly_attack_speedline2"), Hash40::new("dolly_attack_speedline2"), Hash40::new("top"), 3, 7, 8, -20, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.7);
            LAST_EFFECT_SET_RATE(agent, 1);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("legr"), 0, 0, -2.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 1.1);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("kneer"), 0, 0, -2.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 1.1);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("kneer"), 5.0, 0, -2.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 1.1);
        }
        else{
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("kneer"), 5.0, 0.0, 0.0, 0, 0, 0, 0.75, true, *EF_FLIP_YZ, 0.5);
            EFFECT_FLIP_ALPHA(agent, Hash40::new("dolly_attack_speedline2"), Hash40::new("dolly_attack_speedline2"), Hash40::new("top"), 3, 7, 8, -20, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.7);
            LAST_EFFECT_SET_RATE(agent, 1.0);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4);
    agent.acmd("effect_attacks4", effect_attacks4);
    agent.acmd("expression_attacks4", expression_attacks4);

    agent.acmd("game_attackhi4", game_attackhi4);

    agent.acmd("game_attacklw4", game_attacklw4);
    agent.acmd("effect_attacklw4", effect_attacklw4);
}
