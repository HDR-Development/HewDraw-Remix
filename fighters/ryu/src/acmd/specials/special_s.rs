
use super::*;


#[acmd_script( agent = "ryu", script = "game_specialsstart" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 2.000);
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 1.0, 3.5, 8.5, 8.5);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if MeterModule::drain(fighter.battle_object, 1) {
                 VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
            }
        }
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
		}
		else {
            //ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 25, 0, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *COLLISION_SOUND_ATTR_NONE);
        }
		WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {  }
	frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }

}

#[acmd_script( agent = "ryu", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 5.5, 3.0, 9.0, 3.0);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        FT_MOTION_RATE(fighter, 1.000);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_INVINCIBLE);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
			MotionModule::set_rate(boma, 2.500);
			ATTACK(fighter, 0, 0, Hash40::new("legl"), 1.0, 367, 100, 70, 0, 7.0, 0.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("kneel"), 1.0, 367, 100, 70, 0, 7.0, 0.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		// EX Tatsumaki
		else {
			// Weak
			if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
				MotionModule::set_rate(boma,  1.250);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 67, 40, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
			// Medium
			if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
				MotionModule::set_rate(boma,  1.000);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 60, 67, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
			// Strong
			if ![*FIGHTER_RYU_STRENGTH_M, *FIGHTER_RYU_STRENGTH_W].contains(&WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH)) {
				MotionModule::set_rate(boma,  1.000);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 70, 71, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MotionModule::set_rate(boma, 2.500);
        }
         else {
            if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
                MotionModule::set_rate(boma, 1.250);
                ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 67, 40, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            }
            if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
                FT_MOTION_RATE(fighter, 1.000);
                ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 50, 49, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            }
            if ![*FIGHTER_RYU_STRENGTH_M, *FIGHTER_RYU_STRENGTH_W].contains(&WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH)) {
                FT_MOTION_RATE(fighter, 1.000);
                ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 56, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            }
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }

}

#[acmd_script( agent = "ryu", script = "effect_specials" , category = ACMD_EFFECT , low_priority)]
unsafe fn ryu_special_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(fighter, 200);
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ryu_savingattack_line_r"), Hash40::new("ryu_savingattack_line_l"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.1, false, *EF_FLIP_NONE);

            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if  WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND){
            if WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND){
                FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_ALPHA(fighter, 0.5);
            }
        }
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ryu_tatsumaki_smoke_r"), Hash40::new("ryu_tatsumaki_smoke_l"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.1, false, *EF_FLIP_NONE);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(fighter, 200);
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(fighter);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(fighter, 200);
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
}

#[acmd_script( agent = "ryu", script = "game_specialsend" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(fighter, 0, 0, Hash40::new("legl"), 10.0, 60, 80, 0, 55, 9.0, 0.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("kneel"), 10.0, 60, 80, 0, 55, 9.0, 0.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);

    }

}

#[acmd_script( agent = "ryu", script = "effect_specialsend" , category = ACMD_EFFECT , low_priority)]
unsafe fn ryu_special_s_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT(fighter, Hash40::new("ryu_savingattack_impact"), Hash40::new("top"), 14, 10.5, 3, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND){
            LANDING_EFFECT_FLIP(fighter, Hash40::new("ryu_tatsumaki_smoke_r"), Hash40::new("ryu_tatsumaki_smoke_l"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        }
    }
}

#[acmd_script( agent = "ryu", script = "game_specialairsstart" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 2.000);
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 1.0, 3.5, 8.5, 8.5);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if MeterModule::drain(fighter.battle_object, 1) {
                 VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
            }
        }
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
		}
		else {
            //ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 25, 0, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *COLLISION_SOUND_ATTR_NONE);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {  }frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }

}

#[acmd_script( agent = "ryu", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
			MotionModule::set_rate(boma, 2.500);
			ATTACK(fighter, 0, 0, Hash40::new("legl"), 1.0, 367, 100, 70, 0, 7.0, 0.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("kneel"), 1.0, 367, 100, 70, 0, 7.0, 0.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		// EX Tatsumaki
		else {
			// Weak
			if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
				MotionModule::set_rate(boma,  1.250);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 67, 40, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
			// Medium
			if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
				MotionModule::set_rate(boma,  1.000);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 60, 67, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
			// Strong
			if ![*FIGHTER_RYU_STRENGTH_M, *FIGHTER_RYU_STRENGTH_W].contains(&WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH)) {
				MotionModule::set_rate(boma,  1.000);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 70, 71, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MotionModule::set_rate(boma, 2.500);
        }
         else {
            if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
                MotionModule::set_rate(boma, 1.250);
                ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 67, 40, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            }
            if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
                FT_MOTION_RATE(fighter, 1.000);
                ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 50, 49, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            }
            if ![*FIGHTER_RYU_STRENGTH_M, *FIGHTER_RYU_STRENGTH_W].contains(&WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH)) {
                FT_MOTION_RATE(fighter, 1.000);
                ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 56, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            }
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }

}

#[acmd_script( agent = "ryu", script = "game_specialairsend" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_air_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(fighter, 0, 0, Hash40::new("legl"), 10.0, 60, 80, 0, 55, 9.0, 0.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("kneel"), 10.0, 60, 80, 0, 55, 9.0, 0.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }

}

#[acmd_script( agent = "ryu", script = "effect_specialairsend" , category = ACMD_EFFECT , low_priority)]
unsafe fn ryu_special_air_s_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT(fighter, Hash40::new("ryu_savingattack_impact"), Hash40::new("top"), 14, 10.5, 3, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script( agent = "ryu", script = "effect_specialairs" , category = ACMD_EFFECT , low_priority)]
unsafe fn ryu_special_air_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(fighter, 200);
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ryu_savingattack_line_r"), Hash40::new("ryu_savingattack_line_l"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.1, false, *EF_FLIP_NONE);

            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(fighter, 200);
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(fighter);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(fighter, 200);
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        ryu_special_s_start_game,
        ryu_special_s_game,
        ryu_special_s_effect,
        ryu_special_s_end_game,
        ryu_special_s_end_effect,
        ryu_special_air_s_start_game,
        ryu_special_air_s_game,
        ryu_special_air_s_effect,
        ryu_special_air_s_end_game,
        ryu_special_air_s_end_effect,
    );
}
