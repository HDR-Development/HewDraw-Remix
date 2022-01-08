
use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::utility::*;
use smash_script::*;
use smashline::*;
use smashline::*;
use smash_script::macros::*;
use crate::utils::hdr;
use crate::vars::*;


#[acmd_script( agent = "sonic", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ArticleModule::shoot_exist(boma, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

#[acmd_script( agent = "sonic", script = "game_specialsstart" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    // println!("game_specialsstart");
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
    }
    
}

#[acmd_script( agent = "sonic", script = "effect_specialsstart" , category = ACMD_EFFECT , low_priority)]
unsafe fn sonic_special_s_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    // println!("effect_specialsstart");
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_DASH_FLAG) {
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            EffectModule::req_follow(boma, Hash40::new(SONIC_LIGHTSPEED_DASH_CHARGEUP_EFFECT), Hash40::new("head"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 1.0, false, 0 as u32,0,0,0,0,false,false);
            EffectModule::set_rgb(boma, EffectModule::get_last_handle(boma) as u32, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.x, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.y, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.z);
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            EffectModule::req_follow(boma, Hash40::new(SONIC_LIGHTSPEED_DASH_CHARGEUP_EFFECT), Hash40::new("waist"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 0.5, false, 0 as u32,0,0,0,0,false,false);
            EffectModule::set_rgb(boma, EffectModule::get_last_handle(boma) as u32, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.x, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.y, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.z);
        }
        frame(lua_state, 57.0);
        if is_excute(fighter) {
            EffectModule::req_follow(boma, Hash40::new(SONIC_LIGHTSPEED_DASH_CHARGEUP_EFFECT), Hash40::new("head"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 1.0, false, 0 as u32,0,0,0,0,false,false);
            EffectModule::set_rgb(boma, EffectModule::get_last_handle(boma) as u32, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.x, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.y, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.z);
        }
        wait(lua_state, 20.0);
        if is_excute(fighter) {
            EffectModule::req_follow(boma, Hash40::new(SONIC_LIGHTSPEED_DASH_CHARGEUP_EFFECT), Hash40::new("waist"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 0.5, false, 0 as u32,0,0,0,0,false,false);
            EffectModule::set_rgb(boma, EffectModule::get_last_handle(boma) as u32, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.x, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.y, SONIC_LIGHTSPEED_DASH_FULL_CHARGE_EFFECT_COLOR.z);
        }
    }
    
}

#[acmd_script( agent = "sonic", script = "sound_specialsstart" , category = ACMD_SOUND , low_priority)]
unsafe fn sonic_special_s_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    // println!("sound_specialsstart");
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_DASH_FLAG) {
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            SoundModule::play_se(boma, Hash40::new(SONIC_LIGHTSPEED_DASH_CHARGING_SOUND), false, false, false, false, app::enSEType(0));
        }
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            SoundModule::stop_se(boma, Hash40::new(SONIC_LIGHTSPEED_DASH_CHARGING_SOUND), 0 as u32);
        }
        frame(lua_state, 57.0);
        if is_excute(fighter) {
            SoundModule::play_se(boma, Hash40::new(SONIC_LIGHTSPEED_DASH_CHARGING_SOUND), false, false, false, false, app::enSEType(0));
        }
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            SoundModule::stop_se(boma, Hash40::new(SONIC_LIGHTSPEED_DASH_CHARGING_SOUND), 0 as u32);
        }
    }
    
}

#[acmd_script( agent = "sonic", script = "expression_specialsstart" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn sonic_special_s_start_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    // println!("expression_specialsstart");
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        slope!(fighter, *PHYSICS_SWING_SPECIAL_STATE_CURVE, *CP_FLAG_ROLL);
    }
    
}

#[acmd_script( agent = "sonic", script = "game_specialshold", category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_s_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    // println!("game_specialshold");
    let boma = sv_system::battle_object_module_accessor(lua_state);
    // println!("here");
    frame(lua_state, 19.0);
    if is_excute(fighter) {  }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("waist"), 8.0, 55, 80, 0, 70, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "sonic", script = "sound_specialshold", category = ACMD_SOUND , low_priority)]
unsafe fn sonic_special_s_hold_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    // println!("sound_specialshold");
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 19.0);
    if is_excute(fighter) {  }frame(lua_state, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_sonic_attack05"));
        SoundModule::play_se(boma, Hash40::new("se_sonic_rounddash"), false, false, false, false, app::enSEType(0));
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        SoundModule::stop_se(boma, Hash40::new("se_sonic_rounddash"), 0 as u32);
    }
    
}

