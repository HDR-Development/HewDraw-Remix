use super::*;

#[acmd_script( agent = "metaknight", script = "game_attack100", category = ACMD_GAME, low_priority )]
unsafe fn game_attack100(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 8, 0, 40, 7.0, 0.0, 6.5, 8.0, Some(0.0), Some(6.5), Some(13.5), 0.5, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 4);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 8, 0, 40, 7.0, 0.0, 6.5, 8.0, Some(0.0), Some(6.5), Some(13.5), 0.5, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 4);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 8, 0, 40, 4.0, 0.0, 6.0, -3.0, Some(0.0), Some(6.0), Some(-9.5), 0.5, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 4);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 40, 0, 60, 7.0, 0.0, 6.5, 8.0, Some(0.0), Some(6.5), Some(13.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 4);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 8, 0, 40, 7.0, 0.0, 6.5, 8.0, Some(0.0), Some(6.5), Some(13.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 40, 0, 60, 4.0, 0.0, 6.0, -3.0, Some(0.0), Some(6.0), Some(-9.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 4);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 4);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(fighter, 20.0, 26.0, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE);
    }
}

#[acmd_script( agent = "metaknight", script = "effect_attack100", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attack100(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 1, 6, -1, 20, 165, 105, 0.5, true);
        EffectModule::set_disable_render_offset_last(boma);
        EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.65, 0.7));
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 0, 7, 0, 90, 0, 25, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.8, 0.65));
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 1, 6.5, -1, -165, 20, -80, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.5, 0.75));
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 0, 6, 2, -205, 160, -145, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.7, 0.85));
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 2, 6, 1, 0, -155, 105, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.7, 0.7));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 0, 7, 0, 90, 0, -45, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.85, 0.65));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 2, 4, 4, -165, 20, -95, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.5, 0.75));
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), -2, 6, -4, -25, -30, -60, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.5, 0.75));
    }
}

// #[acmd_script( agent = "metaknight", script = "game_attack100end", category = ACMD_GAME, low_priority )]
// unsafe fn game_attack100end(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = fighter.boma();
//     AttackModule::clear_all(boma);
//     frame(lua_state, 3.0);
//     if is_excute(fighter) {
//         ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 100, 0, 40, 5.0, 0.0, 8.0, 11.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//         ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 100, 0, 40, 6.0, 0.0, 8.0, 16.5, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//     }
//     wait(lua_state, 1.0);
//     if is_excute(fighter) {
//         AttackModule::clear_all(boma);
//     }
//     FT_MOTION_RATE(fighter, 0.87);
// }

// #[acmd_script( agent = "metaknight", script = "effect_attack100end", category = ACMD_EFFECT, low_priority )]
// unsafe fn effect_attack100end(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = fighter.boma();
//     if is_excute(fighter) {
//         LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//         EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_attack"), false, false);
//         EffectModule::set_disable_render_offset_last(boma);
//     }
//     frame(lua_state, 3.0);
//     if is_excute(fighter) {
//         EFFECT_FOLLOW(fighter, Hash40::new("metaknight_attack_end"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 0.9, true);
//         EffectModule::set_disable_render_offset_last(boma);
//         EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
//     }
//     frame(lua_state, 26.0);
//     if is_excute(fighter) {
//         FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//     }
// }

#[acmd_script( agent = "metaknight", script = "sound_attack100start", category = ACMD_SOUND, low_priority )]
unsafe fn sound_attack100start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_metaknight_attack100"));
        // match smash::app::sv_math::rand(smash::hash40("fighter"), 2) {
        //     0 => PLAY_SE(fighter, Hash40::new("vc_metaknight_attack100")),
        //     1 => PLAY_SE(fighter, Hash40::new("vc_metaknight_attack07")),
        //     _ => {}
        // };
    }
}

#[acmd_script( agent = "metaknight", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn metaknight_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.78);
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 7.0, 3.0);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("footl"), 7.5, 65, 107, 0, 65, 5.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("footl"), 7.5, 70, 107, 0, 65, 4.0, 0.0, -6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 6.5, 75, 107, 0, 67, 3.5, 0.0, -9.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("footl"), 5.5, 70, 107, 0, 65, 4.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::clear(boma, 1, false);
        AttackModule::clear(boma, 2, false);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        game_attack100,
        effect_attack100,
        sound_attack100start,
        metaknight_attack_dash_game,
    );
}

