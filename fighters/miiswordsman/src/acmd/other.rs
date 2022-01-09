
use super::*;
use smash::app::sv_battle_object;

#[acmd_script( agent = "miiswordsman", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.3, 0.0, 6.6, 0.0, Some(0.0), Some(6.6), Some(9.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "miiswordsman", script = "game_catchdash" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_catch_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.3, 0.0, 6.6, 0.0, Some(0.0), Some(6.6), Some(13.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}


#[acmd_script( agent = "miiswordsman", script = "game_catchturn" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_catch_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.3, 0.0, 6.6, 0.0, Some(0.0), Some(6.6), Some(-16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}


#[acmd_script( agent = "miiswordsman", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "miiswordsman", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "miiswordsman", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "miiswordsman_tornadoshot", script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_tornadoshot_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    // Heavy
    //if heavy_attack[hdr::get_player_number(owner_module_accessor)]{
    if VarModule::is_flag(get_battle_object_from_accessor(owner_module_accessor), vars::common::IS_HEAVY_ATTACK){
        if is_excute(fighter) {
            //disable_gale_strike[hdr::get_player_number(owner_module_accessor)] = true;
            AREA_WIND_2ND_RAD_arg9(fighter, 0, 2, 0.05, 200, 1, 3, 3, 25, 30);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.15, 367, 5, 0, 10, 13.0, 0.0, 13.0, 3.2, Some(0.0), Some(7.0), Some(3.2), 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -2, 0.0, 5, true, false, false, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 1, Hash40::new("top"), 5.0, 80, 10, 0, 90, 5.0, 0.0, 5.0, 1.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, 0.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 2, 1, Hash40::new("top"), 5.0, 80, 10, 0, 90, 5.0, 0.0, 11.0, 1.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, 0.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
            AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 0.1);
        }
        frame(lua_state, 62.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 115, 70, 0, 80, 5.0, 0.0, 11.0, 1.2, Some(0.0), Some(0.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
            //AttackModule::clear(boma, 1, false);
            //AttackModule::clear(boma, 2, false);
        }
        frame(lua_state, 68.0);
        if is_excute(fighter) {
            //disable_gale_strike[hdr::get_player_number(owner_module_accessor)] = false;
        }
    }
    
    // Light
    else{
        if is_excute(fighter) {
            //disable_gale_strike[hdr::get_player_number(owner_module_accessor)] = true;
            AREA_WIND_2ND_RAD_arg9(fighter, 0, 2, 0.05, 200, 1, 3, 3, 25, 30);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 86, 100, 150, 0, 5.0, 0.0, 11.0, 1.2, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -6.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
        }
        frame(lua_state, 18.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 86, 100, 150, 0, 5.0, 0.0, 11.0, 1.2, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -5.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
        }
        frame(lua_state, 36.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 86, 100, 150, 0, 5.0, 0.0, 11.0, 1.2, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
        }
        frame(lua_state, 50.0);
        if is_excute(fighter) {
            //disable_gale_strike[hdr::get_player_number(owner_module_accessor)] = false;
        }
        frame(lua_state, 54.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 86, 100, 150, 0, 5.0, 0.0, 11.0, 1.2, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -4.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
        }
    
    }
}

#[acmd_script( agent = "miiswordsman_wave", script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_wave_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(get_battle_object_from_accessor(owner_module_accessor), vars::miiswordsman::WAVE_SPECIAL_N) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 55, 60, 0, 38, 19.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 25, 70, 0, 20, 19.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 2.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(get_battle_object_from_accessor(owner_module_accessor), vars::miiswordsman::WAVE_SPECIAL_N) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 55, 60, 0, 38, 19.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.4, 32, 170, 0, 12, 19.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 2.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(get_battle_object_from_accessor(owner_module_accessor), vars::miiswordsman::WAVE_SPECIAL_N) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 195, 0, 5, 19.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 2.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(get_battle_object_from_accessor(owner_module_accessor), vars::miiswordsman::WAVE_SPECIAL_N) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
        }
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        if VarModule::is_flag(get_battle_object_from_accessor(owner_module_accessor), vars::miiswordsman::WAVE_SPECIAL_N) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
        }
    }
}

#[acmd_script( agent = "miiswordsman_chakram", script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_chakram_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    // Tap Input
    if !WorkModule::is_flag(boma, *WEAPON_MIISWORDSMAN_CHAKRAM_INSTANCE_WORK_ID_FLAG_FLICK){
        if is_excute(fighter) {
            VarModule::on_flag(get_battle_object_from_accessor(owner_module_accessor), vars::miiswordsman::CHAKRAM_STICK_ATTACK);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 365, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(boma, 0, -3.0, false);
            AttackModule::enable_safe_pos(boma);
        }
        frame(lua_state, 37.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 0, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(boma, 0, -3.0, false);
            AttackModule::enable_safe_pos(boma);
        }
    }
    // Hold Input
    else{
        if is_excute(fighter) {
            VarModule::on_flag(get_battle_object_from_accessor(owner_module_accessor), vars::miiswordsman::CHAKRAM_STICK_ATTACK);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 125, 40, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(boma);
        }
    }
}

#[acmd_script( agent = "miiswordsman_chakram", script = "game_flynormalsub" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_chakram_fly_normal_sub_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 365, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(boma, 0, -2.0, false);
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 0, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(boma, 0, -2.0, false);
        AttackModule::enable_safe_pos(boma);
    }
}

#[acmd_script( agent = "miiswordsman_chakram", script = "game_flyflicksub" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_chakram_fly_flick_sub_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 125, 40, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

#[acmd_script( agent = "miiswordsman_chakram", script = "game_hop" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_chakram_hop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        VarModule::off_flag(get_battle_object_from_accessor(owner_module_accessor), vars::miiswordsman::CHAKRAM_STICK_ATTACK);
    }
}

#[acmd_script( agent = "miiswordsman_chakram", script = "game_stick" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_chakram_stick_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        if VarModule::is_flag(get_battle_object_from_accessor(owner_module_accessor), vars::miiswordsman::CHAKRAM_STICK_ATTACK) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 80, 60, 0, 60, 4.5, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            //SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
    }
    frame(lua_state, 120.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
        
}

#[acmd_script( agent = "miiswordsman_chakram", script = "effect_stick" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_chakram_stick_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if VarModule::is_flag(get_battle_object_from_accessor(owner_module_accessor), vars::miiswordsman::CHAKRAM_STICK_ATTACK) {
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 0, 2.0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 3.0);
        for _ in 0..7 {
            if is_excute(fighter) {
                EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 2.0, 2.0, -3.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 2.0, 4.0, 2.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), -2.0, 5.0, -4.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 0, 0.0, 1.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 2.0, 1.0, 0.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            }
            wait(lua_state, 3.0);
        }
    }
    frame(lua_state, 142.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
        
}

pub fn install() {
    install_acmd_scripts!(
        miiswordsman_catch_game,
        miiswordsman_catch_dash_game,
        miiswordsman_catch_turn_game,
        dash_game,
        dash_effect,
        turn_dash_game,
        miiswordsman_tornadoshot_fly_game,
        miiswordsman_wave_fly_game,
        miiswordsman_chakram_fly_game,
        miiswordsman_chakram_fly_normal_sub_game,
        miiswordsman_chakram_fly_flick_sub_game,
        //miiswordsman_chakram_hop_game,
        miiswordsman_chakram_stick_game,
        miiswordsman_chakram_stick_effect,
    );
}

