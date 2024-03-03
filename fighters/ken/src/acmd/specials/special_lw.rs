
use super::*;

unsafe extern "C" fn game_speciallwstepf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 6.0 / 2.0);
    if is_excute(fighter) {
        GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    frame(lua_state, 2.0);
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
        .acmd("game_speciallwstepf", game_speciallwstepf)
        .acmd("effect_speciallwstepf", effect_speciallwstepf)
        .acmd("game_specialairlwstepf", game_specialairlwstepf)
        .acmd("effect_specialairlwstepf", effect_specialairlwstepf)
        .acmd("effect_speciallwstart", effect_speciallwstart)
        .acmd("effect_specialairlwstart", effect_specialairlwstart)
        .install();
}
