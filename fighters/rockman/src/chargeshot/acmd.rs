use super::*;

unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let scale = PostureModule::scale(boma);
    let is_charge_max = 1.0 <= WorkModule::get_float(boma, *WEAPON_ROCKMAN_CHARGESHOT_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
    let (dmg, kbg, bkb, shield_dmg) = if is_charge_max {
        (20.0, 69, 41, 7)
    } else {
        (9.0, 78, 50, 6)
    };
    let early_power_mul = 0.8;
    let mid_power_mul = 0.9;
    let late_power_mul = 1.0;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), dmg * early_power_mul, 361, kbg, 0, bkb, 2.6, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, shield_dmg, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.32);
        AttackModule::enable_safe_pos(boma);
        PostureModule::set_scale(boma, scale * early_power_mul, false);
    }
    wait(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), dmg * mid_power_mul, 361, kbg, 0, bkb, 2.6, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, shield_dmg, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.32);
        PostureModule::set_scale(boma, scale * mid_power_mul, false);
    }
    wait(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), dmg * late_power_mul, 361, kbg, 0, bkb, 2.6, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, shield_dmg, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.32);
        PostureModule::set_scale(boma, scale * late_power_mul, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", game_regular, Priority::Low);
}
