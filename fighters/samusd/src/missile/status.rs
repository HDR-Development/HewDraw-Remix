use super::*;

unsafe extern "C" fn homing_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = weapon.get_param_int("param_missile", "h_life");
    //println!("life: {}", life);
    //println!();
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

    smashline::original_status(Init, weapon, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING)(weapon)
}


pub fn install(agent: &mut Agent) {
    agent.status(Init, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, homing_init);
}