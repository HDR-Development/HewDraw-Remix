
use super::*;

unsafe extern "C" fn game_speciallwinstall(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 11.0, 5.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn effect_speciallwinstall(agent: &mut L2CAgentBase) { }

unsafe extern "C" fn sound_speciallwinstall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ken_appeal_l01"));
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ken_appeal_l01"));
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        let sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_common_final_cutin"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, sfx_handle as i32, 0.5, 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ken_appeal_l02"));
    }
}

unsafe extern "C" fn expression_speciallwinstall(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallwstepf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 6.0 / 2.0);
    if is_excute(fighter) {
        GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_ENABLE_SPECIAL_LW_INSTALL) {
            VarModule::set_flag(
                fighter.battle_object, 
                vars::shotos::status::IS_ENABLE_MAGIC_SERIES_CANCEL, 
                MeterModule::level(fighter.battle_object) >= 6
            );
            if !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
                MeterModule::drain_direct(fighter.battle_object, 1.0 * MeterModule::meter_per_level(fighter.battle_object));
            }
        } else {
            VarModule::off_flag(fighter.battle_object, vars::shotos::status::IS_ENABLE_MAGIC_SERIES_CANCEL);
        }
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_ENABLE_SPECIAL_LW_INSTALL);
    }
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(fighter, 1.0 / (26.0 - 18.0));
}

unsafe extern "C" fn effect_speciallwstepf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true);

    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_specialairlwstepf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 6.0 / 2.0);
    if is_excute(fighter) {
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.75);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn effect_specialairlwstepf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
}

unsafe extern "C" fn effect_speciallwstart(fighter: &mut L2CAgentBase) {
    // stub
}

unsafe extern "C" fn effect_specialairlwstart(fighter: &mut L2CAgentBase) {
    // stub
}

pub fn install() {
    smashline::Agent::new("ken")
        .acmd("game_speciallwinstall", game_speciallwinstall)
        .acmd("effect_speciallwinstall", effect_speciallwinstall)
        .acmd("sound_speciallwinstall", sound_speciallwinstall)
        .acmd("expression_speciallwinstall", expression_speciallwinstall)
        .acmd("game_speciallwstepf", game_speciallwstepf)
        .acmd("effect_speciallwstepf", effect_speciallwstepf)
        .acmd("game_specialairlwstepf", game_specialairlwstepf)
        .acmd("effect_specialairlwstepf", effect_specialairlwstepf)
        .acmd("effect_speciallwstart", effect_speciallwstart)
        .acmd("effect_specialairlwstart", effect_specialairlwstart)
        .install();
}
