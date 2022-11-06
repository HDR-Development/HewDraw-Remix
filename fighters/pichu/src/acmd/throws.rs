
use super::*;

#[acmd_script( agent = "pichu", script = "game_throwf" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_throw_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0 * damage_mul, 45, 110, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0 * damage_mul, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_ADD_DAMAGE(fighter, 0.5 * recoil_mul);
    }
    for _ in 0..4{
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5 * damage_mul, 361, 100, 0, 0, 5.5, 0.0, 8.5, 4.7, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            AttackModule::set_catch_only_all(boma, true, false);
        }
        wait(lua_state, 4.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 6, 26);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}
#[acmd_script( agent = "pichu", script = "game_throwb" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_throw_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(fighter) {
        if charged {
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0 * damage_mul, 80, 50, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        else {
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 135, 50, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if charged {
            FT_MOTION_RATE(fighter, 0.5);
            FT_ADD_DAMAGE(fighter, 1.0);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, -6, 4);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}
#[acmd_script( agent = "pichu", script = "effect_throwb" , category = ACMD_EFFECT , low_priority)]
unsafe fn pichu_throw_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 1, 0, 0, 90, 0.55, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 1, 0, 30, 90, 0.55, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -0.5, 0, 0, 90, 0.55, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -0.5, 0, 30, 90, 0.55, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        if charged {
            EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}
#[acmd_script( agent = "pichu", script = "game_throwhi" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_throw_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(fighter) {
        if charged {
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 90, 90, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        else {
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 90, 45, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hat"), 5.0 * damage_mul, 80, 60, 0, 70, 4.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(boma, true, false);
        FT_ADD_DAMAGE(fighter, 1.0 * recoil_mul);
        CHECK_FINISH_CAMERA(fighter, 2, 17);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}
pub fn install() {
    install_acmd_scripts!(
        pichu_throw_f_game,
        pichu_throw_b_game,
        pichu_throw_b_effect,
        pichu_throw_hi_game,
    );
}

