
use super::*;

unsafe extern "C" fn game_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    if is_excute(fighter) {
        let dmg = if variation == 2 {
            15.3
        } else {
            9.0
        };
        ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, dmg * p.dmg, 45 + p.angle, 60, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, p.attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        let target = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK);
        let target_group = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    if is_excute(fighter) {
        let dmg = if variation == 2 {
            11.9
        } else {
            7.0
        };
        ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, dmg * p.dmg, 361, 60, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, p.attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK);
        let target_group = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    if is_excute(fighter) {
        let dmg = if variation == 2 {
            12.5
        } else {
            7.0
        };
        ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, dmg * p.dmg, 85 + p.angle, 80, 0, 81, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, p.attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        let dmg = 1.0;
        ATTACK(fighter, 0, 0, Hash40::new("head1"), dmg * p.dmg, 90 + p.angle, 200, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_S, p.sound, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        let target = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK);
        let target_group = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        let dmg = 1.0;
        ATTACK(fighter, 0, 0, Hash40::new("top"), dmg * p.dmg, 90 + p.angle, 200, 0, 50, 7.0, 0.0, 4.0, 0.0, None, None, None, p.hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, dmg * p.shield_dmg, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, p.attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(boma, true, false);
        let dmg = if variation == 2 {
            11.9
        } else {
            7.0
        };
        ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, dmg * p.dmg, 70 + p.angle, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, p.attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        let target = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK);
        let target_group = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

pub fn install() {
    smashline::Agent::new("pikmin_pikmin")
        .acmd("game_throwb", game_throwb)
        .acmd("game_throwb_b", game_throwb)
        .acmd("game_throwb_v", game_throwb)
        .acmd("game_throwb_w", game_throwb)
        .acmd("game_throwb_y", game_throwb)
        .acmd("game_throwf", game_throwf)
        .acmd("game_throwf_b", game_throwf)
        .acmd("game_throwf_v", game_throwf)
        .acmd("game_throwf_w", game_throwf)
        .acmd("game_throwf_y", game_throwf)
        .acmd("game_throwhi", game_throwhi)
        .acmd("game_throwhi_b", game_throwhi)
        .acmd("game_throwhi_v", game_throwhi)
        .acmd("game_throwhi_w", game_throwhi)
        .acmd("game_throwhi_y", game_throwhi)
        .acmd("game_throwlw", game_throwlw)
        .acmd("game_throwlw_b", game_throwlw)
        .acmd("game_throwlw_v", game_throwlw)
        .acmd("game_throwlw_w", game_throwlw)
        .acmd("game_throwlw_y", game_throwlw)
        .install();
}
