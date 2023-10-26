
use super::*;


#[acmd_script( agent = "purin", script = "game_specialnstartr", category = ACMD_GAME, low_priority )]
unsafe fn purin_special_n_start_r_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        GroundModule::set_shape_flag(boma, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FIX as u16, true);
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "purin", script = "game_specialairnstartr", category = ACMD_GAME, low_priority )]
unsafe fn purin_special_air_n_start_r_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        GroundModule::set_shape_flag(boma, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FIX as u16, true);
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "purin", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn purin_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    if is_excute(fighter) {
        GroundModule::set_shape_flag(boma, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FIX as u16, true);
        JostleModule::set_status(boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 30, 60, 0, 60, 2.5, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
}

#[acmd_script( agent = "purin", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn purin_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        GroundModule::set_shape_flag(boma, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FIX as u16, true);
        JostleModule::set_status(boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 30, 60, 0, 60, 2.5, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
}

#[acmd_script( agent = "purin", script = "game_specialnhold", category = ACMD_GAME, low_priority )]
unsafe fn purin_special_n_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        GroundModule::set_shape_flag(boma, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FIX as u16, true);
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "purin", script = "game_specialairnhold", category = ACMD_GAME, low_priority )]
unsafe fn purin_special_air_n_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        GroundModule::set_shape_flag(boma, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FIX as u16, true);
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "purin", script = "game_specialnturn", category = ACMD_GAME, low_priority )]
unsafe fn purin_special_n_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        GroundModule::set_shape_flag(boma, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FIX as u16, true);
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
}

#[acmd_script( agent = "purin", script = "game_specialairnturn", category = ACMD_GAME, low_priority )]
unsafe fn purin_special_air_n_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        GroundModule::set_shape_flag(boma, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FIX as u16, true);
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
}

#[acmd_script( agent = "purin", scripts = ["game_specialairnendl", "game_specialairnendr", "game_specialnendr", "game_specialnendl"], category = ACMD_GAME, low_priority )]
unsafe fn purin_special_n_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
        GroundModule::set_shape_flag(boma, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FIX as u16, false);
    }
}

#[acmd_script( agent = "purin", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn purin_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 75, 75, 0, 52, 4.5, 0.0, 4.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 17, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 75, 75, 0, 52, 4.5, 0.0, 5.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 17, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "purin", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn purin_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 75, 75, 0, 52, 3.5, 0.0, 4.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 17, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 75, 75, 0, 52, 3.5, 0.0, 5.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 17, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
        WorkModule::on_flag(boma, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_INPUT);
        WorkModule::set_int(boma, *FIGHTER_PURIN_SPECIAL_S_ENERGY_MODE_BRAKE, *FIGHTER_PURIN_STATUS_SPECIAL_S_WORK_INT_ENERGY_MODE);
        WorkModule::on_flag(boma, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_CHANGE_ENERGY);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, *FIGHTER_PURIN_SPECIAL_S_ENERGY_MODE_FALL, *FIGHTER_PURIN_STATUS_SPECIAL_S_WORK_INT_ENERGY_MODE);
        WorkModule::on_flag(boma, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_CHANGE_ENERGY);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

#[acmd_script( agent = "purin", scripts = ["game_speciallwl", "game_speciallwr", "game_specialairlwl", "game_specialairlwr"] , category = ACMD_GAME , low_priority)]
unsafe fn purin_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("bust"), 25.0, 361, 66, 0, 106, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
        if(AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)){
            if (DamageModule::damage(boma, 0) > 5.0) {
                DamageModule::add_damage(boma, -5.0, 0);
            }
        }
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        purin_special_n_start_r_game,
        purin_special_air_n_start_r_game,
        purin_special_n_game,
        purin_special_air_n_game,
        purin_special_n_hold_game,
        purin_special_air_n_hold_game,
        purin_special_n_turn_game,
        purin_special_air_n_turn_game,
        purin_special_n_end,
        purin_special_s_game,
        purin_special_air_s_game,
        purin_special_lw_game,
    );
}

