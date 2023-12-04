
use super::*;

#[acmd_script( agent = "packun", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn packun_catch_game(fighter: &mut L2CAgentBase) {
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
        CATCH(fighter, 0, Hash40::new("top"), 4.3, 0.0, 6.6, 0.0, Some(0.0), Some(6.6), Some(12.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "packun", script = "game_catchattack", category = ACMD_GAME, low_priority )]
unsafe fn packun_catch_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(fighter) {
        VarModule::on_flag(boma.object(), vars::common::status::PUMMEL_OVERRIDE_GLOBAL_STATS);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        let damage = if stance != 1 { 0.0 } else { 0.3 };
        let effect = if stance != 1 { Hash40::new("collision_attr_normal") } else { Hash40::new("collision_attr_purple") };
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.4 + damage, 361, 100, 30, 0, 5.0, 0.0, 10.0, 10.0, None, None, None, 3.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "packun", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "packun", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_packun_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_STEP_FLIPPABLE(fighter, Hash40::new("se_packun_step_right_m"), Hash40::new("se_packun_step_left_m"));
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_STEP_FLIPPABLE(fighter, Hash40::new("se_packun_step_left_m"), Hash40::new("se_packun_step_right_m"));
    }
}

#[acmd_script( agent = "packun", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "packun", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "packun", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "packun", scripts = [ "game_appealhil", "game_appealhir" ], category = ACMD_GAME , low_priority)]
unsafe fn appeal_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let cur_stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10.0);
        VarModule::on_flag(boma.object(), vars::packun::instance::STANCE_REVERSE);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        let advance = if VarModule::is_flag(boma.object(), vars::packun::instance::STANCE_REVERSE) {2} else {1};
        VarModule::set_int(boma.object(), vars::packun::instance::CURRENT_STANCE, (cur_stance + advance) % 3);
        VarModule::on_flag(fighter.object(), vars::packun::instance::STANCE_INIT);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

#[acmd_script( agent = "packun", scripts = ["sound_appealhil", "sound_appealhir"], category = ACMD_SOUND, low_priority )]
unsafe fn appeal_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::packun::status::CLOUD_COVER) {
            PLAY_SE(fighter, Hash40::new("se_packun_special_s02"));
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::packun::status::CLOUD_COVER) {
            PLAY_SE(fighter, Hash40::new("se_packun_appear01"));
        }
    }
}

#[acmd_script( agent = "packun", script = "game_appealhi2", category = ACMD_GAME , low_priority)]
unsafe fn appeal_hi_2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let cur_stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::on_flag(boma.object(), vars::packun::instance::STANCE_REVERSE);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("foot"), true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        let advance = if VarModule::is_flag(boma.object(), vars::packun::instance::STANCE_REVERSE) {2} else {1};
        VarModule::set_int(boma.object(), vars::packun::instance::CURRENT_STANCE, (cur_stance + advance) % 3);
        VarModule::on_flag(fighter.object(), vars::packun::instance::STANCE_INIT);
    }
    frame(lua_state, 107.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("foot"), false);
    }
}

#[acmd_script( agent = "packun", script = "effect_appealhi2", category = ACMD_EFFECT , low_priority)]
unsafe fn appeal_hi_2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

#[acmd_script( agent = "packun", script = "sound_appealhi2", category = ACMD_SOUND , low_priority)]
unsafe fn appeal_hi_2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        PLAY_STEP_FLIPPABLE(fighter, Hash40::new("se_packun_step_right_m"), Hash40::new("se_packun_step_left_m"));
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        PLAY_STEP_FLIPPABLE(fighter, Hash40::new("se_packun_step_left_m"), Hash40::new("se_packun_step_right_m"));
    }
    frame(lua_state, 78.0);
    if is_excute(fighter) {
        PLAY_STEP_FLIPPABLE(fighter, Hash40::new("se_packun_step_right_m"), Hash40::new("se_packun_step_left_m"));
    }
}

#[acmd_script( agent = "packun", script = "expression_appealhi2", category = ACMD_EXPRESSION , low_priority)]
unsafe fn appeal_hi_2_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 78.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "packun", scripts = ["game_appealsl", "game_appealsr"] , category = ACMD_GAME , low_priority)]
unsafe fn appeal_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let cur_stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::on_flag(boma.object(), vars::packun::instance::STANCE_REVERSE);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        let advance = if VarModule::is_flag(boma.object(), vars::packun::instance::STANCE_REVERSE) {2} else {1};
        VarModule::set_int(boma.object(), vars::packun::instance::CURRENT_STANCE, (cur_stance + advance) % 3);
        VarModule::on_flag(fighter.object(), vars::packun::instance::STANCE_INIT);
    }
}

