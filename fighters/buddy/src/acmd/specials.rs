use super::*;


pub fn install() {
    install_acmd_scripts!(
        buddy_special_s_dash_game,
        buddy_special_air_s_dash_game
    );
}

#[acmd_script( agent = "buddy", script = "game_specialsdash" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_special_s_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag( boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR);
        JostleModule::set_status( boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 43, 72, 0, 66, 3.8, 0.0, 4.2, 1.8, Some(0.0), Some(4.2), Some(3.2), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 43, 72, 0, 66, 4.2, 0.0, 9.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        //AttackModule::set_captured_same_time_attack(boma, 0, true);
        //AttackModule::set_captured_same_time_attack(boma, 1, true);
        //AttackModule::set_captured_same_time_attack_damage_mul(boma, 0, 0.25);
        //AttackModule::set_captured_same_time_attack_damage_mul(boma, 1, 0.25);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.48);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        HIT_NO(fighter, 0, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 1, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 2, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 3, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 4, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 5, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 6, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 7, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 8, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 9, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 10, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 11, *HIT_STATUS_NORMAL);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        WorkModule::on_flag( boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_CLIFF_CHECK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 43, 63, 0, 66, 3.8, 0.0, 4.2, 1.8, Some(0.0), Some(4.2), Some(3.2), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 43, 63, 0, 66, 4.2, 0.0, 9.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12.0);
        //AttackModule::set_captured_same_time_attack(boma, 0, true);
        //AttackModule::set_captured_same_time_attack(boma, 1, true);
        //AttackModule::set_captured_same_time_attack_damage_mul(boma, 0, 0.25);
        //AttackModule::set_captured_same_time_attack_damage_mul(boma, 1, 0.25);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.28);
    }
}

#[acmd_script( agent = "buddy", script = "game_specialairsdash" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_special_air_s_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag( boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR);
        JostleModule::set_status( boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 43, 72, 0, 66, 3.8, 0.0, 4.2, 1.8, Some(0.0), Some(4.2), Some(3.2), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 43, 72, 0, 66, 4.2, 0.0, 9.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.48);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 7.0);
        WorkModule::on_flag( boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_CLIFF_CHECK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 43, 63, 0, 66, 3.8, 0.0, 4.2, 1.8, Some(0.0), Some(4.2), Some(3.2), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 43, 63, 0, 66, 4.2, 0.0, 9.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12.0);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.28);
    }
}