
use super::*;


#[acmd_script( agent = "ryu", scripts = ["game_specialn", "game_specialairn"] , category = ACMD_GAME , low_priority)]
unsafe fn game_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if boma.is_button_on(Buttons::Guard | Buttons::GuardHold)
        && !ArticleModule::is_exist(boma, *FIGHTER_RYU_GENERATE_ARTICLE_HADOKEN) {
            WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED);
        } else {
            WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
            if fighter.kind() != *FIGHTER_KIND_KIRBY 
            && !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                MeterModule::add(fighter.battle_object, 2.0);
            }
        }
    }
    frame(lua_state, 14.0);
    if WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        FT_MOTION_RATE_RANGE(fighter, 14.0, 58.0, 18.0);
    } else {
        FT_MOTION_RATE_RANGE(fighter, 14.0, 58.0, 36.0);
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 58.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "ryu", scripts = ["effect_specialn", "effect_specialairn"] , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 4.0);
    if !WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        if is_excute(fighter) {
            if fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE) == 0 {
                EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
            } else {
                EFFECT_FOLLOW(fighter, Hash40::new("ryu_syakunetsu_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            } else {
                FLASH(fighter, 0.392, 1, 1, 0.353);
            }
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 11.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
            
            if fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE) == 0 {
                EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_shot"), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);
            } else {
                EFFECT_FOLLOW(fighter, Hash40::new("ryu_syakunetsu_shot"), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);
            }
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("ryu_hadoken_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        for _ in 0..6 {
            wait(lua_state, 3.0);
            if is_excute(fighter)
            && VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter)
            && VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter)
        && VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(fighter);
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

pub fn install() {
    install_acmd_scripts!(
        game_specialn,
        effect_specialn
    );
}
