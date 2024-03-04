
use super::*;

unsafe extern "C" fn ryu_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.909);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma, *FIGHTER_RYU_GENERATE_ARTICLE_HADOKEN){
            VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_CURRENT_HADOKEN_EX);
            if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                if MeterModule::drain(fighter.battle_object, 1) {
                    VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
                    VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_CURRENT_HADOKEN_EX);
                }
            }
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            FT_MOTION_RATE(fighter, 0.667);
        }
        else {
            WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
            FT_MOTION_RATE(fighter, 0.702);
        }

    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if fighter.kind() != *FIGHTER_KIND_KIRBY {
            MeterModule::add(fighter.battle_object, 3.0);
        }
     }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }

}

unsafe extern "C" fn ryu_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.909);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma, *FIGHTER_RYU_GENERATE_ARTICLE_HADOKEN){
            VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_CURRENT_HADOKEN_EX);
            if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                if MeterModule::drain(fighter.battle_object, 1) {
                    VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
                    VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_CURRENT_HADOKEN_EX);
                }
            }
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            FT_MOTION_RATE(fighter, 0.667);
        }
        else {
            WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
            FT_MOTION_RATE(fighter, 0.702);
        }

    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if fighter.kind() != *FIGHTER_KIND_KIRBY {
            MeterModule::add(fighter.battle_object, 3.0);
        }
     }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }

}

pub fn install() {
    smashline::Agent::new("ryu")
        .acmd("game_specialn", ryu_special_n_game)
        .acmd("game_specialairn", ryu_special_air_n_game)
        .install();
}
