
use super::*;

#[acmd_script( agent = "edge", script = "game_specialhi1end" , category = ACMD_GAME , low_priority)]
unsafe fn edge_special_hi1_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.08);
    }
}

#[acmd_script( agent = "edge", script = "game_specialhi2end" , category = ACMD_GAME , low_priority)]
unsafe fn edge_special_hi2_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 7.0, 361, 104, 0, 68, 10.0, 0.0, 0.0, 6.0, None, None, None, 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 7.0, 361, 104, 0, 68, 10.0, 0.0, 0.0, 18.0, None, None, None, 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("rot"), 2.3, 60, 60, 0, 90, 8.5, 0.0, 0.0, -2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MotionModule::set_rate(boma, 1.256);
    }
}

#[acmd_script( agent = "edge", script = "game_specialairhi2end" , category = ACMD_GAME , low_priority)]
unsafe fn edge_special_air_hi_2_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut kb_angle = 0;

    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 3.0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if WorkModule::get_float(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE) <= 0.0 {
            kb_angle = (WorkModule::get_float(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE) * -1.0) as u64;
        }
        else {
            kb_angle = 361 as u64;
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 7.0, kb_angle, 104, 0, 68, 10.0, 0.0, 0.0, 6.0, None, None, None, 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 7.0, kb_angle, 104, 0, 68, 10.0, 0.0, 0.0, 18.0, None, None, None, 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("rot"), 2.3, 60, 60, 0, 90, 8.5, 0.0, 0.0, -2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_USE_LANDING_SPEED_MUL);
    }
}

pub fn install() {
    install_acmd_scripts!(
        edge_special_hi1_end_game,
        edge_special_hi2_end_game,
        edge_special_air_hi_2_end_game
    );
}

