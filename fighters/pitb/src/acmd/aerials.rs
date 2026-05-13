use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE_RANGE(agent, 5.0, 24.0, 11.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "top", dmg: 4.0, angle: 366, kbg: 100, fkb: 40, bkb: 0, size: 8.0, x: 1.0, y: 9.0, z: 5.5, facing: LrCheck::F, });
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 0, bone: "top", dmg: 7.0, angle: 361, kbg: 85, bkb: 50, size: 9.0, x: 1.0, y: 10.0, z: 3.0, facing: LrCheck::F, });
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pitb_atk_air_n_hdr"), Hash40::new("bowr"), 0, 0, 0, -90, 90, 0, 0.9, true);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("bowr"), 0, 0.87, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("bowr"), 0, -0.87, 0, 180, -90, 0, 0.9, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_atk_air_n_hdr"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_sword"), false, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pitb_atk_air_n_hdr"), Hash40::new("bowl"), 0, 0, 0, -90, 90, 0, 1.3, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 3, Hash40::new("swordr1"), 0.0, 1.0, -0.2, Hash40::new("swordr1"), 0.0, 11.4, -1.2, true, Hash40::new("null"), Hash40::new("swordr1"), 0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 9, 2, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_atk_air_n_hdr"), true, true);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn sound_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_pitb_attackair_n01"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_pitb_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_pitb_swing_m"));
    }
}

