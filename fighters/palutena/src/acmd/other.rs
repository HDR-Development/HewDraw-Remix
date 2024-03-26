
use super::*;

unsafe extern "C" fn damageflyhi_sound(agent: &mut L2CAgentBase) {
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
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyroll_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));
    }
}

unsafe extern "C" fn dash_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.4);
    }
	frame(lua_state, 11.0); // Effectively F15
    if is_excute(agent) {
		FT_MOTION_RATE(agent, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn dash_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_palutena_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn turn_dash_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.2);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0); // Effectively F15
    if is_excute(agent) {
		FT_MOTION_RATE(agent, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn palutena_explosiveflame_reserve_wait_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
		FT_MOTION_RATE(agent, 10.0);
    }
    
}

unsafe extern "C" fn palutena_reflectionboard_shoot_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 125, 40, 0, 75, 5.0, 0.0, 8.5, 0.0, Some(0.0), Some(-4.5), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 40, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.1);
    }
    frame(lua_state, 210.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn palutena_autoaimbullet_shot_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let palutena = owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA;
    let damage = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {10.0} else {6.0};
    let paralyze = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {0.6} else {0.4};
    if !palutena {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 361, 41, 0, 40, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1.7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_palutena_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beamss"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), damage, 65, 40, 0, 75, 2.3, 0.0, 0.0, 0.0, None, None, None, paralyze, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beamss"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

unsafe extern "C" fn palutena_autoaimbullet_shot_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let palutena = owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA;
    let power = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {Hash40::new("sys_hit_elec")} else {Hash40::new("sys_hit_elec_s")};
    let size = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {2.0} else {1.0};
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_bullet_shot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        if palutena {
            LAST_EFFECT_SET_COLOR(agent, 0.85, 0.40, 0.001);
        }
    }
    if palutena {
        for _ in 0..99 {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 2.2, 1.2, 0, 0, 0, 0.23 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 0.2, -1.4, 0, 0, 0, 0.17 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 1.7, 0.1, 0, 0, 0, 0.32 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
                LAST_EFFECT_SET_RATE(agent, 1);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 1.4, 1.0, 0, 0, 0, 0.2 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 2.3, -1.4, 0, 0, 0, 0.15 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
        }
    }
}

unsafe extern "C" fn palutena_explosiveflame_explode_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 160, 100, 50, 0, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.7, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 6.0);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 7.2);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 8.4);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 9.6);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 10.8);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 12.0);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        //AREA_WIND_2ND_RAD(agent, 0, 1, 0.02, 1000, 1, 0, 0, 29);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 5.5, 84, 141, 0, 60, 15.5, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn escape_air_game(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn escape_air_slide_game(agent: &mut L2CAgentBase) {
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

pub fn install() {
    smashline::Agent::new("palutena")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflyhi_sound)
        .acmd("sound_damageflyn", damageflyhi_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflyhi_sound)
        .acmd("game_dash", dash_game)
        .acmd("sound_dash", dash_sound)
        .acmd("game_turndash", turn_dash_game)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .install();
    smashline::Agent::new("palutena_autoaimbullet")
        .acmd("game_shot", palutena_autoaimbullet_shot_game)
        .acmd("effect_shot", palutena_autoaimbullet_shot_effect)
        .install();
    smashline::Agent::new("palutena_reflectionboard")
        .acmd("game_shoot", palutena_reflectionboard_shoot_game)
        .install();
    smashline::Agent::new("palutena_explosiveflame")
        .acmd("game_explode", palutena_explosiveflame_explode_game)
        .install();
    // smashline::Agent::new("palutena_explosiveflame_reserve")
    //     .acmd("effect_wait", palutena_explosiveflame_reserve_wait_effect)
    //     .install();
}
