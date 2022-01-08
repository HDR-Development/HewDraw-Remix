
use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::utility::*;
use smash_script::*;
use smashline::*;
use smashline::*;
use smash_script::macros::*;
use crate::utils::hdr;
use crate::vars::*;
use crate::hooks::sys_line::meter::*;


#[acmd_script( agent = "dolly", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn dolly_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        magic_series_cancel[hdr::get_player_number(boma)] = false;
        fire_kick[hdr::get_player_number(boma)] = false;
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 10, 0, 27, 4.5, 0.0, 12.6, 4.5, Some(0.0), Some(5.0), Some(4.5), 2.25, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 180, 10, 0, 27, 4.5, 0.0, 12.6, 11.0, Some(0.0), Some(5.0), Some(11.0), 2.25, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 361, 10, 0, 27, 4.5, 0.0, 12.6, 2.5, Some(0.0), Some(5.0), Some(2.5), 2.25, 1.15, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 4.0, false);
        FT_MOTION_RATE(fighter, 1.000);
        meter_gain(boma, 4);
     }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
    
}

#[acmd_script( agent = "dolly", script = "game_attack12" , category = ACMD_GAME , low_priority)]
unsafe fn dolly_attack_12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 180, 15, 0, 28, 5.0, 0.0, 10.0, 9.5, None, None, None, 2.0, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 190, 15, 0, 25, 5.0, 0.0, 10.0, 9.5, None, None, None, 2.0, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 15, 0, 15, 4.5, 0.0, 10.0, 6.5, None, None, None, 2.0, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        meter_gain(boma, 4);
     }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 180, 15, 0, 35, 5.0, 0.0, 11.0, 12.0, None, None, None, 2.5, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 190, 15, 0, 35, 5.0, 0.0, 11.0, 12.0, None, None, None, 2.5, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 15, 0, 25, 4.5, 0.0, 12.0, 10.0, None, None, None, 2.5, 0.85, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 6.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 2.0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    
}

#[acmd_script( agent = "dolly", script = "game_attack13" , category = ACMD_GAME , low_priority)]
unsafe fn dolly_attack_13_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if fire_kick[hdr::get_player_number(boma)] {
            speed_vector_1[hdr::get_player_number(boma)] = Vector3f { x: 1.5, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &speed_vector_1[hdr::get_player_number(boma)]);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		/*
        if fire_kick[hdr::get_player_number(boma)] {
            speed_vector_1[hdr::get_player_number(boma)] = Vector3f { x: -1.5, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &speed_vector_1[hdr::get_player_number(boma)]);
        }
		*/
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 65, 70, 0, 60, 5.0, 0.0, 4.0, 4.0, Some(0.0), Some(16.0), Some(11.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 65, 70, 0, 60, 4.0, 0.0, 11.0, 3.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        meter_gain(boma, 5);
         if fire_kick[hdr::get_player_number(boma)] {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 78, 95, 0, 80, 7.0, 0.0, 4.0, 4.0, Some(0.0), Some(18.0), Some(12.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_KICK);
            meter_gain(boma, 8);
         }
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
		/*
        if fire_kick[hdr::get_player_number(boma)] {
            speed_vector_1[hdr::get_player_number(boma)] = Vector3f {x: -1.5, y: 0.0, z: 0.0};
            KineticModule::add_speed(boma, &speed_vector_1[hdr::get_player_number(boma)]);
        }
		*/
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    
}

#[acmd_script( agent = "dolly", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn dolly_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        heavy_attack[hdr::get_player_number(boma)] = false;
        ex_special[hdr::get_player_number(boma)] = false;
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            heavy_attack[hdr::get_player_number(boma)] = true;
         }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if heavy_attack[hdr::get_player_number(boma)] {
            FT_MOTION_RATE(fighter, 2.667);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if heavy_attack[hdr::get_player_number(boma)] {
            if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                if get_meter_level(boma) >= 2 {
                    use_meter_level(fighter, boma, 2);
                    ex_special[hdr::get_player_number(boma)] = true;
                }
            }
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 70, 48, 0, 73, 5.0, 0.0, 10.0, 3.0, Some(0.0), Some(6.0), Some(3.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
        meter_gain(boma, 11);
         if heavy_attack[hdr::get_player_number(boma)] {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 55, 88, 0, 65, 5.0, 0.0, 10.0, 3.0, Some(0.0), Some(6.0), Some(3.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
            meter_gain(boma, 15);
         }
        if ex_special[hdr::get_player_number(boma)] {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 5, 100, 50, 0, 5.0, 0.0, 10.0, 3.0, Some(0.0), Some(6.0), Some(3.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
            meter_gain(boma, 6);
         }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 75, 63, 0, 65, 4.0, 0.0, 10.0, 4.0, Some(0.0), Some(6.0), Some(4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
        meter_gain(boma, 9);
         if heavy_attack[hdr::get_player_number(boma)] {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 75, 73, 0, 65, 4.0, 0.0, 10.0, 4.0, Some(0.0), Some(6.0), Some(4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
            meter_gain(boma, 11);
         }
        if ex_special[hdr::get_player_number(boma)] {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 5, 100, 50, 0, 5.0, 0.0, 10.0, 3.0, Some(0.0), Some(6.0), Some(3.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
            meter_gain(boma, 6);
         }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if ex_special[hdr::get_player_number(boma)] {
            if ControlModule::get_stick_y(boma) > 0.5 {
                ATTACK(fighter, 0, 1, Hash40::new("top"), 1.5, 85, 100, 60, 0, 5.0, 0.0, 10.0, -2.0, Some(0.0), Some(6.0), Some(-2.0), 4.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
                ATTACK(fighter, 1, 1, Hash40::new("top"), 1.5, 90, 100, 50, 0, 5.0, 0.0, 10.0, 3.0, Some(0.0), Some(6.0), Some(3.0), 4.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
            }
            if !(ControlModule::get_stick_y(boma) > 0.5) {
                ATTACK(fighter, 0, 1, Hash40::new("top"), 1.5, 60, 100, 120, 0, 5.0, 0.0, 10.0, -2.0, Some(0.0), Some(6.0), Some(-2.0), 4.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
                ATTACK(fighter, 1, 1, Hash40::new("top"), 1.5, 60, 100, 90, 0, 5.0, 0.0, 10.0, 3.0, Some(0.0), Some(6.0), Some(3.0), 4.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
            }
        }
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        meter_gain(boma, 3);
         if heavy_attack[hdr::get_player_number(boma)] {
            FT_MOTION_RATE(fighter, 1.250);
        }
        if ex_special[hdr::get_player_number(boma)] {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                if ControlModule::get_stick_y(boma) > 0.5 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
                }
                if !(ControlModule::get_stick_y(boma) > 0.5) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                }
            }
        }
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        dolly_attack_11_game,
        dolly_attack_12_game,
        dolly_attack_13_game,
        dolly_attack_dash_game,
    );
}

