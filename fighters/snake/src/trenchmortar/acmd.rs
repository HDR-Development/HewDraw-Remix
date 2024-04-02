use super::*;

unsafe extern "C" fn game_impact(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
        KineticModule::unable_energy(boma, *WEAPON_SNAKE_TRENCHMORTAR_BULLET_KINETIC_ENERGY_ID_GRAVITY);
        VisibilityModule::set_int64(boma, hash40("main") as i64, hash40("impact") as i64);
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 77, 80, 0, 45, 12.0, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_SNAKE_TRENCHMORTAR_BULLET_STATUS_FLAG_ENABLE_ADVANCE_STATUS);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_impact", game_impact);
}