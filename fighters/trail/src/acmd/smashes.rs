
use super::*;


#[acmd_script( agent = "trail", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 5.0/(17.0-7.0));
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 15.0, 40, 103, 0, 29, 3.8, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 15.0, 40, 103, 0, 29, 3.8, 0.0, 4.6, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 15.0, 40, 107, 0, 30, 3.8, 0.0, 9.2, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.5);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 13.5, 40, 106, 0, 32, 3.4, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 13.5, 40, 106, 0, 32, 3.4, 0.0, 3.2, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 13.5, 40, 109, 0, 35, 3.4, 0.0, 7.8, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.6);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "trail", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
   
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }

    frame(lua_state, 11.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
    }
   
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.5);
    }

    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 2.0);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.2, 76, 100, 0, 45, 3.8, 0.0, -2.4, 0.0, Some(0.0), Some(1.6), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("throw"), 15.2, 76, 100, 0, 45, 3.4, 0.0, 3.0, -6.0, Some(0.0), Some(3.0), Some(7.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("throw"), 14.2, 76, 94, 0, 45, 3.4, 0.0, 3.0, -13.0, Some(0.0), Some(3.0), Some(14.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 5.0, 4.0);
    }
}


pub fn install() {
    install_acmd_scripts!(
        game_attacks4,
        game_attacklw4,
    );
}

