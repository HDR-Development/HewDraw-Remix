use super::*;
use super::PikminInfo;
use std::ops::Index;
use globals::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 6.0, 2.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 22.0, 20.0);
    if is_excute(agent) {
        let dmg = 8.8;
        ATTACK(agent, 0, 0, Hash40::new("head2"), dmg * p.dmg, 75 + p.angle, 100, 0, 40, 2.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head2"), dmg * p.dmg, 75 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg * p.dmg, 75 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg * p.dmg, 75 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        let dmg = 7.6;
        ATTACK(agent, 0, 0, Hash40::new("head2"), dmg * p.dmg, 62 + p.angle, 100, 0, 40, 2.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head2"), dmg * p.dmg, 62 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg * p.dmg, 62 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg * p.dmg, 62 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        let dmg = 6.4;
        ATTACK(agent, 0, 0, Hash40::new("head2"), dmg * p.dmg, 50 + p.angle, 100, 0, 40, 2.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head2"), dmg * p.dmg, 50 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg * p.dmg, 50 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg * p.dmg, 50 + p.angle, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_smash_trail"), Hash40::new("waist"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_smash_trail"), Hash40::new("hip"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_smash_trail"), Hash40::new("hip"), -2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);

        EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_flash"), Hash40::new("waist"), 0, 0, 0.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        let dmg = 8.0;
        ATTACK(agent, 0, 0, Hash40::new("head2"), dmg * p.dmg, 50 + p.angle, 76, 0, 60, 2.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head2"), dmg * p.dmg, 50 + p.angle, 76, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg * p.dmg, 50 + p.angle, 76, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg * p.dmg, 50 + p.angle, 76, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let dmg = 11.2;
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg * p.dmg, 50 + p.angle, 76, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg * p.dmg, 50 + p.angle, 76, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        let dmg = 12.0;
        ATTACK(agent, 0, 0, Hash40::new("head1"), dmg * p.dmg, 40 + p.angle, 105, 0, 30, 4.0, 0.0, 5.5, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head1"), dmg * p.dmg, 40 + p.angle, 105, 0, 20, 4.0, 0.0, -1.5, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        let dmg = 7.2;
        ATTACK(agent, 0, 0, Hash40::new("head2"), dmg * p.dmg, 95 + p.angle, 74, 0, 65, 3.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head2"), dmg * p.dmg, 95 + p.angle, 74, 0, 65, 3.5, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg * p.dmg, 95 + p.angle, 74, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg * p.dmg, 95 + p.angle, 74, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        let dmg = 10.08;
        /* Ground-only */
        ATTACK(agent, 0, 0, Hash40::new("head1"), dmg * p.dmg, 270 + p.angle, 79, 0, 30, 3.5, 0.0, 3.0, 0.0, Some(0.0), Some(-1.5), Some(0.0), p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        /* Air-only */
        ATTACK(agent, 1, 0, Hash40::new("head1"), dmg * p.dmg, 270 + p.angle, 59, 0, 30, 3.5, 0.0, 3.0, 0.0, Some(0.0), Some(-1.5), Some(0.0), p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        let dmg = 6.4;
        ATTACK(agent, 0, 0, Hash40::new("head1"), dmg * p.dmg, 50, 60, 0, 40, 2.5, 0.0, 3.0, 0.0, Some(0.0), Some(-1.5), Some(0.0), p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_S, p.sound, *ATTACK_REGION_PIKMIN);
        AttackModule::clear(boma, 1, false);
    }
    wait(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub fn install(agent: &mut Agent) {
    smashline::Agent::new("pikmin_pikmin")
        .acmd("game_attackairn", game_attackairn)
        .acmd("game_attackairn_y", game_attackairn)
        .acmd("game_attackairn_b", game_attackairn)
        .acmd("game_attackairn_w", game_attackairn)
        .acmd("game_attackairn_v", game_attackairn)
        .acmd("effect_attackairn", effect_attackairn)
        .acmd("effect_attackairn_y", effect_attackairn)
        .acmd("effect_attackairn_b", effect_attackairn)
        .acmd("effect_attackairn_w", effect_attackairn)
        .acmd("effect_attackairn_v", effect_attackairn)
        .acmd("game_attackairf", game_attackairf)
        .acmd("game_attackairf_y", game_attackairf)
        .acmd("game_attackairf_b", game_attackairf)
        .acmd("game_attackairf_w", game_attackairf)
        .acmd("game_attackairf_v", game_attackairf)
        .acmd("game_attackairb", game_attackairb)
        .acmd("game_attackairb_y", game_attackairb)
        .acmd("game_attackairb_b", game_attackairb)
        .acmd("game_attackairb_w", game_attackairb)
        .acmd("game_attackairb_v", game_attackairb)
        .acmd("game_attackairhi", game_attackairhi)
        .acmd("game_attackairhi_y", game_attackairhi)
        .acmd("game_attackairhi_b", game_attackairhi)
        .acmd("game_attackairhi_w", game_attackairhi)
        .acmd("game_attackairhi_v", game_attackairhi)
        .acmd("game_attackairlw", game_attackairlw)
        .acmd("game_attackairlw_y", game_attackairlw)
        .acmd("game_attackairlw_b", game_attackairlw)
        .acmd("game_attackairlw_w", game_attackairlw)
        .acmd("game_attackairlw_v", game_attackairlw)
}
