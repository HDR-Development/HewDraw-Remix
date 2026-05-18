use super::*;

hitbox_templates!(
    pub KROOL_PROPELLER = {
        extends: BASE_HITBOX,
        effect: "collision_attr_cutup",
        hit_sound: CollisionSound::Cutup,
        region: AttackRegion::Object,
        clank: SetOff::Off,
    };
    pub KROOL_PROPELLER_S = {
        extends: KROOL_PROPELLER,
        sound_level: SoundLevel::S,
        hitlag: 0.5,
    };
    pub KROOL_PROPELLER_M = {
        extends: KROOL_PROPELLER,
        sound_level: SoundLevel::M,
        hitlag: 0.5,
    };
    pub KROOL_PROPELLER_L = {
        extends: KROOL_PROPELLER,
        sound_level: SoundLevel::L,
    };
);

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: KROOL_PROPELLER_M, id: 0, bone: "top",  dmg: 3.0, angle: 100, kbg: 100, fkb: 85, bkb: 0, size: 3.5, x: 0.0, y: 16.5, z: -6.0, x2: 0.0, y2: 16.5, z2: 6.0, rehit: 10, set_weight: true, });
        hitbox!(agent, { extends: KROOL_PROPELLER_S, id: 1, bone: "head", dmg: 3.0, angle:  90, kbg: 100, fkb: 50, bkb: 0, size: 5.0, x: 0.0, y:  0.0, z:  0.0, rehit: 10, set_weight: true, });
    }
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    for _ in 0..50 {
        wait(lua_state, 12.0);
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 12.0);
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_h02"));
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK) {
            ArticleModule::change_motion(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, Hash40::new("fly"), false, 0.0);
        }
        hitbox!(agent, { extends: KROOL_PROPELLER_M, id: 0, part: 1, bone: "top",  dmg: 3.0, angle: 100, kbg: 100, fkb: 85, bkb: 0, size: 3.5, x: 0.0, y: 16.5, z: -6.0, x2: 0.0, y2: 16.5, z2: 6.0, rehit: 8, set_weight: true, });
        hitbox!(agent, { extends: KROOL_PROPELLER_S, id: 1, part: 1, bone: "head", dmg: 3.0, angle:  90, kbg: 100, fkb: 50, bkb: 0, size: 5.0, x: 0.0, y:  0.0, z:  0.0, rehit: 8, set_weight: true, });
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_krool_special_h02"));
        PLAY_STATUS(agent, Hash40::new("se_krool_special_h01"));
        PLAY_SE(agent, Hash40::new("se_common_swing_08"));
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_jet"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let charge = (VarModule::get_int(boma.object(), vars::krool::instance::SPECIAL_HI_FUEL));
        if charge >= 35 {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        } else {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
}

unsafe extern "C" fn game_specialhiairend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let charge = VarModule::get_int(agent.object(), vars::krool::instance::SPECIAL_HI_FUEL) as f32;
        let damage =  4.0 + if charge > 0.0 { (charge * 4.0/charge).clamp(0.0, 4.0) } else { 0.0 };
        let mut hitbox1 = decl_hitbox!{extends: KROOL_PROPELLER_L, id: 0, part: 2, bone: "top",  dmg: 0.0, angle: 90, kbg: 100, bkb: 60, size: 4.5, x: 0.0, y: 16.0, z: -6.0, x2: 0.0, y2: 16.0, z2: 6.0 };
        let mut hitbox0 = decl_hitbox!{extends: KROOL_PROPELLER_L, id: 1, part: 2, bone: "head", dmg: 0.0, angle: 90, kbg: 100, bkb: 60, size: 6.0, x: 0.0, y:  0.0, z:  0.0, };
        hitbox0.dmg = damage;
        hitbox1.dmg = damage;
        create_hitbox(agent, &hitbox0);
        create_hitbox(agent, &hitbox1);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);
    agent.acmd("effect_specialhistart", effect_specialhistart, Priority::Low);
    agent.acmd("sound_specialhistart", sound_specialhistart, Priority::Low);
    agent.acmd("sound_specialairhistart", sound_specialhistart, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", acmd_stub, Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi, Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi, Priority::Low);
    agent.acmd("game_specialhiairend", game_specialhiairend, Priority::Low);
}