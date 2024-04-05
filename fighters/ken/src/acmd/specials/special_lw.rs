use super::*;

unsafe extern "C" fn game_speciallwinstall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 11.0, 5.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallwinstall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.2, true);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 5, 12, 3, 0, 0, 0, 0.8, true, *EF_FLIP_AXIS_YZ);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn sound_speciallwinstall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ken_appeal_l01"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ken_appeal_l01"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        let sfx_handle = SoundModule::play_se(boma, Hash40::new("se_common_final_cutin"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, sfx_handle as i32, 0.5, 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ken_appeal_l02"));
    }
}

unsafe extern "C" fn expression_speciallwinstall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallwstepf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 6.0 / 2.0);
    if is_excute(agent) {
        GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_ENABLE_SPECIAL_LW_INSTALL) {
            VarModule::set_flag(
                agent.battle_object, 
                vars::shotos::status::IS_ENABLE_MAGIC_SERIES_CANCEL, 
                MeterModule::level(agent.battle_object) >= 6
            );
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
                MeterModule::drain_direct(agent.battle_object, 0.5 * MeterModule::meter_per_level(agent.battle_object));
            } else {
                MeterModule::drain_direct(agent.battle_object, 1.0 * MeterModule::meter_per_level(agent.battle_object));
            }
        } else {
            VarModule::off_flag(agent.battle_object, vars::shotos::status::IS_ENABLE_MAGIC_SERIES_CANCEL);
        }
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_ENABLE_SPECIAL_LW_INSTALL);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0 / (26.0 - 18.0));
}

unsafe extern "C" fn effect_speciallwstepf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true);

    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_specialairlwstepf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 6.0 / 2.0);
    if is_excute(agent) {
        sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.75);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_ENABLE_SPECIAL_LW_INSTALL) {
            VarModule::set_flag(
                agent.battle_object, 
                vars::shotos::status::IS_ENABLE_MAGIC_SERIES_CANCEL, 
                MeterModule::level(agent.battle_object) >= 6
            );
            if !VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
                MeterModule::drain_direct(agent.battle_object, 1.0 * MeterModule::meter_per_level(agent.battle_object));
            }
        } else {
            VarModule::off_flag(agent.battle_object, vars::shotos::status::IS_ENABLE_MAGIC_SERIES_CANCEL);
        }
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_ENABLE_SPECIAL_LW_INSTALL);
    }
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialairlwstepf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
}

unsafe extern "C" fn stub(agent: &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallwinstall", game_speciallwinstall);
    agent.acmd("effect_speciallwinstall", effect_speciallwinstall);
    agent.acmd("sound_speciallwinstall", sound_speciallwinstall);
    agent.acmd("expression_speciallwinstall", expression_speciallwinstall);
    agent.acmd("game_speciallwstepf", game_speciallwstepf);
    agent.acmd("effect_speciallwstepf", effect_speciallwstepf);
    agent.acmd("game_specialairlwstepf", game_specialairlwstepf);
    agent.acmd("effect_specialairlwstepf", effect_specialairlwstepf);
    agent.acmd("effect_speciallwstart", stub);
    agent.acmd("effect_specialairlwstart", stub);
}
