use super::*;

unsafe extern "C" fn start2_main(weapon: &mut L2CFighterCommon) -> L2CValue {
    let life = weapon.get_param_int("param_flash", "life2");
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("flash2"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(start2_main_loop as *const () as _))
}

unsafe extern "C" fn start2_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_PARRY | *COLLISION_KIND_MASK_REFLECTOR) {
        weapon.change_status(WEAPON_BRAVE_FLASH_STATUS_KIND_START2.into(), false.into());
        return 1.into();
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.change_status(WEAPON_BRAVE_FLASH_STATUS_KIND_HIT2.into(), false.into());
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_BRAVE_FLASH_STATUS_KIND_START2, start2_main);
}