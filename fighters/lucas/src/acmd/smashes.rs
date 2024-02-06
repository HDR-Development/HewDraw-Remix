
use super::*;


unsafe extern "C" fn lucas_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        if is_excute(fighter) {
            VisibilityModule::set_int64(boma, Hash40::new("bat").hash as i64, Hash40::new("bat_visible").hash as i64);
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 9.0);
        if is_excute(fighter) {
            if fighter.stick_y() > 0.3 {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN);
                VarModule::on_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP);
            }
            else if fighter.stick_y() < -0.3 {
                VarModule::on_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP);
            }
            else {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP);
            }
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
        }
        frame(lua_state, 11.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
        }
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            //println!("Stick Y Pos: {} | Flags: Low={} High={}", fighter.stick_y(), VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN), VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP));
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 361, 89, 0, 54, 3.7, 0.0, 4.4, 9.8, Some(0.0), Some(6.6), Some(4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
                ATTACK(fighter, 1, 0, Hash40::new("top"), 20.0, 361, 92, 0, 54, 3.7, 0.0, 2.6, 15.0, Some(0.0), Some(4.4), Some(9.8), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BAT);
            }
            else if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 89, 0, 54, 3.7, 0.0, 9.2, 7.0, Some(0.0), Some(5.6), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
                ATTACK(fighter, 1, 0, Hash40::new("top"), 22.0, 361, 92, 0, 54, 3.7, 0.0, 12.6, 12.0, Some(0.0), Some(9.2), Some(7.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BAT);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 19.0, 361, 89, 0, 54, 3.7, 0.0, 5.6, 7.0, Some(0.0), Some(5.6), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
                ATTACK(fighter, 1, 0, Hash40::new("top"), 21.0, 361, 92, 0, 54, 3.7, 0.0, 5.6, 13.0, Some(0.0), Some(5.6), Some(7.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BAT);
            }
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN);
            VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP);
            WorkModule::on_flag(boma, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_END);
        }
    } 
    else {
        if is_excute(fighter) {
            VisibilityModule::set_int64(boma, Hash40::new("bat").hash as i64, Hash40::new("bat_visible").hash as i64);
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 9.0);
        if is_excute(fighter) {
            if fighter.stick_y() > 0.3 {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN);
                VarModule::on_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP);
            }
            else if fighter.stick_y() < -0.3 {
                VarModule::on_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP);
            }
            else {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP);
            }
        }
        frame(lua_state, 11.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
        }
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            //println!("Stick Y Pos: {} | Flags: Low={} High={}", fighter.stick_y(), VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN), VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP));
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 89, 0, 45, 3.7, 0.0, 4.1, 10.5, Some(0.0), Some(6.6), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCAS_BAT, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 361, 92, 0, 45, 3.7, 0.0, 2.6, 15.0, Some(0.0), Some(4.1), Some(10.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BAT);
            }
            else if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 361, 89, 0, 45, 3.7, 0.0, 9.7, 8.0, Some(0.0), Some(5.6), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCAS_BAT, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 361, 92, 0, 45, 3.7, 0.0, 12.6, 12.0, Some(0.0), Some(9.7), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BAT);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 89, 0, 45, 3.7, 0.0, 5.6, 8.0, Some(0.0), Some(5.6), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCAS_BAT, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 361, 92, 0, 45, 3.7, 0.0, 5.6, 13.0, Some(0.0), Some(5.6), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BAT);
            }
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN);
            VarModule::off_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP);
            WorkModule::on_flag(boma, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_END);
        }
    }
}


unsafe extern "C" fn lucas_attack_s4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(fighter){
        STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
        STOP_SE(fighter, Hash40::new("vc_lucas_008"));
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_s02"));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_n04_ll"));
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_electric_hit_l"));
        }
    }
    wait(lua_state, 4.0);
    if is_excute(fighter){
        PLAY_SE(fighter, Hash40::new("vc_lucas_attack04"));
    }
}