#[acmd_script( agent = "packun", scripts = ["sound_appealsl", "sound_appealsr"], category = ACMD_SOUND, low_priority )]
unsafe fn appeal_s_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::packun::status::CLOUD_COVER) {
            PLAY_SE(fighter, Hash40::new("se_packun_appeal_s01"));
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::packun::status::CLOUD_COVER) {
            PLAY_SE(fighter, Hash40::new("se_packun_special_s02"));
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::packun::status::CLOUD_COVER) {
            PLAY_SE(fighter, Hash40::new("se_packun_appeal_s02"));
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::packun::status::CLOUD_COVER) {
            PLAY_SE(fighter, Hash40::new("se_packun_appeal_s03"));
        }
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::packun::status::CLOUD_COVER) {
            PLAY_SE(fighter, Hash40::new("se_packun_appeal_s04"));
        }
    }
}

#[acmd_script( agent = "packun", scripts = [ "game_appeallwl", "game_appeallwr" ], category = ACMD_GAME , low_priority)]
unsafe fn appeal_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let cur_stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::on_flag(boma.object(), vars::packun::instance::STANCE_REVERSE);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        let advance = if VarModule::is_flag(boma.object(), vars::packun::instance::STANCE_REVERSE) {2} else {1};
        VarModule::set_int(boma.object(), vars::packun::instance::CURRENT_STANCE, (cur_stance + advance) % 3);
        VarModule::on_flag(fighter.object(), vars::packun::instance::STANCE_INIT);
    }
}

#[acmd_script( agent = "packun", scripts = ["sound_appeallwl", "sound_appeallwr"], category = ACMD_SOUND, low_priority )]
unsafe fn appeal_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::packun::status::CLOUD_COVER) {
            PLAY_SE(fighter, Hash40::new("se_packun_appeal_l01"));
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::packun::status::CLOUD_COVER) {
            PLAY_SE(fighter, Hash40::new("se_packun_special_s02"));
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::packun::status::CLOUD_COVER) {
            PLAY_SE(fighter, Hash40::new("se_packun_appeal_l02"));
        }
    }
}

#[acmd_script( agent = "packun_spikeball", script = "game_start", category = ACMD_GAME, low_priority )]
unsafe fn packun_spikeball_game_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {0} else {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)};
    let scale = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {1.0} else {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)};
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ModelModule::set_scale(boma, scale);
        MotionModule::set_rate(boma, 1.4);
        if stance == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

#[acmd_script( agent = "packun_spikeball", script = "game_startair", category = ACMD_GAME, low_priority )]
unsafe fn packun_spikeball_game_start_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {0} else {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)};
    let scale = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {1.0} else {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)};
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ModelModule::set_scale(boma, scale);
        MotionModule::set_rate(boma, 1.4);
        if stance == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

#[acmd_script( agent = "packun_spikeball", script = "game_loop", category = ACMD_GAME, low_priority )]
unsafe fn packun_spikeball_game_loop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {0} else {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)};
    let scale = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {1.0} else {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)};
    if is_excute(fighter) {
        if stance == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

#[acmd_script( agent = "packun_spikeball", script = "effect_loop" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_spikeball_loop_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {0} else {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)};
    if stance == 1 {    
        if is_excute(fighter) {
            //EFFECT_BRANCH_SITUATION(fighter, Hash40::new("null"), Hash40::new("sys_bound_smoke"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_mouth"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, false);
        }
    }
}

