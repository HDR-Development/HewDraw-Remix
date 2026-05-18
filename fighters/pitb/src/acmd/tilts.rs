use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 9.0, 7.0);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 10.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "top", dmg: 7.0, angle: 361, kbg: 100, bkb: 40, size: 4.0, x: 0.0, y: 7.5, z:  8.0, x2: 0.0, y2: 7.5, z2: 6.0 });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 1, bone: "top", dmg: 9.0, angle: 361, kbg: 100, bkb: 40, size: 4.0, x: 0.0, y: 7.5, z: 14.0, x2: 0.0, y2: 7.5, z2: 6.0, hitlag: 1.1, });
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("swordr2"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, 0, 0, 180, 90, 0, 1, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pitb_atk_s3_hdr"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_sword"), true, true);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_KICK_HITBOX_S, id: 0, bone: "top", dmg: 4.0, angle: 368, kbg: 100, fkb: 45, bkb: 0, size: 4.0, x: 0.0, y: 24.0, z: 2.4, x2: 0.0, y2: 10.0, z2: 8.0, sdi: 0.5, facing: LrCheck::F, set_weight: true, });
        let hitVec = Vector2f { x: 5.5, y: 21.75 };
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hitVec, 10, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_KICK_HITBOX_L, id: 0, bone: "top", dmg: 6.0, angle: 69, kbg: 44, bkb: 71, size: 6.0, x: 0.0, y: 23.5, z: 6.0, x2: 0.0, y2: 20.0, z2: 5.0, facing: LrCheck::F, });
    }
    frame(lua_state, 17.0);
    FT_MOTION_RATE_RANGE(agent, 17.0, 34.0, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "top", dmg: 7.0, angle: 69, kbg: 53, bkb: 60, size: 3.5, x: 0.0, y: 3.0, z: 19.5, x2: 0.0, y2: 5.0, z2: 9.0, });
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(agent, 8.0, 30.0, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pitb_atk_wind_hdr"), Hash40::new("top"), -7, 6, 2.3, -12, -42, 168, 1.3, true);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("swordr2"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_sword"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_atk_wind"), true, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3, Priority::Low);
    agent.acmd("effect_attacks3", effect_attacks3, Priority::Low);

    agent.acmd("game_attackhi3", game_attackhi3, Priority::Low);
    
    agent.acmd("game_attacklw3", game_attacklw3, Priority::Low);
    agent.acmd("effect_attacklw3", effect_attacklw3, Priority::Low);
}
