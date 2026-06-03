use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_brave_damagefly02"));
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
                PLAY_FLY_VOICE(agent, Hash40::new("seq_brave_rnd_futtobi01"), Hash40::new("seq_brave_rnd_futtobi02"));
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
            PLAY_SE(agent, Hash40::new("vc_brave_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_brave_rnd_futtobi01"), Hash40::new("seq_brave_rnd_futtobi02"));
        }
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.4);
    frame(lua_state, 11.0); // Effectively F15
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_brave_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.2);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0); // Effectively F15
    FT_MOTION_RATE(agent, 1.0);
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

unsafe extern "C" fn game_appealhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if app::smashball::is_training_mode() && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            let mut brave_fighter = app::Fighter {
                battle_object: *(agent.battle_object),
            };
            FighterSpecializer_Brave::add_sp(&mut brave_fighter, 100.0);
        }
    }
}

unsafe extern "C" fn game_appeals(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if app::smashball::is_training_mode() {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                let index = VarModule::get_int(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_INDEX);
                if index < 20 {
                    if index + 1 == 15 {
                        // account for removed Zoom
                        VarModule::inc_int(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_INDEX);
                    }
                    VarModule::inc_int(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_INDEX);
                } else {
                    VarModule::set_int(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_INDEX, 0);
                }
            } else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                let index = VarModule::get_int(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_INDEX);
                if index > 0 {
                    if index - 1 == 15 {
                        // account for removed Zoom
                        VarModule::dec_int(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_INDEX);
                    }
                    VarModule::dec_int(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_INDEX);
                } else {
                    VarModule::set_int(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_INDEX, 20);
                }
            }
        }
    }
}

unsafe extern "C" fn effect_appeals(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if app::smashball::is_training_mode() {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 10, 5, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
            } else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 10, -5, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
            }
        }
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if app::smashball::is_training_mode() && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            if !VarModule::is_flag(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_LOCK) {
                VarModule::on_flag(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_LOCK);
                VarModule::set_int(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_INDEX, 0);
            } else {
                VarModule::off_flag(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_LOCK);
            }
        }
    }
}

unsafe extern "C" fn effect_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if app::smashball::is_training_mode() && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            if !VarModule::is_flag(agent.battle_object, vars::brave::instance::MENU_TRAINING_MODE_LOCK) {
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 15, 0, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            } else {
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 15, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
            }
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_cliffattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 6, 3, -5, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword_hdr"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

unsafe extern "C" fn effect_downattackd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword_hdr"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword_hdr"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

unsafe extern "C" fn effect_downattacku(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword_hdr"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 1);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword_hdr"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

unsafe extern "C" fn effect_slipattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword_hdr"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword_hdr"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 3, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

unsafe extern "C" fn effect_win1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 8, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -15, 0, 80, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 6, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword_hdr"), Hash40::new("tex_brave_sword2"), 8, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 67.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -15, 0, 50, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 72.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

unsafe extern "C" fn effect_win2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 46.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("brave_lightning3_hit2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_LIGHTNING_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, false);
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, false);
        EFFECT(agent, Hash40::new("brave_lightning3_hit"), Hash40::new("top"), 0, 10.2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("brave_lightning3_lightning2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 110.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_swordspark"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 115.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_zapsword"), Hash40::new("tex_brave_sword2"), 10, Hash40::new("sword1"), 3, 0, 0, Hash40::new("sword1"), 14, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 120.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("brave_lightning3_rolllightning"), Hash40::new("top"), 0, 10, 0, 0, 0, 4, 2, false);
    }
    frame(lua_state, 130.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 10);
        EFFECT_OFF_KIND(agent, Hash40::new("brave_lightning3_swordspark"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffescape", acmd_stub, Priority::Low);

    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damageflyroll, Priority::Low);

    agent.acmd("game_dash", game_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);

    agent.acmd("effect_cliffattack", effect_cliffattack, Priority::Low);
    agent.acmd("effect_downattackd", effect_downattackd, Priority::Low);
    agent.acmd("effect_downattacku", effect_downattacku, Priority::Low);
    agent.acmd("effect_slipattack", effect_slipattack, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("game_appealhil", game_appealhi, Priority::Low);
    agent.acmd("game_appealhir", game_appealhi, Priority::Low);

    agent.acmd("game_appealsl", game_appeals, Priority::Low);
    agent.acmd("game_appealsr", game_appeals, Priority::Low);
    agent.acmd("effect_appealsl", effect_appeals, Priority::Low);
    agent.acmd("effect_appealsr", effect_appeals, Priority::Low);

    agent.acmd("game_appeallwl", game_appeallw, Priority::Low);
    agent.acmd("game_appeallwr", game_appeallw, Priority::Low);
    agent.acmd("effect_appeallwl", effect_appeallw, Priority::Low);
    agent.acmd("effect_appeallwr", effect_appeallw, Priority::Low);

    agent.acmd("effect_win1", effect_win1, Priority::Low);
    agent.acmd("effect_win2", effect_win2, Priority::Low);
}
