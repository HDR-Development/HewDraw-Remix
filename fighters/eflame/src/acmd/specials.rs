
use super::*;

#[acmd_script(agent = "eflame", script = "game_speciallw", category = ACMD_GAME)]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(12.0-1.0));

    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "eflame", script = "game_specialairlw", category = ACMD_GAME)]
unsafe fn game_specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(12.0-1.0));

    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "eflame_firepillar", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn eflame_firepillar_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 60, 100, 0, 90, 8.0, 0.0, 8.0, 1.0, Some(0.0), Some(8.0), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 60, 100, 0, 90, 7.0, 0.0, 15.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 60, 100, 0, 90, 5.0, 0.0, 23.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 0, 90, 8.0, 0.0, 8.0, 1.0, Some(0.0), Some(8.0), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 100, 0, 90, 7.0, 0.0, 15.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 60, 100, 0, 80, 5.0, 0.0, 27.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

#[acmd_script( agent = "eflame_firepillar", script = "effect_specialhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn eflame_firepillar_special_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar_ground"), Hash40::new("top"), 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar"), Hash40::new("top"), 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.06, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar_impact"), Hash40::new("top"), 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.06, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_speciallw,
        game_specialairlw,

        eflame_firepillar_special_hi_game,
        eflame_firepillar_special_hi_effect,
    );
}

