use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 21.0, 6.0);
    // without set_speed_mul 0.0 here, motion speed on this frame makes the 366 fail
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "top", dmg: 5.0, angle: 366, kbg: 100, fkb: 53, bkb: 0, size: 5.4, x: 0.0, y: 7.0, z: 5.0, x2: 0.0, y2: 7.0, z2: 12.0, sdi: 1.15, facing: LrCheck::F, set_weight: true, });
    }
    frame(lua_state, 11.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 0, part: 1, bone: "top", dmg: 10.0, angle: 361, kbg: 121, bkb: 42, size: 6.0, x: 0.0, y: 8.0, z: 13.5, x2: 0.0, y2: 8.0, z2: 6.0, facing: LrCheck::F, shield_dmg: ShieldDamage::Add(10.0), });
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 55.0, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, -5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 3, Hash40::new("swordr1"), 0, 0, -0.2, Hash40::new("swordr1"), 0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 5, Hash40::new("swordr1"), 0, 0.9, -0.2, Hash40::new("swordr1"), 0, 11, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0.9, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordl"), 0, -0.9, -0.2, Hash40::new("swordl"), 0, -11, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, -0.9, 0, 180, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 5);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 0);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 52, size: 5.0, x: 0.0, y: 26.5, z:  0.0, hitlag: 1.25, sdi: 0.5, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 1, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 52, size: 5.5, x: 0.0, y: 24.0, z:  7.0, x2: 0.0, y2: 24.0, z2:  4.5, hitlag: 1.25, sdi: 0.5, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 2, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 52, size: 5.5, x: 0.0, y: 24.0, z: -6.0, x2: 0.0, y2: 24.0, z2: -4.5, hitlag: 1.25, sdi: 0.5, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 3, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 52, size: 5.5, x: 0.0, y: 24.0, z:  7.0, x2: 0.0, y2: 24.0, z2: -6.0, hitlag: 1.25, sdi: 0.5, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 4, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 52, size: 5.5, x: 0.0, y: 18.0, z: -5.5, x2: 0.0, y2: 18.0, z2: -4.5, hitlag: 1.25, sdi: 0.5, });
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE_RANGE(agent, 7.0, 9.0, 1.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 52, size: 6.0, x: 0.0, y: 14.0, z: 9.0, x2: 0.0, y2: 12.0, z2: 7.0, hitlag: 1.25, sdi: 0.5, });
        AttackModule::clear(boma, 1, false);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
        AttackModule::clear(boma, 4, false);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 12.0, 1.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 0, part: 1, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 62, size: 5.5, x: 0.0, y: 28.0, z:  0.0, hitlag: 1.25, sdi: 0.5, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 1, part: 1, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 62, size: 6.0, x: 0.0, y: 24.0, z:  6.0, hitlag: 1.25, sdi: 0.5, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 2, part: 1, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 62, size: 6.0, x: 0.0, y: 24.0, z: -6.0, hitlag: 1.25, sdi: 0.5, });
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE_RANGE(agent, 12.0, 16.0, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 18.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 0, part: 2, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 62, size: 6.0, x: 0.0, y: 34.0, z:  0.0, hitlag: 1.25, shield_dmg: ShieldDamage::Add(10.0), });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 1, part: 2, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 62, size: 7.0, x: 0.0, y: 24.0, z:  0.0, hitlag: 1.25, shield_dmg: ShieldDamage::Add(10.0), });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 2, part: 2, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 62, size: 7.0, x: 0.0, y: 31.0, z:  6.0, hitlag: 1.25, shield_dmg: ShieldDamage::Add(10.0), });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 3, part: 2, bone: "top", dmg: 6.0, angle: 90, kbg: 139, bkb: 62, size: 7.0, x: 0.0, y: 31.0, z: -6.0, hitlag: 1.25, shield_dmg: ShieldDamage::Add(10.0), });
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, -8, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 5, Hash40::new("swordr1"), 0, 0, -0.2, Hash40::new("swordr1"), 0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 5, Hash40::new("swordl"), 0, 0, -0.2, Hash40::new("swordl"), 0, -10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, 0, 0, 180, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 5, Hash40::new("swordr1"), 0, 0, -0.2, Hash40::new("swordr1"), 0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 0, bone: "top", dmg: 12.0, angle: 48, kbg: 98, bkb: 40, size: 3.7, x: 0.0, y: 3.3, z:  6.4, shield_dmg: ShieldDamage::Add(5.0), });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 1, bone: "top", dmg: 12.0, angle: 48, kbg: 98, bkb: 40, size: 3.6, x: 0.0, y: 2.8, z: 12.4, shield_dmg: ShieldDamage::Add(5.0), });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 2, bone: "top", dmg: 10.0, angle: 48, kbg: 93, bkb: 35, size: 3.4, x: 0.0, y: 2.0, z: 17.4, shield_dmg: ShieldDamage::Add(5.0), });
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 0, bone: "top", dmg: 12.0, angle: 30, kbg: 93, bkb: 25, size: 3.7, x: 0.0, y: 3.3, z:  -8.4, shield_dmg: ShieldDamage::Add(5.0), });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 1, bone: "top", dmg: 12.0, angle: 30, kbg: 93, bkb: 25, size: 3.6, x: 0.0, y: 2.8, z: -14.4, shield_dmg: ShieldDamage::Add(5.0), });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 2, bone: "top", dmg: 10.0, angle: 30, kbg: 93, bkb: 25, size: 3.4, x: 0.0, y: 2.0, z: -19.4, shield_dmg: ShieldDamage::Add(5.0), });
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, -3, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0, 0, -0.2, Hash40::new("swordr1"), 0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 5.0);
    if PostureModule::lr(agent.module_accessor) < 0.0 {
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        else {
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, -2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    }
}
frame(lua_state, 7.0);
if is_excute(agent) {
    AFTER_IMAGE_OFF(agent, 3);
}
frame(lua_state, 16.0);
if is_excute(agent) {
    EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, 0, 0, 180, 90, 0, 1, true);
}
frame(lua_state, 17.0);
if PostureModule::lr(agent.module_accessor) < 0.0 {
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, -3.5, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    else {
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 3.5, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}
}
frame(lua_state, 18.0);
if is_excute(agent) {
EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 3, 3, -12, 23, 197, 6, 0.6, true);
}
frame(lua_state, 22.0);
if is_excute(agent) {
EFFECT_OFF_KIND(agent, Hash40::new("pitb_sword"), false, true);
}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);
    agent.acmd("effect_attacks4", effect_attacks4, Priority::Low);
    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    agent.acmd("effect_attackhi4", effect_attackhi4, Priority::Low);
    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
    agent.acmd("effect_attacklw4", effect_attacklw4, Priority::Low);
}