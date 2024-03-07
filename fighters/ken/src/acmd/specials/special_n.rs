
use super::*;

unsafe extern "C" fn ken_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 12.0);
    // checking SITUATION_KIND_AIR so we don't get a ground hadouken on the 1 frame of landing
    // I could just rewrite the status script to prevent this but thats a lot.
    if is_excute(fighter) && !boma.is_prev_situation(*SITUATION_KIND_AIR) {
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_CURRENT_HADOKEN_AIR);
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
        if fighter.kind() != *FIGHTER_KIND_KIRBY && !fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
            MeterModule::add(fighter.battle_object, 2.0);
        }
    }
    frame(lua_state, 14.0);
    if boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        FT_MOTION_RATE_RANGE(fighter, 14.0, 58.0, 18.0);
    } else {
        FT_MOTION_RATE_RANGE(fighter, 14.0, 58.0, 36.0);
    }
    if is_excute(fighter) {
        boma.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        boma.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 58.0);
    FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn ken_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_CURRENT_HADOKEN_AIR);
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
        if fighter.kind() != *FIGHTER_KIND_KIRBY && !fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
            MeterModule::add(fighter.battle_object, 2.0);
        }
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 36.0 / (70.0 - 15.0));
    if is_excute(fighter) {
        boma.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        boma.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        boma.off_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 70.0);
    FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn sound_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_ken_special_n03"));
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("vc_ken_special_n02"));
        }
    } else{
        if !boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("se_ken_special_n01"));
            }
            wait(lua_state, 10.0);
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("se_ken_special_n03"));
            }
            wait(lua_state, 2.0);
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("vc_ken_special_n01"));
            }
        } else {
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("se_ken_command_success"));
                PLAY_SE(fighter, Hash40::new("se_ken_special_n01"));
            }
            wait(lua_state, 10.0);
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("se_ken_special_n03"));
            }
            wait(lua_state, 2.0);
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("vc_ken_special_n01_command"));
            }
        }
    }
}

unsafe extern "C" fn effect_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 4.0);
    if !boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            } else {
                FLASH(fighter, 0.392, 1, 1, 0.353);
            }
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 11.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_shot"), Hash40::new("top"), 0, 11.5, 14.5, 0, 0, 0, 1, true);
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("ken_hadoken_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        for _ in 0..6 {
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                    FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
                }
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                    FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
                }
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                COL_NORMAL(fighter);
            }
        }
    }
    else{
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 12, 0, -4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_misfire"), Hash40::new("throw"), 0, 0, 0, 90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn effect_specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    if !boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        frame(lua_state, 4.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_hold"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            } else {
                FLASH(fighter, 0.392, 1, 1, 0.353);
            }
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 11.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_shot"), Hash40::new("top"), 0, 6, 11, 30, 0, 0, 1, true);
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND) {
                COL_NORMAL(fighter);
            }
        }
    } else {
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_misfire"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

pub fn install() {
    smashline::Agent::new("ken")
        .acmd("game_specialn", ken_special_n_game)
        .acmd("game_specialairn", ken_special_air_n_game)
        .acmd("sound_specialn", sound_specialn)
        .acmd("sound_specialairn", sound_specialn)
        .acmd("effect_specialn", effect_specialn)
        .acmd("effect_specialairn", effect_specialairn)
        .install();
}
