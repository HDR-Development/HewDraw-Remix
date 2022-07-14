
use super::*;

#[acmd_script( agent = "ike", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "ike", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "ike", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "ike", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "ike", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "ike", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn ike_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 5.0/(7.0-1.0));
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.1, 0.0, 8.0, 0.0, Some(0.0), Some(8.0), Some(11.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "ike", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "ike", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "ike", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "ike", script = "sound_appeallwl" , category = ACMD_SOUND , low_priority)]
unsafe fn ike_appeal_lw_l_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ike_swing_l"));
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            let costume_type = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2;
            // Path of Radiance voiceline
            if costume_type == 0 {
                PLAY_SE(fighter, Hash40::new("vc_ike_win02"));
            }
            // Radiant Dawn voiceline
            else{
                PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
            }
        }
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_stab"));
    }
    wait(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            //PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
        }
        else{
            PLAY_SE(fighter, Hash40::new("vc_ike_appeal03"));
        }
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_04"));
    }
    frame(lua_state, 98.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_pullout"));
    }
}

#[acmd_script( agent = "ike", script = "sound_appeallwr" , category = ACMD_SOUND , low_priority)]
unsafe fn ike_appeal_lw_r_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ike_swing_l"));
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            let costume_type = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2;
            // Path of Radiance voiceline
            if costume_type == 0 {
                PLAY_SE(fighter, Hash40::new("vc_ike_win02"));
            }
            // Radiant Dawn voiceline
            else{
                PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
            }
        }
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_stab"));
    }
    wait(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            //PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
        }
        else{
            PLAY_SE(fighter, Hash40::new("vc_ike_appeal03"));
        }
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_04"));
    }
    frame(lua_state, 98.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_pullout"));
    }
}

#[acmd_script( agent = "ike_sword", script = "game_appeallw" , category = ACMD_GAME , low_priority)]
unsafe fn ike_sword_appeal_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        // Air-only
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 9.0, 270, 80, 0, 100, 4.0, 0.0, 6.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 9.0, 270, 80, 0, 100, 7.0, 0.0, 12.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        // Ground-only
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 9.0, 270, 100, 275, 0, 4.0, 0.0, 6.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 9.0, 270, 100, 275, 0, 7.0, 0.0, 12.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 2, 15.0, false);
        AttackModule::set_add_reaction_frame(boma, 3, 15.0, false);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            SlowModule::set_whole(boma, 4, 45);
        }
    }
    for _ in 0..3{
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                SlowModule::set_whole(boma, 4, 45);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "ike_sword", script = "effect_appeallw" , category = ACMD_EFFECT , low_priority)]
unsafe fn ike_sword_appeal_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            EFFECT(fighter, Hash40::new("ike_volcano_ground"), Hash40::new("top"), 0, 0.0, -8.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("ike_volcano"), Hash40::new("top"), 0, 0.0, -8.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        ike_catch_game,
        dash_game,
        //dash_effect,
        turn_dash_game,
        ike_appeal_lw_l_sound,
        ike_appeal_lw_r_sound,
        ike_sword_appeal_lw_game,
        ike_sword_appeal_lw_effect,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}

