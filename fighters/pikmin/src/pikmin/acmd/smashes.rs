use super::*;
use super::PikminInfo;
use std::ops::Index;
use globals::*;

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
        let dmg = 18.2;
        ATTACK(agent, 0, 0, Hash40::new("top"), dmg * p.dmg, 83 + p.angle, 72, 0, 50, 6.0, 0.0, 3.5, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
        let dmg = 14.0;
        ATTACK(agent, 1, 0, Hash40::new("top"), dmg * p.dmg, 94 + p.angle, 64, 0, 50, 4.0, 0.0, 3.5, 9.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        let dmg = 15.4;
        ATTACK(agent, 0, 0, Hash40::new("top"), dmg * p.dmg, 60 + p.angle, 68, 0, 50, 5.0, 0.0, 3.5, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, dmg * p.shield_dmg, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
        let dmg = 15.4;
        ATTACK(agent, 0, 0, Hash40::new("head1"), dmg * p.dmg, 28 + p.angle, 71, 0, 32, 4.5, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, dmg * p.shield_dmg, 0.2, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dmg = 12.6;
        ATTACK(agent, 0, 0, Hash40::new("head1"), dmg * p.dmg, 28 + p.angle, 71, 0, 25, 4.5, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, dmg * p.shield_dmg, 0.2, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn game_attacks4sjump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
        let dmg = 20.3;
        ATTACK(agent, 0, 0, Hash40::new("head1"), dmg * p.dmg, 361, 75, 0, 33, 4.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -10.0 + (dmg * p.shield_dmg), 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_L, p.sound, *ATTACK_REGION_PIKMIN);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        let dmg = 14.0;
        ATTACK(agent, 0, 0, Hash40::new("head1"), dmg * p.dmg, 361, 75, 0, 33, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -7.0 + (dmg * p.shield_dmg), 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_M, p.sound, *ATTACK_REGION_PIKMIN);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        let dmg = 8.4;
        ATTACK(agent, 0, 0, Hash40::new("head1"), dmg * p.dmg, 361, 75, 0, 33, 3.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4.2 + (dmg * p.shield_dmg), 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_S, p.sound, *ATTACK_REGION_PIKMIN);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }

}

pub fn install(agent: &mut Agent) {
    smashline::Agent::new("pikmin_pikmin")
        .acmd("game_attackhi4", game_attackhi4)
        .acmd("game_attackhi4_b", game_attackhi4)
        .acmd("game_attackhi4_v", game_attackhi4)
        .acmd("game_attackhi4_w", game_attackhi4)
        .acmd("game_attackhi4_y", game_attackhi4)
        .acmd("game_attacklw4", game_attacklw4)
        .acmd("game_attacklw4_b", game_attacklw4)
        .acmd("game_attacklw4_v", game_attacklw4)
        .acmd("game_attacklw4_w", game_attacklw4)
        .acmd("game_attacklw4_y", game_attacklw4)
        .acmd("game_attacks4sjump", game_attacks4sjump)
        .acmd("game_attacks4sjump_b", game_attacks4sjump)
        .acmd("game_attacks4sjump_v", game_attacks4sjump)
        .acmd("game_attacks4sjump_w", game_attacks4sjump)
        .acmd("game_attacks4sjump_y", game_attacks4sjump)
}
