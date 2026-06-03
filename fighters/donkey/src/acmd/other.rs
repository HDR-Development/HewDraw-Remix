use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_donkey_damagefly02"));
        } else {
            let damage_speed_x = agent.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            let damage_speed_y = agent.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);

            let speed_vector = sv_math::vec2_length(damage_speed_x, damage_speed_y);

            let play_vc = if speed_vector < 3.8 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {
                PLAY_FLY_VOICE(agent, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
            }
        }
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_donkey_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
        }
    }
}

unsafe extern "C" fn expression_landingheavy(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl"), 0, false, 0x50000000 /* default value */);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        if !agent.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_AIR]) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_donkey_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_donkey_step_left_m"));
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_donkey_step_right_m"));
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }

}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }

}

unsafe extern "C" fn game_itemheavyget(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 13.0);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if !ItemModule::is_have_item(boma, 0) {
        let itemmanager = smash2::app::ItemManager::instance().unwrap();
        let barrel_count = smash2::app::ItemManager::get_num_of_ownered_item(itemmanager, boma.battle_object_id, smash2::app::ItemKind::Barrel);
        if barrel_count == 0 {
            VarModule::on_flag(agent.object(), vars::donkey::instance::SPECIAL_LW_BARREL_GENERATED);
            ItemModule::have_item(boma, ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
            EFFECT(agent, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
        }
    } else {
        VarModule::off_flag(agent.object(), vars::donkey::instance::SPECIAL_LW_BARREL_GENERATED);
    }
    if VarModule::is_flag(agent.object(), vars::donkey::instance::SPECIAL_LW_BARREL_GENERATED) {
        VarModule::off_flag(agent.object(), vars::donkey::instance::SPECIAL_LW_BARREL_GENERATED);
        FT_MOTION_RATE_RANGE(agent, 8.0, 25.0, 35.0); //26
    } else {
        FT_MOTION_RATE_RANGE(agent, 8.0, 25.0, 22.0); //18
    }
    if is_excute(agent) {
        ItemModule::pickup_item(
            agent.module_accessor,
            ItemSize {
                _address: *ITEM_SIZE_HEAVY as u8,
            },
            *FIGHTER_HAVE_ITEM_WORK_MAIN,
            *ITEM_TRAIT_ALL,
            QuickItemTreatType {
                _address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8,
            },
            ItemPickupSearchMode {
                _address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8,
            },
        );
    }
}

unsafe extern "C" fn expression_superliftlanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn expression_shoulderlanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        //QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffescape", acmd_stub, Priority::Low);

    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damageflyroll, Priority::Low);

    agent.acmd("expression_landingheavy", expression_landingheavy, Priority::Low);

    agent.acmd("game_dash", game_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("game_itemheavyget", game_itemheavyget, Priority::Low);

    agent.acmd("expression_superliftlanding", expression_superliftlanding, Priority::Low);
    agent.acmd("expression_shoulderlanding", expression_shoulderlanding, Priority::Low);
}
