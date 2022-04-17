
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

#[acmd_script( agent = "packun", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
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

// also used for the hop status
#[acmd_script( agent = "packun_spikeball", script = "game_shoot" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_spikeball_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *WEAPON_PACKUN_SPIKEBALL_INSTANCE_WORK_ID_FLAG_REACTIVE){
            ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 55, 80, 0, 50, 1.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 55, 80, 0, 50, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        DamageModule::add_damage(owner_module_accessor, 10.0, 0);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 55, 80, 0, 45, 1.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
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

pub fn install() {
    install_acmd_scripts!(
        packun_catch_game,
        dash_game,
        dash_effect,
        turn_dash_game,
        packun_spikeball_shoot_game,
        packun_spikeball_shoot_effect,
        packun_spikeball_loop_effect,
        packun_spikeball_fall_effect,
        packun_spikeball_wait_effect,
    );
}

