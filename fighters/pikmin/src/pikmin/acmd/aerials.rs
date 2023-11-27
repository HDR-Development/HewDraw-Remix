use super::*;
use super::PikminInfo;
use std::ops::Index;
use globals::*;

#[acmd_script( agent = "pikmin_pikmin", scripts = ["game_attackairn", "game_attackairn_y", "game_attackairn_b", "game_attackairn_w", "game_attackairn_v" ] , category = ACMD_GAME , low_priority)]
unsafe fn game_attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(fighter, 2.0, 6.0, 2.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(fighter, 6.0, 22.0, 20.0);
    if is_excute(fighter) {
        let damage = 8.8;
        ATTACK(fighter, 0, 0, Hash40::new("head2"), damage * p.dmg, 75 + p.angle, 100, 0, 40, 2.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 1, 0, Hash40::new("head2"), damage * p.dmg, 75 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 2, 0, Hash40::new("head1"), damage * p.dmg, 75 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 3, 0, Hash40::new("waist"), damage * p.dmg, 75 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        let damage = 7.6;
        ATTACK(fighter, 0, 0, Hash40::new("head2"), damage * p.dmg, 62 + p.angle, 100, 0, 40, 2.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 1, 0, Hash40::new("head2"), damage * p.dmg, 62 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 2, 0, Hash40::new("head1"), damage * p.dmg, 62 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 3, 0, Hash40::new("waist"), damage * p.dmg, 62 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        let damage = 6.4;
        ATTACK(fighter, 0, 0, Hash40::new("head2"), damage * p.dmg, 50 + p.angle, 100, 0, 40, 2.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 1, 0, Hash40::new("head2"), damage * p.dmg, 50 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 2, 0, Hash40::new("head1"), damage * p.dmg, 50 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 3, 0, Hash40::new("waist"), damage * p.dmg, 50 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", scripts = ["effect_attackairn", "effect_attackairn_y", "effect_attackairn_b", "effect_attackairn_w", "effect_attackairn_v" ] , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pikmin_smash_trail"), Hash40::new("waist"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5, true);
        LAST_EFFECT_SET_COLOR(fighter, p.color.x, p.color.y, p.color.z);
        EFFECT_FOLLOW(fighter, Hash40::new("pikmin_smash_trail"), Hash40::new("hip"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5, true);
        LAST_EFFECT_SET_COLOR(fighter, p.color.x, p.color.y, p.color.z);
        EFFECT_FOLLOW(fighter, Hash40::new("pikmin_smash_trail"), Hash40::new("hip"), -2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5, true);
        LAST_EFFECT_SET_COLOR(fighter, p.color.x, p.color.y, p.color.z);

        EFFECT_FOLLOW(fighter, Hash40::new("pikmin_attack_flash"), Hash40::new("waist"), 0, 0, 0.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, p.color.x, p.color.y, p.color.z);
    }
}

#[acmd_script( agent = "pikmin_pikmin", scripts = ["game_attackairf", "game_attackairf_y", "game_attackairf_b", "game_attackairf_w", "game_attackairf_v" ] , category = ACMD_GAME , low_priority)]
unsafe fn game_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        let damage = 8.0;
        ATTACK(fighter, 0, 0, Hash40::new("head1"), damage * p.dmg, 50 + p.angle, 76, 0, 60, 3.0, 0.0, 5.5, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 1, 0, Hash40::new("head1"), damage * p.dmg, 50 + p.angle, 76, 0, 60, 4.0, -8.0, -3.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        let damage = 11.2;
        ATTACK(fighter, 1, 0, Hash40::new("head1"), damage * p.dmg, 50 + p.angle, 76, 0, 60, 4.0, 0.0, 1.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}


#[acmd_script( agent = "pikmin_pikmin", scripts = ["game_attackairb", "game_attackairb_y", "game_attackairb_b", "game_attackairb_w", "game_attackairb_v"] , category = ACMD_GAME , low_priority)]
unsafe fn game_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        let damage = 12.0;
        ATTACK(fighter, 0, 0, Hash40::new("head1"), damage * p.dmg, 40 + p.angle, 112 - p.kbg, 0, 40, 4.0, 0.0, 5.5, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 1, 0, Hash40::new("head1"), damage * p.dmg, 40 + p.angle, 112 - p.kbg, 0, 30, 4.0, 0.0, -1.5, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}


#[acmd_script( agent = "pikmin_pikmin", scripts = ["game_attackairhi", "game_attackairhi_y", "game_attackairhi_b", "game_attackairhi_w", "game_attackairhi_v"] , category = ACMD_GAME , low_priority)]
unsafe fn game_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        let damage = 7.2;
        ATTACK(fighter, 0, 0, Hash40::new("head1"), damage * p.dmg, 95, 74, 0, 65, 6.0, 0.0, 3.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(fighter, 1, 0, Hash40::new("head1"), damage * p.dmg, 95, 74, 0, 65, 3.0, 0.0, 6.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}


#[acmd_script( agent = "pikmin_pikmin", scripts = ["game_attackairlw", "game_attackairlw_y", "game_attackairlw_b", "game_attackairlw_w", "game_attackairlw_v"] , category = ACMD_GAME , low_priority)]
unsafe fn game_attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        let damage = 10.08;
        let knockback = [50, 59, 59, 68, 62];
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("head1"), damage * p.dmg, 270, 79, 0, 30, 3.5, 0.0, 3.0, 0.0, Some(0.0), Some(-1.5), Some(0.0), p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        /* Air-only */
        ATTACK(fighter, 1, 0, Hash40::new("head1"), damage * p.dmg, 270, knockback[variation as usize], 0, 30, 3.5, 0.0, 3.0, 0.0, Some(0.0), Some(-1.5), Some(0.0), p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        let damage = 6.4;
        ATTACK(fighter, 0, 0, Hash40::new("head1"), damage * p.dmg, 50, 60, 0, 40, 2.5, 0.0, 3.0, 0.0, Some(0.0), Some(-1.5), Some(0.0), p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_S, p.sound, *ATTACK_REGION_PIKMIN);
        AttackModule::clear(boma, 1, false);
    }
    wait(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        game_attackairn,
        effect_attackairn,
        game_attackairf,
        game_attackairb,
        game_attackairhi,
        game_attackairlw,
    );
}
