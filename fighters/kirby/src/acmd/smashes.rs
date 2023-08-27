use super::*;

#[acmd_script( agent = "kirby", script = "game_attacks4hi" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_attack_s4_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCAS && VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 19.0, 361, 103, 0, 36, 4.2, 0.0, -7.5, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 14.0, 60, 98, 0, 42, 3.5, 0.0, -5.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            AttackModule::clear_all(boma);
        }
    } else {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 15.0, 361, 103, 0, 32, 4.2, 0.0, -7.5, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 11.0, 60, 98, 0, 40, 3.5, 0.0, -5.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    
}

#[acmd_script( agent = "kirby", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_attack_s4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCAS && VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 19.0, 361, 106, 0, 40, 4.2, 0.0, -7.5, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 14.0, 60, 98, 0, 42, 3.5, 0.0, -5.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            AttackModule::clear_all(boma);
        }
    } else {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 15.0, 361, 106, 0, 32, 4.2, 0.0, -7.5, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 11.0, 60, 98, 0, 40, 3.5, 0.0, -5.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }

}

#[acmd_script( agent = "kirby", script = "game_attacks4lw" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_attack_s4_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCAS && VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 19.0, 361, 106, 0, 40, 4.2, 0.0, -7.5, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 14.0, 60, 98, 0, 42, 3.5, 0.0, -5.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            AttackModule::clear_all(boma);
        }
    } else {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 15.0, 361, 106, 0, 32, 4.2, 0.0, -7.5, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footl"), 11.0, 60, 98, 0, 40, 3.5, 0.0, -5.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    
}

#[acmd_script( agent = "kirby", scripts = ["sound_attacks4hi", "sound_attacks4", "sound_attacks4lw"], category = ACMD_SOUND, low_priority )]
unsafe fn kirby_attack_s4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
    frame(lua_state, 8.0);
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_n04_ll"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_electric_hit_m"));
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_attack05"));
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_smash_s01"));
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_landing01"));
    }
}

#[acmd_script( agent = "kirby", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCAS && VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        frame(lua_state, 1.0);
        FT_MOTION_RATE_RANGE(fighter, 1.0, 9.0, 7.0);
        frame(lua_state, 9.0);
        FT_MOTION_RATE_RANGE(fighter, 9.0, 14.0, 3.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
        }
        frame(lua_state, 14.0);
        FT_MOTION_RATE(fighter, 1.0);
        if is_excute(fighter) {
            HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_XLU);
            ATTACK(fighter, 0, 0, Hash40::new("kneer"), 18.0, 75, 104, 0, 36, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("footr"), 18.0, 75, 104, 0, 36, 5.2, 1.5, -2.6, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("footr"), 17.0, 75, 104, 0, 36, 5.2, 1.5, -6.2, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("kneer"), 16.0, 84, 101, 0, 26, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("footr"), 16.0, 84, 101, 0, 26, 6.3, 0.0, -2.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("footr"), 15.0, 84, 101, 0, 26, 6.3, 0.0, -5.2, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("kneer"), 15.0, 40, 78, 0, 21, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("footr"), 15.0, 40, 78, 0, 21, 5.0, 0.0, -2.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("footr"), 14.0, 40, 78, 0, 21, 5.0, 0.0, -5.2, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            AttackModule::clear_all(boma);
            HIT_RESET_ALL(fighter);
        }
    } else {
        frame(lua_state, 1.0);
        FT_MOTION_RATE_RANGE(fighter, 1.0, 9.0, 7.0);
        frame(lua_state, 9.0);
        FT_MOTION_RATE_RANGE(fighter, 9.0, 14.0, 3.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 14.0);
        FT_MOTION_RATE(fighter, 1.0);
        if is_excute(fighter) {
            HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_XLU);
            ATTACK(fighter, 0, 0, Hash40::new("kneer"), 15.0, 75, 107, 0, 36, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("footr"), 15.0, 75, 107, 0, 36, 5.2, 1.5, -2.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("footr"), 14.0, 75, 107, 0, 36, 5.2, 1.5, -6.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("kneer"), 14.0, 88, 102, 0, 20, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("footr"), 14.0, 88, 102, 0, 20, 6.3, 0.0, -2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("footr"), 13.0, 88, 102, 0, 20, 6.3, 0.0, -5.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("kneer"), 13.0, 50, 52, 0, 10, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("footr"), 13.0, 50, 52, 0, 10, 5.0, 0.0, -2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("footr"), 12.0, 50, 52, 0, 10, 5.0, 0.0, -5.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            HIT_RESET_ALL(fighter);
        }
    }
    
}

