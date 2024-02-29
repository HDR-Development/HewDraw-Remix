use super::*;

unsafe extern "C" fn koopa_attack_s4_hold_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        let eff_handle = EffectModule::req_follow(boma, Hash40::new("sys_explosion_sign"), Hash40::new("haver"), &Vector3f::zero(), &Vector3f::zero(), 0.75, false, 0, 0, 0, 0, 0, false, false);
        VarModule::set_int64(fighter.battle_object, vars::koopa::instance::CHARGE_EFFECT_HANDLER, eff_handle as u64);
    }
    frame(lua_state, 2.0);
    for _ in 0..34 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -8, 0, 0, 0, 1, 20, 0, 8, 0, 0, 0, false);
        }
        wait(lua_state, 5.0);
        if is_excute(fighter) {
            EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 6, 3, 6, 0, 0, 0, true, *EF_FLIP_YZ);
        }
    }
}

unsafe extern "C" fn koopa_attack_s4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::koopa::instance::IS_EXCELLENT_PUNCH);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::koopa::status::PUNCH_CAN_ZOOM);
        let excellent = VarModule::is_flag(fighter.battle_object, vars::koopa::instance::IS_EXCELLENT_PUNCH);
        let damage = if excellent { 2.5 } else { 0.0 };
        let attr = if excellent { Hash40::new("collision_attr_magic") } else { Hash40::new("collision_attr_normal") };
        let sound = if excellent { *COLLISION_SOUND_ATTR_HEAVY } else { *COLLISION_SOUND_ATTR_PUNCH };
        ATTACK(fighter, 0, 0, Hash40::new("top"), 23.0 + damage, 361, 97, 0, 28, 8.0, 0.0, 8.5, 20.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 23.0 + damage, 361, 97, 0, 28, 6.0, 0.0, 8.5, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE_RANGE(fighter, 26.0, 40.0, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        SlowModule::clear_whole(fighter.module_accessor);
        CameraModule::reset_all(fighter.module_accessor);
        CAM_ZOOM_OUT(fighter);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(fighter, 40.0, 85.0, 36.0);
    frame(lua_state, 85.0);
    FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn koopa_attack_s4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 12, 15, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopa_smash_line"), Hash40::new("koopa_smash_line"), Hash40::new("top"), 0, 13, -20, 0, 0, 0, 2.3, true, *EF_FLIP_YZ);
        let eff_handle = VarModule::get_int64(fighter.battle_object, vars::koopa::instance::CHARGE_EFFECT_HANDLER);
        EffectModule::set_scale(boma, eff_handle as u32, &Vector3f::zero());
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_explosion_sign"), false, false);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopa_smash_line"), Hash40::new("koopa_smash_line"), Hash40::new("top"), 0, 10, -26, 0, 0, 0, 2.3, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("handr"), 3, 0, 0, 0, 0, 0, 1.65, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(fighter.module_accessor);
        CameraModule::reset_all(fighter.module_accessor);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line"), true, true);
    }
}	

unsafe extern "C" fn koopa_attack_s4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_nailswing02"));
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack06"));
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_smash_h01"));
    }
}

unsafe extern "C" fn koopa_attack_s4_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 4.0);
    app::sv_animcmd::execute(lua_state, 4.0);
    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK){
        if is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 2);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 8);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        let excellent = VarModule::is_flag(fighter.battle_object, vars::koopa::instance::IS_EXCELLENT_PUNCH);
        if excellent {
            RUMBLE_HIT(fighter, Hash40::new("rbkind_attack_finish"), 0);
        } else {
            RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
        }
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

unsafe extern "C" fn koopa_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_INVINCIBLE);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 22.0, 90, 90, 0, 23, 8.0, 0.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 22.0, 90, 90, 0, 23, 6.5, 0.0, -3.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("rot"), 22.0, 90, 90, 0, 23, 6.5, 0.0, -3.0, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}

unsafe extern "C" fn koopa_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9.0);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 16.0, 40, 90, 0, 56, 6.0, 0.0, 0.0, 1.0, Some(5.3), Some(0.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 14.5, 40, 90, 0, 56, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 14.5, 40, 90, 0, 56, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(fighter, 15.0, 27.0, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 15.0, 40, 96, 0, 56, 6.0, 0.0, -0.6, 0.5, Some(4.5), Some(-0.6), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.5, 40, 96, 0, 56, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 13.5, 40, 96, 0, 56, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    smashline::Agent::new("koopa")
        .acmd("effect_attacks4charge", koopa_attack_s4_hold_effect)
        .acmd("game_attacks4", koopa_attack_s4_game)
        .acmd("effect_attacks4", koopa_attack_s4_effect)
        .acmd("sound_attacks4", koopa_attack_s4_sound)
        .acmd("expression_attacks4", koopa_attack_s4_expression)
        .acmd("game_attackhi4", koopa_attack_hi4_game)
        .acmd("game_attacklw4", koopa_attack_lw4_game)
        .install();
}
