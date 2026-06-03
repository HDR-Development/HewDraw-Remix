use super::*;
use globals::*;

unsafe extern "C" fn end_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.set_int(6, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("end"), 0.0, 1.0, false, 0.0, false, false);
    HitModule::set_whole(weapon.module_accessor, app::HitStatus(*HIT_STATUS_OFF), 0);
    
    weapon.fastshift(L2CValue::Ptr(end_main_loop as *const () as _))
}

unsafe extern "C" fn end_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // this doesn't actually work, I hate vtable
    if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    else {
        weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *WEAPON_KROOL_IRONBALL_STATUS_KIND_END, end_main);
}