unsafe extern "C" fn lucas_attack_s4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.0, 8, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkt_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 1.6, true);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_smash_arc"), Hash40::new("lucas_smash_arc"), Hash40::new("top"), 1, 6, 3.5, 20, -20, 10, 1, true, *EF_FLIP_YZ);
        }
        else if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_smash_arc"), Hash40::new("lucas_smash_arc"), Hash40::new("top"), 1, 6, 3.5, -30, -20, 10, 1, true, *EF_FLIP_YZ);
        }
        else {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_smash_arc"), Hash40::new("lucas_smash_arc"), Hash40::new("top"), 1, 6, 3.5, 0, -20, 10, 1, true, *EF_FLIP_YZ);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN) {
            EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -2, 3, 15, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true, 0.6);
        }
        else if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP) {
            EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -2, 13, 13, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true, 0.6);
        }
        else {
            EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -2, 6.5, 14, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true, 0.6);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkt_hold"), false, false);
    }
}


unsafe extern "C" fn lucas_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        frame(lua_state, 5.0);
        app::sv_animcmd::execute(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 7.0);
        if is_excute(fighter) {
            //println!("Whiffchk starts");
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            MotionModule::set_rate(boma, 1.751);
        }
        frame(lua_state, 8.751);
        if is_excute(fighter) {
        }
        frame(lua_state, 15.0);
        if is_excute(fighter) {
            HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        }
        frame(lua_state, 26.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, 1.0);
        }
        frame(lua_state, 30.0);
        if is_excute(fighter) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 24.0, 95, 74, 0, 48, 12.0, 0.0, 25.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("head"), 24.0, 95, 74, 0, 48, 5.0, 3.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(lua_state, 43.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            MotionModule::set_rate(boma, 1.0);
            AttackModule::clear_all(boma);
        }
    }
    else {
        frame(lua_state, 5.0);
        app::sv_animcmd::execute(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 7.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, 1.751);
        }
        frame(lua_state, 15.0);
        if is_excute(fighter) {
            HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        }
        frame(lua_state, 26.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, 1.0);
        }
        frame(lua_state, 30.0);
        if is_excute(fighter) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
            ATTACK(fighter, 0, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 48, 9.0, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("head"), 21.0, 95, 77, 0, 48, 5.0, 3.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(lua_state, 33.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("throw"), 20.0, 95, 77, 0, 42, 8.0, 0.0, -2.5, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("head"), 20.0, 95, 77, 0, 42, 5.0, 3.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(lua_state, 38.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("throw"), 18.0, 95, 77, 0, 37, 7.0, 0.0, 2.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("head"), 18.0, 95, 77, 0, 37, 5.0, 3.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(lua_state, 43.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, 1.0);
            AttackModule::clear_all(boma);
        }
    }
}


unsafe extern "C" fn lucas_attack_hi4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkt_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 1.6, true);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("lucas_psi_hold"), Hash40::new("haver"), 0, 0, 0.3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), true, true);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_catch"), Hash40::new("lucas_psi_catch"), Hash40::new("top"), 0, 15, -0.5, 0, 0, 0, 1.5, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            let handle = EffectModule::req_follow(boma, Hash40::new("sys_crown"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(fighter.module_accessor, handle, 0.0 / 255.0, 204.0 / 255.0, 255.0 / 255.0);
        }
        else {
            LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            EFFECT(fighter, Hash40::new("lucas_psi_atk_hi"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            EFFECT(fighter, Hash40::new("lucas_psi_atk_hi"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_catch"), false, false);
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 1.6, true);
            LAST_EFFECT_SET_RATE(fighter, 0.33);
            EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1.0, true);
            EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkt_hold"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 2.0, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 15.0, 0, -90, 0, 0, 1.2, true);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkfr_bullet_ed"), true, true);
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 1.0, true);
        }
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_crown"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), true, true);
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkt_hold"), false, false);
    }
}


