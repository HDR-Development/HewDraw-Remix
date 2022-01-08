
use super::*;

#[acmd_script( agent = "metaknight", script = "game_speciallwsubairb" , category = ACMD_GAME , low_priority)]
unsafe fn metaknight_special_lw_sub_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 11.2, 0.0, 7.5, -6.0, Some(0.0), Some(7.5), Some(-8.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 7.5, 0.0, 6.5, -21.0, Some(0.0), Some(6.5), Some(2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
}

pub fn install() {
    install_acmd_scripts!(
        metaknight_special_n_spin_game,
        metaknight_special_n_end_game,
        metaknight_special_air_n_end_game,
        metaknight_special_hi_game,
        metaknight_special_hi_loop_game,
        metaknight_special_lw_f_game,
        metaknight_special_lw_sub_f_game,
        metaknight_special_lw_b_game,
        metaknight_special_lw_sub_b_game,
        metaknight_special_air_lw_f_game,
        metaknight_special_lw_sub_air_f_game,
        metaknight_special_air_lw_b_game,
        metaknight_special_lw_sub_air_b_game,
    );
}
