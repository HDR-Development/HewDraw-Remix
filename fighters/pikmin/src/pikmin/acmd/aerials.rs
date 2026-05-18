use super::*;
use super::PikminInfo;
use std::ops::Index;
use globals::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let p = PikminInfo::from(agent.boma());
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 6.0, 2.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 22.0, 20.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_XLU);
        let dmg = 8.0;
        ATTACK(agent, 0, 0, Hash40::new("head2"), dmg, 75 + p.angle, 108.mul_f32(p.kbg), 0, 40, 2.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head2"), dmg, 75 + p.angle, 108.mul_f32(p.kbg), 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg, 75 + p.angle, 108.mul_f32(p.kbg), 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg, 75 + p.angle, 108.mul_f32(p.kbg), 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        let dmg = 7.0;
        ATTACK(agent, 0, 0, Hash40::new("head2"), dmg, 62 + p.angle, 108.mul_f32(p.kbg), 0, 40, 2.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head2"), dmg, 62 + p.angle, 108.mul_f32(p.kbg), 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg, 62 + p.angle, 108.mul_f32(p.kbg), 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg, 62 + p.angle, 108.mul_f32(p.kbg), 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        let dmg = 6.0;
        ATTACK(agent, 0, 0, Hash40::new("head2"), dmg, 50 + p.angle, 108.mul_f32(p.kbg), 0, 40, 2.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head2"), dmg, 50 + p.angle, 108.mul_f32(p.kbg), 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg, 50 + p.angle, 108.mul_f32(p.kbg), 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg, 50 + p.angle, 108.mul_f32(p.kbg), 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
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
    let p = PikminInfo::from(agent.boma());
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
    let p = PikminInfo::from(agent.boma());
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_XLU);
        let dmg = 8.0;
        ATTACK(agent, 0, 0, Hash40::new("head2"), dmg, 50 + p.angle, 76.mul_f32(p.kbg), 0, 60, 2.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head2"), dmg, 50 + p.angle, 76.mul_f32(p.kbg), 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg, 50 + p.angle, 76.mul_f32(p.kbg), 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg, 50 + p.angle, 76.mul_f32(p.kbg), 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let dmg = 11.0;
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg, 50 + p.angle, 76.mul_f32(p.kbg), 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg, 50 + p.angle, 76.mul_f32(p.kbg), 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let p = PikminInfo::from(agent.boma());
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_arc_hdr"), Hash40::new("top"), 3, 6, 3.5, 10, -10, -70, 1, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z); 
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_flash"), Hash40::new("waist"), 0, 0, 0.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let p = PikminInfo::from(agent.boma());
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 8.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_XLU);
        let dmg = 12.0;
        ATTACK(agent, 0, 0, Hash40::new("head1"), dmg, 40 + p.angle, 110.mul_f32(p.kbg), 0, 30, 4.0, 0.0, 5.5, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head1"), dmg, 40 + p.angle, 110.mul_f32(p.kbg), 0, 22, 4.0, 0.0, -1.5, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let p = PikminInfo::from(agent.boma());
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_arc_hdr"), Hash40::new("top"), 1, 7, -1, 180, 0, 60, 1.1, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_flash"), Hash40::new("waist"), 0, 0, 0.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let p = PikminInfo::from(agent.boma());
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 7.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_XLU);
        let dmg = 8.0;
        ATTACK(agent, 0, 0, Hash40::new("head2"), dmg, 95 + p.angle, 113.mul_f32(p.kbg), 0, 43, 3.5, 2.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 1, 0, Hash40::new("head2"), dmg, 95 + p.angle, 113.mul_f32(p.kbg), 0, 43, 3.5, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 2, 0, Hash40::new("head1"), dmg, 95 + p.angle, 113.mul_f32(p.kbg), 0, 43, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
        ATTACK(agent, 3, 0, Hash40::new("waist"), dmg, 95 + p.angle, 113.mul_f32(p.kbg), 0, 43, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let p = PikminInfo::from(agent.boma());
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_arc_hdr"), Hash40::new("top"), 0, 12, 1, -110, -90, 0, 0.75, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_flash"), Hash40::new("waist"), 0, 0, 0.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_arc_hdr"), Hash40::new("top"), 0, 11, -1, 0, 90, 90, 0.7, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
        LAST_EFFECT_SET_RATE(agent, 1.4);
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_flash"), Hash40::new("waist"), 0, 0, 0.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
    }
}



unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let p = PikminInfo::from(agent.boma());
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_XLU);
        let dmg = 10.0;
        ATTACK(agent, 0, 0, Hash40::new("head1"), dmg, 270 + p.angle, 64.mul_f32(p.kbg), 0, 32, 3.5, 0.0, 3.0, 0.0, Some(0.0), Some(-1.5), Some(0.0), p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        let dmg = 6.0;
        ATTACK(agent, 0, 0, Hash40::new("head1"), dmg, 50 + p.angle, 60.mul_f32(p.kbg), 0, 40, 2.5, 0.0, 3.0, 0.0, Some(0.0), Some(-1.5), Some(0.0), p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_S, p.sound, *ATTACK_REGION_PIKMIN);
        AttackModule::clear(boma, 1, false);
    }
    wait(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let p = PikminInfo::from(agent.boma());
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_arc_hdr"), Hash40::new("top"), 0, 1, -1, 260, -180, -250, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_flash"), Hash40::new("waist"), 0, 0, 0.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, p.color.x, p.color.y, p.color.z);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn, Priority::Low);
    agent.acmd("game_attackairn_y", game_attackairn, Priority::Low);
    agent.acmd("game_attackairn_b", game_attackairn, Priority::Low);
    agent.acmd("game_attackairn_w", game_attackairn, Priority::Low);
    agent.acmd("game_attackairn_v", game_attackairn, Priority::Low);
    agent.acmd("effect_attackairn", effect_attackairn, Priority::Low);
    agent.acmd("effect_attackairn_y", effect_attackairn, Priority::Low);
    agent.acmd("effect_attackairn_b", effect_attackairn, Priority::Low);
    agent.acmd("effect_attackairn_w", effect_attackairn, Priority::Low);
    agent.acmd("effect_attackairn_v", effect_attackairn, Priority::Low);

    agent.acmd("game_attackairf", game_attackairf, Priority::Low);
    agent.acmd("game_attackairf_y", game_attackairf, Priority::Low);
    agent.acmd("game_attackairf_b", game_attackairf, Priority::Low);
    agent.acmd("game_attackairf_w", game_attackairf, Priority::Low);
    agent.acmd("game_attackairf_v", game_attackairf, Priority::Low);
    agent.acmd("effect_attackairf", effect_attackairf, Priority::Low);
    agent.acmd("effect_attackairf_y", effect_attackairf, Priority::Low);
    agent.acmd("effect_attackairf_b", effect_attackairf, Priority::Low);
    agent.acmd("effect_attackairf_w", effect_attackairf, Priority::Low);
    agent.acmd("effect_attackairf_v", effect_attackairf, Priority::Low);

    agent.acmd("game_attackairb", game_attackairb, Priority::Low);
    agent.acmd("game_attackairb_y", game_attackairb, Priority::Low);
    agent.acmd("game_attackairb_b", game_attackairb, Priority::Low);
    agent.acmd("game_attackairb_w", game_attackairb, Priority::Low);
    agent.acmd("game_attackairb_v", game_attackairb, Priority::Low);
    agent.acmd("effect_attackairb", effect_attackairb, Priority::Low);
    agent.acmd("effect_attackairb_y", effect_attackairb, Priority::Low);
    agent.acmd("effect_attackairb_b", effect_attackairb, Priority::Low);
    agent.acmd("effect_attackairb_w", effect_attackairb, Priority::Low);
    agent.acmd("effect_attackairb_v", effect_attackairb, Priority::Low);

    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);
    agent.acmd("game_attackairhi_y", game_attackairhi, Priority::Low);
    agent.acmd("game_attackairhi_b", game_attackairhi, Priority::Low);
    agent.acmd("game_attackairhi_w", game_attackairhi, Priority::Low);
    agent.acmd("game_attackairhi_v", game_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi", effect_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi_y", effect_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi_b", effect_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi_w", effect_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi_v", effect_attackairhi, Priority::Low);
    
    
    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
    agent.acmd("game_attackairlw_y", game_attackairlw, Priority::Low);
    agent.acmd("game_attackairlw_b", game_attackairlw, Priority::Low);
    agent.acmd("game_attackairlw_w", game_attackairlw, Priority::Low);
    agent.acmd("game_attackairlw_v", game_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw", effect_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw_y", effect_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw_b", effect_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw_w", effect_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw_v", effect_attackairlw, Priority::Low);
}