unsafe extern "C" fn expression_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_status_default_int64(boma, Hash40::new("weapon").hash as i64, 0x11242751f5 as i64);
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("swordr1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashss"), 0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE_RANGE(agent, 12.0, 24.0, 9.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "armr", dmg:  9.0, angle:  40, kbg:  91, bkb: 22, size: 3.0, x: 0.0, y: 0.0, z: 0.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 1, bone: "top",  dmg:  9.0, angle:  40, kbg:  91, bkb: 22, size: 3.0, x: 0.0, y: 5.0, z: 7.0, x2: 0.0, y2: 5.0, z2: 10.0 });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 2, bone: "top",  dmg: 11.0, angle: 361, kbg: 104, bkb: 22, size: 3.0, x: 0.0, y: 5.0, z: 7.0, x2: 0.0, y2: 5.0, z2: 21.0 });
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "armr", dmg: 9.0, angle: 40, kbg: 91, bkb: 22, size: 3.0, x: 0.0, y: 0.0, z: 0.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 1, bone: "top",  dmg: 9.0, angle: 40, kbg: 91, bkb: 22, size: 3.0, x: 0.0, y: 5.0, z: 7.0, x2: 0.0, y2: 5.0, z2: 10.0 });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 2, bone: "top",  dmg: 9.0, angle: 40, kbg: 91, bkb: 22, size: 3.0, x: 0.0, y: 5.0, z: 7.0, x2: 0.0, y2: 5.0, z2: 21.0 });
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 39.0, 18.0);
    frame(lua_state, 34.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 39.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pitb_atk_air_n_hdr"), Hash40::new("bowr"), 0, 0, 0, 90, -90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("bowr"), 0, 0.87, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("bowr"), 0, -0.87, 0, 180, -90, 0, 1, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_atk_air_n_hdr"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_sword"), false, false);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 11.0, 3.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "top", dmg: 10.0, angle: 43, kbg: 100, bkb: 28, size: 3.5, x: 0.0, y: 6.2, z: -11.0, x2: 0.0, y2: 6.2, z2:  -6.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 1, bone: "top", dmg: 13.0, angle: 43, kbg: 101, bkb: 31, size: 3.7, x: 0.0, y: 6.2, z: -18.9, x2: 0.0, y2: 6.2, z2: -15.0, });
    }
    frame(lua_state, 10.66);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "top", dmg: 8.0, angle: 43, kbg: 100, bkb: 30, size: 3.5, x: 0.0, y: 6.2, z: -16.4, x2: 0.0, y2: 6.2, z2: -6.0, });
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pit_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("pit_sword"), Hash40::new("swordl"), 0, 0, 0, 180, 90, 0, 1, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -6, 0, 180, 0, 0.8, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pit_sword"), false, false);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    FT_MOTION_RATE_RANGE(agent, 5.0, 13.0, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 0, bone: "top", dmg:  9.0, angle: 75, kbg: 114, bkb: 20, size: 3.5, x: 0.0, y: 11.0, z:  0.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 1, bone: "top", dmg: 11.0, angle: 75, kbg: 114, bkb: 20, size: 4.5, x: 0.0, y: 17.0, z: -5.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 2, bone: "top", dmg: 11.0, angle: 75, kbg: 114, bkb: 20, size: 4.5, x: 0.0, y: 17.0, z:  7.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 3, bone: "top", dmg: 11.0, angle: 75, kbg: 114, bkb: 20, size: 4.5, x: 0.0, y: 17.0, z:  2.0, });
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "top", dmg: 7.0, angle: 75, kbg: 108, bkb: 20, size: 3.5, x: 0.0, y:  9.6, z:  0.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 1, bone: "top", dmg: 9.0, angle: 75, kbg: 108, bkb: 20, size: 4.0, x: 0.0, y: 17.0, z: -5.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 2, bone: "top", dmg: 9.0, angle: 75, kbg: 108, bkb: 20, size: 4.0, x: 0.0, y: 17.0, z:  7.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 3, bone: "top", dmg: 9.0, angle: 75, kbg: 108, bkb: 20, size: 4.0, x: 0.0, y: 17.0, z:  2.0, });
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pitb_atk_air_n_hdr"), Hash40::new("swordr2"), 0, 0, 0, -90, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("swordr2"), 0, -0.8, 0, 180, -90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_sword"), Hash40::new("swordr2"), 0, 0.8, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_atk_air_n_hdr"), false, false);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_sword"), false, false);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(agent, 9.0, 10.0, 2.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "arml", dmg: 9.0, angle: 54, kbg: 88, bkb: 45, size: 3.0, x: 2.0, y:  0.0, z:  0.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 1, bone: "bowl", dmg: 9.0, angle: 54, kbg: 88, bkb: 45, size: 3.5, x: 0.0, y: -2.0, z: -2.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 2, bone: "bowl", dmg: 9.0, angle: 54, kbg: 88, bkb: 45, size: 3.5, x: 0.0, y: -6.0, z: -2.0, });
    }
    frame(lua_state, 9.5);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "arml", dmg:  9.0, angle:  54, kbg: 88, bkb: 45, size: 3.0, x: 2.0, y:  0.0, z:  0.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 1, bone: "bowl", dmg: 10.0, angle: 270, kbg: 56, bkb: 22, size: 3.5, x: 0.0, y: -2.0, z: -2.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_L, id: 2, bone: "bowl", dmg: 10.0, angle: 270, kbg: 56, bkb: 22, size: 3.5, x: 0.0, y: -6.0, z: -2.0, });
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 11.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 0, bone: "arml", dmg: 9.0, angle: 54, kbg: 76, bkb: 39, size: 3.0, x: 2.0, y:  0.0, z:  0.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 1, bone: "bowl", dmg: 9.0, angle: 54, kbg: 76, bkb: 39, size: 3.5, x: 0.0, y: -2.0, z: -2.0, });
        hitbox!(agent, { extends: PITB_SWORD_HITBOX_M, id: 2, bone: "bowl", dmg: 9.0, angle: 54, kbg: 76, bkb: 39, size: 3.5, x: 0.0, y: -6.0, z: -2.0, });
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("swordl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pitb_sword_hdr"), Hash40::new("tex_pitb_sword2"), 5, Hash40::new("swordl"), 0, 0, -0.2, Hash40::new("swordl"), 0, -10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, 0, 0, 180, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn expression_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_normal") as i64);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn, Priority::Low);
    agent.acmd("effect_attackairn", effect_attackairn, Priority::Low);
    agent.acmd("sound_attackairn", sound_attackairn, Priority::Low);
    agent.acmd("expression_attackairn", expression_attackairn, Priority::Low);

    agent.acmd("game_attackairf", game_attackairf, Priority::Low);
    agent.acmd("effect_attackairf", effect_attackairf, Priority::Low);

    agent.acmd("game_attackairb", game_attackairb, Priority::Low);
    agent.acmd("effect_attackairb", effect_attackairb, Priority::Low);

    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi", effect_attackairhi, Priority::Low);
    
    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw", effect_attackairlw, Priority::Low);
    agent.acmd("expression_attackairlw", expression_attackairlw, Priority::Low);
}
