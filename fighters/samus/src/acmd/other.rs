
use super::*;

#[acmd_script( agent = "samus", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_samus_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_samus_step_right_m"));
    }
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_samus_step_left_m"))
    }
}

#[acmd_script( agent = "samus", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn samus_turn_dash_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "samus_supermissile", script = "game_ready" , category = ACMD_GAME , low_priority)]
unsafe fn samus_supermissile_ready_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
		AttackModule::enable_safe_pos(boma);
	}
    
}

#[acmd_script( agent = "samus_supermissile", script = "game_straight" , category = ACMD_GAME , low_priority)]
unsafe fn samus_supermissile_straight_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
	}
    
}

#[acmd_script( agent = "samus_cshot", script = "game_shoot" , category = ACMD_GAME , low_priority)]
unsafe fn samus_cshot_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 42, 0, 14, 1.9, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 25.0, 40, 64, 0, 45, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
	}
    
}

#[acmd_script( agent = "samus_cshot", script = "sound_shoot", category = ACMD_SOUND, low_priority)]
unsafe fn samus_cshot_shoot_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 0.0);
    if is_excute(fighter) {
        if WorkModule::get_float(boma, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.25 {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n02"));
        }
        else if WorkModule::get_float(boma, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.625 {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n03"));
        }
        else if WorkModule::get_float(boma, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.875 {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n04"));
        }
        else {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n05"));
        }
    }
    
}

#[acmd_script( agent = "samus", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "samus", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "samus", scripts = ["effect_appealsl","effect_appealsr"], category = ACMD_EFFECT)]
unsafe fn effect_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 7.046, 0.555, -0.197, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 37.0);
}
#[acmd_script( agent = "samus", scripts = ["sound_appealsl","sound_appealsr"], category = ACMD_SOUND)]
unsafe fn sound_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_swing_s"));
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_squat"));
    }
    //frame(agent.lua_state_agent, 52.0);
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_rise"));
    }
}
#[acmd_script( agent = "samus", scripts = ["expression_appealsl","expression_appealsr"], category = ACMD_EXPRESSION)]
unsafe fn expression_appeals(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "samus", script = "effect_ice_lance", category = ACMD_EFFECT)]
unsafe fn effect_ice_lance(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("handr"), 5.0, 0.0, 0.0, 0, 0, -90, 0.2, true);
        LAST_EFFECT_SET_SCALE_W(agent,0.18,14.0*0.06,0.18);
    }
}

#[acmd_script( agent = "samus", script = "effect_ice_lance_break", category = ACMD_EFFECT)]
unsafe fn effect_ice_lance_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("sys_ice"),false,false);
        macros::EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("handr"), 7.5, 0.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        macros::AFTER_IMAGE_OFF(agent, 0);
    }
}

#[acmd_script( agent = "samus", script = "effect_ice_punch", category = ACMD_EFFECT)]
unsafe fn effect_ice_punch(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("handr"), -1.5, 0.0, 0.0, 0, 0, 90, 0.15, true);
        LAST_EFFECT_SET_SCALE_W(agent,0.175,0.275,0.175);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("handr"), 2.5, 0.0, 0.0, 0, 0, -90, 0.15, true);
        LAST_EFFECT_SET_SCALE_W(agent,0.175,0.275,0.175);
    }
}
#[acmd_script( agent = "samus", script = "effect_ice_punch_break", category = ACMD_EFFECT)]
unsafe fn effect_ice_punch_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("sys_ice"),false,false);
        macros::EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "samus", script = "sound_ice_break", category = ACMD_SOUND)]
unsafe fn sound_ice_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
    }
}

#[acmd_script( agent = "samus", scripts = ["game_squatf","game_squatb","game_squatn"], category = ACMD_GAME)]
unsafe fn game_crawl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
    }
}

#[acmd_script( agent = "samus", scripts = ["effect_squatf","effect_squatb"], category = ACMD_EFFECT)]
unsafe fn effect_crawl(fighter: &mut L2CAgentBase) {
    //frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        if MotionModule::motion_kind(fighter.module_accessor) == Hash40::new("special_lw").hash{
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script( agent = "samus", scripts = ["expression_squatf","expression_squatb"], category = ACMD_EXPRESSION)]
unsafe fn expression_crawl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10, true);
    }
    for i in 1..i32::MAX{
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(fighter.lua_state_agent, 30.0);
    }
}

#[acmd_script( agent = "samus", script = "game_speciallw", category = ACMD_GAME)]
unsafe fn game_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
}
#[acmd_script( agent = "samus", script = "effect_speciallw", category = ACMD_EFFECT)]
unsafe fn effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
    }
}


#[acmd_script( agent = "samus_supermissile", script = "game_straight", category = ACMD_GAME)]
unsafe fn game_straight(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let isThrow = PostureModule::rot_x(agent.module_accessor, 0) < -89.9;
        if isThrow {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 80, 65, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        }
        else{
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 65, 65, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        }
    }
}
#[acmd_script( agent = "samus_missile", script = "game_homing", category = ACMD_GAME)]
unsafe fn game_homing(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let isThrow = PostureModule::rot_x(agent.module_accessor, 0) < 0.0;
        if isThrow {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 80, 65, 0, 50, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        }
        else{
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 65, 65, 0, 50, 2.4, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        }
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}
#[acmd_script( agent = "samus_missile", script = "effect_homing", category = ACMD_EFFECT)]
unsafe fn effect_homing(agent: &mut L2CAgentBase) {
    let mut rot = 0.0;
    if macros::is_excute(agent) {
        //rot = PostureModule::rot_x(agent.module_accessor, 0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_missile_homing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);
    }
    for i in 1..i32::MAX{
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 0, 0, -rot, 0, 0, 1.1, true);
            LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);
        }
        wait(agent.lua_state_agent, 15.0);
    }
}

#[acmd_script( agent = "samus_missile", script = "game_hburst", category = ACMD_GAME)]
unsafe fn game_hburst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}
#[acmd_script( agent = "samus_missile", script = "effect_hburst", category = ACMD_EFFECT)]
unsafe fn effect_hburst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}
#[acmd_script( agent = "samus_missile", script = "sound_hburst", category = ACMD_SOUND)]
unsafe fn sound_hburst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        dash_sound,
        samus_turn_dash_game,
        samus_supermissile_ready_game,
        samus_supermissile_straight_game,
        samus_cshot_shoot_game,
        samus_cshot_shoot_sound,
        escape_air_game,
        escape_air_slide_game,

        effect_appeals,
        sound_appeals,
        expression_appeals,
        
        effect_ice_lance,
        effect_ice_lance_break,
        effect_ice_punch,
        effect_ice_punch_break,

        sound_ice_break,

        game_crawl,
        effect_crawl,
        expression_crawl,

        game_speciallw,
        effect_speciallw,

        
        game_homing,
        effect_homing,

        game_hburst,
        effect_hburst,
        sound_hburst,

        game_straight,
    );
}

