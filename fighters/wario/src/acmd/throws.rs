
use super::*;

#[acmd_script( agent = "wario", script = "game_catchattack" , category = ACMD_GAME , low_priority)]
unsafe fn catch_attack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 361, 100, 30, 0, 5.0, 0.0, 10.0, 7.0, None, None, None, 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        catch_attack,
    );
}

