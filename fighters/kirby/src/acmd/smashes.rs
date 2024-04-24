use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let copy_lucas = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCAS
    && VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        if copy_lucas { VarModule::on_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF); };
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        let damage = if copy_lucas { 4.0 } else { 0.0 };
        let bkb = if copy_lucas { 4 } else { 0 };
        let collision_attr = if copy_lucas { Hash40::new("collision_attr_elec") } else { Hash40::new("collision_attr_normal") };
        let collision_sfx = if copy_lucas { *COLLISION_SOUND_ATTR_ELEC } else { *COLLISION_SOUND_ATTR_KICK };
        ATTACK(agent, 0, 0, Hash40::new("footl"), 15.0 + damage, 361, 106, 0, 36 + bkb, 4.2, 0.0, -7.5, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, collision_sfx, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        let damage = if copy_lucas { 3.0 } else { 0.0 };
        let bkb = if copy_lucas { 2 } else { 0 };
        let collision_attr = if copy_lucas { Hash40::new("collision_attr_elec") } else { Hash40::new("collision_attr_normal") };
        let collision_sfx = if copy_lucas { *COLLISION_SOUND_ATTR_ELEC } else { *COLLISION_SOUND_ATTR_KICK };
        ATTACK(agent, 0, 0, Hash40::new("footl"), 11.0 + damage, 60, 98, 0, 40 + bkb, 3.5, 0.0, -5.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_M, collision_sfx, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if copy_lucas {
            if VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
        }
    }
}