unsafe extern "C" fn lucas_attack_hi4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(fighter) {
	    STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
	    PLAY_SE(fighter, Hash40::new("se_lucas_smash_h01"));
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_n04_ll"));
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_electric_hit_l"));
        }
    }
    wait(lua_state, 18.0);
    if is_excute(fighter) {
	    PLAY_STATUS(fighter, Hash40::new("se_lucas_smash_h02"));
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
	    PLAY_SE(fighter, Hash40::new("vc_lucas_attack05"));
    }
    wait(lua_state, 56.0);
    if is_excute(fighter) {
	    PLAY_SE(fighter, Hash40::new("se_lucas_landing01"));
    }
}


unsafe extern "C" fn lucas_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            //println!("Whiffchk starts");
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 21.0, 361, 80, 0, 50, 8.0, 0.0, 2.0, 9.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        } 
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
        }
    }
    else {
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 361, 85, 0, 46, 6.0, 0.0, 3.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        wait(lua_state, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    }
}


unsafe extern "C" fn lucas_attack_lw4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l04"));
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_n04_ll"));
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_electric_hit_l"));
        }
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_attack07"));
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l01"));
    }
}


unsafe extern "C" fn lucas_attack_lw4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkt_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 1.6, true);
        }
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("lucas_psi_hold"), Hash40::new("havel"), 0.5, 0.5, 1.3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("top"), 0, 3.5, 9, 0, 0, 0, 0.85, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
            EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, 3.5, 9, 0, 0, 0, 0.5, true);
        } else {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 0, 2.0, 9, 0, 0, 0, 0.65, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 2, 2, 2, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkt_hold"), false, false);
    }
}


unsafe extern "C" fn lucas_attack_lw4_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 6.0); 
    if is_excute(fighter) {
        WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beamm"), 0, false, 0 as u32);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_RAD_arg9(fighter, 1, 0.1, 0.2, 3, 0.2, 9, 4, 18, 80);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(boma, 1);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

// LW4PT2


unsafe extern "C" fn lucas_attack_lw4_pt2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        frame(lua_state, 4.0);
        if is_excute(fighter) {
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 361, 80, 0, 50, 8.0, 0.0, 2.0, 9.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 5.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            AttackModule::clear_all(boma);
            AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        }
    }
    else {
        frame(lua_state, 4.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 361, 85, 0, 46, 6.0, 0.0, 3.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        }
    }
}


unsafe extern "C" fn lucas_attack_lw4_pt2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l03"));
    }
}


unsafe extern "C" fn lucas_attack_lw4_pt2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("top"), 0, 3.5, 9, 0, 0, 0, 0.85, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
            EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, 3.5, 9, 0, 0, 0, 0.5, true);
        } else {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 0, 2.0, 9, 0, 0, 0, 0.65, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkt_hold"), false, false);
    }
}


unsafe extern "C" fn lucas_attack_lw4_pt2_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beamm"), 0, false, 0 as u32);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_RAD_arg9(fighter, 1, 0.1, 0.2, 3, 0.2, 9, 4, 18, 80);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(boma, 1);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}




pub fn install() {
    smashline::Agent::new("lucas")
        .acmd("game_attacks4", lucas_attack_s4_s_game)
        .acmd("sound_attacks4", lucas_attack_s4_sound)
        .acmd("effect_attacks4", lucas_attack_s4_effect)
        .acmd("game_attackhi4", lucas_attack_hi4_game)
        .acmd("effect_attackhi4", lucas_attack_hi4_effect)
        .acmd("sound_attackhi4", lucas_attack_hi4_sound)
        .acmd("game_attacklw4", lucas_attack_lw4_game)
        .acmd("sound_attacklw4", lucas_attack_lw4_sound)
        .acmd("effect_attacklw4", lucas_attack_lw4_effect)
        .acmd("expression_attacklw4", lucas_attack_lw4_expression)
        .acmd("game_attacklw42", lucas_attack_lw4_pt2_game)
        .acmd("sound_attacklw42", lucas_attack_lw4_pt2_sound)
        .acmd("effect_attacklw42", lucas_attack_lw4_pt2_effect)
        .acmd("expression_attacklw42", lucas_attack_lw4_pt2_expression)
        .install();
}
