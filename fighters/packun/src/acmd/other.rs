
use super::*;

#[acmd_script( agent = "packun", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn packun_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.200);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
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

    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "packun", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "packun_spikeball", scripts = ["game_start", "game_start_air"] , category = ACMD_GAME , low_priority)]
unsafe fn packun_spikeball_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = StanceInfo::from(VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ModelModule::set_scale(boma, stance.ptooie_size);
        MotionModule::set_rate(boma, 1.4);
        if stance.label == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0 * stance.damage_nspecial, 55, 80, 0, 50, 5.0 * stance.ptooie_size, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, /*ID*/ 0, /*Frames*/ 181, /*Rehit*/ 45, /*Damage*/ 2.0, /*Unk*/ false);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0 * stance.damage_nspecial, 55, 80, 0, 50, 5.0 * stance.ptooie_size, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

// also used for the hop status
#[acmd_script( agent = "packun_spikeball", script = "game_shoot" , category = ACMD_GAME , low_priority)]
unsafe fn packun_spikeball_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = StanceInfo::from(VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE));
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *WEAPON_PACKUN_SPIKEBALL_INSTANCE_WORK_ID_FLAG_REACTIVE){
            if stance.label == 1 {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0 * stance.damage_nspecial, 55, 80, 0, 50, 1.0 * stance.ptooie_size, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                AttackModule::set_poison_param(boma, /*ID*/ 0, /*Frames*/ 181, /*Rehit*/ 45, /*Damage*/ 2.0, /*Unk*/ false);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0 * stance.damage_nspecial, 55, 80, 0, 50, 1.0 * stance.ptooie_size, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
        else{
            if stance.label == 1 {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0 * stance.damage_nspecial, 55, 80, 0, 50, 3.8 * stance.ptooie_size, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                AttackModule::set_poison_param(boma, /*ID*/ 0, /*Frames*/ 181, /*Rehit*/ 45, /*Damage*/ 2.0, /*Unk*/ false);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0 * stance.damage_nspecial, 55, 80, 0, 50, 3.8 * stance.ptooie_size, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
        
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if stance.label == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0 * stance.damage_nspecial, 55, 80, 0, 45, 1.0 * stance.ptooie_size, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, /*ID*/ 0, /*Frames*/ 181, /*Rehit*/ 45, /*Damage*/ 2.0, /*Unk*/ false);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0 * stance.damage_nspecial, 55, 80, 0, 45, 1.0 * stance.ptooie_size, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

#[acmd_script( agent = "packun_spikeball", scripts = ["game_fall", "game_loop", "game_wait"] , category = ACMD_GAME , low_priority)]
unsafe fn packun_spikeball_fall_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = StanceInfo::from(VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE));
    if is_excute(fighter) {
        if stance.label == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0 * stance.damage_nspecial, 55, 80, 0, 50, 5.0 * stance.ptooie_size, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, /*ID*/ 0, /*Frames*/ 181, /*Rehit*/ 45, /*Damage*/ 2.0, /*Unk*/ false);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("trans"), 14.0 * stance.damage_nspecial, 55, 80, 0, 50, 5.0 * stance.ptooie_size, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

#[acmd_script( agent = "packun_spikeball", script = "effect_shoot" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_spikeball_shoot_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    for _ in 0..999 {
        if is_excute(fighter) {
            if boma.is_status(*WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_HOP){
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 3.0, false);
            }
        }
        wait(lua_state, 3.0);
    }
}

#[acmd_script( agent = "packun_spikeball", script = "effect_loop" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_spikeball_loop_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        //EFFECT_BRANCH_SITUATION(fighter, Hash40::new("null"), Hash40::new("sys_bound_smoke"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, false);
    }
}

#[acmd_script( agent = "packun_spikeball", script = "effect_fall" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_spikeball_fall_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        //EFFECT_BRANCH_SITUATION(fighter, Hash40::new("null"), Hash40::new("sys_bound_smoke"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, false);
    }
}

