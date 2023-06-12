
use super::*;


#[acmd_script( agent = "pichu", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(fighter){
        if !charged {
            MeterModule::watch_damage(fighter.battle_object, true);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if !charged {
            MeterModule::add(fighter.battle_object, 4.0);
        }
        FT_ADD_DAMAGE(fighter, 2.5 * recoil_mul);
    }
    frame(lua_state, 16.0);
    for _ in 0..5 {
        if is_excute(fighter) {
            if !charged {
                MeterModule::watch_damage(fighter.battle_object, true);
            }
            ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0 * damage_mul, 366, 50, 0, 10, 5.3 * damage_mul, 0.0, 4.7, 11.4, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0 * damage_mul, 20, 50, 0, 10, 4.3 * damage_mul, 0.0, 4.7, 8.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            MeterModule::watch_damage(fighter.battle_object, false);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        if !charged {
            MeterModule::watch_damage(fighter.battle_object, true);
        }
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0 * damage_mul, 40, 130, 0, 65, 5.3 * damage_mul, 0.0, 4.7, 11.4, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0 * damage_mul, 40, 130, 0, 65, 4.3 * damage_mul, 0.0, 4.7, 8.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
    
}

#[acmd_script( agent = "pichu", script = "effect_attacks4" , category = ACMD_EFFECT , low_priority)]
unsafe fn pichu_attack_s4_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek2"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1.0 * damage_mul, true);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek2"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1.0 * damage_mul, true);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8 * damage_mul, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("pichu_cheek2"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1.0 * damage_mul, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek_elec"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1.0 * damage_mul, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_elec_shock"), Hash40::new("top"), 0, 5.5, 13, 0, 0, 0, 0.85 * damage_mul, true);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_elec_shock_finish"), Hash40::new("top"), 0, 5.5, 13, 0, 0, 0, 0.85 * damage_mul, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pichu_elec_shock"), false, true);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek_elec"), false, true);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek2"), false, true);
    }
}

#[acmd_script( agent = "pichu", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if !charged {
            ATTACK(fighter, 0, 0, Hash40::new("neck"), 15.0, 95, 98, 0, 40, 6.0, 4.7, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 15.0, 95, 98, 0, 15, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("neck"), 18.0, 95, 98, 0, 40, 6.0, 4.7, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 18.0, 95, 98, 0, 15, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            FT_ADD_DAMAGE(fighter, 2.5);
        }
        HIT_NODE(fighter, Hash40::new("mimir1"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mimil1"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}

#[acmd_script( agent = "pichu", script = "effect_attackhi4" , category = ACMD_EFFECT , low_priority)]
unsafe fn pichu_attack_hi4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        if charged {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek2"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter){
        if charged {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek2"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("pichu_tail_arc3"), Hash40::new("pichu_tail_arc3"), Hash40::new("top"), 1, 5, 0, 10, -40, -110, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 3, -0.5, 32, -75, -138, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        if charged {
            EFFECT_FOLLOW(fighter, Hash40::new("pichu_cheek2"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek_elec"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        if charged {
            EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek2"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek_elec"), false, true);
        }
    }
}

#[acmd_script( agent = "pichu", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 2.0);
        if !charged {
            MeterModule::watch_damage(fighter.battle_object, true);
            MeterModule::add(fighter.battle_object, 3.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 55, 120, 0, 45, 6.0, 0.0, 4.5, -5.5, Some(0.0), Some(4.5), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.4, 125, 120, 0, 45, 6.0, 0.0, 4.5, -5.5, Some(0.0), Some(4.5), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        FT_ADD_DAMAGE(fighter, 1.5 * recoil_mul);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        if !charged {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 55, 90, 0, 45, 4.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(-5.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }

}

pub fn install() {
    install_acmd_scripts!(
        pichu_attack_s4_s_game,
        pichu_attack_s4_s_effect,
        pichu_attack_hi4_game,
        pichu_attack_hi4_effect,
        pichu_attack_lw4_game,
    );
}