#[acmd_script( agent = "sonic", script = "effect_specialshold", category = ACMD_EFFECT , low_priority)]
unsafe fn sonic_special_s_hold_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    // println!("sound_specialshold");
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 19.0);
    if is_excute(fighter) {  }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EffectModule::req_on_joint(boma, Hash40::new("sys_sp_flash"),Hash40::new("waist"),&hdr::DEFAULT_VECTOR3,&hdr::DEFAULT_VECTOR3,1.0,&hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3,false,0,0,0);
        EffectModule::req_follow(boma, Hash40::new("sonic_spintrace"), Hash40::new("waist"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 1.0, false, 0 as u32,0,0,0,0,false,false);
        EffectModule::set_rgb(boma, EffectModule::get_last_handle(boma) as u32, 0.711000025, 0.930999994, 1.0);
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        EffectModule::kill_kind(boma, Hash40::new("sonic_spintrace"), false, true);
    }
    
}

#[acmd_script( agent = "sonic", script = "expression_specialshold", category = ACMD_EXPRESSION , low_priority)]
unsafe fn sonic_special_s_hold_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    // println!("expression_specialshold");
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash as i64, Hash40::new("body_normal").hash as i64);
        for bone in ["waist", "head", "s_stingd1", "shoulderr", "shoulderl", "armr", "arml", "legr", "legl", "kneer", "kneel", "footr", "footl"].iter() {
            HIT_NODE(fighter, Hash40::new(bone), *COLLISION_PART_BODY);
        }
        HIT_NODE(fighter, Hash40::new("rot"), *FIGHTER_STATUS_KIND_BURY);
        ItemModule::set_have_item_visibility(boma, false, 0);
        ItemModule::set_attach_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new_raw(0x11140ef559), 0, true, 0 as u32);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    
}

#[acmd_script( agent = "sonic", script = "game_speciallwhold" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let id = hdr::get_player_number(boma);
    if SONIC_PULSE_HITBOX[id] {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 1.0, 365, 0, 0, 58, 8.3, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            SONIC_PULSE_HITBOX[id] = false;
         }
    }
    
}

#[acmd_script( agent = "sonic", script = "game_speciallwstart" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, false);
    }
    
}

#[acmd_script( agent = "sonic", script = "game_specialairlwstart" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_air_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, false);
    }
    
}

#[acmd_script( agent = "sonic", script = "game_specialnhoming" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_n_homing_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        FT_MOTION_RATE(fighter, 0.013);
        if !WorkModule::is_flag(boma, *FIGHTER_SONIC_STATUS_SPECIAL_N_HOMING_FLAG_IS_KIRBY) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.0, 72, 120, 0, 15, 5.0, 0.5, 1.5, 0.0, Some(0.5), Some(1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
            AttackModule::set_captured_same_time_attack(boma, 0, true);
        }
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
    
}

#[acmd_script( agent = "sonic", script = "sound_specialnhoming" , category = ACMD_SOUND )]
unsafe fn sonic_special_n_homing_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);
    let mut sound = "";

    if rng == 0 { sound = "vc_sonic_004"; }
    else { sound = "vc_sonic_attack02"; }

    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_sonic_rounddash"));
        PLAY_SE(fighter, Hash40::new(sound));
        PLAY_SE(fighter, Hash40::new("se_sonic_special_n01"));
    }
}

#[acmd_script( agent = "sonic", script = "game_specialnhit" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_n_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        let temp = Vector3f { x: -0.3, y: 1.0, z: 0.0 };
		KineticModule::add_speed(boma, &temp);
    }
    FT_MOTION_RATE(fighter, 0.6);
    
}

#[acmd_script( agent = "sonic", script = "effect_specialnhit" , category = ACMD_EFFECT , low_priority)]
unsafe fn sonic_special_n_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
}

pub fn install() {
    install_acmd_scripts!(
        sonic_special_hi_game,
        //sonic_special_s_start_game,
        //sonic_special_s_start_effect,
        //sonic_special_s_start_sound,
        //sonic_special_s_start_expression,
        //sonic_special_s_hold_game,
        //sonic_special_s_hold_sound,
        //sonic_special_s_hold_effect,
        //sonic_special_s_hold_expression,
        sonic_special_lw_hold_game,
        sonic_special_lw_start_game,
        sonic_special_air_lw_start_game,
        sonic_special_n_homing_game,
        sonic_special_n_hit_game,
        sonic_special_n_hit_effect,
        sonic_special_n_homing_sound
    );
}