#[acmd_script( agent = "packun_spikeball", script = "effect_wait" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_spikeball_wait_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        //EFFECT_BRANCH_SITUATION(fighter, Hash40::new("null"), Hash40::new("sys_bound_smoke"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, false);
    }
}

#[acmd_script( agent = "packun_poisoncloud", script = "game_start" , category = ACMD_GAME , low_priority)]
unsafe fn packun_poisoncloud_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = StanceInfo::from(VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.2 * stance.damage_sspecial, 361, 0, 0, 0 + stance.bkb_sspecial, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4 * stance.rehit_sspecial, true, false, false, stance.flinchless_sspecial, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6 * stance.damage_sspecial, 361, 0, 0, 0 + stance.bkb_sspecial, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4 * stance.rehit_sspecial, true, false, false, stance.flinchless_sspecial, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_no_hop_opponent_all(boma, stance.flinchless_sspecial, false);
    }
}

#[acmd_script( agent = "packun_poisoncloud", script = "game_startmax" , category = ACMD_GAME , low_priority)]
unsafe fn packun_poisoncloud_start_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = StanceInfo::from(VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8 * stance.damage_sspecial, 361, 0, 0, 0 + stance.bkb_sspecial, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 7 * stance.rehit_sspecial, true, false, false, stance.flinchless_sspecial, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6 * stance.damage_sspecial, 361, 0, 0, 0 + stance.bkb_sspecial, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 7 * stance.rehit_sspecial, true, false, false, stance.flinchless_sspecial, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_no_hop_opponent_all(boma, stance.flinchless_sspecial, false);
    }
}

#[acmd_script( agent = "packun_poisoncloud", script = "game_shoot" , category = ACMD_GAME , low_priority)]
unsafe fn packun_poisoncloud_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = StanceInfo::from(VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE));
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.2 * stance.damage_sspecial, 361, 0, 0, 0 + stance.bkb_sspecial, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4 * stance.rehit_sspecial, true, false, false, stance.flinchless_sspecial, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6 * stance.damage_sspecial, 361, 0, 0, 0 + stance.bkb_sspecial, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4 * stance.rehit_sspecial, true, false, false, stance.flinchless_sspecial, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_no_hop_opponent_all(boma, stance.flinchless_sspecial, false);
    }
    frame(lua_state, 85.0);
    if stance.label == 1 {
        if is_excute(fighter) {
            AttackModule::set_no_hop_opponent_all(boma, false, false);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 0, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 0, 0, 20, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
    }
}

#[acmd_script( agent = "packun_poisoncloud", script = "game_shootmax" , category = ACMD_GAME , low_priority)]
unsafe fn packun_poisoncloud_shoot_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = StanceInfo::from(VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE));
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8 * stance.damage_sspecial, 361, 0, 0, 0 + stance.bkb_sspecial, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 7 * stance.rehit_sspecial, true, false, false, stance.flinchless_sspecial, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6 * stance.damage_sspecial, 361, 0, 0, 0 + stance.bkb_sspecial, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 7 * stance.rehit_sspecial, true, false, false, stance.flinchless_sspecial, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_no_hop_opponent_all(boma, stance.flinchless_sspecial, false);
    }
    frame(lua_state, 125.0);
    if stance.label == 1 {
        if is_excute(fighter) {
            AttackModule::set_no_hop_opponent_all(boma, false, false);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 0, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 0, 0, 20, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
        packun_catch_game,
        dash_game,
        dash_sound,
        turn_dash_game,
        packun_spikeball_start_game,
        packun_spikeball_shoot_game,
        packun_spikeball_fall_game,
        packun_spikeball_shoot_effect,
        packun_spikeball_loop_effect,
        packun_spikeball_fall_effect,
        packun_spikeball_wait_effect,
        packun_poisoncloud_start_game,
        packun_poisoncloud_start_max_game,
        packun_poisoncloud_shoot_game,
        packun_poisoncloud_shoot_max_game,
    );
}

