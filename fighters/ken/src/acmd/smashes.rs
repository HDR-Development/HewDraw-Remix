
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


#[acmd_script( agent = "ken", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn ken_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if magic_series_cancel[hdr::get_player_number(boma)] {
            magic_series_cancel[hdr::get_player_number(boma)] = false;
             ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 116, 0, 28, 3.0, 0.0, 12.5, 1.5, Some(0.0), Some(11.5), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("kneer"), 15.0, 361, 116, 0, 28, 3.3, 3.5, 0.0, 0.0, Some(8.5), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            meter_gain(boma, 14);
         }
         else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 120, 0, 28, 3.0, 0.0, 12.5, 1.5, Some(0.0), Some(11.5), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("kneer"), 16.0, 361, 120, 0, 28, 3.3, 3.5, 0.0, 0.0, Some(8.5), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            meter_gain(boma, 15);
         }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    
}

#[acmd_script( agent = "ken", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn ken_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        combo_scaling[hdr::get_player_number(boma)] = false;
         if magic_series_cancel[hdr::get_player_number(boma)] {
            magic_series_cancel[hdr::get_player_number(boma)] = false;
            combo_scaling[hdr::get_player_number(boma)] = true;
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 70, 25, 0, 52, 5.3, 0.0, 9.0, 8.5, Some(0.0), Some(7.0), Some(9.7), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 7.0, false);
            meter_gain(boma, 10);
         }
         else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 70, 25, 0, 52, 5.3, 0.0, 9.0, 8.5, Some(0.0), Some(7.0), Some(9.7), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 7.0, false);
            meter_gain(boma, 13);
         }
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if combo_scaling[hdr::get_player_number(boma)] {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.5, 70, 25, 0, 52, 4.3, -2.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 7.0, false);
            meter_gain(boma, 9);
         }
         else {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.5, 70, 25, 0, 52, 4.3, -2.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 7.0, false);
            meter_gain(boma, 12);
         }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if combo_scaling[hdr::get_player_number(boma)] {
            combo_scaling[hdr::get_player_number(boma)] = false;
             ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 70, 25, 0, 48, 4.8, 0.0, 21.5, 5.0, Some(0.0), Some(17.0), Some(6.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 7.0, false);
            meter_gain(boma, 9);
         }
         else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 70, 25, 0, 48, 4.8, 0.0, 21.5, 5.0, Some(0.0), Some(17.0), Some(6.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 7.0, false);
            meter_gain(boma, 12);
         }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
    
}

#[acmd_script( agent = "ken", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn ken_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if magic_series_cancel[hdr::get_player_number(boma)] {
            magic_series_cancel[hdr::get_player_number(boma)] = false;
             ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 60, 0, 45, 4.0, 0.0, 2.5, 18.0, Some(0.0), Some(3.0), Some(8.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 45, 60, 0, 45, 4.0, 0.0, 4.5, 18.0, Some(0.0), Some(4.5), Some(8.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 45, 60, 0, 45, 4.0, 0.0, 3.0, 2.5, None, None, None, 1.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 45, 60, 0, 45, 4.0, 0.0, 3.0, 2.5, Some(0.0), Some(3.0), Some(3.0), 1.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            meter_gain(boma, 10);
         }
         else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 92, 25, 0, 25, 4.0, 0.0, 2.5, 18.0, Some(0.0), Some(3.0), Some(8.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 92, 25, 0, 25, 4.0, 0.0, 4.5, 18.0, Some(0.0), Some(4.5), Some(8.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 86, 25, 0, 25, 4.0, 0.0, 3.0, 2.5, None, None, None, 1.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0, 86, 25, 0, 25, 4.0, 0.0, 3.0, 2.5, Some(0.0), Some(3.0), Some(3.0), 1.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            meter_gain(boma, 11);
         }
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        ken_attack_s4_s_game,
        ken_attack_hi4_game,
        ken_attack_lw4_game,
    );
}

