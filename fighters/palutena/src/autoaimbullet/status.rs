use super::*;

unsafe extern "C" fn shot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shot"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(shot_main_loop as *const () as _))
}

unsafe extern "C" fn shot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos = *PostureModule::pos(weapon.module_accessor);
    if life <= 0 {
        EffectModule::req(weapon.module_accessor, Hash40::new("palutena_bullet_break"), &Vector3f::new(pos.x, pos.y, pos.z), &Vector3f::zero(), 1.0, 0, -1, false, 0);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 1.into();
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_crown_collision"), &Vector3f::new(pos.x, pos.y - 3.0, pos.z), &Vector3f::zero(), 0.6, 0, -1, false, 0);
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f::new(pos.x, pos.y, pos.z), &Vector3f::zero(), 0.9, 0, -1, false, 0);
        let handle = SoundModule::play_se(weapon.module_accessor, Hash40::new("se_item_pasaran_landing"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(weapon.module_accessor, handle as i32, 2.0, 0);
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_common_down_soil_ss"), true, false, false, false, app::enSEType(0));
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 1.into();
    }
    // clear whatever vanilla fx
    EFFECT_OFF_KIND(weapon, Hash40::new("palutena_bullet"), false, true);
    // tick life
    if !StopModule::is_stop(weapon.module_accessor) {
        weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_PALUTENA_AUTOAIMBULLET_STATUS_KIND_SHOT, shot_main);
}