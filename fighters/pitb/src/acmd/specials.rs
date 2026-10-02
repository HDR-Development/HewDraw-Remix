use super::*;

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(agent, 16.0, 31.0, 13.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: BASE_SEARCHBOX, id: 0, bone: "top", size: 2.0, x: 0.0, y: 12.0, z: 9.0, x2: 0.0, y2: 4.0, z2: 9.0, facing: LrCheck::F, });
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        agent.on_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        agent.on_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF);
    }
    frame(lua_state, 31.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        agent.on_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MTRANS_AIR_UNABLE);
        agent.off_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        // agent.off_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF);
    }
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: BASE_HITBOX, id: 0, bone: "top", dmg: 16.0, angle: 40, kbg: 80, bkb: 57, size: 6.0, x: 0.0, y: 4.0, z: 9.0, x2: 0.0, y2: 10.0, z2: 9.0, hitlag: 1.15, clank:SetOff::Thru, facing: LrCheck::F, shield_dmg: ShieldDamage::Add(16.0), effect: "collision_attr_elec", sound_level: SoundLevel::L, hit_sound: CollisionSound::Elec, region: AttackRegion::Punch, });
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE_RANGE(agent, 5.0, 16.0, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(agent, 16.0, 52.0, 30.0);
    frame(lua_state, 52.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 20.0, 15.0);
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_s");
        agent.on_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT);
    }
    frame(lua_state, 20.0); // F16
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        hitbox!(agent, { extends: BASE_SEARCHBOX, id: 0, bone: "top", size: 2.0, x: 0.0, y: 14.0, z: 9.0, x2: 0.0, y2: 4.0, z2: 9.0, facing: LrCheck::F, });
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        agent.on_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let start_frame = ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_hi.start_frame") as f32;
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 15.0, start_frame);
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi_start");
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: BASE_HITBOX, id: 0, bone: "bust", dmg: 14.0, angle: 75, kbg: 73, bkb: 63, size: 4.0, x: 0.0, y: -3.0, z: -1.0, facing: LrCheck::F, effect: "collision_attr_magic", sound_level: SoundLevel::L, hit_sound: CollisionSound::Magic, region: AttackRegion::Body, });
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: BASE_HITBOX, id: 0, bone: "bust", dmg: 8.0, angle: 75, kbg: 73, bkb: 73, size: 3.5, x: 0.0, y: -3.0, z: -1.0, facing: LrCheck::F, effect: "collision_attr_magic", sound_level: SoundLevel::M, hit_sound: CollisionSound::Magic, region: AttackRegion::Body, });
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_FIX_ANGLE);
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_BACK_ANGLE);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhiend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 3.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 6.9, 1.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
    }
    frame(lua_state, 6.9);
    FT_MOTION_RATE_RANGE(agent, 6.9, 7.0, 3.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: BASE_HITBOX, id: 0, bone: "top", dmg: 5.0, angle: 70, kbg: 85, bkb: 77, size: 7.5, x: 0.0, y: 7.0, z: -1.5, x2: 0.0, y2: 7.0, z2: 1.5, hitlag: 0.72, clank: SetOff::Off, effect: "collision_attr_magic", sound_level: SoundLevel::M, hit_sound: CollisionSound::Magic, region: AttackRegion::None, });
    }
    frame(lua_state, 6.933);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let facing = PostureModule::lr(boma);
        let rot1 = if facing > 0.0 { 240 } else { 300 };
        let rot2 = if facing > 0.0 { 120 } else { 60 };
        EFFECT_FOLLOW(agent, Hash40::new("pitb_guardian_shield"), Hash40::new("virtualguardianf"), 2.0 * facing, 3, -2, 0, rot1, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 5.0, 1.0, 1.0);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_guardian_shield"), Hash40::new("virtualguardianb"), 2.0 * facing, 3, 2, 0, rot2, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 5.0, 1.0, 1.0);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pitb_special_l01"));
    }
}

unsafe extern "C" fn game_speciallwhold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
    }
}

unsafe extern "C" fn effect_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let facing = PostureModule::lr(boma);
        let rot1 = if facing > 0.0 { 240 } else { 300 };
        let rot2 = if facing > 0.0 { 120 } else { 60 };
        EFFECT(agent, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), 2.0 * facing, 3, -2, 0, rot1, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), 2.0 * facing, 3, 2, 0, rot2, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_guardian_shield"), false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialsend", game_specialsend, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialairsstart, Priority::Low);

    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhiend", game_specialairhiend, Priority::Low);
    
    agent.acmd("game_speciallwstartl", game_speciallwstart, Priority::Low);
    agent.acmd("game_speciallwstartr", game_speciallwstart, Priority::Low);
    agent.acmd("game_specialairlwstartl", game_speciallwstart, Priority::Low);
    agent.acmd("game_specialairlwstartr", game_speciallwstart, Priority::Low);
    agent.acmd("effect_speciallwstartl", effect_speciallwstart, Priority::Low);
    agent.acmd("effect_speciallwstartr", effect_speciallwstart, Priority::Low);
    agent.acmd("effect_specialairlwstartl", effect_speciallwstart, Priority::Low);
    agent.acmd("effect_specialairlwstartr", effect_speciallwstart, Priority::Low);
    agent.acmd("sound_speciallwstartl", sound_speciallwstart, Priority::Low);
    agent.acmd("sound_speciallwstartr", sound_speciallwstart, Priority::Low);
    agent.acmd("sound_specialairlwstartl", sound_speciallwstart, Priority::Low);
    agent.acmd("sound_specialairlwstartr", sound_speciallwstart, Priority::Low);

    agent.acmd("game_speciallwhold", game_speciallwhold, Priority::Low);
    agent.acmd("game_specialairlwhold", game_speciallwhold, Priority::Low);

    agent.acmd("effect_speciallwendl", effect_speciallwend, Priority::Low);
    agent.acmd("effect_speciallwendr", effect_speciallwend, Priority::Low);
    agent.acmd("effect_specialairlwendl", effect_speciallwend, Priority::Low);
    agent.acmd("effect_specialairlwendr", effect_speciallwend, Priority::Low);
}