#[acmd_script( agent = "kirby", script = "sound_attackhi4", category = ACMD_SOUND, low_priority )]
unsafe fn kirby_attack_hi4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
    frame(lua_state, 10.0);
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_n04_ll"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_electric_hit_m"));
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_attack06"));
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_smash_h01"));
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_landing01"));
    }
}

#[acmd_script( agent = "kirby", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCAS && VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        frame(lua_state, 1.0);
        FT_MOTION_RATE_RANGE(fighter, 1.0, 10.0, 6.0);
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_XLU);
        }
        frame(lua_state, 10.0);
        FT_MOTION_RATE(fighter, 1.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footr"), 16.0, 29, 89, 0, 28, 3.5, 0.0, -7.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("footl"), 16.0, 29, 89, 0, 28, 3.5, 0.0, -7.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 90, 70, 0, 80, 4.0, 0.0, 4.5, 1.0, Some(0.0), Some(4.5), Some(-1.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            ATTACK(fighter, 3, 0, Hash40::new("footr"), 16.0, 29, 89, 0, 28, 4.0, 0.0, -3.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            ATTACK(fighter, 4, 0, Hash40::new("footl"), 16.0, 29, 89, 0, 28, 4.0, 0.0, -3.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        frame(lua_state, 15.0);
        if is_excute(fighter) {
            HIT_RESET_ALL(fighter);
            ATK_POWER(fighter, 0, 13);
            ATK_POWER(fighter, 1, 13);
            ATK_POWER(fighter, 3, 14);
            ATK_POWER(fighter, 4, 14);
        }
        frame(lua_state, 22.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            AttackModule::clear_all(boma);
        }
    } else {
        frame(lua_state, 1.0);
        FT_MOTION_RATE_RANGE(fighter, 1.0, 10.0, 6.0);
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_XLU);
        }
        frame(lua_state, 10.0);
        FT_MOTION_RATE(fighter, 1.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("footr"), 14.0, 29, 92, 0, 25, 3.5, 0.0, -7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("footl"), 14.0, 29, 92, 0, 25, 3.5, 0.0, -7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 90, 40, 0, 80, 4.0, 0.0, 4.5, 1.0, Some(0.0), Some(4.5), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 3, 0, Hash40::new("footr"), 14.0, 29, 92, 0, 25, 4.0, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 4, 0, Hash40::new("footl"), 14.0, 29, 92, 0, 25, 4.0, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        frame(lua_state, 15.0);
        if is_excute(fighter) {
            HIT_RESET_ALL(fighter);
            ATK_POWER(fighter, 0, 10);
            ATK_POWER(fighter, 1, 10);
            ATK_POWER(fighter, 3, 10);
            ATK_POWER(fighter, 4, 10);
        }
        frame(lua_state, 22.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    
}

#[acmd_script( agent = "kirby", script = "sound_attacklw4", category = ACMD_SOUND, low_priority )]
unsafe fn kirby_attack_lw4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_n04_ll"));
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_electric_hit_m"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_attack07"));
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_smash_l01"));
    }
    wait(lua_state, 40.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_landing01"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        kirby_attack_s4_hi_game,
        kirby_attack_s4_game,
        kirby_attack_s4_lw_game,
        kirby_attack_s4_sound,
        kirby_attack_hi4_game,
        kirby_attack_hi4_sound,
        kirby_attack_lw4_game,
        kirby_attack_lw4_sound,
    );
}

