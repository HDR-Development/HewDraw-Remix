use super::*;

unsafe extern "C" fn game_lucasspecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 10, 0, 0, 55, 14.0, 0.0, 10.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn sound_lucasspecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_lucas_smash_l03"));
        PLAY_SE_REMAIN(agent, Hash40::new("vc_kirby_010"));
    }
}

unsafe extern "C" fn game_lucasspecialnhold(agent: &mut L2CAgentBase) {
    // INTENTIONALLY LEFT BLANK
    /* if agent.kind() == *FIGHTER_KIND_KIRBY {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    } */
}

unsafe extern "C" fn effect_lucasspecialnhold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 9, 0, 9, 0, 0, 0, false);
        FLASH(agent, 0.01, 0.5, 1, 0.4);
    }
    for i in 1..=50 {
        if is_excute(agent) {
            if i%2==0 {
                EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_hold"), false, false);
                EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_hold"), Hash40::new("top"), 0, sv_math::rand(hash40("fighter"), 4) as i32 + 12, sv_math::rand(hash40("fighter"), 4) as i32 - 2, 0, 0, 0, 0.5, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_status_defense_up"), false, false);
                EFFECT_FLW_POS(agent, Hash40::new("sys_status_defense_up"), Hash40::new("top"), 0, sv_math::rand(hash40("fighter"), 4) as i32 + 12, sv_math::rand(hash40("fighter"), 4) as i32 - 2, 0, 0, 0, 0.2, true);
            }
            if i%4==0 {
                EFFECT_FLW_POS(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, false);
            }
            FLASH(agent, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(agent){
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(agent){
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn sound_lucasspecialnhold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_lucas_special_h02"));
        PLAY_STATUS(agent, Hash40::new("se_lucas_pk_charge"));
    }
}

unsafe extern "C" fn game_lucasspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT) {
        frame(lua_state, 2.0);
        if is_excute(agent) {
            VarModule::on_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 115, 0, 50, 3.0, 0.0, 10.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 100, 0, 50, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            AttackModule::clear_all(boma);
        }
    } else {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_hold"), false, false);
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
        }

    }
}

unsafe extern "C" fn effect_lucasspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("lucas_pkt_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.9, true);
        EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.5, true);
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 1..=5 {
        if is_excute(agent) {
            FLASH(agent, 0.01, 0.5, 1, 0.4);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 3.0)
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkt_hold"), false, false);
    }
}

unsafe extern "C" fn sound_lucasspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("vc_kirby_attack04"));
        PLAY_SE_REMAIN(agent, Hash40::new("se_lucas_special_n04_l"));
        PLAY_SE_REMAIN(agent, Hash40::new("se_common_electric_hit_m"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_lucasspecialnstart", game_lucasspecialnstart);
    agent.acmd("game_lucasspecialairnstart", game_lucasspecialnstart);
    agent.acmd("sound_lucasspecialnstart", sound_lucasspecialnstart);
    agent.acmd("sound_lucasspecialairnstart", sound_lucasspecialnstart);
    agent.acmd("game_lucasspecialnhold", game_lucasspecialnhold);
    agent.acmd("game_lucasspecialairnhold", game_lucasspecialnhold);
    agent.acmd("effect_lucasspecialnhold", effect_lucasspecialnhold);
    agent.acmd("effect_lucasspecialairnhold", effect_lucasspecialnhold);
    agent.acmd("sound_lucasspecialairnhold", effect_lucasspecialnhold);
    agent.acmd("sound_lucasspecialnhold", effect_lucasspecialnhold);
    agent.acmd("game_lucasspecialnfire", game_lucasspecialnfire);
    agent.acmd("game_lucasspecialairnfire", game_lucasspecialnfire);
    agent.acmd("effect_lucasspecialnfire", effect_lucasspecialnfire);
    agent.acmd("effect_lucasspecialairnfire", effect_lucasspecialnfire);
    agent.acmd("sound_lucasspecialairnfire", sound_lucasspecialnfire);
    agent.acmd("sound_lucasspecialnfire", sound_lucasspecialnfire);
}