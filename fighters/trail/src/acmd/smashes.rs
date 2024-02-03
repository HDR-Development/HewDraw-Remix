
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
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 15.0, 40, 103, 0, 29, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 15.0, 40, 103, 0, 29, 3.8, 0.0, 4.6, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 15.0, 40, 107, 0, 30, 3.8, 0.0, 9.2, 0.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.5);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 13.5, 40, 106, 0, 32, 3.4, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 13.5, 40, 106, 0, 32, 3.4, 0.0, 3.2, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 13.5, 40, 109, 0, 35, 3.4, 0.0, 7.8, 0.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
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

#[acmd_script( agent = "trail", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 115, 100, 170, 0, 3.2, 0.0, 7.4, 4.2, Some(0.0), Some(7.4), Some(11.4), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.6, 84, 100, 0, 52, 4.8, 0.0, 28.6, 3.8, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_KICK, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.6, 84, 90, 0, 52, 9.8, 0.0, 28.6, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_KICK, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 14.6, 84, 90, 0, 52, 3.6, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_KICK, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 14.6, 84, 90, 0, 52, 3.6, 0.0, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_KICK, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "trail", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    FT_MOTION_RATE(fighter, 0.75);
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
    frame(lua_state, 18.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 2.0);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.0, 85, 20, 0, 88, 3.8, 0.0, 4.0, 0.0, Some(0.0), Some(7.5), Some(0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 12.5, 76, 85, 0, 45, 4.0, 0.0, 4.0, -6.0, Some(0.0), Some(4.0), Some(6.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("throw"), 11.5, 76, 80, 0, 45, 4.0, 0.0, 4.0, -12.0, Some(0.0), Some(4.0), Some(11.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
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

#[acmd_script( agent = "trail", script = "effect_attacklw4" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_smash_lw_flash"), Hash40::new("haver"), 0, 11, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_smash_lw_speedline"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("trail_smash_lw_flash"), false, true);
        EFFECT(fighter, Hash40::new("trail_smash_lw_impact"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.4);
        EFFECT(fighter, Hash40::new("trail_smash_lw_attack"), Hash40::new("haver"), 0, -2, 0, 90, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("trail_keyblade_flare"), false, true);
    }
    frame(lua_state, 57.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -0.9, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}


pub fn install() {
    install_acmd_scripts!(
        game_attacks4,
        game_attackhi4,
        game_attacklw4,
        effect_attacklw4,
    );
}

