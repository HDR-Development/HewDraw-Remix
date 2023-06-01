
use super::*;

#[acmd_script( agent = "demon", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "demon", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "demon", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "demon", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "demon", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "demon", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_demon_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

#[acmd_script( agent = "demon", script = "sound_dashb" , category = ACMD_SOUND , low_priority)]
unsafe fn dashb_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_demon_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

#[acmd_script( agent = "demon", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, 29.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "demon", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "demon", script = "game_appealsr" , category = ACMD_GAME , low_priority)]
unsafe fn demon_appeal_s_r_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 11.0/(18.0-1.0));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 180, 5, 0, 20, 4.0, 0.0, 16.5, 10.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 180, 5, 0, 20, 3.5, 0.0, 13.0, 6.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 180, 5, 0, 20, 3.5, 0.0, 10.0, 3.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 8.0, false);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        ATTACK(fighter, 3, 1, Hash40::new("top"), 3.0, 180, 5, 0, 20, 3.5, 0.0, 11.0, 8.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 1, Hash40::new("top"), 3.0, 180, 5, 0, 20, 3.0, 0.0, 12.5, 5.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 4, 10.0, false);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        ATTACK(fighter, 5, 2, Hash40::new("top"), 3.0, 180, 5, 0, 20, 3.5, 0.0, 6.5, 10.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 6, 2, Hash40::new("top"), 3.0, 180, 5, 0, 20, 3.0, 0.0, 9.0, 5.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 5, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 6, 9.0, false);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 62.0);
    if is_excute(fighter) {
        ATTACK(fighter, 7, 3, Hash40::new("top"), 6.0, 40, 60, 0, 80, 3.5, 0.0, 10.5, 8.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_APPEAL, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 8, 3, Hash40::new("top"), 6.0, 40, 60, 0, 80, 3.0, 0.0, 13.0, 4.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_APPEAL, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 7, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 8, 10.0, false);
        AttackModule::set_attack_camera_quake_forced(boma, 7, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 8, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 63.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "demon", script = "game_appealsl" , category = ACMD_GAME , low_priority)]
unsafe fn demon_appeal_s_l_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 11.0/(18.0-1.0));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 180, 5, 0, 20, 4.0, 0.0, 16.5, 10.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 180, 5, 0, 20, 3.5, 0.0, 13.0, 6.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 180, 5, 0, 20, 3.5, 0.0, 10.0, 3.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 8.0, false);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        ATTACK(fighter, 3, 1, Hash40::new("top"), 3.0, 180, 5, 0, 20, 3.5, 0.0, 11.0, 8.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 1, Hash40::new("top"), 3.0, 180, 5, 0, 20, 3.0, 0.0, 12.5, 5.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 4, 10.0, false);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        ATTACK(fighter, 5, 2, Hash40::new("top"), 3.0, 180, 5, 0, 20, 3.5, 0.0, 6.5, 10.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 6, 2, Hash40::new("top"), 3.0, 180, 5, 0, 20, 3.0, 0.0, 9.0, 5.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 5, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 6, 9.0, false);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 62.0);
    if is_excute(fighter) {
        ATTACK(fighter, 7, 3, Hash40::new("top"), 6.0, 40, 60, 0, 80, 3.5, 0.0, 10.5, 8.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_APPEAL, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 8, 3, Hash40::new("top"), 6.0, 40, 60, 0, 80, 3.0, 0.0, 13.0, 4.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_APPEAL, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 7, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 8, 10.0, false);
        AttackModule::set_attack_camera_quake_forced(boma, 7, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 8, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 63.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        dash_sound,
        dashb_sound,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound,
        escape_air_game,
        escape_air_slide_game,
        demon_appeal_s_r_game,
        demon_appeal_s_l_game,
    );
}

