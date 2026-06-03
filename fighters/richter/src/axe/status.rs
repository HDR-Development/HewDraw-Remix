use super::*;

pub unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let ret = smashline::original_status(Main, weapon, *WEAPON_SIMON_AXE_STATUS_KIND_FLY)(weapon);
    
    let owner_boma = weapon.get_owner_boma();
    if owner_boma.is_situation(*SITUATION_KIND_AIR) {
        // change to -42 degrees
        let lr = PostureModule::lr(owner_boma);
        KineticModule::reflect_speed(weapon.module_accessor, &Vector3f::new(0.3584, lr * 0.9336, 0.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_SIMON_AXE_STATUS_KIND_FLY, fly_main);
}