#[acmd_script( agent = "packun_spikeball", script = "game_shoot", category = ACMD_GAME, low_priority )]
unsafe fn packun_spikeball_game_shoot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {0} else {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)};
    let scale = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {1.0} else {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)};
    if WorkModule::is_flag(boma, *WEAPON_PACKUN_SPIKEBALL_INSTANCE_WORK_ID_FLAG_REACTIVE) {
        if is_excute(fighter) {
            if stance == 1 {
                ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
            }
            else {
                let kbg = if stance == 0 { 0 } else { 5 };
                ATTACK(fighter, 0, 0, Hash40::new("trans"), 18.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
    }
    else {
        if is_excute(fighter) {
            if stance == 1 {
                ATTACK(fighter, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::set_poison_param(boma, 0, 121, 30, 3.0, false);
            }
            else {
                let kbg = if stance == 0 { 0 } else { 5 };
                ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if stance == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 18.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

#[acmd_script( agent = "packun_spikeball", script = "effect_shoot" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_spikeball_shoot_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {0} else {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)};
    if stance == 1 {
        for _ in 0..999 {
            if is_excute(fighter) {
                if boma.is_status(*WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_HOP){
                    EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_mouth"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 3.0, false);
                }
            }
            wait(lua_state, 3.0);
        }
    }
}

#[acmd_script( agent = "packun_spikeball", script = "game_fall", category = ACMD_GAME, low_priority )]
unsafe fn packun_spikeball_game_fall(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {0} else {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)};
    let scale = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {1.0} else {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)};
    if is_excute(fighter) {
        if stance == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

#[acmd_script( agent = "packun_spikeball", script = "effect_fall" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_spikeball_fall_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {0} else {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)};
    if stance == 1 {
        if is_excute(fighter) {
            //EFFECT_BRANCH_SITUATION(fighter, Hash40::new("null"), Hash40::new("sys_bound_smoke"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_mouth"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, false);
        }
    }
}

#[acmd_script( agent = "packun_spikeball", script = "game_wait", category = ACMD_GAME, low_priority )]
unsafe fn packun_spikeball_game_wait(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {0} else {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)};
    let scale = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {1.0} else {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)};
    if is_excute(fighter) {
        if stance == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    /*frame(lua_state, 55.0);
    if is_excute(fighter) {
        if stance == 1 {
            WorkModule::off_flag(boma, *WEAPON_PACKUN_SPIKEBALL_STATUS_HOP_WORK_FLAG_CLEARED_ATTACK);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 55, 80, 0, 50, 999.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 361, 45, 3.0, false);
        }
    }*/
}

#[acmd_script( agent = "packun_spikeball", script = "effect_wait" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_spikeball_wait_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if owner_module_accessor.kind() != *FIGHTER_KIND_PACKUN {0} else {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)};
    if stance == 1 {
        if is_excute(fighter) {
            //EFFECT_BRANCH_SITUATION(fighter, Hash40::new("null"), Hash40::new("sys_bound_smoke"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_mouth"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, false);
        }
    }
}

#[acmd_script( agent = "packun_spikeball", script = "game_explode", category = ACMD_GAME, low_priority )]
unsafe fn packun_spikeball_game_explode(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ModelModule::set_scale(boma, 0.1);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 45, 120, 0, 45, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(boma, 0, 151, 30, 3.5, false);
    }
}

#[acmd_script( agent = "packun_spikeball", script = "effect_explode", category = ACMD_EFFECT, low_priority )]
unsafe fn packun_spikeball_effect_explode(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flame"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 16.0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.01, 0.7);
        LAST_EFFECT_SET_RATE(fighter, 1.25);
    }
}

#[acmd_script( agent = "packun_spikeball", script = "sound_explode", category = ACMD_SOUND, low_priority )]
unsafe fn packun_spikeball_sound_explode(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}

#[acmd_script( agent = "packun_poisonbreath", script = "game_shoot", category = ACMD_GAME, low_priority )]
unsafe fn game_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.9, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.2, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 2, 3);
        AttackModule::set_no_hop_opponent_all(boma, true, false);
    }
}

#[acmd_script( agent = "packun_poisonbreath", script = "game_shootmax", category = ACMD_GAME, low_priority )]
unsafe fn game_shootmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 7, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 7, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_no_hop_opponent_all(boma, true, false);
    }
}

#[acmd_script( agent = "packun_poisonbreath", script = "game_start", category = ACMD_GAME, low_priority )]
unsafe fn game_start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.9, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.2, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 2, 3);
        AttackModule::set_no_hop_opponent_all(boma, true, false);
    }
}

#[acmd_script( agent = "packun_poisonbreath", script = "game_startmax", category = ACMD_GAME, low_priority )]
unsafe fn game_startmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 7, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 7, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_no_hop_opponent_all(boma, true, false);
    }
}

#[acmd_script(agent = "packun_poisonbreath", script =  "game_explode", category = ACMD_GAME, low_priority)]
unsafe fn packun_poisonbreath_game_explode(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 55, 90, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 55, 95, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		WorkModule::set_int(boma, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	}
}	

#[acmd_script(agent = "packun_poisonbreath", script =  "effect_explode", category = ACMD_EFFECT, low_priority)]
unsafe fn packun_poisonbreath_effect_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
	if is_excute(agent) {
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_breath"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_breath2"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_gas"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_max"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_max_smoke"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_mouth"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_mouth2"), false, false);
		EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 0.7);
    }
}

#[acmd_script(agent = "packun_poisonbreath", script =  "sound_explode", category = ACMD_SOUND, low_priority)]
unsafe fn packun_poisonbreath_sound_explode(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
        packun_catch_game,
        packun_catch_attack_game,
        dash_game,
        dash_sound,
        turn_dash_game,
        appeal_hi_game,
        appeal_hi_sound,
        appeal_hi_2_game,
        appeal_hi_2_effect,
        appeal_hi_2_sound,
        appeal_hi_2_expression,
        appeal_s_game,
        appeal_s_sound,
        appeal_lw_game,
        appeal_lw_sound,
        packun_spikeball_game_start,
        packun_spikeball_game_start_air,
        packun_spikeball_game_loop,
        packun_spikeball_loop_effect,
        packun_spikeball_game_shoot,
        packun_spikeball_shoot_effect,
        packun_spikeball_game_fall,
        packun_spikeball_fall_effect,
        packun_spikeball_game_wait,
        packun_spikeball_wait_effect,
        packun_spikeball_game_explode,
        packun_spikeball_effect_explode,
        packun_spikeball_sound_explode,
        packun_poisonbreath_game_explode,
        packun_poisonbreath_effect_explode,
        packun_poisonbreath_sound_explode,
    );
}

