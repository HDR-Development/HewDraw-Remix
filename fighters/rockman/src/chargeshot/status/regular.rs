use super::*;

unsafe extern "C" fn rockman_chargeshot_regular_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // Original Implementation
    // let life_min = WorkModule::get_param_int(weapon.module_accessor, hash40("param_chargeshot"), hash40("life_min"));
    // let life_max = WorkModule::get_param_int(weapon.module_accessor, hash40("param_chargeshot"), hash40("life_max"));
    // let scale_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_chargeshot"), hash40("scale_min"));
    // let scale_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_chargeshot"), hash40("scale_max"));
    // let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_chargeshot"), hash40("speed_min"));
    // let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_chargeshot"), hash40("speed_max"));
    // let hold_rate = WorkModule::get_float(weapon.module_accessor, *WEAPON_ROCKMAN_CHARGESHOT_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
    // let inverse = 1.0 - hold_rate;
    // let parent_id = LinkModule::get_parent_id(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, true) as u32;
    // let attack_mul = if BattleObjectManager::is_active_find_battle_object(singletons::BattleObjectManager(), parent_id) {
    //     let parent_boma = sv_battle_object::module_accessor(parent_id);
    //     WorkModule::get_param_float(parent_boma, hash40("attack_s4_smash_hold_attack_up"), 0)
    // }
    // else {
    //     1.0
    // };
    // let life = ((life_min as f32 * inverse) + (life_max as f32 * hold_rate)) as i32;
    // WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    // WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    // let lr = PostureModule::lr(weapon.module_accessor);
    // let speed = (speed_min * inverse) + (speed_max * hold_rate) * lr;
    // sv_kinetic_energy!(
    //     set_speed,
    //     weapon,
    //     WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
    //     speed,
    //     0.0
    // );
    // sv_kinetic_energy!(
    //     set_stable_speed,
    //     weapon,
    //     WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
    //     -1.0,
    //     -1.0
    // );
    // sv_kinetic_energy!(
    //     set_accel,
    //     weapon,
    //     WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
    //     0.0,
    //     0.0
    // );
    // let scale = (scale_min * inverse) + (scale_max * hold_rate);
    // PostureModule::set_scale(weapon.module_accessor, scale, false);
    // let attack_mul = inverse + (attack_mul * hold_rate);
    // AttackModule::set_power_mul_status(weapon.module_accessor, attack_mul);
    let is_charge_max = 1.0 <= WorkModule::get_float(weapon.module_accessor, *WEAPON_ROCKMAN_CHARGESHOT_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
    let life_param;
    let speed_param;
    let scale_param;
    if is_charge_max {
        life_param = hash40("life_max");
        speed_param = hash40("speed_max");
        scale_param = hash40("scale_max");
    }
    else {
        life_param = hash40("life_min");
        speed_param = hash40("speed_min");
        scale_param = hash40("scale_min");
    }
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_chargeshot"), life_param);
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_chargeshot"), speed_param);
    let scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_chargeshot"), scale_param);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let lr = PostureModule::lr(weapon.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed * lr,
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -1.0,
        -1.0
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        0.0
    );
    PostureModule::set_scale(weapon.module_accessor, scale, false);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(
            Init,
            *WEAPON_ROCKMAN_CHARGESHOT_STATUS_KIND_REGULAR,
            rockman_chargeshot_regular_init,
        );
}