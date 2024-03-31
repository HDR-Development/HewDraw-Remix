use super::*;

unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    if is_excute(agent) {
        let is_charge_max = 1.0 <= WorkModule::get_float(boma, *WEAPON_ROCKMAN_CHARGESHOT_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
        let damage;
        let bkb;
        let kbg;
        if is_charge_max {
            damage = 15.0;
            bkb = 40;
            kbg = 90;
        }
        else {
            damage = 9.0;
            bkb = 50;
            kbg = 85;
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), damage, 361, kbg, 0, bkb, 2.6, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.32);
        AttackModule::enable_safe_pos(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", game_regular);
}
