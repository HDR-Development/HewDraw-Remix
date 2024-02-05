
use super::*;


unsafe extern "C" fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ken_rnd_futtobi01"), Hash40::new("seq_ken_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ken_rnd_futtobi01"), Hash40::new("seq_ken_rnd_futtobi02"));}
    }
}


unsafe extern "C" fn damageflylw_sound(fighter: &mut L2CAgentBase) {
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ken_rnd_futtobi01"), Hash40::new("seq_ken_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ken_rnd_futtobi01"), Hash40::new("seq_ken_rnd_futtobi02"));}
    }
}


unsafe extern "C" fn damageflyn_sound(fighter: &mut L2CAgentBase) {
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ken_rnd_futtobi01"), Hash40::new("seq_ken_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ken_rnd_futtobi01"), Hash40::new("seq_ken_rnd_futtobi02"));}
    }
}


unsafe extern "C" fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_ken_rnd_futtobi01"), Hash40::new("seq_ken_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ken_rnd_futtobi01"), Hash40::new("seq_ken_rnd_futtobi02"));
    }
}


unsafe extern "C" fn damageflytop_sound(fighter: &mut L2CAgentBase) {
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ken_rnd_futtobi01"), Hash40::new("seq_ken_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ken_rnd_futtobi01"), Hash40::new("seq_ken_rnd_futtobi02"));}
    }
}


unsafe extern "C" fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_ken_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}


unsafe extern "C" fn ken_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}


unsafe extern "C" fn ken_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.2);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 9.0, 0.0, Some(0.0), Some(9.0), Some(7.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}


unsafe extern "C" fn escape_air_game(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn sound_guarddamage(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF) {
            PLAY_SE(fighter, Hash40::new("se_ken_guard"));
        }
    }
}


unsafe extern "C" fn game_movewms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::enable_safe_pos(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 45, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if (VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_CURRENT_HADOKEN_AIR)) {
            let lr = PostureModule::lr(owner_module_accessor);
            GroundModule::set_rhombus_offset(boma, &Vector2f{x: -4.0 * lr, y: 3.0});
            KineticModule::reflect_speed(boma, &Vector3f{x: 0.26, y: lr * 0.97, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            VarModule::off_flag(owner_module_accessor.object(), vars::shotos::instance::IS_CURRENT_HADOKEN_AIR);
        }
        else{
            GroundModule::set_rhombus_offset(boma, &Vector2f{x: 0.0, y: 0.0});
        }
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 45, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
    }
}


unsafe extern "C" fn effect_movewms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, false);
        } else {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, false);
        }
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("ken_hadoken_bullethand_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
}


unsafe extern "C" fn sound_movewms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ken_special_n04"));
    }
}


unsafe extern "C" fn game_movespwms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::enable_safe_pos(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 80, 4, 0, 43, 3.5, 0.0, -5.2, 0.0, Some(0.0), Some(-5.2), Some(0.0), 1.4, 0.0,   *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 366, 4, 0, 40, 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-5.2), Some(0.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if (VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_CURRENT_HADOKEN_AIR)) {
            let lr = PostureModule::lr(owner_module_accessor);
            GroundModule::set_rhombus_offset(boma, &Vector2f{x: -4.0 * lr, y: 3.0});
            KineticModule::reflect_speed(boma, &Vector3f{x: 0.26, y: lr * 0.97, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            VarModule::off_flag(owner_module_accessor.object(), vars::shotos::instance::IS_CURRENT_HADOKEN_AIR);
        }
        else{
            GroundModule::set_rhombus_offset(boma, &Vector2f{x: 0.0, y: 0.0}); 
        }
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 80, 4, 0, 43, 1.15, 0.0, -0.9, 0.95, Some(0.0), Some(-0.9), Some(-5.15), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 366, 4, 0, 40, 2.3, 0.0, 0.25, 0.3, Some(0.0), Some(0.25), Some(-4.5), 1.4, 0.0,   *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 80, 4, 0, 43, 1.4, 0.0, -2.4, -1.1, Some(0.0), Some(-2.4), Some(-2.9), 1.4, 0.0,   *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 366, 4, 0, 40, 2.8, 0.0, 0.0, -2.0, Some(0.0), Some(-1.0), Some(-2.0), 1.4, 0.0,   *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 4.0, false);
    }
}



unsafe extern "C" fn game_movespwms_last(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.3, 85, 50, 0, 58, 5.0, 0.0, 0.0, -1.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving_ken"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_ENERGY);
    }
}


unsafe extern "C" fn effect_movespwms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, false);
        } else {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, false);
        }
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("ken_hadoken_bullethand_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("top"), 0, -5.0, -1.0, 0, 0, 0, 3.0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_impact"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 0.5, false);
        EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_hit2"), Hash40::new("top"), 0, 0, -1.0, 180, 0, 0, 1.0, false);
    }
}


unsafe extern "C" fn sound_movespwms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ken_special_n04"));
    }
}




pub fn install() {
    smashline::Agent::new("ken")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflylw_sound)
        .acmd("sound_damageflyn", damageflyn_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflytop_sound)
        .acmd("sound_dash", dash_sound)
        .acmd("game_turndash", ken_turn_dash_game)
        .acmd("game_catch", ken_catch_game)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .acmd("sound_guarddamage", sound_guarddamage)
        .install();
    smashline::Agent::new("ken_hadoken")
        .acmd("game_movew", game_movewms)
        .acmd("game_movem", game_movewms)
        .acmd("game_moves", game_movewms)
        .acmd("effect_movew", effect_movewms)
        .acmd("effect_movem", effect_movewms)
        .acmd("effect_moves", effect_movewms)
        .acmd("sound_movew", sound_movewms)
        .acmd("sound_movem", sound_movewms)
        .acmd("sound_moves", sound_movewms)
        .acmd("game_movespw", game_movespwms)
        .acmd("game_movespm", game_movespwms)
        .acmd("game_movesps", game_movespwms)
        .acmd("game_movespw_last", game_movespwms_last)
        .acmd("game_movespm_last", game_movespwms_last)
        .acmd("game_movesps_last", game_movespwms_last)
        .acmd("effect_movespw", effect_movespwms)
        .acmd("effect_movespm", effect_movespwms)
        .acmd("effect_movesps", effect_movespwms)
        .acmd("sound_movespw", sound_movespwms)
        .acmd("sound_movespm", sound_movespwms)
        .acmd("sound_movesps", sound_movespwms)
        .install();
}
