
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

#[acmd_script( agent = "samus", scripts = ["game_appealsl","game_appealsr"], category = ACMD_GAME, low_priority)]
unsafe fn game_appeals(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 1.0);
    if macros::is_excute(fighter) {
        let new_ice = !VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE);
        VarModule::set_flag(fighter.battle_object, vars::samus::instance::ICE_MODE, new_ice);
        suit_effect(fighter.module_accessor,fighter.battle_object);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        let eff_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX);
        effect!(fighter, MA_MSC_EFFECT_REMOVE, eff_max, 1);
    }
}

#[acmd_script( agent = "samus", scripts = ["effect_appealsl","effect_appealsr"], category = ACMD_EFFECT, low_priority)]
unsafe fn effect_appeals(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 13.0);
    let is_ice = VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_appeal_s"), Hash40::new("armr"), 7, 0, 0, 0, 0, 90, 1, true);
        if is_ice{
            LAST_EFFECT_SET_COLOR(fighter,0.0, 0.875,1.25);
        }
    }
    frame(fighter.lua_state_agent , 32.0);
    if macros::is_excute(fighter) {
        if is_ice{
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("armr"), 7, 0, 0, 0, 0, 90, 1, true);
            LAST_EFFECT_SET_COLOR(fighter,0.375, 1.0,1.0);

            macros::EFFECT_FOLLOW(fighter,Hash40::new("sys_hit_ice"), Hash40::new("armr"), 8, 0, 0, 0, 0, 90, 0.2, true);
        }
    }
}

#[acmd_script( agent = "samus", scripts = ["sound_appealsl","sound_appealsr"], category = ACMD_SOUND, low_priority)]
unsafe fn sound_appeals(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_appeal_s01"));
    }
    frame(fighter.lua_state_agent , 32.0);
    if macros::is_excute(fighter) {
        let is_ice = VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE);
        if !is_ice{
            macros::PLAY_SE(fighter, Hash40::new("se_samus_appeal_s03"));
        }
        else{
            macros::PLAY_SE(fighter, Hash40::new("se_common_frieze_ll"));
        }
    }
    frame(fighter.lua_state_agent , 85.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_samus_appeal_s03"));
        macros::STOP_SE(fighter, Hash40::new("se_common_frieze_ll"));
        macros::PLAY_SE(fighter, Hash40::new("se_samus_appeal_s04"));
    }
}
#[acmd_script( agent = "samus", scripts = ["expression_appealsl","expression_appealsr"], category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_appeals(agent: &mut L2CAgentBase) {
    let mut is_ice = false;
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("appeal_sl"), false, -1.0);
    }
    frame(agent.lua_state_agent, 2.0);
    is_ice = VarModule::is_flag(agent.battle_object, vars::samus::instance::ICE_MODE);
    if macros::is_excute(agent) {
        LinkModule::send_event_nodes(agent.module_accessor, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        let rumble = if is_ice {Hash40::new("rbkind_15_iceberg_sp")} else {Hash40::new("rbkind_elecattacks")};
        ControlModule::set_rumble(agent.module_accessor, rumble, 40, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 168.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
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
    if macros::is_excute(fighter) {
        let isThrow = PostureModule::rot_x(fighter.module_accessor, 0) < -89.9;
        if isThrow {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 80, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        }
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

#[acmd_script( agent = "samus", script = "effect_ice_lance", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_ice_lance(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("handr"), 5.0, 0.0, 0.0, 0, 0, -90, 0.2, true);
        LAST_EFFECT_SET_SCALE_W(fighter,0.18,14.0*0.06,0.18);
    }
}

#[acmd_script( agent = "samus", script = "effect_ice_lance_break", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_ice_lance_break(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
        macros::EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("handr"), 7.5, 0.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        macros::AFTER_IMAGE_OFF(fighter, 0);
    }
}

#[acmd_script( agent = "samus", script = "effect_ice_punch", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_ice_punch(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("handr"), -1.5, 0.0, 0.0, 0, 0, 90, 0.15, true);
        LAST_EFFECT_SET_SCALE_W(fighter,0.175,0.275,0.175);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("handr"), 2.5, 0.0, 0.0, 0, 0, -90, 0.15, true);
        LAST_EFFECT_SET_SCALE_W(fighter,0.175,0.275,0.175);
    }
}
#[acmd_script( agent = "samus", script = "effect_ice_punch_break", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_ice_punch_break(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
        macros::EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "samus", script = "sound_ice_break", category = ACMD_SOUND, low_priority)]
unsafe fn sound_ice_break(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_item_ice_crash"));
    }
}

#[acmd_script( agent = "samus_missile", script = "game_ice", category = ACMD_GAME, low_priority)]
unsafe fn game_ice(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 80, 65, 0, 50, 2.4, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}
#[acmd_script( agent = "samus_missile", script = "effect_ice", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_ice(agent: &mut L2CAgentBase) {
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
        wait(agent.lua_state_agent , 15.0);
    }
}

#[acmd_script( agent = "samus_missile", script = "sound_ice", category = ACMD_SOUND, low_priority)]
unsafe fn sound_ice(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_special_s02"));
    }
}

#[acmd_script( agent = "samus_missile", script = "game_iburst", category = ACMD_GAME, low_priority)]
unsafe fn game_iburst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent , 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}
#[acmd_script( agent = "samus_missile", script = "effect_iburst", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_iburst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}
#[acmd_script( agent = "samus_missile", script = "sound_iburst", category = ACMD_SOUND, low_priority)]
unsafe fn sound_iburst(agent: &mut L2CAgentBase) {
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
        game_appeals,
        expression_appeals,
        
        effect_ice_lance,
        effect_ice_lance_break,
        effect_ice_punch,
        effect_ice_punch_break,

        sound_ice_break,
        
        game_ice,
        effect_ice,
        sound_ice,

        game_iburst,
        effect_iburst,
        sound_iburst,

    );
}

