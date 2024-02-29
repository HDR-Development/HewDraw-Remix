use super::*;

unsafe extern "C" fn rockman_hardknuckle_regular_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 270, 61, 0, 8, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 270, 74, 0, 38, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 3.0);
	if is_excute(fighter) {
        WorkModule::on_flag(boma, *WEAPON_ROCKMAN_HARDKNUCKLE_INSTANCE_WORK_ID_FLAG_ATTACK_VECTOR_REVERSE_UD_CHECK);
    }
    frame(lua_state, 5.0);
	if is_excute(fighter) {
        sv_kinetic_energy!(
            set_speed,
            fighter,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            -0.5
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_brake,
            fighter,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            0.05
        );
    }
    frame(lua_state, 6.0);
	if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn rockman_hardknuckle_regular_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("rockman_hardknuckle"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rockman_hardknuckle"), true, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", rockman_hardknuckle_regular_game);
    agent.acmd("effect_regular", rockman_hardknuckle_regular_effect);
}
