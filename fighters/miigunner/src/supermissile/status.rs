use super::*;

unsafe extern "C" fn straight_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::detach_all(weapon.module_accessor, 5);
    println!("life: {}", weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE));
    println!("life <= 0? {}", weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0);
    println!();
    if StatusModule::status_kind_next(weapon.module_accessor) == *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_S_BURST
    && weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        VarModule::on_flag(weapon.battle_object, vars::miigunner_supermissile::instance::PULSE_DETONATE);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_STRAIGHT, straight_end);
}