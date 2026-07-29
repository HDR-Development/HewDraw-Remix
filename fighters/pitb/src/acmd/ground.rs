use super::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 5.0, 3.0);
    frame(lua_state, 5.0);
    FT_MOTION_RATE_RANGE(agent, 5.0, 5.5, 1.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 0, bone: "armr", dmg: 3.0, angle: 74, kbg: 60, bkb: 20, size: 3.0, x: 2.0, y: 0.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::GA_d, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 1, bone: "bowr", dmg: 3.0, angle: 74, kbg: 60, bkb: 20, size: 3.0, x: 0.0, y: 2.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::GA_d, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 2, bone: "bowr", dmg: 3.0, angle: 74, kbg: 60, bkb: 20, size: 3.0, x: 0.0, y: 7.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::GA_d, });
        // Jab lock hitboxes
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 3, bone: "armr", dmg: 3.0, angle: 361, kbg: 15, bkb: 30, size: 3.0, x: 2.0, y: 0.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::G, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 4, bone: "bowr", dmg: 3.0, angle: 361, kbg: 15, bkb: 30, size: 3.0, x: 0.0, y: 2.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::G, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 5, bone: "bowr", dmg: 3.0, angle: 361, kbg: 15, bkb: 30, size: 3.0, x: 0.0, y: 7.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::G, });
        AttackModule::set_down_only(boma, 3, true);
        AttackModule::set_down_only(boma, 4, true);
        AttackModule::set_down_only(boma, 5, true);
    }
    frame(lua_state, 5.5);
    FT_MOTION_RATE_RANGE(agent, 5.5, 7.0, 1.0);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn effect_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0, 0, -0.2, Hash40::new("swordr1"), 0, 10.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.0, 1.0);
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 5.0);
    FT_MOTION_RATE_RANGE(agent, 5.0, 5.5, 1.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 0, bone: "arml", dmg: 3.0, angle: 80, kbg: 60, bkb: 20, size: 3.0, x: 2.0, y:  0.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::GA_d, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 1, bone: "bowl", dmg: 3.0, angle: 80, kbg: 60, bkb: 20, size: 3.2, x: 0.0, y: -2.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::GA_d, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 2, bone: "bowl", dmg: 3.0, angle: 80, kbg: 60, bkb: 20, size: 3.2, x: 0.0, y: -7.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::GA_d, });
        // Jab lock hitboxes
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 3, bone: "arml", dmg: 3.0, angle: 361, kbg: 15, bkb: 30, size: 3.0, x: 2.0, y:  0.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::G, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 4, bone: "bowl", dmg: 3.0, angle: 361, kbg: 15, bkb: 30, size: 3.2, x: 0.0, y: -2.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::G, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_S, id: 5, bone: "bowl", dmg: 3.0, angle: 361, kbg: 15, bkb: 30, size: 3.2, x: 0.0, y: -7.0, z: 0.0, facing: LrCheck::F, situation: CollisionSituation::G, });
        AttackModule::set_down_only(boma, 3, true);
        AttackModule::set_down_only(boma, 4, true);
        AttackModule::set_down_only(boma, 5, true);
    }
    frame(lua_state, 5.5);
    FT_MOTION_RATE_RANGE(agent, 5.5, 7.0, 1.0);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn effect_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 3, Hash40::new("swordl"), 0, 0, -0.2, Hash40::new("swordl"), 0, -10.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, 0, 0, 180, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    frame(lua_state, 3.0);
    FT_MOTION_RATE_RANGE(agent, 3.0, 4.0, 3.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "armr", dmg: 4.0, angle: 70, kbg: 100, bkb: 60, size: 4.0, x: 2.0, y: 0.0, z: 0.0, facing: LrCheck::F, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 1, bone: "bowr", dmg: 4.0, angle: 70, kbg: 100, bkb: 60, size: 4.0, x: 0.0, y: 2.0, z: 0.0, facing: LrCheck::F, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 2, bone: "bowr", dmg: 4.0, angle: 70, kbg: 100, bkb: 60, size: 4.0, x: 0.0, y: 6.0, z: 0.0, facing: LrCheck::F, });
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE_RANGE(agent, 4.0, 32.0, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 3, Hash40::new("swordr1"), 0, 0, -0.2, Hash40::new("swordr1"), 0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn effect_attack100(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("pitb_atk100_hdr"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true, 0.3);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("swordr2"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, 0, 0, 180, 90, 0, 1, true);
    }
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 15, 0, 4, 0, 0, 0, false);
    }
    wait(lua_state, 10.0);
}

unsafe extern "C" fn effect_attack100end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_sword"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_atk100_hdr"), true, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pitb_atk_wind_hdr"), Hash40::new("top"), 0.471, 12, 4, 11.218, -45.295, 46.981, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("swordr2"), 0, 0.8, 0, 0, 90, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_sword"), true, true);
    }
}

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.85);
    frame(lua_state, 7.0);
    FT_MOTION_RATE_RANGE(agent, 7.0, 8.0, 3.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.85 * 3.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, part: 0, bone: "top", dmg: 8.0, angle: 64, kbg: 124, bkb: 29, size: 3.5, x: 0.0, y: 4.0, z: 11.5, x2: 0.0, y2: 7.0, z2: 4.0, facing: LrCheck::F, });
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.75);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0, 0, -0.2, Hash40::new("swordr1"), 0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack11", game_attack11, Priority::Low);
    agent.acmd("effect_attack11", effect_attack11, Priority::Low);
    agent.acmd("game_attack12", game_attack12, Priority::Low);
    agent.acmd("effect_attack12", effect_attack12, Priority::Low);
    agent.acmd("game_attack13", game_attack13, Priority::Low);
    agent.acmd("effect_attack13", effect_attack13, Priority::Low);
    agent.acmd("effect_attack100", effect_attack100, Priority::Low);
    agent.acmd("effect_attack100end", effect_attack100end, Priority::Low);

    
    agent.acmd("game_attackdash", game_attackdash, Priority::Low);
    agent.acmd("effect_attackdash", effect_attackdash, Priority::Low);
}