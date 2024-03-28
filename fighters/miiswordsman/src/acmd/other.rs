use super::*;
use smash::app::sv_battle_object;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_miiswordsman_rnd_futtobi01"), Hash40::new("seq_miiswordsman_rnd_futtobi02"));
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_miiswordsman_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_miiswordsman_step_right_m"));
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_miiswordsman_step_left_m"));
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

// #[acmd_script( agent = "miiswordsman_tornadoshot", script = "game_fly" , category = ACMD_GAME , low_priority)]
// unsafe fn miiswordsman_tornadoshot_fly_game(agent: &mut L2CAgentBase) {
//     let lua_state = agent.lua_state_agent;
//     let boma = agent.boma();
//     if is_excute(agent) {
//         AREA_WIND_2ND_RAD_arg9(agent, 0, 2, 0.05, 200, 1, 3, 3, 25, 30);
//         ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 366, 100, 30, 0, 4.5, 0.0, 9.5, 3.2, Some(0.0), Some(9.5), Some(3.2), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
//         ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 90, 100, 50, 0, 4.5, 0.0, 9.5, 3.2, Some(0.0), Some(3.0), Some(2.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
//     }
//     frame(lua_state, 37.0);
//     if is_excute(agent) {
//         AttackModule::clear_all(boma);
//         ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 80, 40, 0, 100, 4.5, 0.0, 9.5, 3.2, Some(0.0), Some(3.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
//     }
// }

unsafe extern "C" fn miiswordsman_wave_fly_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 55, 60, 0, 38, 9.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
       ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 55, 60, 0, 38, 9.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn miiswordsman_wave_fly_effect(agent: &mut L2CAgentBase) {
        let lua_state = agent.lua_state_agent;
        let boma = agent.boma();
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let mut lead_wave : u32 = std::u32::MAX;
        let mut wave_2 : u32 = std::u32::MAX;
        let mut wave_3 : u32 = std::u32::MAX;
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_hikari_syuriken"), false, true);
            lead_wave = EffectModule::req_follow(boma, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), &Vector3f{x: 0.0, y: 3.0, z: 0.0}, &Vector3f{x: 80.6, y: -69.5, z: 0.0}, 0.9, true, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rate(boma, lead_wave, 0.7);

            wave_2 = EffectModule::req_follow(boma, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), &Vector3f{x: 0.0, y: 3.0, z: -4.0}, &Vector3f{x: 80.6, y: -69.5, z: 0.0}, 0.7, true, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rate(boma, wave_2, 0.7);
            EffectModule::set_alpha(boma, wave_2, 0.4);
            //Ray check here is used for checking if you're on the ground. Unfortunately is_touch and is_wall_touch_line didnt work for this. Sorry!
            if GroundModule::ray_check(
                agent.module_accessor, 
                &Vector2f{ x: PostureModule::pos_x(agent.module_accessor), y: PostureModule::pos_y(agent.module_accessor)}, 
                &Vector2f{ x: 0.0, y: -7.0}, true
            ) == 1 {
                FOOT_EFFECT(agent, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(agent, 0.3);
                FOOT_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            EffectModule::set_rate(boma, lead_wave, 0.000001);
            EffectModule::set_rate(boma, wave_2, 0.000001);
            EffectModule::set_alpha(boma, wave_2, 0.4);
            wave_3 = EffectModule::req_follow(boma, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), &Vector3f{x: 0.0, y: 3.0, z: -8.0}, &Vector3f{x: 80.6, y: -69.5, z: 0.0}, 0.5, true, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rate(boma, wave_3, 0.7);
            EffectModule::set_alpha(boma, wave_3, 0.4);
            if GroundModule::ray_check(
                agent.module_accessor, 
                &Vector2f{ x: PostureModule::pos_x(agent.module_accessor), y: PostureModule::pos_y(agent.module_accessor)}, 
                &Vector2f{ x: 0.0, y: -7.0}, true
            ) == 1 {
                FOOT_EFFECT(agent, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(agent, 0.3);
                FOOT_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            EffectModule::set_rate(boma, wave_3, 0.000001);
            if GroundModule::ray_check(
                agent.module_accessor, 
                &Vector2f{ x: PostureModule::pos_x(agent.module_accessor), y: PostureModule::pos_y(agent.module_accessor)}, 
                &Vector2f{ x: 0.0, y: -7.0}, true
            ) == 1 {
                FOOT_EFFECT(agent, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(agent, 0.3);
                FOOT_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            }
        }
        for _ in 0..i32::MAX {
            wait(lua_state, 4.0);
            if is_excute(agent) {
                    if GroundModule::ray_check(
                        agent.module_accessor, 
                        &Vector2f{ x: PostureModule::pos_x(agent.module_accessor), y: PostureModule::pos_y(agent.module_accessor)}, 
                        &Vector2f{ x: 0.0, y: -7.0}, true
                    ) == 1 {
                        FOOT_EFFECT(agent, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 4, -0.5, 0, 0, 0, 0, 5.5, 0, 0, 0, 0, 0, 0, false);
                        LAST_EFFECT_SET_RATE(agent, 0.3);
                        FOOT_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 4, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
                    }
            }
        }
}

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));
    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_damageflyhi", sound_damagefly);
    agent.acmd("sound_damageflylw", sound_damagefly);
    agent.acmd("sound_damageflyn", sound_damagefly);
    agent.acmd("sound_damageflytop", sound_damagefly);
    agent.acmd("sound_damageflyroll", sound_damageflyroll);
    
    agent.acmd("game_dash", game_dash);
    agent.acmd("sound_dash", sound_dash);
    agent.acmd("game_turndash", game_turndash);

    agent.acmd("game_escapeair", game_escapeair);
    agent.acmd("game_escapeairslide", game_escapeairslide);
}
