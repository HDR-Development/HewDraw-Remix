
use super::*;

#[acmd_script( agent = "miiswordsman", script = "game_specialairlw3endair" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_lw3_end_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 1, Hash40::new("haver"), 1.0, 100, 10, 0, 10, 4.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 4.0, y: 5.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 0.0, y: 17.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 4, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        //REVERSE_LR(fighter);
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 17.0, 0.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -6.0, y: 15.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 3, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        //REVERSE_LR(fighter);
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 15.0, -6.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -12.0, y: 0.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 2, false);
    }
    /*
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 0.1, 365, 30, 0, 0, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        //let hit1 = Vector2f {x: 12.0, y: 0.0};
        //AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    */
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        ATTACK(fighter, 0, 2, Hash40::new("haver"), 5.0, 80, 100, 0, 60, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        miiswordsman_special_n1_game,
        miiswordsman_special_n1_effect,
        miiswordsman_special_air_n1_game,
        miiswordsman_special_air_n1_effect,
        miiswordsman_special_n2_game,
        miiswordsman_special_n2_effect,
        miiswordsman_special_n2_sound,
        miiswordsman_special_air_n2_game,
        miiswordsman_special_air_n2_effect,
        miiswordsman_special_air_n2_sound,
        miiswordsman_special_n3_end_game,
        miiswordsman_special_n3_end_effect,
        miiswordsman_special_n3_end_sound,
        miiswordsman_special_n3_end_turn_game,
        miiswordsman_special_n3_end_turn_effect,
        miiswordsman_special_n3_end_turn_sound,
        miiswordsman_special_n3_end_max_game,
        miiswordsman_special_n3_end_max_effect,
        miiswordsman_special_n3_end_max_sound,
        miiswordsman_special_n3_end_max_turn_game,
        miiswordsman_special_n3_end_max_turn_effect,
        miiswordsman_special_n3_end_max_turn_sound,
        miiswordsman_special_air_n3_end_game,
        miiswordsman_special_air_n3_end_effect,
        miiswordsman_special_air_n3_end_sound,
        miiswordsman_special_air_n3_end_turn_game,
        miiswordsman_special_air_n3_end_turn_effect,
        miiswordsman_special_air_n3_end_turn_sound,
        miiswordsman_special_air_n3_end_max_game,
        miiswordsman_special_air_n3_end_max_effect,
        miiswordsman_special_air_n3_end_max_sound,
        miiswordsman_special_air_n3_end_max_turn_game,
        miiswordsman_special_air_n3_end_max_turn_effect,
        miiswordsman_special_air_n3_end_max_turn_sound,

        miiswordsman_special_s1_start_game,
        miiswordsman_special_air_s1_start_game,
        miiswordsman_special_s1_game,
        miiswordsman_special_air_s1_game,
        miiswordsman_special_s1_hit_game,
        miiswordsman_special_s2_dash_game,
        miiswordsman_special_s2_attack_game,
        miiswordsman_special_s2_attack_effect,
        miiswordsman_special_air_s2_dash_game,
        miiswordsman_special_air_s2_attack_game,
        miiswordsman_special_air_s2_attack_effect,
        miiswordsman_special_s3_1_game,
        miiswordsman_special_s3_1_hi_game,
        miiswordsman_special_s3_1_lw_game,
        miiswordsman_special_air_s3_1_game,
        miiswordsman_special_air_s3_1_hi_game,
        miiswordsman_special_air_s3_1_lw_game,

        miiswordsman_special_hi1_start_game,
        miiswordsman_special_air_hi1_game,
        miiswordsman_special_hi1_end_game,
        miiswordsman_special_air_hi1_end_game,
        miiswordsman_special_hi2_hold_game,
        miiswordsman_special_hi2_hold_air_game,
        miiswordsman_special_hi2_landing_game,
        miiswordsman_special_hi2_fall_game,
        miiswordsman_special_hi2_game,
        miiswordsman_special_hi3_game,
        miiswordsman_special_air_hi3_game,

        miiswordsman_special_lw1_hit_game,
        miiswordsman_special_air_lw1_hit_game,
        miiswordsman_special_lw1_hit_lv1_game,
        miiswordsman_special_air_lw1_hit_lv1_game,
        miiswordsman_special_lw1_hit_lv2_game,
        miiswordsman_special_air_lw1_hit_lv2_game,
        miiswordsman_special_lw2_game,
        miiswordsman_special_lw2_effect,
        miiswordsman_special_air_lw2_game,
        miiswordsman_special_air_lw2_effect,
        miiswordsman_special_air_lw2_game,
        miiswordsman_special_lw3_game,
        miiswordsman_special_lw3_end_game,
        miiswordsman_special_air_lw3_game,
        miiswordsman_special_air_lw3_end_game,
        miiswordsman_special_air_lw3_end_air_game,
    );
}