unsafe extern "C" fn sound_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCAS && VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            PLAY_SE_REMAIN(agent, Hash40::new("se_lucas_special_n04_ll"));
            PLAY_SE_REMAIN(agent, Hash40::new("se_common_electric_hit_m"));
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_attack05"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_smash_s01"));
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_landing01"));
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let copy_lucas = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCAS
    && VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 9.0, 7.0);
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(agent, 9.0, 14.0, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        if copy_lucas {
            VarModule::on_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
        }
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_XLU);
        let damage_foot = if copy_lucas { 18.0 } else { 15.0 };
        let damage_body = if copy_lucas { 18.0 } else { 14.0 };
        let hitlag = if copy_lucas { 1.1 } else { 1.0 };
        let collision_attr = if copy_lucas { Hash40::new("collision_attr_elec") } else { Hash40::new("collision_attr_cutup") };
        let collision_sfx = if copy_lucas { *COLLISION_SOUND_ATTR_ELEC } else { *COLLISION_SOUND_ATTR_KICK };
        ATTACK(agent, 0, 0, Hash40::new("kneer"), damage_foot, 75, 107, 0, 36, 5.2, 0.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, collision_sfx, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("footr"), damage_foot, 75, 107, 0, 36, 5.2, 1.5, -2.6, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, collision_sfx, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footr"), damage_body, 75, 107, 0, 36, 5.2, 1.5, -6.2, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, collision_sfx, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        let damage_foot = if copy_lucas { 16.0 } else { 14.0 };
        let damage_body = if copy_lucas { 16.0 } else { 13.0 };
        let hitlag = if copy_lucas { 1.1 } else { 1.0 };
        let collision_attr = if copy_lucas { Hash40::new("collision_attr_elec") } else { Hash40::new("collision_attr_normal") };
        let collision_sfx = if copy_lucas { *COLLISION_SOUND_ATTR_ELEC } else { *COLLISION_SOUND_ATTR_KICK };
        ATTACK(agent, 0, 0, Hash40::new("kneer"), damage_foot, 88, 102, 0, 20, 6.3, 0.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_M, collision_sfx, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("footr"), damage_foot, 88, 102, 0, 20, 6.3, 0.0, -2.5, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_M, collision_sfx, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footr"), damage_body, 88, 102, 0, 20, 6.3, 0.0, -5.2, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_M, collision_sfx, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        let damage_foot = if copy_lucas { 15.0 } else { 13.0 };
        let damage_body = if copy_lucas { 15.0 } else { 12.0 };
        let kbg = if copy_lucas { 78 } else { 52 };
        let collision_attr = if copy_lucas { Hash40::new("collision_attr_elec") } else { Hash40::new("collision_attr_normal") };
        let collision_sfx = if copy_lucas { *COLLISION_SOUND_ATTR_ELEC } else { *COLLISION_SOUND_ATTR_KICK };
        ATTACK(agent, 0, 0, Hash40::new("kneer"), damage_foot, 50, kbg, 0, 10, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_M, collision_sfx, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("footr"), damage_foot, 50, kbg, 0, 10, 5.0, 0.0, -2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_M, collision_sfx, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footr"), damage_body, 50, kbg, 0, 10, 5.0, 0.0, -5.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_M, collision_sfx, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_RESET_ALL(agent);
        if copy_lucas {
            if VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
        }
    }
}

unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCAS && VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            PLAY_SE_REMAIN(agent, Hash40::new("se_lucas_special_n04_ll"));
            PLAY_SE_REMAIN(agent, Hash40::new("se_common_electric_hit_m"));
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_attack06"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_smash_h01"));
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_landing01"));
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let copy_lucas = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCAS
    && VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 10.0, 6.0);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("toel"), *HIT_STATUS_XLU);
        if copy_lucas { VarModule::on_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF); }
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let damage_foot = if copy_lucas { 16.0 } else { 14.0 };
        let bkb_foot = if copy_lucas { 28 } else { 25 };
        let kbg_foot = if copy_lucas { 89 } else { 92 };
        let damage_body = if copy_lucas { 15.0 } else { 10.0 };
        let bkb_body = if copy_lucas { 80 } else { 80 };
        let kbg_body = if copy_lucas { 70 } else { 40 };
        let hitlag = if copy_lucas { 1.15 } else { 1.0 };
        let collision_attr = if copy_lucas { Hash40::new("collision_attr_elec") } else { Hash40::new("collision_attr_normal") };
        let collision_sfx = if copy_lucas { *COLLISION_SOUND_ATTR_ELEC } else { *COLLISION_SOUND_ATTR_KICK };
        ATTACK(agent, 0, 0, Hash40::new("top"), damage_foot, 29, kbg_foot, 0, bkb_foot, 3.5, 0.0, 3.0, 9.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, collision_sfx, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), damage_foot, 29, kbg_foot, 0, bkb_foot, 3.5, 0.0, 3.0, -9.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, collision_sfx, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), damage_body, 90, kbg_body, 0, bkb_body, 4.0, 0.0, 4.5, 1.0, Some(0.0), Some(4.5), Some(-1.0), hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, collision_sfx, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("top"), damage_foot, 29, kbg_foot, 0, bkb_foot, 3.5, 0.0, 3.0, 4.5, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, collision_sfx, *ATTACK_REGION_KICK);
        ATTACK(agent, 4, 0, Hash40::new("top"), damage_foot, 29, kbg_foot, 0, bkb_foot, 3.5, 0.0, 3.0, -4.5, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, collision_sfx, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        HIT_RESET_ALL(agent);
        let damage = if copy_lucas { 3 } else { 0 };
        ATK_POWER(agent, 0, 10 + damage);
        ATK_POWER(agent, 1, 10 + damage);
        ATK_POWER(agent, 3, 10 + damage);
        ATK_POWER(agent, 4, 10 + damage);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if copy_lucas {
            if VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
        }
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 2, -5, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 2, 5, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("kirby_smash_arc_b"), Hash40::new("kirby_smash_arc_b"), Hash40::new("top"), 0, 1.5, -3, 0, 320, 180, 1.3, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("kirby_smash_arc_b"), Hash40::new("kirby_smash_arc_b"), Hash40::new("top"), 0, 1.5, 2, 0, 140, 180, 1.2, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("kirby_smash_arc_b"), Hash40::new("kirby_smash_arc_b"), Hash40::new("top"), 0, 1.5, -3, 0, 320, 180, 1.3, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("kirby_smash_arc_b"), Hash40::new("kirby_smash_arc_b"), Hash40::new("top"), 0, 1.5, 2, 0, 140, 180, 1.2, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn sound_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
        if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCAS && VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            PLAY_SE_REMAIN(agent, Hash40::new("se_lucas_special_n04_ll"));
            PLAY_SE_REMAIN(agent, Hash40::new("se_common_electric_hit_m"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_attack07"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_smash_l01"));
    }
    wait(lua_state, 40.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_landing01"));
    }
}

unsafe extern "C" fn expression_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(lua_state, 5.0);
    app::sv_animcmd::execute(lua_state, 5.0);
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 7);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4);
    agent.acmd("game_attacks4hi", game_attacks4);
    agent.acmd("game_attacks4lw", game_attacks4);
    agent.acmd("sound_attacks4", sound_attacks4);
    agent.acmd("sound_attacks4hi", sound_attacks4);
    agent.acmd("sound_attacks4lw", sound_attacks4);

    agent.acmd("game_attackhi4", game_attackhi4);
    agent.acmd("sound_attackhi4", sound_attackhi4);

    agent.acmd("game_attacklw4", game_attacklw4);
    agent.acmd("effect_attacklw4", effect_attacklw4);
    agent.acmd("sound_attacklw4", sound_attacklw4);
    agent.acmd("expression_attacklw4", expression_attacklw4);
}
