use super::*;

unsafe extern "C" fn regular_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, weapon, *WEAPON_KIRBY_FINALCUTTERSHOT_STATUS_KIND_REGULAR)(weapon);

    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR | *COLLISION_KIND_MASK_PARRY) {
        let life = weapon.get_param_float("param_finalcutter", "life");
        weapon.set_int(*WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE, life as i32);
        let speed = weapon.get_param_float("param_finalcutter", "speed");
        weapon.set_float(speed, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLOAT_SPEED);
        let lr = weapon.lr();
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed * lr, 0.0);
        EffectModule::kill_kind(weapon.module_accessor, Hash40::new("kirby_fcut"), false, false);
        EFFECT_FOLLOW(weapon, Hash40::new("kirby_fcut"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }

    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *WEAPON_KIRBY_FINALCUTTERSHOT_STATUS_KIND_REGULAR, regular_exec);
}