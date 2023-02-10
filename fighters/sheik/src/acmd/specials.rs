
use super::*;


#[acmd_script( agent = "sheik", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn sheik_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && !ItemModule::is_have_item(boma, 0) && VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) < 1 {
            //ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_EXPLOSIONBOMB), 0, 0, false, false);
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 1); // Start counting cooldown timer
            StatusModule::change_status_request_from_script(boma, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_END, true);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_THROW);
        }
    }
    
}

#[acmd_script( agent = "sheik", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn sheik_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && !ItemModule::is_have_item(boma, 0) && VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) < 1 {
            //ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_EXPLOSIONBOMB), 0, 0, false, false);
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 1); // Start counting cooldown timer
            StatusModule::change_status_request_from_script(boma, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_END, true);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_THROW);
        }
    }
    
}

#[acmd_script( agent = "sheik", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn sheik_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 70, 80, 0, 70, 9.0, 0.0, 10.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 100, 50, 0, 11.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "sheik", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn sheik_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 70, 80, 0, 70, 9.0, 0.0, 10.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 100, 50, 0, 11.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FT_SHEIK_STATUS_SPECIAL_HI_FLAG_FALL);
    }
    
}

#[acmd_script( agent = "sheik", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn sheik_special_lw_ground_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SHEIK_STATUS_SPECIAL_LW_FLAG_ACCEL);
    }
}

#[acmd_script( agent = "sheik", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn sheik_special_lw_air_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SHEIK_STATUS_SPECIAL_LW_FLAG_ACCEL);
    }
}

#[acmd_script( agent = "sheik", script = "game_speciallwattack" , category = ACMD_GAME , low_priority)]
unsafe fn sheik_special_lw_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 11.0, 361, 110, 0, 26, 5.0, 5.5, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 11.0, 361, 110, 0, 26, 5.0, -3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
        AttackModule::set_attack_keep_rumble(boma, 1, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 1.75);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        sheik_special_s_game,
        sheik_special_air_s_game,
        sheik_special_hi_game,
        sheik_special_air_hi_game,
		sheik_special_lw_ground_attack_game,
		sheik_special_lw_air_attack_game,
        sheik_special_lw_attack_game
    );
}

